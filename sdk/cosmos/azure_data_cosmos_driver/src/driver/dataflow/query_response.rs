// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Rewritten-query construction and response-envelope plumbing for the
//! cross-partition streaming `ORDER BY` pipeline.
//!
//! - **Request**: [`rewrite_query_body`] swaps only the `"query"` text for
//!   the Gateway's `QueryInfo::rewritten_query` (its resume-filter
//!   placeholder replaced with `true`), preserving `"parameters"`. On
//!   resume, [`with_resume_filter`] additionally inserts the .NET-compatible
//!   structured `"resumeFilter"` field — never rewriting the SQL or the
//!   caller's parameters.
//! - **Response**: [`parse_envelope_page`] parses a backend page into
//!   strict [`EnvelopeRow`]s (envelope shape `{"_rid", "orderByItems",
//!   "payload"}`), retaining `payload` as raw JSON bytes.
//!   [`PageAggregator`] accumulates charge/diagnostics across pages, and
//!   [`PageAggregator::build_page`] emits the ordered payloads as a pre-split
//!   [`ResponseBody::Items`](crate::models::ResponseBody::Items) body, so the
//!   calling SDK reads each document directly without re-parsing an envelope.
//!
//! Backend continuations are never copied onto the emitted page's headers:
//! `OperationPlan::to_continuation_token` owns the client-issued token, and
//! a raw per-partition backend token would be meaningless to the caller.

use std::sync::Arc;

use serde::Deserialize;
use serde_json::value::RawValue;

use crate::diagnostics::{DiagnosticsContext, DiagnosticsContextBuilder};
use crate::models::{
    ActivityId, CosmosResponse, CosmosResponseHeaders, CosmosStatus, ResponseBody, SessionToken,
};
use crate::options::DiagnosticsOptions;

use super::order_by::{self, OrderByItem, OrderByResumeValue};
use super::query_plan::SortOrder;

/// Placeholder emitted by the Gateway in rewritten streaming `ORDER BY`
/// queries so SDKs can inject a continuation resume filter.
const ORDER_BY_FILTER_PLACEHOLDER: &str = "{documentdb-formattableorderbyquery-filter}";

/// Produces the executable rewritten query used for a fresh range by replacing
/// every syntactic occurrence of the Gateway's filter placeholder with `true`,
/// never matching inside a SQL string literal, quoted identifier, or comment.
/// String literals honor Cosmos NoSQL's JSON-style backslash escapes (and
/// tolerate doubled quotes) so an escaped quote cannot desynchronize the scan.
pub(crate) fn rewritten_query_from_beginning(
    rewritten_query: &str,
) -> crate::error::Result<String> {
    let bytes = rewritten_query.as_bytes();
    let placeholder = ORDER_BY_FILTER_PLACEHOLDER.as_bytes();
    let mut offsets = Vec::new();
    let mut index = 0;
    let mut state = SqlScanState::Normal;

    while index < bytes.len() {
        match state {
            SqlScanState::Normal => {
                if bytes[index..].starts_with(placeholder) {
                    offsets.push(index);
                    index += placeholder.len();
                } else if bytes[index] == b'\'' {
                    state = SqlScanState::SingleQuoted;
                    index += 1;
                } else if bytes[index] == b'"' {
                    state = SqlScanState::DoubleQuoted;
                    index += 1;
                } else if bytes[index..].starts_with(b"--") {
                    state = SqlScanState::LineComment;
                    index += 2;
                } else if bytes[index..].starts_with(b"/*") {
                    state = SqlScanState::BlockComment;
                    index += 2;
                } else {
                    index += 1;
                }
            }
            // Cosmos NoSQL string literals use JSON-style backslash escapes
            // (`\'`, `\"`, `\\`), so a backslash always consumes the next
            // byte. Doubled quotes are also tolerated so the scanner stays
            // in sync with dialects (and this repo's own lexer) that use
            // them; both forms leave the scanner in the same state.
            SqlScanState::SingleQuoted => {
                if bytes[index] == b'\\' {
                    index += 2;
                } else if bytes[index] == b'\'' {
                    if bytes.get(index + 1) == Some(&b'\'') {
                        index += 2;
                    } else {
                        state = SqlScanState::Normal;
                        index += 1;
                    }
                } else {
                    index += 1;
                }
            }
            SqlScanState::DoubleQuoted => {
                if bytes[index] == b'\\' {
                    index += 2;
                } else if bytes[index] == b'"' {
                    if bytes.get(index + 1) == Some(&b'"') {
                        index += 2;
                    } else {
                        state = SqlScanState::Normal;
                        index += 1;
                    }
                } else {
                    index += 1;
                }
            }
            SqlScanState::LineComment => {
                if bytes[index] == b'\n' {
                    state = SqlScanState::Normal;
                }
                index += 1;
            }
            SqlScanState::BlockComment => {
                if bytes[index..].starts_with(b"*/") {
                    state = SqlScanState::Normal;
                    index += 2;
                } else {
                    index += 1;
                }
            }
        }
    }

    if offsets.is_empty() {
        return Ok(rewritten_query.to_owned());
    }
    let removed_per_placeholder = ORDER_BY_FILTER_PLACEHOLDER.len() - "true".len();
    let mut result =
        String::with_capacity(rewritten_query.len() - offsets.len() * removed_per_placeholder);
    let mut copied_through = 0;
    for offset in offsets {
        result.push_str(&rewritten_query[copied_through..offset]);
        result.push_str("true");
        copied_through = offset + ORDER_BY_FILTER_PLACEHOLDER.len();
    }
    result.push_str(&rewritten_query[copied_through..]);
    Ok(result)
}

