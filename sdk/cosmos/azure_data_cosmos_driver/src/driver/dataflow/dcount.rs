// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Terminal distinct count over a globally deduplicated and windowed child.
//!
//! No partial count is emitted or saved in a continuation token. Before the first pull, the
//! child's continuation can be forwarded; after the result, the node is drained.

use async_trait::async_trait;
use serde_json::Value;

use crate::{
    error::{CosmosError, Result},
    models::{CosmosResponse, FeedRange},
};

use super::{
    aggregate::parse_rows, query_plan::DCountInfo, query_response::PageAggregator,
    recovery_diagnostics::RecoveryDiagnostics, PageResult, PipelineContext, PipelineNode,
    PipelineNodeState,
};

/// Counts every row from its already deduplicated child and emits one result.
pub(crate) struct DCount {
    child: Box<dyn PipelineNode>,
    alias: Option<String>,
    count: i64,
    pages: Option<PageAggregator>,
    started: bool,
    completed: bool,
    poisoned: bool,
}

impl DCount {
    /// Wraps a globally deduplicated child, using the requested output encoding.
    pub(crate) fn new(child: Box<dyn PipelineNode>, info: &DCountInfo, emit_binary: bool) -> Self {
        Self {
            child,
            alias: info.d_count_alias.clone().filter(|alias| !alias.is_empty()),
            count: 0,
            pages: Some(PageAggregator::new(emit_binary)),
            started: false,
            completed: false,
            poisoned: false,
        }
    }

    fn add_page(&mut self, response: &CosmosResponse) -> Result<()> {
        // Absorb first so parse and overflow errors retain this page's attempts.
        self.pages
            .as_mut()
            .expect("active distinct count has page metadata")
            .absorb(response)?;
        let rows = parse_rows(response.body())?;
        let increment =
            i64::try_from(rows.len()).map_err(|_| body_error("DCOUNT row count exceeds i64"))?;
        self.count = self
            .count
            .checked_add(increment)
            .ok_or_else(|| body_error("DCOUNT overflow"))?;
        Ok(())
    }

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
        if let Some(error) = self.child.take_pending_error() {
            return Err(self.fail(error));
        }
        let encoded = (|| {
            let result = match &self.alias {
                Some(alias) => Value::Object(
                    [(alias.clone(), Value::from(self.count))]
                        .into_iter()
                        .collect(),
                ),
                None => Value::from(self.count),
            };
            let raw = serde_json::value::to_raw_value(&result).map_err(|source| {
                CosmosError::builder()
                    .with_status(crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("failed to serialize DCOUNT result")
                    .with_source(source)
                    .build()
            })?;
            self.pages
                .as_ref()
                .expect("active distinct count has page metadata")
                .encode_item(0, &raw)
        })();
        let item = encoded.map_err(|error| self.fail(error))?;
        let response = self
            .pages
            .take()
            .expect("active distinct count has page metadata")
            .build_page(vec![item]);
        self.completed = true;
        Ok(PageResult::Page {
            response,
            is_terminal: true,
        })
    }
}

fn body_error(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
        .with_message(message)
        .build()
}

fn continuation_error() -> CosmosError {
    CosmosError::builder()
        .with_status(crate::error::status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED)
        .with_message(
            "DCOUNT cannot checkpoint a consumed prefix before emitting its terminal count",
        )
        .build()
}

