// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Full-drain grouping of rewritten partition projections.
//!
//! The split-aware child owns partition recovery. Group state stays in memory
//! until every partition is drained and cannot be exported in a continuation.

use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    hash::{Hash, Hasher},
};

use async_trait::async_trait;
use serde_json::Value;

use crate::{
    error::{status_codes, CosmosError, Result},
    models::{CosmosResponse, FeedRange, MaxItemCountHint, SessionToken},
};

use super::{
    aggregate::parse_rows,
    aggregate_values::SingleGroupAggregator,
    distinct_hash::{hash_value, Hash128},
    order_by::OrderByItem,
    query_plan::QueryInfo,
    query_response::PageAggregator,
    recovery_diagnostics::RecoveryDiagnostics,
    PageResult, PipelineContext, PipelineNode, PipelineNodeState,
};

const DEFAULT_PAGE_SIZE: usize = 100;

/// A structural hash accelerates lookup, but never substitutes for equality.
///
/// Keep the entire array of item wrappers: `{}` (undefined) is a different
/// grouping key from `{"item": null}`. `OrderByItem` supplies recursive,
/// representation-independent numeric and order-independent object equality.
struct GroupKey {
    hash: Hash128,
    value: OrderByItem,
}

impl PartialEq for GroupKey {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for GroupKey {}

impl Hash for GroupKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

/// Combines all partials for each group before emitting paged projections.
pub(crate) struct GroupBy {
    child: Box<dyn PipelineNode>,
    info: QueryInfo,
    groups: HashMap<GroupKey, usize>,
    values: VecDeque<SingleGroupAggregator>,
    pages: Option<PageAggregator>,
    session_token: Option<SessionToken>,
    page_size: usize,
    emit_binary: bool,
    buffering_complete: bool,
    completed: bool,
    poisoned: bool,
}

impl GroupBy {
    pub(crate) fn new(
        child: Box<dyn PipelineNode>,
        info: &QueryInfo,
        max_item_count: Option<MaxItemCountHint>,
        emit_binary: bool,
    ) -> Result<Self> {
        // Validate even when no input rows will create an actual group.
        SingleGroupAggregator::new(info)?;
        let page_size = match max_item_count {
            Some(MaxItemCountHint::Limit(value)) => value.get() as usize,
            Some(MaxItemCountHint::ServerDecides) | None => DEFAULT_PAGE_SIZE,
        };
        Ok(Self {
            child,
            info: info.clone(),
            groups: HashMap::new(),
            values: VecDeque::new(),
            pages: Some(PageAggregator::new(emit_binary)),
            session_token: None,
            page_size,
            emit_binary,
            buffering_complete: false,
            completed: false,
            poisoned: false,
        })
    }

