// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Full-drain aggregation over rewritten per-partition aggregate projections.
//!
//! The child fan-out owns split recovery. No partial aggregate is emitted or
//! serialized into a continuation token; the single terminal page contains the
//! completed projection and metadata from every consumed page.

use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::{value::RawValue, Value};

use crate::{
    error::{CosmosError, Result},
    models::{FeedRange, ResponseBody},
};

use super::{
    aggregate_values::SingleGroupAggregator,
    query_plan::QueryInfo,
    query_response::{normalize_page_body, PageAggregator},
    recovery_diagnostics::RecoveryDiagnostics,
    skip_take_page, PageResult, PipelineContext, PipelineNode, PipelineNodeState,
};

/// Combines all partition-local aggregates before emitting one terminal page.
pub(crate) struct Aggregate {
    child: Box<dyn PipelineNode>,
    values: SingleGroupAggregator,
    pages: Option<PageAggregator>,
    has_select_value: bool,
    completed: bool,
    poisoned: bool,
}

impl Aggregate {
    pub(crate) fn new(
        child: Box<dyn PipelineNode>,
        info: &QueryInfo,
        emit_binary: bool,
    ) -> Result<Self> {
        Ok(Self {
            child,
            values: SingleGroupAggregator::new(info)?,
            pages: Some(PageAggregator::new(emit_binary)),
            has_select_value: info.has_select_value,
            completed: false,
            poisoned: false,
        })
    }

    fn add_page(&mut self, response: &crate::models::CosmosResponse) -> Result<()> {
        // Absorb first so a malformed row still reports the current page's
        // attempts, as well as all preceding pages' attempts.
        self.pages
            .as_mut()
            .expect("active aggregate has page metadata")
            .absorb(response)?;
        for row in parse_rows(response.body())? {
            // Mirrors RewrittenAggregateProjections in .NET: SELECT VALUE
            // returns an array whose first entry is an AggregateItem cell;
            // SELECT lists put the alias-to-cell projection under payload.
            let payload = if self.has_select_value {
                row.as_array()
                    .and_then(|items| items.first())
                    .ok_or_else(|| {
                        body_error("SELECT VALUE aggregate row must be a nonempty array")
                    })?
            } else {
                row.as_object()
                    .and_then(|object| object.get("payload"))
                    .filter(|payload| payload.is_object())
                    .ok_or_else(|| {
                        body_error("aggregate SELECT-list row must contain a payload object")
                    })?
            };
            self.values.add(Some(payload))?;
        }
        Ok(())
    }

    /// Makes consumed state unusable and transfers every outstanding diagnostic.
    fn fail(&mut self, error: CosmosError) -> CosmosError {
        self.poisoned = true;
        let mut pending = RecoveryDiagnostics::default();
        if let Some(error) = self.child.take_pending_error() {
            pending.absorb(error.diagnostics());
        }
        let error = pending.attach_error(error);
        match self.pages.take() {
            Some(pages) => pages.attach_error(error),
            None => error,
        }
    }

    fn finish(&mut self) -> Result<PageResult> {
        // A terminal page can precede a deferred error. Do not turn that
        // handoff into a successful (but incomplete) aggregate.
        if let Some(error) = self.child.take_pending_error() {
            return Err(self.fail(error));
        }
        let encoded = (|| {
            let Some(result) = self.values.result()? else {
                return Ok(Vec::new());
            };
            let raw = serde_json::value::to_raw_value(&result)
                .map_err(|error| parse_error("failed to serialize aggregate result", error))?;
            let item = self
                .pages
                .as_ref()
                .expect("active aggregate has page metadata")
                .encode_item(0, &raw)?;
            Ok(vec![item])
        })();
        let items = encoded.map_err(|error| self.fail(error))?;
        let response = self
            .pages
            .take()
            .expect("active aggregate has page metadata")
            .build_page(items);
        self.completed = true;
        Ok(PageResult::Page {
            response,
            is_terminal: true,
        })
    }
}