#[async_trait]
impl PipelineNode for DCount {
    async fn next_page(&mut self, context: &mut PipelineContext<'_>) -> Result<PageResult> {
        if self.poisoned {
            return Err(body_error(
                "DCOUNT cannot continue after a consumed page or child failed",
            ));
        }
        if self.completed {
            return Ok(PageResult::Drained);
        }
        // A cancelled full-drain future must not expose a source-only checkpoint.
        self.started = true;
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
                            "DCOUNT cannot forward a split; the child fan-out must absorb it",
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
        if self.poisoned {
            return Err(continuation_error());
        }
        if self.completed {
            return Ok(PipelineNodeState::Drained);
        }
        if self.started {
            return Err(continuation_error());
        }
        let state = self.child.snapshot_state()?;
        // A drained source still owes the caller a count of zero.
        if matches!(state, PipelineNodeState::Drained) {
            return Err(continuation_error());
        }
        Ok(state)
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
    use super::DCount;
    use crate::{
        driver::dataflow::{
            aggregate::parse_rows,
            distinct::Distinct,
            distinct_hash::hash_value,
            drain::SequentialDrain,
            mocks::{
                assert_drained, gone_error_with_diagnostics, response,
                response_with_request_diagnostics, unwrap_page, MockLeaf, NoopRequestExecutor,
            },
            node::SplitReplacements,
            query_plan::{DCountInfo, DistinctType},
            skip_take::SkipTake,
            DrainedLeaf, PageResult, PipelineContext, PipelineNode, PipelineNodeState,
        },
        error::{status_codes, Result},
        models::{CosmosResponse, RequestCharge, ResponseBody, SessionToken},
    };
    use bytes::Bytes;
    use serde_json::{json, Value};

    fn info(alias: Option<&str>) -> DCountInfo {
        DCountInfo {
            d_count_alias: alias.map(str::to_owned),
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

    fn page(rows: Value, is_terminal: bool) -> Result<PageResult> {
        Ok(PageResult::Page {
            response: feed(rows, 1.0, 1, None),
            is_terminal,
        })
    }

    async fn pull(node: &mut DCount) -> Result<PageResult> {
        let mut executor = NoopRequestExecutor;
        let mut context = PipelineContext::new(&mut executor, None);
        node.next_page(&mut context).await
    }

    fn rows(response: &CosmosResponse) -> Vec<Value> {
        parse_rows(response.body()).unwrap()
    }

    #[tokio::test]
    async fn overlapping_partition_values_are_counted_after_global_distinct() {
        let first = Box::new(MockLeaf::with_pages(vec![page(
            json!(["red", "blue"]),
            true,
        )]));
        let second = Box::new(MockLeaf::with_pages(vec![page(
            json!(["red", "green"]),
            true,
        )]));
        let drain = Box::new(SequentialDrain::new(vec![first, second]));
        let distinct = Box::new(Distinct::new(drain, DistinctType::Unordered));
        let mut node = DCount::new(distinct, &info(None), false);
        let output = pull(&mut node).await.unwrap();
        assert!(matches!(
            &output,
            PageResult::Page {
                is_terminal: true,
                ..
            }
        ));
        let output = unwrap_page(Ok(output));
        assert_eq!(rows(&output), vec![json!(3)]);
        assert_eq!(output.diagnostics().request_count(), 2);
        assert_eq!(
            output.headers().request_charge,
            Some(RequestCharge::new(2.0))
        );
        assert!(matches!(
            node.snapshot_state().unwrap(),
            PipelineNodeState::Drained
        ));
        assert_drained(pull(&mut node).await);
        assert_drained(pull(&mut node).await);
    }

    #[tokio::test]
    async fn ordered_distinct_deduplicates_across_page_boundaries() {
        let child = Box::new(MockLeaf::with_pages(vec![
            page(json!(["blue", "blue", "green"]), false),
            page(json!(["green", "red", "red"]), true),
        ]));
        let distinct = Box::new(Distinct::new(child, DistinctType::Ordered));
        let mut node = DCount::new(distinct, &info(None), false);
        assert_eq!(rows(&unwrap_page(pull(&mut node).await)), vec![json!(3)]);
    }

    #[tokio::test]
    async fn counts_rows_not_numeric_partition_counts_and_respects_child_windows() {
        let child = Box::new(MockLeaf::with_pages(vec![page(
            json!([100, 100, 200, 300]),
            true,
        )]));
        let distinct = Box::new(Distinct::new(child, DistinctType::Unordered));
        let window = Box::new(SkipTake::new(distinct, 1, Some(1), false));
        let mut node = DCount::new(window, &info(None), false);
        assert_eq!(rows(&unwrap_page(pull(&mut node).await)), vec![json!(1)]);
    }

    #[tokio::test]
    async fn empty_input_and_alias_shapes_match_peer_projections() {
        for alias in [None, Some(""), Some("$1"), Some("total")] {
            for empty in [true, false] {
                let child = MockLeaf::with_pages(if empty {
                    vec![Ok(PageResult::Drained)]
                } else {
                    vec![page(json!([null, {}, 11]), true)]
                });
                let mut node = DCount::new(Box::new(child), &info(alias), false);
                let output = unwrap_page(pull(&mut node).await);
                let count = if empty { 0 } else { 3 };
                let expected = match alias {
                    None | Some("") => json!(count),
                    Some(alias) => json!({(alias): count}),
                };
                assert_eq!(rows(&output), vec![expected]);
                assert_eq!(output.headers().item_count, Some(1));
                assert!(output.headers().continuation.is_none());
                if empty {
                    assert_eq!(
                        output.headers().request_charge,
                        Some(RequestCharge::default())
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn merges_metadata_from_empty_nonempty_and_no_payload_pages_once() {
        let mut final_page = feed(json!([]), 4.0, 4, Some("2:-1#3"));
        let mut final_headers = final_page.headers().clone();
        final_headers.query_metrics = Some("latest-query-metrics".to_owned());
        final_headers.index_metrics = Some("latest-index-metrics".to_owned());
        final_page = CosmosResponse::new(
            ResponseBody::NoPayload,
            final_headers,
            final_page.status(),
            final_page.diagnostics(),
        );
        let expected_activity = final_page.headers().activity_id.clone();
        let child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: feed(json!([]), 2.0, 2, Some("0:-1#1")),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: feed(json!(["red", "green"]), 3.0, 3, Some("1:-1#2")),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: final_page,
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let mut node = DCount::new(Box::new(child), &info(None), false);
        let output = unwrap_page(pull(&mut node).await);
        assert_eq!(rows(&output), vec![json!(2)]);
        assert_eq!(
            output.headers().request_charge,
            Some(RequestCharge::new(9.0))
        );
        assert_eq!(output.diagnostics().request_count(), 9);
        assert_eq!(output.headers().activity_id, expected_activity);
        assert_eq!(
            output.headers().query_metrics.as_deref(),
            Some("latest-query-metrics")
        );
        assert_eq!(
            output.headers().index_metrics.as_deref(),
            Some("latest-index-metrics")
        );
        let expected = SessionToken::new("0:-1#1")
            .merge(&SessionToken::new("1:-1#2"))
            .unwrap()
            .merge(&SessionToken::new("2:-1#3"))
            .unwrap();
        assert_eq!(output.headers().session_token.as_ref(), Some(&expected));
        assert!(output.headers().continuation.is_none());
    }

    #[tokio::test]
    async fn binary_and_text_feeds_and_items_obey_requested_output_encoding() {
        for emit_binary in [false, true] {
            let template = response(b"");
            let bodies = vec![
                ResponseBody::Bytes(Bytes::from_static(br#"{"Documents":[1,2]}"#)),
                ResponseBody::Bytes(
                    crate::binary_json::to_vec(&json!({"Documents": [3, 4]}))
                        .unwrap()
                        .into(),
                ),
                ResponseBody::from_items(vec![
                    Bytes::from_static(b"5"),
                    crate::binary_json::to_vec(&json!(6)).unwrap().into(),
                ]),
            ];
            let pages = bodies
                .into_iter()
                .map(|body| {
                    Ok(PageResult::Page {
                        response: CosmosResponse::new(
                            body,
                            template.headers().clone(),
                            template.status(),
                            template.diagnostics(),
                        ),
                        is_terminal: false,
                    })
                })
                .chain([Ok(PageResult::Drained)])
                .collect();
            let mut node = DCount::new(
                Box::new(MockLeaf::with_pages(pages)),
                &info(None),
                emit_binary,
            );
            let output = unwrap_page(pull(&mut node).await);
            assert_eq!(rows(&output), vec![json!(6)]);
            let ResponseBody::Items(items) = output.body() else {
                panic!("DCOUNT must emit standalone items");
            };
            assert_eq!(items.len(), 1);
            assert_eq!(crate::binary_json::is_binary(&items[0]), emit_binary);
        }
    }

    #[tokio::test]
    async fn source_checkpoint_is_forwarded_until_the_first_pull() {
        let source = PipelineNodeState::Request {
            server_continuation: Some("source-resume".to_owned()),
        };
        let child = Box::new(
            MockLeaf::with_pages(vec![page(json!(["blue", "green", "red"]), true)])
                .with_snapshot(source.clone()),
        );
        let last_hash = Some(hash_value(&json!("blue")).unwrap());
        let distinct = Box::new(Distinct::with_last_hash(
            child,
            DistinctType::Ordered,
            last_hash,
            false,
        ));
        let mut node = DCount::new(distinct, &info(None), false);
        assert_eq!(
            node.snapshot_state().unwrap(),
            PipelineNodeState::Distinct {
                distinct_type: DistinctType::Ordered,
                last_hash,
                child: Box::new(source),
            }
        );
        assert_eq!(rows(&unwrap_page(pull(&mut node).await)), vec![json!(2)]);
        let state = node.snapshot_state().unwrap();
        assert_eq!(state, PipelineNodeState::Drained);
        let mut resumed = DrainedLeaf;
        let mut executor = NoopRequestExecutor;
        let mut context = PipelineContext::new(&mut executor, None);
        assert_drained(resumed.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn unordered_distinct_snapshot_errors_propagate_before_read() {
        let child = Box::new(MockLeaf::with_pages(vec![page(json!([]), true)]));
        let distinct = Distinct::new(child, DistinctType::Unordered);
        let expected = distinct.snapshot_state().unwrap_err();
        let mut node = DCount::new(Box::new(distinct), &info(None), false);
        let error = node.snapshot_state().unwrap_err();
        assert_eq!(error.status(), expected.status());
        assert_eq!(error.to_string(), expected.to_string());
        assert_eq!(rows(&unwrap_page(pull(&mut node).await)), vec![json!(0)]);
        assert_eq!(node.snapshot_state().unwrap(), PipelineNodeState::Drained);
    }

    #[tokio::test]
    async fn drained_source_cannot_checkpoint_before_zero_result_is_emitted() {
        let child = Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Drained)]));
        let mut node = DCount::new(child, &info(None), false);
        assert!(node.snapshot_state().is_err());
        assert_eq!(rows(&unwrap_page(pull(&mut node).await)), vec![json!(0)]);
        assert_eq!(node.snapshot_state().unwrap(), PipelineNodeState::Drained);
    }

    #[tokio::test]
    async fn cancelled_drain_cannot_checkpoint_its_counted_prefix() {
        struct PausingChild(bool);

        #[async_trait::async_trait]
        impl PipelineNode for PausingChild {
            async fn next_page(&mut self, _: &mut PipelineContext<'_>) -> Result<PageResult> {
                if self.0 {
                    futures::future::pending().await
                } else {
                    self.0 = true;
                    page(json!(["red"]), false)
                }
            }

            fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
                Vec::new()
            }

            fn snapshot_state(&self) -> Result<PipelineNodeState> {
                Ok(PipelineNodeState::Request {
                    server_continuation: Some("advanced-source".to_owned()),
                })
            }

            fn topology_can_change(&self) -> bool {
                false
            }

            fn fan_out_width(&self) -> usize {
                1
            }
        }

        let mut node = DCount::new(Box::new(PausingChild(false)), &info(None), false);
        assert!(node.snapshot_state().is_ok());
        {
            let mut executor = NoopRequestExecutor;
            let mut context = PipelineContext::new(&mut executor, None);
            let next = node.next_page(&mut context);
            futures::pin_mut!(next);
            assert!(futures::poll!(next).is_pending());
        }
        assert_eq!(node.count, 1);
        assert_eq!(
            node.snapshot_state().unwrap_err().status(),
            status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED
        );
    }

    #[tokio::test]
    async fn malformed_input_keeps_prior_current_and_deferred_diagnostics() {
        for body in [
            ResponseBody::Bytes(Bytes::from_static(b"{")),
            ResponseBody::Bytes(Bytes::from_static(br#"{}"#)),
            ResponseBody::Bytes(Bytes::from_static(br#"{"Documents":null}"#)),
            ResponseBody::Bytes(Bytes::from_static(br#"{"Documents":{}}"#)),
            ResponseBody::Bytes(Bytes::from_static(br#"[]"#)),
            ResponseBody::from_items(vec![Bytes::from_static(b"{")]),
            ResponseBody::from_items(vec![Bytes::from_static(&[0x80])]),
        ] {
            let template = feed(json!([]), 2.0, 2, None);
            let child = MockLeaf::with_pages(vec![
                page(json!(["red"]), false),
                Ok(PageResult::Page {
                    response: CosmosResponse::new(
                        body,
                        template.headers().clone(),
                        template.status(),
                        template.diagnostics(),
                    ),
                    is_terminal: true,
                }),
            ])
            .with_pending_error(gone_error_with_diagnostics());
            let mut node = DCount::new(Box::new(child), &info(None), false);
            let error = pull(&mut node).await.unwrap_err();
            assert_eq!(
                error.status(),
                status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
            );
            assert_eq!(error.diagnostics().unwrap().request_count(), 4);
            assert!(node.snapshot_state().is_err());
            assert!(node.take_pending_error().is_none());
            assert!(pull(&mut node).await.unwrap_err().diagnostics().is_none());
        }
    }

    #[tokio::test]
    async fn child_errors_and_deferred_errors_preempt_terminal_success() {
        for terminal in [None, Some(false), Some(true)] {
            let next = match terminal {
                None => Err(gone_error_with_diagnostics()),
                Some(false) => Ok(PageResult::Drained),
                Some(true) => page(json!(["green"]), true),
            };
            let pages = vec![page(json!(["red"]), false), next];
            let child = if terminal.is_some() {
                MockLeaf::with_pages(pages).with_pending_error(gone_error_with_diagnostics())
            } else {
                MockLeaf::with_pages(pages)
            };
            let mut node = DCount::new(Box::new(child), &info(None), false);
            let error = pull(&mut node).await.unwrap_err();
            assert_eq!(error.status(), gone_error_with_diagnostics().status());
            assert_eq!(
                error.diagnostics().unwrap().request_count(),
                if terminal == Some(true) { 3 } else { 2 }
            );
            assert!(node.snapshot_state().is_err());
            assert!(node.take_pending_error().is_none());
            assert!(pull(&mut node).await.is_err());
        }
    }

    #[tokio::test]
    async fn session_merge_error_retains_current_prior_and_deferred_diagnostics() {
        let child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: feed(json!(["red"]), 1.0, 1, Some("0:-1#1")),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: feed(json!(["green"]), 2.0, 2, Some("malformed")),
                is_terminal: true,
            }),
        ])
        .with_pending_error(gone_error_with_diagnostics());
        let mut node = DCount::new(Box::new(child), &info(None), false);
        let error = pull(&mut node).await.unwrap_err();
        assert_eq!(error.diagnostics().unwrap().request_count(), 4);
        assert!(node.snapshot_state().is_err());
        assert!(pull(&mut node).await.is_err());
    }

    #[tokio::test]
    async fn count_overflow_is_typed_and_poisoned_without_partial_result() {
        let child = MockLeaf::with_pages(vec![page(json!([1]), true)])
            .with_pending_error(gone_error_with_diagnostics());
        let mut node = DCount::new(Box::new(child), &info(None), false);
        node.count = i64::MAX;
        let error = pull(&mut node).await.unwrap_err();
        assert_eq!(
            error.status(),
            status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
        );
        assert_eq!(error.diagnostics().unwrap().request_count(), 2);
        assert_eq!(node.count, i64::MAX);
        assert!(node.snapshot_state().is_err());
        assert!(pull(&mut node).await.is_err());
    }

    #[tokio::test]
    async fn count_at_i64_boundary_remains_an_integer_in_both_encodings() {
        for emit_binary in [false, true] {
            let child = MockLeaf::with_pages(vec![page(json!([1]), true)]);
            let mut node = DCount::new(Box::new(child), &info(None), emit_binary);
            node.count = i64::MAX - 1;
            assert_eq!(
                rows(&unwrap_page(pull(&mut node).await)),
                vec![json!(i64::MAX)]
            );
        }
    }

    #[tokio::test]
    async fn global_distinct_survives_child_split_recovery() {
        let child = SequentialDrain::new(vec![Box::new(MockLeaf::with_pages(vec![
            page(json!(["red"]), false),
            Ok(PageResult::SplitRequired {
                replacements: SplitReplacements::untiled(vec![
                    Box::new(MockLeaf::with_pages(vec![page(
                        json!(["red", "blue"]),
                        true,
                    )])),
                    Box::new(MockLeaf::with_pages(vec![page(
                        json!(["red", "green"]),
                        true,
                    )])),
                ])
                .with_diagnostics(Some(response_with_request_diagnostics(2).diagnostics())),
            }),
        ]))]);
        let distinct = Box::new(Distinct::new(Box::new(child), DistinctType::Unordered));
        let mut node = DCount::new(distinct, &info(None), false);
        let output = unwrap_page(pull(&mut node).await);
        assert_eq!(rows(&output), vec![json!(3)]);
        assert_eq!(output.diagnostics().request_count(), 5);
        assert_drained(pull(&mut node).await);
    }

    #[tokio::test]
    async fn escaped_split_keeps_consumed_and_recovery_diagnostics() {
        let recovery = gone_error_with_diagnostics().diagnostics();
        let child = MockLeaf::with_pages(vec![
            page(json!(["red"]), false),
            Ok(PageResult::SplitRequired {
                replacements: SplitReplacements::untiled(Vec::new()).with_diagnostics(recovery),
            }),
        ])
        .with_pending_error(gone_error_with_diagnostics());
        let mut node = DCount::new(Box::new(child), &info(None), false);
        let error = pull(&mut node).await.unwrap_err();
        assert_eq!(
            error.status(),
            status_codes::CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT
        );
        assert_eq!(error.diagnostics().unwrap().request_count(), 3);
        assert!(node.snapshot_state().is_err());
        assert!(pull(&mut node).await.is_err());
    }
}
