// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Rewritten-query construction and response-envelope plumbing for the
//! cross-partition streaming `ORDER BY` pipeline.
//!
//! - **Request**: [`rewrite_query_body`] swaps only the `"query"` text for
//!   the Gateway's `QueryInfo::rewritten_query`, preserving `"parameters"`.
//! - **Response**: [`parse_envelope_page`] parses a backend page into
//!   strict [`EnvelopeRow`]s (envelope shape `{"_rid", "orderByItems",
//!   "payload"}`), retaining `payload` as raw JSON bytes.
//!   [`PageAggregator`] accumulates charge/diagnostics across pages, and
//!   [`PageAggregator::build_page`] reconstructs a synthetic
//!   `Documents`-array [`CosmosResponse`] — the same wire shape every
//!   other feed node returns.
//!
//! Backend continuations are never copied onto the emitted page's headers:
//! `OperationPlan::to_continuation_token` owns the client-issued token, and
//! a raw per-partition backend token would be meaningless to the caller.

use std::sync::Arc;

use serde::Deserialize;
use serde_json::value::RawValue;

use crate::diagnostics::{DiagnosticsContext, DiagnosticsContextBuilder};
use crate::models::{
    ActivityId, CosmosResponse, CosmosResponseHeaders, CosmosStatus, ResponseBody,
};
use crate::options::DiagnosticsOptions;

use super::order_by::{self, OrderByItem, ResumeParameter};

/// Placeholder emitted by the Gateway in rewritten streaming `ORDER BY`
/// queries so SDKs can inject a continuation resume filter.
const ORDER_BY_FILTER_PLACEHOLDER: &str = "{documentdb-formattableorderbyquery-filter}";

/// Produces the executable rewritten query used for a fresh range: the
/// Gateway's filter placeholder is replaced with `true` (matching
/// .NET/Java), or left as-is if the placeholder is absent.
pub(crate) fn rewritten_query_from_beginning(rewritten_query: &str) -> String {
    rewritten_query.replace(ORDER_BY_FILTER_PLACEHOLDER, "true")
}

/// Produces the executable rewritten query for a *resumed* range,
/// substituting `where_fragment` (built by
/// [`super::order_by::ResumeFilter::Exact`]) for the Gateway's resume
/// placeholder. Unlike a fresh start, the placeholder is required here — a
/// missing one means the query has no seek predicate and would silently
/// re-emit delivered rows.
///
/// # Errors
///
/// Returns [`CosmosStatus::SERVICE_ORDER_BY_REWRITTEN_QUERY_MISSING_FILTER_PLACEHOLDER`]
/// if `rewritten_query` does not contain the placeholder.
pub(crate) fn rewritten_query_with_resume_filter(
    rewritten_query: &str,
    where_fragment: &str,
) -> crate::error::Result<String> {
    if !rewritten_query.contains(ORDER_BY_FILTER_PLACEHOLDER) {
        return Err(crate::error::CosmosError::builder()
            .with_status(CosmosStatus::SERVICE_ORDER_BY_REWRITTEN_QUERY_MISSING_FILTER_PLACEHOLDER)
            .with_message(format!(
                "streaming ORDER BY rewritten query is missing the required \
                 `{ORDER_BY_FILTER_PLACEHOLDER}` placeholder needed to inject a resume filter"
            ))
            .build());
    }
    Ok(rewritten_query.replace(ORDER_BY_FILTER_PLACEHOLDER, where_fragment))
}

/// Replaces only the `"query"` field of a query operation's JSON body with
/// `rewritten_query`, preserving `"parameters"` (and any other field)
/// verbatim.
///
/// # Errors
///
/// Returns a typed error if `original_body` is missing, is not valid JSON,
/// or is not a JSON object (the query operation body is always expected to
/// be `{"query": ..., "parameters": [...]}`).
pub(crate) fn rewrite_query_body(
    original_body: Option<&[u8]>,
    rewritten_query: &str,
) -> crate::error::Result<Vec<u8>> {
    rewrite_query_body_with_parameters(original_body, rewritten_query, &[])
}