/// Parses feed envelopes or standalone items in either supported encoding.
///
/// Unlike the general trimming helper, a raw feed envelope must explicitly
/// contain `Documents`; a missing field is not an empty aggregate input.
pub(crate) fn parse_rows(body: &ResponseBody) -> Result<Vec<Value>> {
    match body {
        ResponseBody::NoPayload => Ok(Vec::new()),
        ResponseBody::Items(items) => items.iter().map(|item| parse_row(item)).collect(),
        ResponseBody::Bytes(bytes) => {
            let text = normalize_page_body(bytes)?;
            // Borrow values instead of materializing the whole page twice.
            // Deserializing a map also rejects arrays and scalar envelopes.
            let envelope: HashMap<String, &RawValue> = serde_json::from_slice(&text)
                .map_err(|error| parse_error("invalid aggregate feed envelope", error))?;
            if !envelope.contains_key("Documents") {
                return Err(body_error("aggregate feed envelope is missing Documents"));
            }
            skip_take_page::split_feed_envelope(&text)?
                .iter()
                .map(|item| parse_row(item))
                .collect()
        }
    }
}

fn parse_row(bytes: &[u8]) -> Result<Value> {
    if crate::binary_json::is_binary(bytes) {
        let mut value = crate::binary_json::decode(bytes)
            .map_err(|error| parse_error("invalid binary aggregate row", error))?;
        crate::binary_json::normalize_integral_floats(&mut value);
        Ok(value)
    } else {
        serde_json::from_slice(bytes)
            .map_err(|error| parse_error("invalid aggregate row JSON", error))
    }
}

fn body_error(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
        .with_message(message)
        .build()
}