#[derive(Clone, Copy)]
enum SqlScanState {
    Normal,
    SingleQuoted,
    DoubleQuoted,
    LineComment,
    BlockComment,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct QueryExecutionInfo {
    #[serde(default)]
    reverse_rid_enabled: bool,
    #[serde(default)]
    reverse_index_scan: bool,
}

/// Derives the backend's RID ordering for one resumed page. Modern backends
/// supply their index-scan direction; absent/legacy metadata falls back to the
/// first ORDER BY column, matching .NET and Java.
pub(crate) fn effective_rid_direction(
    headers: &CosmosResponseHeaders,
    first_direction: SortOrder,
) -> crate::error::Result<SortOrder> {
    let Some(raw) = headers.query_execution_info.as_deref() else {
        return Ok(first_direction);
    };
    let info: QueryExecutionInfo = serde_json::from_str(raw)
        .map_err(|source| body_error("failed to parse x-ms-cosmos-query-execution-info", source))?;
    if info.reverse_rid_enabled {
        Ok(first_direction)
    } else if info.reverse_index_scan {
        Ok(SortOrder::Descending)
    } else {
        Ok(SortOrder::Ascending)
    }
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
    let mut value = parse_query_body(original_body)?;
    let obj = value
        .as_object_mut()
        .ok_or_else(|| body_error_msg("original query body is not a JSON object"))?;
    obj.insert(
        "query".to_owned(),
        serde_json::Value::String(rewritten_query.to_owned()),
    );
    serde_json::to_vec(&value)
        .map_err(|e| body_error("failed to serialize rewritten query body", e))
}

/// Inserts the .NET-compatible structured `"resumeFilter"` field into an
/// already-rewritten query `body` (its `"query"` text already
/// placeholder-substituted with `true` by [`rewrite_query_body`]),
/// preserving `"query"`, the caller's `"parameters"`, and every other field
/// verbatim. Mirrors `SqlQuerySpec.resumeFilter`: the backend seeks to the
/// resume point with no SDK-side SQL rewriting and no generated parameters.
///
/// Rust persists one boundary per range and resumes each with `rid` present
/// and `exclude:false` (the .NET "target" partition style); the
/// already-emitted prefix of the boundary tie run is trimmed client-side
/// (see [`super::order_by::classify_row_vs_boundary`]).
///
/// # Errors
///
/// Returns a typed error if `body` is missing, is not valid JSON, or is not
/// a JSON object.
pub(crate) fn with_resume_filter(
    body: Option<&[u8]>,
    resume_values: &[OrderByResumeValue],
    rid: Option<&str>,
    exclude: bool,
) -> crate::error::Result<Vec<u8>> {
    let mut value = parse_query_body(body)?;
    let obj = value
        .as_object_mut()
        .ok_or_else(|| body_error_msg("query body is not a JSON object"))?;
    obj.insert(
        "resumeFilter".to_owned(),
        order_by::resume_filter_json(resume_values, rid, exclude),
    );
    serde_json::to_vec(&value)
        .map_err(|e| body_error("failed to serialize query body with resume filter", e))
}

/// Parses a query operation's JSON body, erroring on a missing body or
/// invalid JSON.
fn parse_query_body(body: Option<&[u8]>) -> crate::error::Result<serde_json::Value> {
    let body = body.ok_or_else(|| {
        body_error_msg("cannot rewrite an ORDER BY query operation with no request body")
    })?;
    serde_json::from_slice(body)
        .map_err(|e| body_error("failed to parse original query body as JSON", e))
}

/// One row parsed from a rewritten-envelope backend page, ready for the
/// merge heap. On the text path, `payload` retains the item's exact original
/// JSON bytes (via [`RawValue`]) rather than a re-serialized value, so the
/// emitted item is byte-identical to what the backend returned. On the
/// binary-negotiated path the page is transcoded to text before parsing, so
/// `payload` reflects that canonicalized text (normalized key order, collapsed
/// duplicate keys, canonical number formatting) rather than the raw binary
/// bytes.
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
    payload: Box<RawValue>,
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
    // The envelope parse below is text-only, so decode binary pages first.
    let bytes = normalize_page_body(&bytes)?;
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
    Ok(EnvelopeRow {
        keys,
        rid,
        payload: item.payload,
    })
}

/// Peak-memory bound on per-page diagnostics contexts retained by a single
/// [`PageAggregator`]. A selective ORDER BY can follow many empty-but-continuing
/// backend pages before one output page is emitted, so the retained set is
/// folded at this size instead of only at `build_page`.
const MAX_RETAINED_DIAGNOSTICS_SOURCES: usize = 32;