/// Like [`rewrite_query_body`], but also appends `resume_parameters` to the
/// body's `"parameters"` array (creating it if absent). Every caller field
/// and existing parameter is preserved; the resume filter's `@name`
/// bindings are added after them. Names are collision-free (see
/// [`super::order_by::ResumeFilter::build`]), so no caller binding is
/// overwritten.
///
/// # Errors
///
/// Returns a typed error if `original_body` is missing, is not valid JSON,
/// is not a JSON object, or already has a non-array `"parameters"` field.
pub(crate) fn rewrite_query_body_with_parameters(
    original_body: Option<&[u8]>,
    rewritten_query: &str,
    resume_parameters: &[ResumeParameter],
) -> crate::error::Result<Vec<u8>> {
    let original_body = original_body.ok_or_else(|| {
        body_error_msg("cannot rewrite an ORDER BY query operation with no request body")
    })?;
    let mut value: serde_json::Value = serde_json::from_slice(original_body)
        .map_err(|e| body_error("failed to parse original query body as JSON", e))?;
    let obj = value
        .as_object_mut()
        .ok_or_else(|| body_error_msg("original query body is not a JSON object"))?;
    obj.insert(
        "query".to_owned(),
        serde_json::Value::String(rewritten_query.to_owned()),
    );
    if !resume_parameters.is_empty() {
        let params = obj
            .entry("parameters")
            .or_insert_with(|| serde_json::Value::Array(Vec::new()))
            .as_array_mut()
            .ok_or_else(|| {
                body_error_msg("original query body's `parameters` field is not an array")
            })?;
        for parameter in resume_parameters {
            params.push(serde_json::json!({
                "name": parameter.name,
                "value": parameter.value,
            }));
        }
    }
    serde_json::to_vec(&value)
        .map_err(|e| body_error("failed to serialize rewritten query body", e))
}

/// Extracts the original `"query"` SQL text from a query operation's JSON
/// body, used to locally analyze the query shape (see the planner's
/// multi-row-per-document rejection).
///
/// # Errors
///
/// Returns a typed error if the body is missing, is not valid JSON, is not
/// a JSON object, or has no string `"query"` field.
pub(crate) fn query_text(original_body: Option<&[u8]>) -> crate::error::Result<String> {
    let original_body = original_body
        .ok_or_else(|| body_error_msg("cannot read query text from an operation with no body"))?;
    let value: serde_json::Value = serde_json::from_slice(original_body)
        .map_err(|e| body_error("failed to parse query body as JSON", e))?;
    value
        .get("query")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| body_error_msg("query body is missing a string `query` field"))
}

/// Returns the `@`-prefixed names of every parameter already bound in a
/// query operation's body, used to pick collision-free resume-parameter
/// names. A missing or empty `"parameters"` field yields an empty list; a
/// missing body also yields an empty list (there's nothing to collide with).
///
/// # Errors
///
/// Returns a typed error only if a present body is not valid JSON.
pub(crate) fn query_parameter_names(
    original_body: Option<&[u8]>,
) -> crate::error::Result<Vec<String>> {
    let Some(original_body) = original_body else {
        return Ok(Vec::new());
    };
    let value: serde_json::Value = serde_json::from_slice(original_body)
        .map_err(|e| body_error("failed to parse query body as JSON", e))?;
    let names = value
        .get("parameters")
        .and_then(serde_json::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| entry.get("name").and_then(serde_json::Value::as_str))
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    Ok(names)
}

/// One row parsed from a rewritten-envelope backend page, ready for the
/// merge heap. `payload` retains the item's exact original JSON bytes
/// (via [`RawValue`]) rather than a re-serialized value, so the emitted
/// item is byte-identical to what the backend returned.
#[derive(Debug)]
pub(crate) struct EnvelopeRow {
    pub(crate) keys: Vec<OrderByItem>,
    pub(crate) rid: String,
    pub(crate) payload: Box<RawValue>,
}

/// A single rewritten-envelope item, as deserialized off the wire.
#[derive(Deserialize)]
struct EnvelopeItem {
    #[serde(rename = "_rid")]
    rid: Option<String>,
    #[serde(rename = "orderByItems")]
    order_by_items: Option<serde_json::Value>,
    payload: Option<Box<RawValue>>,
}

/// The standard Cosmos feed-response wire shape:
/// `{"_rid": ..., "Documents": [...], "_count": N}`. Every item is kept as
/// an owned [`RawValue`] so its exact bytes are available for
/// [`parse_envelope_item`] without a second parse-and-copy pass.
#[derive(Deserialize)]
struct RawFeedBody {
    #[serde(alias = "Documents")]
    documents: Vec<Box<RawValue>>,
}