fn parse_error(
    message: &'static str,
    source: impl std::error::Error + Send + Sync + 'static,
) -> CosmosError {
    CosmosError::builder()
        .with_status(crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
        .with_message(message)
        .with_source(source)
        .build()
}

#[async_trait]
impl PipelineNode for Aggregate {
    async fn next_page(&mut self, context: &mut PipelineContext<'_>) -> Result<PageResult> {
        if self.poisoned {
            return Err(body_error(
                "aggregate node cannot continue after a consumed page or child failed",
            ));
        }
        if self.completed {
            return Ok(PageResult::Drained);
        }
        loop {
            let next = self
                .child
                .next_page(context)
                .await
                .map_err(|error| self.fail(error))?;
            match next {
                PageResult::Drained => return self.finish(),
                PageResult::Page {
                    response,
                    is_terminal,
                } => {
                    self.add_page(&response).map_err(|error| self.fail(error))?;
                    if is_terminal {
                        return self.finish();
                    }
                }
                PageResult::SplitRequired { mut replacements } => {
                    let mut recovery = RecoveryDiagnostics::default();
                    recovery.absorb(replacements.take_diagnostics());
                    let error = CosmosError::builder()
                        .with_status(
                            crate::error::status_codes::CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT,
                        )
                        .with_message(
                            "aggregate cannot forward a split; the child fan-out must absorb it",
                        )
                        .build();
                    return Err(self.fail(recovery.attach_error(error)));
                }
            }
        }
    }

    fn take_pending_error(&mut self) -> Option<CosmosError> {
        self.child
            .take_pending_error()
            .map(|error| self.fail(error))
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        vec![self.child]
    }

    fn snapshot_state(&self) -> Result<PipelineNodeState> {
        if self.completed {
            return Ok(PipelineNodeState::Drained);
        }
        Err(CosmosError::builder()
            .with_status(crate::error::status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED)
            .with_message(
                "aggregate continuation is unavailable until the complete aggregate is emitted",
            )
            .build())
    }

    fn feed_range(&self) -> Option<&FeedRange> {
        self.child.feed_range()
    }

    fn fan_out_width(&self) -> usize {
        self.child.fan_out_width()
    }

    fn topology_can_change(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_rows, Aggregate};
    use crate::{
        driver::dataflow::{
            drain::SequentialDrain,
            mocks::{
                assert_drained, gone_error_with_diagnostics, response,
                response_with_request_diagnostics, unwrap_page, MockLeaf, NoopRequestExecutor,
                NoopTopologyProvider,
            },
            node::SplitReplacements,
            query_plan::QueryInfo,
            PageResult, PipelineContext, PipelineNode, PipelineNodeState,
        },
        error::{CosmosError, Result},
        models::{CosmosResponse, RequestCharge, ResponseBody, SessionToken},
    };
    use bytes::Bytes;
    use serde_json::{json, Value};

    fn info(operator: &str, select_value: bool) -> QueryInfo {
        if select_value {
            QueryInfo {
                has_select_value: true,
                aggregates: vec![operator.to_owned()],
                ..Default::default()
            }
        } else {
            QueryInfo {
                group_by_aliases: vec!["total".to_owned()],
                group_by_alias_to_aggregate_type: [("total".to_owned(), json!(operator))].into(),
                ..Default::default()
            }
        }
    }

    fn feed(rows: Value, charge: f64, attempts: usize, token: Option<&str>) -> CosmosResponse {
        let template = response_with_request_diagnostics(attempts);
        let mut headers = template.headers().clone();
        headers.request_charge = Some(RequestCharge::new(charge));
        headers.continuation = Some("backend-token-must-not-leak".to_owned());
        headers.session_token = token.map(|token| SessionToken::new(token.to_owned()));
        CosmosResponse::new(
            serde_json::to_vec(&json!({"Documents": rows})).unwrap(),
            headers,
            template.status(),
            template.diagnostics(),
        )
    }

    fn page(rows: Value, terminal: bool) -> Result<PageResult> {
        Ok(PageResult::Page {
            response: feed(rows, 1.0, 1, None),
            is_terminal: terminal,
        })
    }

    async fn pull(node: &mut Aggregate) -> Result<PageResult> {
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        node.next_page(&mut context).await
    }

    fn result_items(response: &CosmosResponse) -> Vec<Value> {
        parse_rows(response.body()).unwrap()
    }

    #[tokio::test]
    async fn count_drains_pages_and_merges_metadata() {
        let child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: feed(json!([[{"item": 2}]]), 2.5, 2, Some("0:-1#1")),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: feed(json!([[{"item": 3}]]), 3.5, 3, Some("1:-1#2")),
                is_terminal: true,
            }),
        ]);
        let mut node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
        let output = pull(&mut node).await.unwrap();
        assert!(matches!(
            &output,
            PageResult::Page {
                is_terminal: true,
                ..
            }
        ));
        let output = unwrap_page(Ok(output));
        assert_eq!(result_items(&output), vec![json!(5)]);
        assert_eq!(
            output.headers().request_charge,
            Some(RequestCharge::new(6.0))
        );
        assert_eq!(output.headers().item_count, Some(1));
        assert!(output.headers().continuation.is_none());
        assert_eq!(output.diagnostics().request_count(), 5);
        let expected = SessionToken::new("0:-1#1")
            .merge(&SessionToken::new("1:-1#2"))
            .unwrap();
        assert_eq!(output.headers().session_token.as_ref(), Some(&expected));
        assert!(matches!(
            node.snapshot_state().unwrap(),
            PipelineNodeState::Drained
        ));
        assert_drained(pull(&mut node).await);
        assert_drained(pull(&mut node).await);
    }

    #[tokio::test]
    async fn weighted_average_uses_partial_sum_and_count() {
        let child = MockLeaf::with_pages(vec![
            page(json!([[{"item": {"sum": 20, "count": 2}}]]), false),
            page(json!([[{"item": {"sum": 40, "count": 1}}]]), true),
        ]);
        let mut node = Aggregate::new(Box::new(child), &info("Average", true), false).unwrap();
        assert_eq!(
            result_items(&unwrap_page(pull(&mut node).await)),
            vec![json!(20)]
        );
    }

    #[tokio::test]
    async fn aliased_projection_keeps_aggregate_cells_for_value_merger() {
        let child = MockLeaf::with_pages(vec![
            page(json!([{"payload": {"total": {"item": 2}}}]), false),
            page(json!([{"payload": {"total": [{"item2": 3}]}}]), true),
        ]);
        let mut node = Aggregate::new(Box::new(child), &info("Count", false), false).unwrap();
        assert_eq!(
            result_items(&unwrap_page(pull(&mut node).await)),
            vec![json!({"total": 5})]
        );
    }

    #[tokio::test]
    async fn empty_input_still_emits_terminal_count() {
        let child = MockLeaf::with_pages(vec![Ok(PageResult::Drained)]);
        let mut node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
        let output = unwrap_page(pull(&mut node).await);
        assert_eq!(result_items(&output), vec![json!(0)]);
        assert_eq!(
            output.headers().request_charge,
            Some(RequestCharge::default())
        );
        assert_drained(pull(&mut node).await);
    }

    #[tokio::test]
    async fn empty_average_has_no_result_document() {
        let child = MockLeaf::with_pages(vec![Ok(PageResult::Drained)]);
        let mut node = Aggregate::new(Box::new(child), &info("Average", true), false).unwrap();
        let output = unwrap_page(pull(&mut node).await);
        assert_eq!(result_items(&output), Vec::<Value>::new());
        assert_eq!(output.headers().item_count, Some(0));
    }

    #[tokio::test]
    async fn empty_continuing_pages_do_not_complete_the_aggregate() {
        let child = MockLeaf::with_pages(vec![
            page(json!([]), false),
            page(json!([[{"item": 4}]]), false),
            page(json!([]), false),
            Ok(PageResult::Drained),
        ]);
        let mut node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
        let output = unwrap_page(pull(&mut node).await);
        assert_eq!(result_items(&output), vec![json!(4)]);
        assert_eq!(output.diagnostics().request_count(), 3);
        assert_eq!(
            output.headers().request_charge,
            Some(RequestCharge::new(3.0))
        );
    }

    #[tokio::test]
    async fn no_payload_page_retains_its_charge_and_diagnostics() {
        let template = feed(json!([]), 2.0, 2, None);
        let child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: CosmosResponse::new(
                    ResponseBody::NoPayload,
                    template.headers().clone(),
                    template.status(),
                    template.diagnostics(),
                ),
                is_terminal: false,
            }),
            page(json!([[{"item": 3}]]), true),
        ]);
        let mut node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
        let output = unwrap_page(pull(&mut node).await);
        assert_eq!(result_items(&output), vec![json!(3)]);
        assert_eq!(output.diagnostics().request_count(), 3);
        assert_eq!(
            output.headers().request_charge,
            Some(RequestCharge::new(3.0))
        );
    }

    #[tokio::test]
    async fn accepts_standalone_binary_and_text_items_and_obeys_emitted_encoding() {
        for emit_binary in [false, true] {
            let template = response(b"");
            let child = MockLeaf::with_pages(vec![Ok(PageResult::Page {
                response: CosmosResponse::new(
                    ResponseBody::from_items(vec![
                        Bytes::from_static(br#"[{"item":2}]"#),
                        Bytes::from(crate::binary_json::to_vec(&json!([{"item": 3}])).unwrap()),
                    ]),
                    template.headers().clone(),
                    template.status(),
                    template.diagnostics(),
                ),
                is_terminal: true,
            })]);
            let mut node =
                Aggregate::new(Box::new(child), &info("Count", true), emit_binary).unwrap();
            let output = unwrap_page(pull(&mut node).await);
            assert_eq!(result_items(&output), vec![json!(5)]);
            let ResponseBody::Items(items) = output.body() else {
                panic!("aggregate must emit standalone items");
            };
            assert_eq!(crate::binary_json::is_binary(&items[0]), emit_binary);
        }
    }

    #[test]
    fn row_parser_requires_strict_envelopes_and_decodes_binary_feeds() {
        for malformed in [
            br#"{}"#.as_slice(),
            br#"{"documents":[]}"#,
            br#"{"Documents":null}"#,
            br#"{"Documents":{}}"#,
            br#"[]"#,
            br#"null"#,
            b"",
        ] {
            assert!(parse_rows(&ResponseBody::Bytes(Bytes::copy_from_slice(malformed))).is_err());
        }
        let rows = json!([[{"item": 2}], [{"item2": 3}]]);
        let binary = crate::binary_json::to_vec(&json!({"Documents": rows})).unwrap();
        assert_eq!(
            parse_rows(&ResponseBody::Bytes(binary.into())).unwrap(),
            rows.as_array().unwrap().clone()
        );
        assert_eq!(
            parse_rows(&ResponseBody::NoPayload).unwrap(),
            Vec::<Value>::new()
        );
    }

    #[tokio::test]
    async fn late_child_error_retains_absorbed_diagnostics_and_never_emits_partial_success() {
        let child = MockLeaf::with_pages(vec![
            page(json!([[{"item": 2}]]), false),
            Err(gone_error_with_diagnostics()),
        ]);
        let mut node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
        let error = pull(&mut node).await.unwrap_err();
        assert_eq!(error.diagnostics().unwrap().request_count(), 2);
        assert!(pull(&mut node).await.is_err());
        assert!(node.snapshot_state().is_err());
    }

    #[tokio::test]
    async fn malformed_envelopes_and_cells_keep_current_and_prior_diagnostics() {
        for (body, select_value) in [
            (json!({}), true),
            (json!({"Documents": [[]]}), true),
            (json!({"Documents": [{"payload": {"item": 2}}]}), true),
            (
                json!({"Documents": [[{"item": 1}], [{"item": "invalid-count"}]]}),
                true,
            ),
            (json!({"Documents": [{}]}), false),
            (json!({"Documents": [{"payload": null}]}), false),
        ] {
            let template = feed(json!([]), 1.0, 2, None);
            let bad = CosmosResponse::new(
                serde_json::to_vec(&body).unwrap(),
                template.headers().clone(),
                template.status(),
                template.diagnostics(),
            );
            let first = if select_value {
                json!([[{"item": 2}]])
            } else {
                json!([{"payload": {"total": {"item": 2}}}])
            };
            let child = MockLeaf::with_pages(vec![
                page(first, false),
                Ok(PageResult::Page {
                    response: bad,
                    is_terminal: true,
                }),
            ]);
            let mut node =
                Aggregate::new(Box::new(child), &info("Count", select_value), false).unwrap();
            let error = pull(&mut node).await.unwrap_err();
            assert_eq!(error.diagnostics().unwrap().request_count(), 3);
            assert!(pull(&mut node).await.is_err());
            assert!(node.snapshot_state().is_err());
        }
    }

    #[tokio::test]
    async fn deferred_errors_preempt_success_and_join_processing_errors() {
        for malformed in [false, true] {
            let child = MockLeaf::with_pages(vec![page(
                if malformed {
                    json!([[]])
                } else {
                    json!([[{"item": 2}]])
                },
                true,
            )])
            .with_pending_error(gone_error_with_diagnostics());
            let mut node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
            let error = pull(&mut node).await.unwrap_err();
            assert_eq!(error.diagnostics().unwrap().request_count(), 2);
            if malformed {
                assert_eq!(
                    error.status(),
                    crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
                );
            } else {
                assert_eq!(error.status(), gone_error_with_diagnostics().status());
            }
            assert!(node.take_pending_error().is_none());
            assert!(pull(&mut node).await.is_err());
        }
    }

    #[tokio::test]
    async fn drained_child_does_not_hide_a_deferred_error() {
        let child = MockLeaf::with_pages(vec![
            page(json!([[{"item": 2}]]), false),
            Ok(PageResult::Drained),
        ])
        .with_pending_error(gone_error_with_diagnostics());
        let mut node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
        let error = pull(&mut node).await.unwrap_err();
        assert_eq!(error.status(), gone_error_with_diagnostics().status());
        assert_eq!(error.diagnostics().unwrap().request_count(), 2);
        assert!(pull(&mut node).await.is_err());
    }

    #[tokio::test]
    async fn session_merge_error_retains_current_prior_and_deferred_diagnostics() {
        let child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: feed(json!([[{"item": 1}]]), 1.0, 1, Some("0:-1#1")),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: feed(json!([[{"item": 2}]]), 2.0, 2, Some("malformed")),
                is_terminal: true,
            }),
        ])
        .with_pending_error(gone_error_with_diagnostics());
        let mut node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
        let error = pull(&mut node).await.unwrap_err();
        assert_eq!(error.diagnostics().unwrap().request_count(), 4);
        assert!(pull(&mut node).await.is_err());
        assert!(node.snapshot_state().is_err());
    }

    #[tokio::test]
    async fn encoding_error_keeps_diagnostics_and_poisoned_state() {
        let mut deep = json!(1);
        for _ in 0..(crate::binary_json::reader::MAX_DEPTH + 8) {
            deep = Value::Array(vec![deep]);
        }
        let child = MockLeaf::with_pages(vec![page(json!([]), false), page(json!([]), true)]);
        let mut node = Aggregate::new(Box::new(child), &info("Min", true), true).unwrap();
        // Seed the reduced value directly to isolate encoding from the input
        // parser's independent nesting limit.
        node.values.add(Some(&json!({"item": deep}))).unwrap();
        let error = pull(&mut node).await.unwrap_err();
        assert_eq!(
            error.status(),
            crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
        );
        assert!(error.to_string().contains("failed to re-encode"));
        assert_eq!(error.diagnostics().unwrap().request_count(), 2);
        assert!(pull(&mut node).await.is_err());
        assert!(node.snapshot_state().is_err());
    }

    #[tokio::test]
    async fn split_fanout_is_consumed_below_the_retained_aggregate() {
        let split_diagnostics = response_with_request_diagnostics(2).diagnostics();
        let child = SequentialDrain::new(vec![Box::new(MockLeaf::with_pages(vec![
            page(json!([[{"item": 1}]]), false),
            Ok(PageResult::SplitRequired {
                replacements: SplitReplacements::untiled(vec![
                    Box::new(MockLeaf::with_pages(vec![page(
                        json!([[{"item": 2}]]),
                        true,
                    )])),
                    Box::new(MockLeaf::with_pages(vec![page(
                        json!([[{"item": 3}]]),
                        true,
                    )])),
                ])
                .with_diagnostics(Some(split_diagnostics)),
            }),
        ]))]);
        let mut node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
        let output = unwrap_page(pull(&mut node).await);
        assert_eq!(result_items(&output), vec![json!(6)]);
        assert_eq!(output.diagnostics().request_count(), 5);
        assert_drained(pull(&mut node).await);
    }

    #[tokio::test]
    async fn unexpected_split_preserves_recovery_and_pending_diagnostics() {
        let child = MockLeaf::with_pages(vec![
            page(json!([[{"item": 2}]]), false),
            Ok(PageResult::SplitRequired {
                replacements: SplitReplacements::untiled(vec![])
                    .with_diagnostics(Some(response_with_request_diagnostics(2).diagnostics())),
            }),
        ])
        .with_pending_error(gone_error_with_diagnostics());
        let mut node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
        let error = pull(&mut node).await.unwrap_err();
        assert_eq!(
            error.status(),
            crate::error::status_codes::CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT
        );
        assert_eq!(error.diagnostics().unwrap().request_count(), 4);
        assert!(pull(&mut node).await.is_err());
    }

    #[test]
    fn external_continuation_is_rejected_without_serializing_aggregate_state() {
        let child = MockLeaf::with_pages(vec![]);
        let node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
        let error = node.snapshot_state().unwrap_err();
        assert_eq!(
            error.status(),
            crate::error::status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED
        );
        assert_eq!(node.into_children().len(), 1);
    }

    #[tokio::test]
    async fn pending_error_handoff_keeps_absorbed_diagnostics() {
        let child = MockLeaf::with_pages(vec![]).with_pending_error(gone_error_with_diagnostics());
        let mut node = Aggregate::new(Box::new(child), &info("Count", true), false).unwrap();
        let error: CosmosError = node.take_pending_error().unwrap();
        assert_eq!(error.diagnostics().unwrap().request_count(), 1);
        assert!(pull(&mut node).await.is_err());
    }
}