/// Accumulates request charge and diagnostics across every backend page
/// consumed while assembling one emitted output page, then reconstructs a
/// single [`CosmosResponse`] carrying only the raw payload items.
pub(crate) struct PageAggregator {
    request_charge: crate::models::RequestCharge,
    diagnostics_sources: Vec<Arc<DiagnosticsContext>>,
    activity_id: Option<ActivityId>,
    session_token: Option<SessionToken>,
    index_metrics: Option<String>,
    query_metrics: Option<String>,
    status: CosmosStatus,
    /// Whether [`build_page`](Self::build_page) emits the synthetic envelope as
    /// Cosmos binary JSON. Derived from the negotiated operation by the owning
    /// node, never from the bytes of an absorbed page.
    emit_binary: bool,
}

impl PageAggregator {
    /// Creates an aggregator emitting binary when the operation hands binary
    /// back to the caller (see [`CosmosOperation::emits_binary_payload`]) — not
    /// [`negotiates_binary_response`], which is the encoding asked of the
    /// service. The two differ under `request_text_response`, where the wire
    /// stays binary but merged items must be emitted as text.
    ///
    /// [`CosmosOperation::emits_binary_payload`]: crate::models::CosmosOperation::emits_binary_payload
    /// [`negotiates_binary_response`]: crate::models::CosmosOperation::negotiates_binary_response
    pub(crate) fn new(emit_binary: bool) -> Self {
        Self {
            request_charge: crate::models::RequestCharge::default(),
            diagnostics_sources: Vec::new(),
            activity_id: None,
            session_token: None,
            index_metrics: None,
            query_metrics: None,
            status: CosmosStatus::new(azure_core::http::StatusCode::Ok),
            emit_binary,
        }
    }

    pub(crate) fn seed_session_token(&mut self, session_token: Option<SessionToken>) {
        self.session_token = session_token;
    }

    pub(crate) fn session_token(&self) -> Option<&SessionToken> {
        self.session_token.as_ref()
    }

    /// Folds one consumed backend page's headers/diagnostics/status into
    /// the running aggregate. The *last* absorbed response's activity ID
    /// and status win (matching how a single multi-page fetch already
    /// surfaces "the most recent" status to callers). Query/index metrics
    /// retain the latest non-empty service-formatted value; request charge is
    /// summed across every absorbed response.
    pub(crate) fn absorb(&mut self, response: &CosmosResponse) -> crate::error::Result<()> {
        let charge = response.headers().request_charge.unwrap_or_default();
        self.request_charge = self.request_charge + charge;
        self.diagnostics_sources.push(response.diagnostics());
        if let Some(id) = &response.headers().activity_id {
            self.activity_id = Some(id.clone());
        }
        if let Some(token) = &response.headers().session_token {
            self.session_token = Some(match &self.session_token {
                Some(current) => current.merge(token)?,
                None => token.clone(),
            });
        }
        if let Some(metrics) = response
            .headers()
            .index_metrics
            .as_ref()
            .filter(|metrics| !metrics.is_empty())
        {
            self.index_metrics = Some(metrics.clone());
        }
        if let Some(metrics) = response
            .headers()
            .query_metrics
            .as_ref()
            .filter(|metrics| !metrics.is_empty())
        {
            self.query_metrics = Some(metrics.clone());
        }
        self.status = response.status();
        if self.diagnostics_sources.len() >= MAX_RETAINED_DIAGNOSTICS_SOURCES {
            self.fold_diagnostics();
        }
        Ok(())
    }

    /// Collapses the retained per-page contexts into one, bounding peak memory
    /// while keeping attempt counts exact: a folded context reports its true
    /// total via `request_count()`, so a later fold sums originals rather than
    /// retained records.
    fn fold_diagnostics(&mut self) {
        if self.diagnostics_sources.len() < 2 {
            return;
        }
        if let Some(folded) =
            DiagnosticsContext::aggregate_sub_operations(&self.diagnostics_sources)
        {
            self.diagnostics_sources.clear();
            self.diagnostics_sources.push(Arc::new(folded));
        }
    }

    /// Converts one raw item payload into the bytes this aggregator emits.
    ///
    /// Payloads always arrive as text, because [`normalize_page_body`]
    /// transcodes every absorbed page before it is split. When the operation
    /// negotiated binary, the item is re-encoded here so the SDK decodes it
    /// through the binary deserializer — which coerces a service-echoed
    /// integral `Double` into an integer target, exactly as a passthrough
    /// binary query does. Text-negotiated queries copy verbatim.
    ///
    /// This is deliberately separate from [`build_page`](Self::build_page) so
    /// the caller can encode an item *before* committing to having emitted it.
    /// A caller that advanced its continuation state first would be unable to
    /// resume from a failure here: the row it lost is already behind the
    /// resume boundary.
    ///
    /// # Errors
    ///
    /// Returns an error if `payload` cannot be encoded to Cosmos binary JSON.
    pub(crate) fn encode_item(
        &self,
        index: usize,
        payload: &RawValue,
    ) -> crate::error::Result<bytes::Bytes> {
        let text = payload.get().as_bytes();
        if !self.emit_binary {
            return Ok(bytes::Bytes::copy_from_slice(text));
        }
        crate::binary_json::transcode_to_binary(text)
            .map(bytes::Bytes::from)
            .map_err(|e| {
                // The payload reached here as a `RawValue`, which only checks
                // that the bytes are *syntactically* JSON — it does not walk
                // the document. So this is not "the bytes were already
                // validated"; it is the narrower claim that every value the
                // binary encoder can be handed here also round-tripped through
                // `binary_json::decode`, whose limits the encoder now matches.
                // A failure therefore indicates a driver-side encoder gap, not
                // a malformed service response, and is not retriable.
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message(format!(
                        "failed to re-encode merged ORDER BY item {index} to binary: {e}"
                    ))
                    .with_source(e)
                    .build()
            })
    }