/// Parses a child backend page's response body into envelope rows, in wire
/// order (the backend already returned them in this partition's local
/// sort order).
///
/// # Errors
///
/// Returns a typed [`crate::error::CosmosError`] with
/// [`CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID`] for any malformed
/// envelope: a non-feed body shape, an item missing `payload` or a
/// non-empty `_rid`, or an `orderByItems` array whose length does not
/// match `order_by_column_count`.
pub(crate) fn parse_envelope_page(
    body: &ResponseBody,
    order_by_column_count: usize,
) -> crate::error::Result<Vec<EnvelopeRow>> {
    let bytes = match body {
        ResponseBody::NoPayload => return Ok(Vec::new()),
        ResponseBody::Bytes(b) => b.clone(),
        ResponseBody::Items(_) => {
            return Err(envelope_error(
                "rewritten-query backend page returned an already-split `Items` body; \
                 expected a raw `Documents`-array feed body",
            ));
        }
    };
    let feed: RawFeedBody = serde_json::from_slice(&bytes).map_err(|e| {
        body_error(
            "failed to parse rewritten-query backend page as a feed body",
            e,
        )
    })?;
    feed.documents
        .into_iter()
        .map(|raw| parse_envelope_item(&raw, order_by_column_count))
        .collect()
}

fn parse_envelope_item(
    raw: &RawValue,
    order_by_column_count: usize,
) -> crate::error::Result<EnvelopeRow> {
    let item: EnvelopeItem = serde_json::from_str(raw.get())
        .map_err(|e| body_error("failed to parse rewritten envelope item", e))?;
    let rid = item
        .rid
        .filter(|s| !s.is_empty())
        .ok_or_else(|| envelope_error("rewritten envelope item is missing a non-empty `_rid`"))?;
    let order_by_items = item
        .order_by_items
        .ok_or_else(|| envelope_error("rewritten envelope item is missing `orderByItems`"))?;
    let keys = order_by::parse_order_by_items(&order_by_items, order_by_column_count)?;
    let payload = item
        .payload
        .ok_or_else(|| envelope_error("rewritten envelope item is missing `payload`"))?;
    Ok(EnvelopeRow { keys, rid, payload })
}

/// Accumulates request charge and diagnostics across every backend page
/// consumed while assembling one emitted output page, then reconstructs a
/// single [`CosmosResponse`] carrying only the raw payload items.
pub(crate) struct PageAggregator {
    request_charge: crate::models::RequestCharge,
    diagnostics_sources: Vec<Arc<DiagnosticsContext>>,
    activity_id: Option<ActivityId>,
    status: CosmosStatus,
}

impl Default for PageAggregator {
    fn default() -> Self {
        Self {
            request_charge: crate::models::RequestCharge::default(),
            diagnostics_sources: Vec::new(),
            activity_id: None,
            status: CosmosStatus::new(azure_core::http::StatusCode::Ok),
        }
    }
}

impl PageAggregator {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Folds one consumed backend page's headers/diagnostics/status into
    /// the running aggregate. The *last* absorbed response's activity ID
    /// and status win (matching how a single multi-page fetch already
    /// surfaces "the most recent" status to callers); request charge is
    /// summed across every absorbed response.
    pub(crate) fn absorb(&mut self, response: &CosmosResponse) {
        let charge = response.headers().request_charge.unwrap_or_default();
        self.request_charge = self.request_charge + charge;
        self.diagnostics_sources.push(response.diagnostics());
        if let Some(id) = &response.headers().activity_id {
            self.activity_id = Some(id.clone());
        }
        self.status = response.status();
    }