    fn add_page(&mut self, response: &CosmosResponse) -> Result<()> {
        // Malformed rows must retain the current page's diagnostics too.
        self.pages
            .as_mut()
            .expect("buffering group-by has page metadata")
            .absorb(response)?;
        for row in parse_rows(response.body())? {
            let object = row
                .as_object()
                .ok_or_else(|| body_error("GROUP BY row must be an object"))?;
            let keys = object
                .get("groupByItems")
                .and_then(Value::as_array)
                .ok_or_else(|| body_error("GROUP BY row must contain a groupByItems array"))?;
            if !self.info.group_by_expressions.is_empty()
                && keys.len() != self.info.group_by_expressions.len()
            {
                return Err(body_error(
                    "GROUP BY key count does not match the query plan",
                ));
            }
            for key in keys {
                let wrapper = key
                    .as_object()
                    .ok_or_else(|| body_error("GROUP BY key must be an item wrapper object"))?;
                if wrapper.keys().any(|name| name != "item") {
                    return Err(body_error(
                        "GROUP BY key wrapper has an unexpected property",
                    ));
                }
            }
            let raw_keys = Value::Array(keys.clone());
            let key = GroupKey {
                hash: hash_value(&raw_keys)?,
                value: OrderByItem::from_json(&raw_keys),
            };
            let index = match self.groups.entry(key) {
                Entry::Occupied(entry) => *entry.get(),
                Entry::Vacant(entry) => {
                    let aggregator = SingleGroupAggregator::new(&self.info)?;
                    let index = self.values.len();
                    self.values.push_back(aggregator);
                    entry.insert(index);
                    index
                }
            };
            // Unlike ordinary SELECT VALUE aggregation, GROUP BY passes
            // payload directly, not the first element of an outer array.
            // A missing VALUE payload is Cosmos undefined, not JSON null.
            self.values[index].add(object.get("payload"))?;
        }
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

    fn finish_buffering(&mut self) -> Result<()> {
        if let Some(error) = self.child.take_pending_error() {
            return Err(self.fail(error));
        }
        self.groups.clear();
        self.session_token = self
            .pages
            .as_ref()
            .and_then(PageAggregator::session_token)
            .cloned();
        self.buffering_complete = true;
        Ok(())
    }

    fn emit_page(&mut self) -> Result<PageResult> {
        if self.pages.is_none() {
            let mut pages = PageAggregator::new(self.emit_binary);
            pages.seed_session_token(self.session_token.clone());
            self.pages = Some(pages);
        }
        let count = self.page_size.min(self.values.len());
        let encoded = (|| {
            let pages = self.pages.as_ref().expect("group-by has page metadata");
            let mut items = Vec::with_capacity(count);
            for aggregator in self.values.iter().take(count) {
                if let Some(result) = aggregator.result()? {
                    let raw = serde_json::value::to_raw_value(&result).map_err(|error| {
                        CosmosError::builder()
                            .with_status(status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
                            .with_message("failed to serialize GROUP BY result")
                            .with_source(error)
                            .build()
                    })?;
                    items.push(pages.encode_item(items.len(), &raw)?);
                }
            }
            Ok(items)
        })();
        let items = encoded.map_err(|error| self.fail(error))?;
        self.values.drain(..count);
        let response = self
            .pages
            .take()
            .expect("group-by has page metadata")
            .build_page(items);
        self.completed = self.values.is_empty();
        Ok(PageResult::Page {
            response,
            is_terminal: self.completed,
        })
    }
}

fn body_error(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
        .with_message(message)
        .build()
}

#[async_trait]
impl PipelineNode for GroupBy {
    async fn next_page(&mut self, context: &mut PipelineContext<'_>) -> Result<PageResult> {
        if self.poisoned {
            return Err(body_error(
                "GROUP BY cannot continue after a consumed page or child failed",
            ));
        }
        if self.completed {
            return Ok(PageResult::Drained);
        }
        while !self.buffering_complete {
            let next = self
                .child
                .next_page(context)
                .await
                .map_err(|error| self.fail(error))?;
            match next {
                PageResult::Drained => self.finish_buffering()?,
                PageResult::Page {
                    response,
                    is_terminal,
                } => {
                    self.add_page(&response).map_err(|error| self.fail(error))?;
                    if is_terminal {
                        self.finish_buffering()?;
                    }
                }
                PageResult::SplitRequired { mut replacements } => {
                    let mut recovery = RecoveryDiagnostics::default();
                    recovery.absorb(replacements.take_diagnostics());
                    let error = CosmosError::builder()
                        .with_status(status_codes::CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT)
                        .with_message(
                            "GROUP BY cannot forward a split; the child fan-out must absorb it",
                        )
                        .build();
                    return Err(self.fail(recovery.attach_error(error)));
                }
            }
        }
        self.emit_page()
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
        if self.completed && !self.poisoned {
            return Ok(PipelineNodeState::Drained);
        }
        Err(CosmosError::builder()
            .with_status(status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED)
            .with_message("GROUP BY continuation is unavailable until all groups are emitted")
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
    use super::{GroupBy, GroupKey};
    use crate::{
        driver::dataflow::{
            aggregate::parse_rows,
            distinct_hash::hash_value,
            drain::SequentialDrain,
            mocks::{
                assert_drained, gone_error_with_diagnostics, response,
                response_with_request_diagnostics, unwrap_page, MockLeaf, NoopRequestExecutor,
                NoopTopologyProvider,
            },
            node::SplitReplacements,
            order_by::OrderByItem,
            query_plan::QueryInfo,
            PageResult, PipelineContext, PipelineNode, PipelineNodeState,
        },
        error::{status_codes, Result},
        models::{CosmosResponse, MaxItemCountHint, RequestCharge, ResponseBody, SessionToken},
    };
    use bytes::Bytes;
    use serde_json::{json, Value};
    use std::collections::HashMap;

    fn info(operator: Option<&str>, select_value: bool) -> QueryInfo {
        if select_value {
            QueryInfo {
                has_select_value: true,
                aggregates: operator.into_iter().map(str::to_owned).collect(),
                ..Default::default()
            }
        } else {
            QueryInfo {
                group_by_aliases: vec!["key".to_owned(), "total".to_owned()],
                group_by_alias_to_aggregate_type: [
                    ("key".to_owned(), Value::Null),
                    ("total".to_owned(), json!(operator)),
                ]
                .into(),
                ..Default::default()
            }
        }
    }

    fn row(key: Value, payload: Value) -> Value {
        json!({"groupByItems": [{"item": key}], "payload": payload})
    }

    fn feed(rows: Value, charge: f64, attempts: usize, token: Option<&str>) -> CosmosResponse {
        let template = response_with_request_diagnostics(attempts);
        let mut headers = template.headers().clone();
        headers.request_charge = Some(RequestCharge::new(charge));
        headers.continuation = Some("backend-continuation-must-not-leak".to_owned());
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

    async fn pull(node: &mut GroupBy) -> Result<PageResult> {
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        node.next_page(&mut context).await
    }

    fn items(response: &CosmosResponse) -> Vec<Value> {
        parse_rows(response.body()).unwrap()
    }

    #[tokio::test]
    async fn repeated_groups_across_pages_and_ranges_are_fully_combined() {
        let child = SequentialDrain::new(vec![
            Box::new(MockLeaf::with_pages(vec![
                page(json!([row(json!("a"), json!({"item": 2}))]), false),
                page(json!([row(json!("b"), json!({"item": 3}))]), true),
            ])),
            Box::new(MockLeaf::with_pages(vec![page(
                json!([row(json!("a"), json!({"item": 5}))]),
                true,
            )])),
        ]);
        let mut node =
            GroupBy::new(Box::new(child), &info(Some("Count"), true), None, false).unwrap();
        let output = unwrap_page(pull(&mut node).await);
        assert_eq!(items(&output), vec![json!(7), json!(3)]);
        assert_eq!(output.diagnostics().request_count(), 3);
        assert_eq!(
            output.headers().request_charge,
            Some(RequestCharge::new(3.0))
        );
        assert!(output.headers().continuation.is_none());
        assert_drained(pull(&mut node).await);
    }

    #[tokio::test]
    async fn no_aggregate_projection_preserves_first_scalar_and_value_payload() {
        for select_value in [false, true] {
            let payload = if select_value {
                json!({"item": "ordinary object, not an aggregate cell"})
            } else {
                json!({"key": "a", "total": null})
            };
            let child = MockLeaf::with_pages(vec![
                page(json!([row(json!("a"), payload.clone())]), false),
                page(json!([row(json!("a"), payload.clone())]), true),
            ]);
            let mut node =
                GroupBy::new(Box::new(child), &info(None, select_value), None, false).unwrap();
            let output = unwrap_page(pull(&mut node).await);
            assert_eq!(items(&output), vec![payload]);
        }
    }

    #[tokio::test]
    async fn weighted_average_merges_sum_and_count_not_local_averages() {
        let child = MockLeaf::with_pages(vec![
            page(
                json!([row(json!("a"), json!({"item": {"sum": 20, "count": 2}}))]),
                false,
            ),
            page(
                json!([row(json!("a"), json!({"item": {"sum": 40, "count": 1}}))]),
                true,
            ),
        ]);
        let mut node =
            GroupBy::new(Box::new(child), &info(Some("Average"), true), None, false).unwrap();
        assert_eq!(items(&unwrap_page(pull(&mut node).await)), vec![json!(20)]);
    }

    #[tokio::test]
    async fn aliases_combine_multiple_aggregates_and_omit_only_undefined() {
        let info = QueryInfo {
            group_by_aliases: vec![
                "key".to_owned(),
                "count".to_owned(),
                "average".to_owned(),
                "missing".to_owned(),
                "null".to_owned(),
            ],
            group_by_alias_to_aggregate_type: [
                ("key".to_owned(), Value::Null),
                ("count".to_owned(), json!("Count")),
                ("average".to_owned(), json!("Average")),
                ("missing".to_owned(), Value::Null),
                ("null".to_owned(), Value::Null),
            ]
            .into(),
            ..Default::default()
        };
        let child = MockLeaf::with_pages(vec![
            page(
                json!([row(
                    json!("a"),
                    json!({
                        "key": "a", "count": {"item": 2},
                        "average": {"item": {"sum": 20, "count": 2}}, "null": null
                    })
                )]),
                false,
            ),
            page(
                json!([row(
                    json!("a"),
                    json!({
                        "key": "a", "count": {"item": 1},
                        "average": {"item": {"sum": 40, "count": 1}},
                        "missing": "must not overwrite first undefined", "null": "ignored"
                    })
                )]),
                true,
            ),
        ]);
        let mut node = GroupBy::new(Box::new(child), &info, None, false).unwrap();
        assert_eq!(
            items(&unwrap_page(pull(&mut node).await)),
            vec![json!({"key": "a", "count": 3, "average": 20, "null": null})]
        );
    }

    #[tokio::test]
    async fn missing_value_payload_is_undefined_but_null_is_a_result() {
        let child = MockLeaf::with_pages(vec![page(
            json!([
                {"groupByItems": [{}]},
                row(Value::Null, Value::Null),
                {"groupByItems": [{}], "payload": "later value cannot replace undefined"}
            ]),
            true,
        )]);
        let mut node = GroupBy::new(Box::new(child), &info(None, true), None, false).unwrap();
        assert_eq!(
            items(&unwrap_page(pull(&mut node).await)),
            vec![Value::Null]
        );
    }

    #[tokio::test]
    async fn grouping_keys_use_structural_and_lossless_numeric_equality() {
        let child = MockLeaf::with_pages(vec![page(
            json!([
                {"groupByItems": [{}], "payload": {"item": 1}},
                row(Value::Null, json!({"item": 2})),
                row(json!(5), json!({"item": 3})),
                row(json!(5.0), json!({"item": 4})),
                row(json!(-0.0), json!({"item": 5})),
                row(json!(0), json!({"item": 6})),
                row(json!({"a": 1, "b": [2, 3]}), json!({"item": 7})),
                row(json!({"b": [2.0, 3], "a": 1.0}), json!({"item": 8})),
                row(json!([1, 2]), json!({"item": 9})),
                row(json!([2, 1]), json!({"item": 10})),
                row(json!(9_007_199_254_740_992_u64), json!({"item": 11})),
                row(json!(9_007_199_254_740_993_u64), json!({"item": 12})),
                row(json!(9_007_199_254_740_992.0), json!({"item": 13})),
                row(json!(u64::MAX), json!({"item": 14})),
                row(json!(-1), json!({"item": 15}))
            ]),
            true,
        )]);
        let mut node =
            GroupBy::new(Box::new(child), &info(Some("Count"), true), None, false).unwrap();
        assert_eq!(
            items(&unwrap_page(pull(&mut node).await)),
            vec![
                json!(1),
                json!(2),
                json!(7),
                json!(11),
                json!(15),
                json!(9),
                json!(10),
                json!(24),
                json!(12),
                json!(14),
                json!(15)
            ]
        );
    }

    #[tokio::test]
    async fn composite_keys_preserve_position_and_undefined_components() {
        let mut info = info(Some("Count"), true);
        info.group_by_expressions = vec!["c.a".to_owned(), "c.b".to_owned()];
        let child = MockLeaf::with_pages(vec![page(
            json!([
                {"groupByItems": [{}, {"item": 1}], "payload": {"item": 2}},
                {"groupByItems": [{"item": null}, {"item": 1}], "payload": {"item": 3}},
                {"groupByItems": [{"item": 1}, {}], "payload": {"item": 4}},
                {"groupByItems": [{}, {"item": 1.0}], "payload": {"item": 5}}
            ]),
            true,
        )]);
        let mut node = GroupBy::new(Box::new(child), &info, None, false).unwrap();
        assert_eq!(
            items(&unwrap_page(pull(&mut node).await)),
            vec![json!(7), json!(3), json!(4)]
        );
    }

    #[test]
    fn hash_collisions_do_not_merge_structurally_different_keys() {
        let hash = hash_value(&Value::Null).unwrap();
        let first = GroupKey {
            hash,
            value: OrderByItem::from_json(&json!([{"item": 1}])),
        };
        let second = GroupKey {
            hash,
            value: OrderByItem::from_json(&json!([{"item": 2}])),
        };
        let equivalent = GroupKey {
            hash,
            value: OrderByItem::from_json(&json!([{"item": 1.0}])),
        };
        let mut groups = HashMap::new();
        groups.insert(first, 1);
        groups.insert(second, 2);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups.get(&equivalent), Some(&1));
    }

    #[tokio::test]
    async fn empty_input_has_no_groups_but_preserves_empty_page_metadata() {
        for with_page in [false, true] {
            let child = MockLeaf::with_pages(if with_page {
                vec![page(json!([]), false), Ok(PageResult::Drained)]
            } else {
                vec![Ok(PageResult::Drained)]
            });
            let mut node =
                GroupBy::new(Box::new(child), &info(Some("Count"), true), None, false).unwrap();
            let result = pull(&mut node).await.unwrap();
            assert!(matches!(
                result,
                PageResult::Page {
                    is_terminal: true,
                    ..
                }
            ));
            let output = unwrap_page(Ok(result));
            assert!(items(&output).is_empty());
            assert_eq!(output.diagnostics().request_count(), usize::from(with_page));
            assert_eq!(output.headers().item_count, Some(0));
            assert_drained(pull(&mut node).await);
        }
    }

    #[tokio::test]
    async fn default_page_size_is_not_a_retained_group_limit() {
        for max_item_count in [None, Some(MaxItemCountHint::ServerDecides)] {
            let rows: Vec<_> = (0..205)
                .map(|key| row(json!(key), json!({"item": 1})))
                .collect();
            let template = response_with_request_diagnostics(2);
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
                page(json!(rows), true),
            ]);
            let mut node = GroupBy::new(
                Box::new(child),
                &info(Some("Count"), true),
                max_item_count,
                false,
            )
            .unwrap();
            for (index, expected_count) in [100, 100, 5].into_iter().enumerate() {
                let output = unwrap_page(pull(&mut node).await);
                assert_eq!(items(&output), vec![json!(1); expected_count]);
                assert_eq!(
                    output.diagnostics().request_count(),
                    if index == 0 { 3 } else { 0 }
                );
            }
            assert_drained(pull(&mut node).await);
        }
    }

    #[tokio::test]
    async fn group_paging_charges_metadata_once_and_checkpoints_do_not_mutate_state() {
        let child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: feed(
                    json!([row(json!("a"), json!({"item": 1}))]),
                    2.5,
                    2,
                    Some("0:-1#1"),
                ),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: feed(
                    json!([
                        row(json!("b"), json!({"item": 2})),
                        row(json!("c"), json!({"item": 3}))
                    ]),
                    3.5,
                    3,
                    Some("1:-1#2"),
                ),
                is_terminal: true,
            }),
        ]);
        let mut node = GroupBy::new(
            Box::new(child),
            &info(Some("Count"), true),
            Some(MaxItemCountHint::Limit(1.try_into().unwrap())),
            false,
        )
        .unwrap();
        let expected_session = SessionToken::new("0:-1#1")
            .merge(&SessionToken::new("1:-1#2"))
            .unwrap();
        for index in 0..3 {
            assert_eq!(
                node.snapshot_state().unwrap_err().status(),
                status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED
            );
            let result = pull(&mut node).await.unwrap();
            assert!(matches!(
                &result,
                PageResult::Page { is_terminal, .. } if *is_terminal == (index == 2)
            ));
            let output = unwrap_page(Ok(result));
            assert_eq!(items(&output), vec![json!(index + 1)]);
            assert_eq!(
                output.headers().request_charge,
                Some(RequestCharge::new(if index == 0 { 6.0 } else { 0.0 }))
            );
            assert_eq!(
                output.diagnostics().request_count(),
                if index == 0 { 5 } else { 0 }
            );
            assert_eq!(
                output.headers().session_token.as_ref(),
                Some(&expected_session)
            );
            assert!(output.headers().continuation.is_none());
        }
        assert!(matches!(
            node.snapshot_state().unwrap(),
            PipelineNodeState::Drained
        ));
        assert_drained(pull(&mut node).await);
    }

    #[tokio::test]
    async fn mixed_item_and_feed_encodings_obey_explicit_output_encoding() {
        for emit_binary in [false, true] {
            let template = response(b"");
            let child = MockLeaf::with_pages(vec![
                Ok(PageResult::Page {
                    response: CosmosResponse::new(
                        ResponseBody::from_items(vec![
                            Bytes::from(
                                serde_json::to_vec(&row(json!(1), json!({"item": 2}))).unwrap(),
                            ),
                            Bytes::from(
                                crate::binary_json::to_vec(&row(json!(1.0), json!({"item": 3})))
                                    .unwrap(),
                            ),
                        ]),
                        template.headers().clone(),
                        template.status(),
                        template.diagnostics(),
                    ),
                    is_terminal: false,
                }),
                Ok(PageResult::Page {
                    response: CosmosResponse::new(
                        crate::binary_json::to_vec(&json!({
                            "Documents": [row(json!(1), json!({"item": 4}))]
                        }))
                        .unwrap(),
                        template.headers().clone(),
                        template.status(),
                        template.diagnostics(),
                    ),
                    is_terminal: true,
                }),
            ]);
            let mut node = GroupBy::new(
                Box::new(child),
                &info(Some("Count"), true),
                None,
                emit_binary,
            )
            .unwrap();
            let output = unwrap_page(pull(&mut node).await);
            assert_eq!(items(&output), vec![json!(9)]);
            let ResponseBody::Items(items) = output.body() else {
                panic!("GROUP BY must emit standalone items");
            };
            assert_eq!(crate::binary_json::is_binary(&items[0]), emit_binary);
        }
    }

    #[tokio::test]
    async fn malformed_rows_poison_state_and_preserve_current_prior_and_deferred_diagnostics() {
        for malformed in [
            json!(null),
            json!([]),
            json!({}),
            json!({"groupByItems": null, "payload": {"item": 1}}),
            json!({"groupByItems": [null], "payload": {"item": 1}}),
            json!({"groupByItems": [{"wrong": 1}], "payload": {"item": 1}}),
            row(json!("a"), json!({"item": "invalid count"})),
        ] {
            let child = MockLeaf::with_pages(vec![
                page(json!([row(json!("a"), json!({"item": 1}))]), false),
                Ok(PageResult::Page {
                    response: feed(json!([malformed]), 2.0, 2, None),
                    is_terminal: true,
                }),
            ])
            .with_pending_error(gone_error_with_diagnostics());
            let mut node =
                GroupBy::new(Box::new(child), &info(Some("Count"), true), None, false).unwrap();
            let error = pull(&mut node).await.unwrap_err();
            assert_eq!(
                error.status(),
                status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
            );
            assert_eq!(error.diagnostics().unwrap().request_count(), 4);
            assert!(pull(&mut node).await.is_err());
            assert!(node.snapshot_state().is_err());
            assert!(node.take_pending_error().is_none());
        }
    }

    #[tokio::test]
    async fn select_list_missing_payload_and_wrong_key_arity_are_invalid() {
        for malformed in [
            json!({"groupByItems": [{"item": "a"}]}),
            json!({"groupByItems": [{"item": "a"}], "payload": null}),
            json!({"groupByItems": [], "payload": {"key": "a", "total": {"item": 1}}}),
        ] {
            let mut info = info(Some("Count"), false);
            info.group_by_expressions = vec!["c.key".to_owned()];
            let child = MockLeaf::with_pages(vec![page(json!([malformed]), true)]);
            let mut node = GroupBy::new(Box::new(child), &info, None, false).unwrap();
            assert!(pull(&mut node).await.is_err());
        }
    }

    #[tokio::test]
    async fn child_errors_and_deferred_terminal_errors_preempt_any_output() {
        for terminal in [false, true] {
            let pages = if terminal {
                vec![page(json!([row(json!("a"), json!({"item": 1}))]), true)]
            } else {
                vec![
                    page(json!([row(json!("a"), json!({"item": 1}))]), false),
                    Err(gone_error_with_diagnostics()),
                ]
            };
            let mut child = MockLeaf::with_pages(pages);
            if terminal {
                child = child.with_pending_error(gone_error_with_diagnostics());
            }
            let mut node =
                GroupBy::new(Box::new(child), &info(Some("Count"), true), None, false).unwrap();
            let error = pull(&mut node).await.unwrap_err();
            assert_eq!(error.status(), gone_error_with_diagnostics().status());
            assert_eq!(error.diagnostics().unwrap().request_count(), 2);
            assert!(pull(&mut node).await.is_err());
        }
    }

    #[tokio::test]
    async fn malformed_session_token_preserves_all_outstanding_diagnostics() {
        let child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: feed(
                    json!([row(json!("a"), json!({"item": 1}))]),
                    1.0,
                    1,
                    Some("0:-1#1"),
                ),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: feed(
                    json!([row(json!("a"), json!({"item": 2}))]),
                    2.0,
                    2,
                    Some("malformed"),
                ),
                is_terminal: true,
            }),
        ])
        .with_pending_error(gone_error_with_diagnostics());
        let mut node =
            GroupBy::new(Box::new(child), &info(Some("Count"), true), None, false).unwrap();
        let error = pull(&mut node).await.unwrap_err();
        assert_eq!(error.diagnostics().unwrap().request_count(), 4);
        assert!(node.snapshot_state().is_err());
        assert!(pull(&mut node).await.is_err());
    }

    #[tokio::test]
    async fn encoding_failure_keeps_diagnostics_and_does_not_remove_group() {
        let child = MockLeaf::with_pages(vec![page(json!([]), false), page(json!([]), true)]);
        let mut node = GroupBy::new(Box::new(child), &info(None, true), None, true).unwrap();
        let mut deep = json!(1);
        for _ in 0..(crate::binary_json::reader::MAX_DEPTH + 8) {
            deep = Value::Array(vec![deep]);
        }
        // Isolate output encoding from the input parser's nesting limit.
        let mut aggregator = super::SingleGroupAggregator::new(&info(None, true)).unwrap();
        aggregator.add(Some(&deep)).unwrap();
        node.values.push_back(aggregator);
        let error = pull(&mut node).await.unwrap_err();
        assert_eq!(
            error.status(),
            status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
        );
        assert!(error.to_string().contains("failed to re-encode"));
        assert_eq!(error.diagnostics().unwrap().request_count(), 2);
        assert_eq!(node.values.len(), 1);
        assert!(pull(&mut node).await.is_err());
        assert!(node.snapshot_state().is_err());
    }

    #[tokio::test]
    async fn split_replacements_are_consumed_below_grouping_with_diagnostics_once() {
        let child = SequentialDrain::new(vec![Box::new(MockLeaf::with_pages(vec![
            page(json!([row(json!("a"), json!({"item": 1}))]), false),
            Ok(PageResult::SplitRequired {
                replacements: SplitReplacements::untiled(vec![
                    Box::new(MockLeaf::with_pages(vec![page(
                        json!([row(json!("a"), json!({"item": 2}))]),
                        true,
                    )])),
                    Box::new(MockLeaf::with_pages(vec![page(
                        json!([
                            row(json!("b"), json!({"item": 3})),
                            row(json!("a"), json!({"item": 4}))
                        ]),
                        true,
                    )])),
                ])
                .with_diagnostics(Some(response_with_request_diagnostics(2).diagnostics())),
            }),
        ]))]);
        let mut node = GroupBy::new(
            Box::new(child),
            &info(Some("Count"), true),
            Some(MaxItemCountHint::Limit(1.try_into().unwrap())),
            false,
        )
        .unwrap();
        let first = unwrap_page(pull(&mut node).await);
        assert_eq!(items(&first), vec![json!(7)]);
        assert_eq!(first.diagnostics().request_count(), 5);
        let second = unwrap_page(pull(&mut node).await);
        assert_eq!(items(&second), vec![json!(3)]);
        assert_eq!(second.diagnostics().request_count(), 0);
        assert_drained(pull(&mut node).await);
    }

    #[tokio::test]
    async fn unexpected_split_preserves_current_prior_and_pending_diagnostics() {
        let child = MockLeaf::with_pages(vec![
            page(json!([row(json!("a"), json!({"item": 1}))]), false),
            Ok(PageResult::SplitRequired {
                replacements: SplitReplacements::untiled(vec![])
                    .with_diagnostics(Some(response_with_request_diagnostics(2).diagnostics())),
            }),
        ])
        .with_pending_error(gone_error_with_diagnostics());
        let mut node =
            GroupBy::new(Box::new(child), &info(Some("Count"), true), None, false).unwrap();
        let error = pull(&mut node).await.unwrap_err();
        assert_eq!(
            error.status(),
            status_codes::CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT
        );
        assert_eq!(error.diagnostics().unwrap().request_count(), 4);
        assert!(pull(&mut node).await.is_err());
    }

    #[tokio::test]
    async fn pending_error_handoff_poisoning_and_drained_child_errors() {
        let child = MockLeaf::with_pages(vec![]).with_pending_error(gone_error_with_diagnostics());
        let mut node =
            GroupBy::new(Box::new(child), &info(Some("Count"), true), None, false).unwrap();
        assert_eq!(
            node.take_pending_error()
                .unwrap()
                .diagnostics()
                .unwrap()
                .request_count(),
            1
        );
        assert!(pull(&mut node).await.is_err());

        let child = MockLeaf::with_pages(vec![page(json!([]), false), Ok(PageResult::Drained)])
            .with_pending_error(gone_error_with_diagnostics());
        let mut node =
            GroupBy::new(Box::new(child), &info(Some("Count"), true), None, false).unwrap();
        assert_eq!(
            pull(&mut node)
                .await
                .unwrap_err()
                .diagnostics()
                .unwrap()
                .request_count(),
            2
        );
    }
}