    /// Builds the emitted page from the accumulated aggregate plus the
    /// final ordered list of encoded item bytes.
    ///
    /// The body is a [`ResponseBody::Items`] whose entries are the ordered
    /// `items` — one per document, each already in the emitted encoding (see
    /// [`encode_item`](Self::encode_item)). Emitting pre-split items (rather
    /// than re-serializing a `{"Documents":[...]}` envelope only for the SDK to
    /// re-parse) lets the calling SDK read each document directly.
    ///
    /// Infallible by construction: every fallible step happens in
    /// `encode_item`, which the caller runs while it can still un-emit a row.
    ///
    /// It's valid for no backend page to have been absorbed (page
    /// assembled entirely from previously-buffered rows); it then reports
    /// zero charge and a fresh, empty [`DiagnosticsContext`].
    pub(crate) fn build_page(self, items: Vec<bytes::Bytes>) -> CosmosResponse {
        let diagnostics = DiagnosticsContext::aggregate_sub_operations(&self.diagnostics_sources)
            .map(Arc::new)
            .unwrap_or_else(empty_diagnostics);

        let headers = CosmosResponseHeaders {
            activity_id: self.activity_id,
            request_charge: Some(self.request_charge),
            session_token: self.session_token,
            item_count: Some(items.len() as u32),
            index_metrics: self.index_metrics,
            query_metrics: self.query_metrics,
            // Omitted: `continuation` (owned by
            // `OperationPlan::to_continuation_token`) and `etag` (not
            // meaningful once pages are interleaved).
            ..Default::default()
        };

        CosmosResponse::new(
            ResponseBody::from_items(items),
            headers,
            self.status,
            diagnostics,
        )
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

/// Decodes a Cosmos binary JSON feed page to text, passing a text page through
/// unchanged (allocation-free — `Bytes::clone` is a refcount bump).
///
/// The single choke point every raw-feed consumer shares, so each is
/// binary-correct by construction rather than by convention.
pub(crate) fn normalize_page_body(bytes: &bytes::Bytes) -> crate::error::Result<bytes::Bytes> {
    if !crate::binary_json::is_binary(bytes) {
        return Ok(bytes.clone());
    }
    crate::binary_json::transcode_to_text(bytes)
        .map(bytes::Bytes::from)
        .map_err(|source| {
            crate::error::CosmosError::builder()
                .with_status(CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("failed to transcode binary query page to text JSON")
                .with_source(source)
                .build()
        })
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
    use crate::driver::dataflow::mocks::{self, response};

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
    fn with_resume_filter_inserts_target_style_filter_and_preserves_caller_parameters() {
        // The caller's query text and parameters are untouched; the resume
        // point is a top-level structured `resumeFilter` (rid present,
        // exclude false), never SDK-generated SQL or parameters.
        let plain = br#"{"query":"SELECT c._rid FROM c WHERE true","parameters":[{"name":"@p","value":1}]}"#;
        let resume = [
            OrderByResumeValue::Number {
                value: 9_007_199_254_740_993_i64.into(),
            },
            OrderByResumeValue::String {
                value: "a' OR 1=1".to_owned(),
            },
        ];
        let body = with_resume_filter(Some(plain), &resume, Some("rid-1"), false).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        // Caller query and parameters are byte-for-byte preserved.
        assert_eq!(value["query"], "SELECT c._rid FROM c WHERE true");
        assert_eq!(
            value["parameters"],
            serde_json::json!([{"name": "@p", "value": 1}]),
            "caller parameters must be preserved with nothing appended"
        );
        // The resume filter is the .NET structured shape; the exact integer
        // survives the JSON round-trip and the adversarial string is a plain
        // value (never interpolated into SQL).
        assert_eq!(
            value["resumeFilter"],
            serde_json::json!({
                "value": [9_007_199_254_740_993_i64, "a' OR 1=1"],
                "rid": "rid-1",
                "exclude": false,
            })
        );
    }

    #[test]
    fn with_resume_filter_creates_no_parameters_array_and_omits_absent_rid() {
        let plain = br#"{"query":"SELECT 1"}"#;
        let body =
            with_resume_filter(Some(plain), &[OrderByResumeValue::Null], None, true).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(
            value.get("parameters").is_none(),
            "no parameters must be synthesized when the caller had none"
        );
        assert_eq!(
            value["resumeFilter"],
            serde_json::json!({"value": [null], "exclude": true})
        );
        assert!(
            value["resumeFilter"].get("rid").is_none(),
            "an absent rid must be omitted from the wire filter"
        );
    }

    #[test]
    fn rewritten_query_from_beginning_replaces_order_by_filter_placeholder() {
        let rewritten = "SELECT * FROM c WHERE {documentdb-formattableorderbyquery-filter}";
        assert_eq!(
            rewritten_query_from_beginning(rewritten).unwrap(),
            "SELECT * FROM c WHERE true"
        );
    }

    #[test]
    fn rewritten_query_from_beginning_preserves_query_without_placeholder() {
        let rewritten = "SELECT * FROM c";
        assert_eq!(
            rewritten_query_from_beginning(rewritten).unwrap(),
            rewritten
        );
    }

    #[test]
    fn rewritten_query_from_beginning_preserves_placeholder_in_literals_and_comments() {
        let rewritten = "SELECT * FROM c WHERE c.value = \
            '{documentdb-formattableorderbyquery-filter}' \
            AND {documentdb-formattableorderbyquery-filter} \
            /* {documentdb-formattableorderbyquery-filter} */";
        let result = rewritten_query_from_beginning(rewritten).unwrap();

        assert!(result.contains("c.value = '{documentdb-formattableorderbyquery-filter}'"));
        assert!(result.contains("AND true"));
        assert!(result.contains("/* {documentdb-formattableorderbyquery-filter} */"));
    }

    #[test]
    fn rewritten_query_from_beginning_replaces_multiple_syntactic_placeholders() {
        let rewritten = "SELECT * FROM c WHERE \
            {documentdb-formattableorderbyquery-filter} OR \
            {documentdb-formattableorderbyquery-filter}";
        assert_eq!(
            rewritten_query_from_beginning(rewritten).unwrap(),
            "SELECT * FROM c WHERE true OR true"
        );
    }

    #[test]
    fn rewritten_query_from_beginning_handles_backslash_escaped_quotes() {
        // Cosmos NoSQL literals escape quotes with a backslash. A scanner that
        // only understands doubled quotes desyncs on `\'` and then either
        // misses the real placeholder or substitutes inside a user literal.
        for (rewritten, expected) in [
            (
                "SELECT * FROM c WHERE c.n = 'don\\'t' AND \
                 {documentdb-formattableorderbyquery-filter}",
                "SELECT * FROM c WHERE c.n = 'don\\'t' AND true",
            ),
            (
                "SELECT * FROM c WHERE c.n = 'a\\\\' AND \
                 {documentdb-formattableorderbyquery-filter}",
                "SELECT * FROM c WHERE c.n = 'a\\\\' AND true",
            ),
            (
                "SELECT * FROM c WHERE c.n = \"say \\\"hi\\\"\" AND \
                 {documentdb-formattableorderbyquery-filter}",
                "SELECT * FROM c WHERE c.n = \"say \\\"hi\\\"\" AND true",
            ),
            (
                // The placeholder text inside a backslash-escaped literal is
                // still data, not a substitution point.
                "SELECT * FROM c WHERE c.n = 'x\\'{documentdb-formattableorderbyquery-filter}' \
                 AND {documentdb-formattableorderbyquery-filter}",
                "SELECT * FROM c WHERE c.n = 'x\\'{documentdb-formattableorderbyquery-filter}' \
                 AND true",
            ),
        ] {
            assert_eq!(rewritten_query_from_beginning(rewritten).unwrap(), expected);
        }
    }

    #[test]
    fn effective_rid_direction_honors_modern_execution_metadata() {
        let headers = CosmosResponseHeaders {
            query_execution_info: Some(
                r#"{"reverseRidEnabled":false,"reverseIndexScan":true}"#.to_owned(),
            ),
            ..Default::default()
        };
        assert_eq!(
            effective_rid_direction(&headers, SortOrder::Ascending).unwrap(),
            SortOrder::Descending
        );

        let headers = CosmosResponseHeaders {
            query_execution_info: Some(
                r#"{"reverseRidEnabled":false,"reverseIndexScan":false}"#.to_owned(),
            ),
            ..Default::default()
        };
        assert_eq!(
            effective_rid_direction(&headers, SortOrder::Descending).unwrap(),
            SortOrder::Ascending
        );
    }

    #[test]
    fn effective_rid_direction_uses_legacy_first_column_fallback() {
        assert_eq!(
            effective_rid_direction(&CosmosResponseHeaders::default(), SortOrder::Descending)
                .unwrap(),
            SortOrder::Descending
        );
        let headers = CosmosResponseHeaders {
            query_execution_info: Some(
                r#"{"reverseRidEnabled":true,"reverseIndexScan":false}"#.to_owned(),
            ),
            ..Default::default()
        };
        assert_eq!(
            effective_rid_direction(&headers, SortOrder::Descending).unwrap(),
            SortOrder::Descending
        );
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
    fn parse_envelope_page_decodes_binary_envelope() {
        // A binary-encoded page must parse into the same rows as its text form.
        let text = br#"{"_rid":"abc","Documents":[{"_rid":"r1","orderByItems":[{"item":1}],"payload":{"id":"d1"}},{"_rid":"r2","orderByItems":[{}],"payload":{"id":"d2"}}],"_count":2}"#;
        let binary = crate::binary_json::transcode_to_binary(text).unwrap();
        assert!(crate::binary_json::is_binary(&binary));

        let rows = parse_envelope_page(&ResponseBody::from_bytes(binary), 1).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].rid, "r1");
        assert_eq!(rows[0].keys, vec![OrderByItem::Number(1.0.into())]);
        assert_eq!(rows[0].payload.get(), r#"{"id":"d1"}"#);
        assert_eq!(rows[1].rid, "r2");
        assert_eq!(rows[1].keys, vec![OrderByItem::Undefined]);
        assert_eq!(rows[1].payload.get(), r#"{"id":"d2"}"#);
    }

    #[test]
    fn parse_envelope_page_empty_body_yields_no_rows() {
        let rows = parse_envelope_page(&ResponseBody::NoPayload, 1).unwrap();
        assert!(rows.is_empty());
    }

    /// A boundary parsed from a binary page must render the byte-identical
    /// `resumeFilter` as one parsed from text: the service stores every number
    /// as a `Double` and renders an integral one as `5`.
    ///
    /// `OrderByItem`'s numeric equality is value-based across variants
    /// (`I64(5) == F64(5.0)`), so only the rendered bytes can catch this.
    #[test]
    fn binary_and_text_envelopes_render_identical_resume_filters() {
        let text = br#"{"_rid":"abc","Documents":[{"_rid":"r1","orderByItems":[{"item":5}],"payload":{"id":"d1"}}],"_count":1}"#;

        // Encode the key as a `Double`, as the service stores it;
        // `transcode_to_binary` would preserve the text integer marker.
        let binary = crate::binary_json::encode(&serde_json::json!({
            "_rid": "abc",
            "Documents": [{
                "_rid": "r1",
                "orderByItems": [{ "item": serde_json::Number::from_f64(5.0).unwrap() }],
                "payload": { "id": "d1" },
            }],
            "_count": 1,
        }));

        let resume_filter = |body: Vec<u8>| {
            let rows = parse_envelope_page(&ResponseBody::from_bytes(body), 1).unwrap();
            let values: Vec<_> = rows[0]
                .keys
                .iter()
                .map(|key| key.to_resume_value().expect("scalar key"))
                .collect();
            serde_json::to_string(&crate::driver::dataflow::order_by::resume_filter_json(
                &values,
                Some(&rows[0].rid),
                false,
            ))
            .unwrap()
        };

        let from_text = resume_filter(text.to_vec());
        assert_eq!(
            from_text,
            resume_filter(binary),
            "a binary-sourced resume boundary must ship the same wire bytes as text"
        );
        assert!(
            from_text.contains(r#""value":[5]"#),
            "an integral ORDER BY key must stay an integer on the wire, got {from_text}"
        );
    }

    /// A binary-negotiated merge must carry a wide integral `Double` through the
    /// **real** page path — binary envelope in, `parse_envelope_page`,
    /// `build_page` out — and emit it as binary with its value intact.
    ///
    /// Driving the whole path matters: [`normalize_page_body`] rewrites the
    /// integral `Double` to an integer literal while splitting the envelope, so
    /// a test that hands `build_page` a hand-built float payload exercises an
    /// input the pipeline can no longer produce. 2^53 is the first integer whose
    /// neighbors are not representable, so a lossy round trip shifts it.
    #[test]
    fn binary_page_round_trips_wide_integer_through_merge() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Doc {
            id: String,
            wide: u64,
        }

        const WIDE: u64 = 9_007_199_254_740_992; // 2^53

        // Encode `wide` as a `Double`, exactly as the service stores every
        // number — not via `transcode_to_binary`, which would preserve the
        // integer marker from a text literal and never exercise the coercion.
        let page = crate::binary_json::encode(&serde_json::json!({
            "_rid": "abc",
            "Documents": [{
                "_rid": "r1",
                "orderByItems": [{ "item": 1 }],
                "payload": {
                    "id": "d1",
                    "wide": serde_json::Number::from_f64(WIDE as f64).unwrap(),
                },
            }],
            "_count": 1,
        }));

        let rows = parse_envelope_page(&ResponseBody::from_bytes(page), 1).unwrap();
        let payloads: Vec<_> = rows.into_iter().map(|row| row.payload).collect();
        let aggregator = PageAggregator::new(true);
        let items = encoded_items(&aggregator, &payloads).unwrap();
        let response = aggregator.build_page(items);

        let items = match response.body() {
            ResponseBody::Items(items) => items.clone(),
            other => panic!("expected an items body, got {other:?}"),
        };
        assert_eq!(items.len(), 1);
        assert!(
            crate::binary_json::is_binary(&items[0]),
            "a binary-negotiated query must produce binary merged items"
        );

        // Exact bytes, not just an equal decode: the emitted item must be the
        // canonical binary encoding of the expected document, so a change in
        // how the merge re-encodes (key order, number marker width) is caught
        // rather than absorbed by a tolerant comparison.
        let expected = crate::binary_json::encode(&serde_json::json!({
            "id": "d1",
            "wide": WIDE,
        }));
        assert_eq!(
            items[0].as_ref(),
            expected.as_slice(),
            "merged binary item must match the canonical encoding byte for byte"
        );

        let doc: Doc = crate::binary_json::from_slice(&items[0]).unwrap();
        assert_eq!(
            doc,
            Doc {
                id: "d1".to_owned(),
                wide: WIDE,
            },
        );
    }

    /// The text deserializer rejects the float spelling of a wide integer, which
    /// is why the page path normalizes integral `Double`s rather than leaving
    /// the coercion to whichever deserializer happens to run.
    #[test]
    fn text_deserializer_rejects_float_spelling_of_wide_integer() {
        #[derive(serde::Deserialize, Debug)]
        struct Doc {
            #[allow(dead_code)]
            wide: u64,
        }

        let float_spelled = serde_json::json!({
            "wide": serde_json::Number::from_f64(9_007_199_254_740_992.0).unwrap(),
        })
        .to_string();
        assert!(
            serde_json::from_str::<Doc>(&float_spelled).is_err(),
            "a float literal must not deserialize into a u64 field"
        );
    }

    #[test]
    fn build_page_text_source_emits_text_items() {
        // A text-negotiated query keeps the item bytes verbatim — no binary
        // re-encode, so existing text ORDER BY users are unaffected.
        let payload = serde_json::value::to_raw_value(&serde_json::json!({"id":"d1"})).unwrap();
        let aggregator = PageAggregator::new(false);
        let items = encoded_items(&aggregator, &[payload]).unwrap();
        let response = aggregator.build_page(items);
        match response.body() {
            ResponseBody::Items(items) => {
                assert_eq!(items.len(), 1);
                assert!(
                    !crate::binary_json::is_binary(&items[0]),
                    "a text source must stay text"
                );
            }
            other => panic!("expected an items body, got {other:?}"),
        }
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
    fn parse_envelope_page_preserves_explicit_null_payload() {
        let body = ResponseBody::from_bytes(
            br#"{"Documents":[{"_rid":"r1","orderByItems":[{"item":1}],"payload":null}]}"#.to_vec(),
        );
        let rows = parse_envelope_page(&body, 1).unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].payload.get(), "null");
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
    fn page_aggregator_sums_charge_and_builds_items_body() {
        let mut aggregator = PageAggregator::new(false);
        aggregator.absorb(&response(b"{}")).unwrap();
        aggregator.absorb(&response(b"{}")).unwrap();
        let payloads: Vec<Box<RawValue>> = vec![
            RawValue::from_string(r#"{"id":"a"}"#.to_owned()).unwrap(),
            RawValue::from_string(r#"{"id":"b"}"#.to_owned()).unwrap(),
        ];
        let items = encoded_items(&aggregator, &payloads).unwrap();
        let page = aggregator.build_page(items);
        // The emitted body is an `Items` list of the payload bytes verbatim.
        assert_eq!(item_bytes(&page), vec![r#"{"id":"a"}"#, r#"{"id":"b"}"#]);
        assert_eq!(page.headers().item_count, Some(2));
        assert!(page.headers().continuation.is_none());
    }

    #[test]
    fn page_aggregator_merges_session_tokens() {
        fn response_with_token(token: &'static str) -> CosmosResponse {
            let mut headers = CosmosResponseHeaders::default();
            headers.session_token = Some(SessionToken::new(token));
            CosmosResponse::new(
                ResponseBody::NoPayload,
                headers,
                CosmosStatus::new(azure_core::http::StatusCode::Ok),
                empty_diagnostics(),
            )
        }

        let mut aggregator = PageAggregator::new(false);
        aggregator.absorb(&response_with_token("0:1#10")).unwrap();
        aggregator.absorb(&response_with_token("1:1#20")).unwrap();
        let page = aggregator.build_page(Vec::new());

        assert_eq!(
            page.headers()
                .session_token
                .as_ref()
                .map(SessionToken::as_str),
            Some("0:1#10,1:1#20")
        );
    }

    /// A selective ORDER BY can consume many empty-but-continuing backend pages
    /// before emitting one output page. Retaining a full context per page would
    /// grow with page count, so the aggregator folds them periodically.
    #[test]
    fn page_aggregator_folds_diagnostics_to_bound_retained_sources() {
        let mut aggregator = PageAggregator::new(false);
        for _ in 0..(MAX_RETAINED_DIAGNOSTICS_SOURCES * 4) {
            aggregator
                .absorb(&mocks::response_with_request_diagnostics(1))
                .unwrap();
            assert!(
                aggregator.diagnostics_sources.len() <= MAX_RETAINED_DIAGNOSTICS_SOURCES,
                "retained contexts must stay bounded regardless of pages consumed, got {}",
                aggregator.diagnostics_sources.len()
            );
        }
    }

    /// Folding must not lose attempts: a folded context reports its true total
    /// through `request_count()`, so the counts stay exact across any number of
    /// folds even once the retained records are capped.
    #[test]
    fn page_aggregator_folding_preserves_exact_request_count() {
        // Enough attempts to exceed the default 512-record cap, so the fold
        // path is exercised against real compaction rather than a no-op.
        const PAGES: usize = MAX_RETAINED_DIAGNOSTICS_SOURCES * 8;
        const REQUESTS_PER_PAGE: usize = 3;

        let mut aggregator = PageAggregator::new(false);
        let mut folded_before_build = false;
        for _ in 0..PAGES {
            let before = aggregator.diagnostics_sources.len();
            aggregator
                .absorb(&mocks::response_with_request_diagnostics(REQUESTS_PER_PAGE))
                .unwrap();
            // An absorb normally grows the vec by one, so a shrink means the
            // fold ran.
            folded_before_build |= aggregator.diagnostics_sources.len() < before;
        }
        let page = aggregator.build_page(Vec::new());

        assert!(
            folded_before_build,
            "the fold must engage, or this test would only cover the terminal fold"
        );
        let diagnostics = page.diagnostics();
        assert!(
            diagnostics.retained_request_count() < PAGES * REQUESTS_PER_PAGE,
            "the cap must actually engage, or this test would not exercise count preservation"
        );
        assert_eq!(
            diagnostics.request_count(),
            PAGES * REQUESTS_PER_PAGE,
            "every attempt must still be counted after incremental folding"
        );
    }

    #[test]
    fn page_aggregator_preserves_latest_non_empty_metrics() {
        fn response_with_metrics(
            index_metrics: Option<&str>,
            query_metrics: Option<&str>,
        ) -> CosmosResponse {
            let headers = CosmosResponseHeaders {
                index_metrics: index_metrics.map(str::to_owned),
                query_metrics: query_metrics.map(str::to_owned),
                ..Default::default()
            };
            CosmosResponse::new(
                ResponseBody::NoPayload,
                headers,
                CosmosStatus::new(azure_core::http::StatusCode::Ok),
                empty_diagnostics(),
            )
        }

        let mut aggregator = PageAggregator::new(false);
        aggregator
            .absorb(&response_with_metrics(
                Some(r#"{"UtilizedSingleIndexes":["first"]}"#),
                Some("retrievedDocumentCount=1"),
            ))
            .unwrap();
        aggregator
            .absorb(&response_with_metrics(Some(""), Some("")))
            .unwrap();
        let page = aggregator.build_page(Vec::new());

        assert_eq!(
            page.headers().index_metrics.as_deref(),
            Some(r#"{"UtilizedSingleIndexes":["first"]}"#)
        );
        assert_eq!(
            page.headers().query_metrics.as_deref(),
            Some("retrievedDocumentCount=1")
        );
    }

    /// Encodes every payload through the aggregator, mirroring the loop in
    /// `StreamingOrderedMerge`. Tests go through the same seam production uses
    /// so the encode step is never silently skipped.
    fn encoded_items(
        aggregator: &PageAggregator,
        payloads: &[Box<RawValue>],
    ) -> crate::error::Result<Vec<bytes::Bytes>> {
        payloads
            .iter()
            .enumerate()
            .map(|(index, payload)| aggregator.encode_item(index, payload))
            .collect()
    }

    /// The exact per-item bytes of a feed page's `Items` body, so tests assert
    /// the emitted output directly rather than a re-parsed shape.
    fn item_bytes(page: &CosmosResponse) -> Vec<String> {
        match page.body() {
            ResponseBody::Items(items) => items
                .iter()
                .map(|b| String::from_utf8(b.to_vec()).unwrap())
                .collect(),
            ResponseBody::NoPayload => Vec::new(),
            ResponseBody::Bytes(_) => panic!("expected an Items feed body"),
        }
    }

    #[test]
    fn page_aggregator_builds_empty_page_when_polled_children_yielded_no_rows() {
        // At least one page absorbed, but zero rows contributed.
        let mut aggregator = PageAggregator::new(false);
        aggregator.absorb(&response(b"{}")).unwrap();
        let page = aggregator.build_page(Vec::new());
        assert!(item_bytes(&page).is_empty());
        assert_eq!(page.headers().item_count, Some(0));
    }

    #[test]
    fn page_aggregator_with_no_absorbed_response_builds_zero_charge_page() {
        // No new fetch needed; must still build a valid zero-charge page.
        let aggregator = PageAggregator::new(false);
        let payloads: Vec<Box<RawValue>> =
            vec![RawValue::from_string(r#"{"id":"a"}"#.to_owned()).unwrap()];
        let items = encoded_items(&aggregator, &payloads).unwrap();
        let page = aggregator.build_page(items);
        assert_eq!(
            page.headers().request_charge,
            Some(crate::models::RequestCharge::new(0.0))
        );
        // The emitted body is the payload bytes verbatim, one item per document.
        assert_eq!(item_bytes(&page), vec![r#"{"id":"a"}"#]);
        assert_eq!(page.headers().item_count, Some(1));
    }
}