    /// Builds the emitted page from the accumulated aggregate plus the
    /// final ordered list of raw item payloads.
    ///
    /// Body is a synthetic `{"_rid": "", "Documents": [...], "_count": N}`
    /// envelope of `payloads`' bytes unmodified — the wire shape every
    /// other feed node returns. `_rid` is left empty (per-item `_rid`
    /// identifies each item, not the feed-level one).
    ///
    /// It's valid for no backend page to have been absorbed (page
    /// assembled entirely from previously-buffered rows); it then reports
    /// zero charge and a fresh, empty [`DiagnosticsContext`].
    pub(crate) fn build_page(
        self,
        payloads: &[Box<RawValue>],
    ) -> crate::error::Result<CosmosResponse> {
        let mut body =
            Vec::with_capacity(64 + payloads.iter().map(|p| p.get().len() + 1).sum::<usize>());
        body.extend_from_slice(br#"{"_rid":"","Documents":["#);
        for (i, payload) in payloads.iter().enumerate() {
            if i > 0 {
                body.push(b',');
            }
            body.extend_from_slice(payload.get().as_bytes());
        }
        body.extend_from_slice(br#"],"_count":"#);
        body.extend_from_slice(payloads.len().to_string().as_bytes());
        body.push(b'}');

        let diagnostics = DiagnosticsContext::aggregate_sub_operations(&self.diagnostics_sources)
            .map(Arc::new)
            .unwrap_or_else(empty_diagnostics);

        let headers = CosmosResponseHeaders {
            activity_id: self.activity_id,
            request_charge: Some(self.request_charge),
            item_count: Some(payloads.len() as u32),
            // Omitted: `continuation` (owned by
            // `OperationPlan::to_continuation_token`), `session_token`/`etag`
            // (not meaningful once pages are interleaved).
            ..Default::default()
        };

        Ok(CosmosResponse::new(
            ResponseBody::from_bytes(body),
            headers,
            self.status,
            diagnostics,
        ))
    }
}

/// A fresh, empty [`DiagnosticsContext`] for a page needing no new backend
/// fetch. Uses a new activity ID since it corresponds to no real request.
fn empty_diagnostics() -> Arc<DiagnosticsContext> {
    let mut builder = DiagnosticsContextBuilder::new(
        ActivityId::new_uuid(),
        Arc::new(DiagnosticsOptions::default()),
    );
    builder.set_operation_status(azure_core::http::StatusCode::Ok, None);
    Arc::new(builder.complete())
}

fn envelope_error(message: impl Into<std::borrow::Cow<'static, str>>) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID)
        .with_message(message)
        .build()
}

fn body_error(message: &'static str, source: serde_json::Error) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID)
        .with_message(message)
        .with_source(source)
        .build()
}

fn body_error_msg(message: &'static str) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID)
        .with_message(message)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::dataflow::mocks::response;

    #[test]
    fn rewrite_query_body_replaces_query_and_preserves_parameters() {
        let original = br#"{"query":"SELECT * FROM c","parameters":[{"name":"@p","value":1}]}"#;
        let rewritten = rewrite_query_body(Some(original), "SELECT c._rid FROM c").unwrap();
        let value: serde_json::Value = serde_json::from_slice(&rewritten).unwrap();
        assert_eq!(value["query"], "SELECT c._rid FROM c");
        assert_eq!(value["parameters"][0]["name"], "@p");
        assert_eq!(value["parameters"][0]["value"], 1);
    }

    #[test]
    fn rewrite_query_body_with_parameters_appends_after_caller_parameters() {
        let original = br#"{"query":"SELECT * FROM c","parameters":[{"name":"@p","value":1}]}"#;
        let resume = [
            ResumeParameter {
                name: "@r0".to_owned(),
                value: serde_json::json!(9_007_199_254_740_993_i64),
            },
            ResumeParameter {
                name: "@r1".to_owned(),
                value: serde_json::Value::String("a' OR 1=1".to_owned()),
            },
        ];
        let rewritten =
            rewrite_query_body_with_parameters(Some(original), "SELECT c._rid FROM c", &resume)
                .unwrap();
        let value: serde_json::Value = serde_json::from_slice(&rewritten).unwrap();
        // Caller's parameter is preserved first; resume params append after.
        assert_eq!(value["parameters"][0]["name"], "@p");
        assert_eq!(value["parameters"][1]["name"], "@r0");
        // Exact integer precision survives the JSON round-trip (no f64 drift).
        assert_eq!(
            value["parameters"][1]["value"],
            serde_json::json!(9_007_199_254_740_993_i64)
        );
        assert_eq!(value["parameters"][2]["name"], "@r1");
        assert_eq!(value["parameters"][2]["value"], "a' OR 1=1");
        // The adversarial string is only in the parameter value, not the SQL.
        assert_eq!(value["query"], "SELECT c._rid FROM c");
    }

    #[test]
    fn rewrite_query_body_with_parameters_creates_parameters_array_when_absent() {
        let original = br#"{"query":"SELECT * FROM c"}"#;
        let resume = [ResumeParameter {
            name: "@r0".to_owned(),
            value: serde_json::json!(true),
        }];
        let rewritten =
            rewrite_query_body_with_parameters(Some(original), "SELECT 1", &resume).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&rewritten).unwrap();
        assert_eq!(value["parameters"][0]["name"], "@r0");
        assert_eq!(value["parameters"][0]["value"], true);
    }

    #[test]
    fn query_text_extracts_original_sql() {
        let body = br#"{"query":"SELECT * FROM c JOIN t IN c.tags","parameters":[]}"#;
        assert_eq!(
            query_text(Some(body)).unwrap(),
            "SELECT * FROM c JOIN t IN c.tags"
        );
    }

    #[test]
    fn query_text_rejects_missing_or_malformed_body() {
        assert!(query_text(None).is_err());
        assert!(query_text(Some(b"not json")).is_err());
        assert!(query_text(Some(br#"{"parameters":[]}"#)).is_err());
    }

    #[test]
    fn query_parameter_names_reads_existing_names() {
        let body = br#"{"query":"SELECT * FROM c WHERE c.a=@a","parameters":[{"name":"@a","value":1},{"name":"@b","value":2}]}"#;
        assert_eq!(
            query_parameter_names(Some(body)).unwrap(),
            vec!["@a".to_owned(), "@b".to_owned()]
        );
        // No body / no parameters field => nothing to collide with.
        assert!(query_parameter_names(None).unwrap().is_empty());
        assert!(query_parameter_names(Some(br#"{"query":"SELECT 1"}"#))
            .unwrap()
            .is_empty());
    }

    #[test]
    fn rewritten_query_from_beginning_replaces_order_by_filter_placeholder() {
        let rewritten = "SELECT * FROM c WHERE {documentdb-formattableorderbyquery-filter}";
        assert_eq!(
            rewritten_query_from_beginning(rewritten),
            "SELECT * FROM c WHERE true"
        );
    }

    #[test]
    fn rewritten_query_from_beginning_preserves_query_without_placeholder() {
        let rewritten = "SELECT * FROM c";
        assert_eq!(rewritten_query_from_beginning(rewritten), rewritten);
    }

    #[test]
    fn rewrite_query_body_rejects_missing_body() {
        let err = rewrite_query_body(None, "SELECT 1").unwrap_err();
        assert_eq!(
            err.status(),
            CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }

    #[test]
    fn rewrite_query_body_rejects_non_object_body() {
        let err = rewrite_query_body(Some(b"[1,2,3]"), "SELECT 1").unwrap_err();
        assert_eq!(
            err.status(),
            CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }

    #[test]
    fn parse_envelope_page_parses_documents_array() {
        let body = ResponseBody::from_bytes(
            br#"{"_rid":"abc","Documents":[{"_rid":"r1","orderByItems":[{"item":1}],"payload":{"id":"d1"}},{"_rid":"r2","orderByItems":[{}],"payload":{"id":"d2"}}],"_count":2}"#.to_vec(),
        );
        let rows = parse_envelope_page(&body, 1).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].rid, "r1");
        assert_eq!(rows[0].keys, vec![OrderByItem::Number(1.0.into())]);
        assert_eq!(rows[0].payload.get(), r#"{"id":"d1"}"#);
        assert_eq!(rows[1].rid, "r2");
        assert_eq!(rows[1].keys, vec![OrderByItem::Undefined]);
    }

    #[test]
    fn parse_envelope_page_empty_body_yields_no_rows() {
        let rows = parse_envelope_page(&ResponseBody::NoPayload, 1).unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn parse_envelope_page_rejects_missing_rid() {
        let body = ResponseBody::from_bytes(
            br#"{"Documents":[{"orderByItems":[{"item":1}],"payload":{"id":"d1"}}]}"#.to_vec(),
        );
        let err = parse_envelope_page(&body, 1).unwrap_err();
        assert_eq!(
            err.status(),
            CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }

    #[test]
    fn parse_envelope_page_rejects_missing_payload() {
        let body = ResponseBody::from_bytes(
            br#"{"Documents":[{"_rid":"r1","orderByItems":[{"item":1}]}]}"#.to_vec(),
        );
        let err = parse_envelope_page(&body, 1).unwrap_err();
        assert_eq!(
            err.status(),
            CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }

    #[test]
    fn parse_envelope_page_rejects_items_body_shape() {
        let body = ResponseBody::from_items(vec![]);
        let err = parse_envelope_page(&body, 1).unwrap_err();
        assert_eq!(
            err.status(),
            CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }

    #[test]
    fn rewritten_query_with_resume_filter_replaces_placeholder_in_place() {
        let rewritten = "SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload \
             FROM c WHERE {documentdb-formattableorderbyquery-filter} ORDER BY c.rank ASC";
        // The full-tie disjunct carries no `_rid` clause; the rid cut-off is
        // applied client-side (see `order_by::ResumeFilter`).
        let where_fragment = "((IS_NUMBER(c.rank) AND c.rank > 5) OR \
             (IS_NUMBER(c.rank) AND c.rank = 5))";
        let filtered = rewritten_query_with_resume_filter(rewritten, where_fragment).unwrap();
        // Placeholder gone, seek predicate spliced in place, no outer wrapper.
        assert!(
            !filtered.contains("{documentdb-formattableorderbyquery-filter}"),
            "{filtered}"
        );
        assert!(!filtered.contains("SELECT VALUE r FROM ("), "{filtered}");
        assert!(filtered.contains("c.rank > 5"), "{filtered}");
        // `c._rid` still appears once, in the envelope's `SELECT` projection
        // (needed for the client-side numeric discard) — but never as part
        // of the substituted filter itself, which carries no `_rid` clause.
        assert_eq!(
            filtered.matches("_rid").count(),
            1,
            "{filtered}: rid stays client-side, not in the filter"
        );
        assert!(filtered.contains("ORDER BY c.rank ASC"), "{filtered}");
    }

    #[test]
    fn rewritten_query_with_resume_filter_rejects_missing_placeholder() {
        let err =
            rewritten_query_with_resume_filter("SELECT * FROM c ORDER BY c.rank ASC", "(true)")
                .unwrap_err();
        assert_eq!(
            err.status(),
            CosmosStatus::SERVICE_ORDER_BY_REWRITTEN_QUERY_MISSING_FILTER_PLACEHOLDER
        );
    }

    #[test]
    fn page_aggregator_sums_charge_and_builds_documents_envelope() {
        let mut aggregator = PageAggregator::new();
        aggregator.absorb(&response(b"{}"));
        aggregator.absorb(&response(b"{}"));
        let payloads: Vec<Box<RawValue>> = vec![
            RawValue::from_string(r#"{"id":"a"}"#.to_owned()).unwrap(),
            RawValue::from_string(r#"{"id":"b"}"#.to_owned()).unwrap(),
        ];
        let page = aggregator.build_page(&payloads).unwrap();
        let body = page.body_bytes();
        let value: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(value["Documents"].as_array().unwrap().len(), 2);
        assert_eq!(value["Documents"][0]["id"], "a");
        assert_eq!(value["_count"], 2);
        assert!(page.headers().continuation.is_none());
    }

    #[test]
    fn page_aggregator_builds_empty_page_when_polled_children_yielded_no_rows() {
        // At least one page absorbed, but zero rows contributed.
        let mut aggregator = PageAggregator::new();
        aggregator.absorb(&response(b"{}"));
        let page = aggregator.build_page(&[]).unwrap();
        let value: serde_json::Value = serde_json::from_slice(page.body_bytes()).unwrap();
        assert_eq!(value["Documents"].as_array().unwrap().len(), 0);
        assert_eq!(value["_count"], 0);
    }

    #[test]
    fn page_aggregator_with_no_absorbed_response_builds_zero_charge_page() {
        // No new fetch needed; must still build a valid zero-charge page.
        let aggregator = PageAggregator::new();
        let payloads: Vec<Box<RawValue>> =
            vec![RawValue::from_string(r#"{"id":"a"}"#.to_owned()).unwrap()];
        let page = aggregator.build_page(&payloads).unwrap();
        assert_eq!(
            page.headers().request_charge,
            Some(crate::models::RequestCharge::new(0.0))
        );
        let value: serde_json::Value = serde_json::from_slice(page.body_bytes()).unwrap();
        assert_eq!(value["Documents"][0]["id"], "a");
    }
}
