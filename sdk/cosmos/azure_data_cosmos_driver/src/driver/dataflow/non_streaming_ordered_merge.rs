// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Buffered merge for finite, non-streaming ORDER BY queries.

use super::{
    binary_heap,
    order_by::compare_key_tuples,
    query_plan::SortOrder,
    query_response::{parse_envelope_page, EnvelopeRow, PageAggregator},
    PageResult, PipelineContext, PipelineNode, PipelineNodeState,
};
use crate::{
    error::{CosmosError, CosmosStatus},
    models::{FeedRange, MaxItemCountHint, SessionToken},
};
use async_trait::async_trait;
use serde_json::value::RawValue;
use std::{cmp::Ordering, collections::VecDeque, mem, sync::Arc};

const DEFAULT_PAGE_SIZE: usize = 100;

struct RetainedRow {
    row: EnvelopeRow,
    ordinal: u64,
}

/// Drains rewritten partition queries before emitting the finite globally ordered window.
pub(crate) struct NonStreamingOrderedMerge {
    child: Box<dyn PipelineNode>,
    directions: Arc<[SortOrder]>,
    retention_limit: usize,
    skip: usize,
    take: usize,
    page_size: usize,
    emit_binary: bool,
    retained: Vec<RetainedRow>,
    next_ordinal: u64,
    results: VecDeque<Box<RawValue>>,
    aggregator: Option<PageAggregator>,
    session_token: Option<SessionToken>,
    buffering_complete: bool,
    exhausted: bool,
}

impl NonStreamingOrderedMerge {
    pub(crate) fn new(
        child: Box<dyn PipelineNode>,
        directions: Vec<SortOrder>,
        retention_limit: usize,
        skip: usize,
        take: usize,
        max_item_count: Option<MaxItemCountHint>,
        emit_binary: bool,
    ) -> Self {
        let page_size = match max_item_count {
            Some(MaxItemCountHint::Limit(value)) => value.get() as usize,
            Some(MaxItemCountHint::ServerDecides) | None => DEFAULT_PAGE_SIZE,
        };

        Self {
            child,
            directions: directions.into(),
            retention_limit,
            skip,
            take,
            page_size,
            emit_binary,
            retained: Vec::with_capacity(retention_limit),
            next_ordinal: 0,
            results: VecDeque::new(),
            aggregator: Some(PageAggregator::new(emit_binary)),
            session_token: None,
            buffering_complete: false,
            exhausted: false,
        }
    }

    fn compare_rows(&self, left: &RetainedRow, right: &RetainedRow) -> Ordering {
        compare_key_tuples(
            left.row.keys.as_ref(),
            right.row.keys.as_ref(),
            &self.directions,
        )
        .then_with(|| left.ordinal.cmp(&right.ordinal))
    }

    fn retain(&mut self, row: EnvelopeRow) -> crate::error::Result<()> {
        let ordinal = self.next_ordinal;
        self.next_ordinal = self.next_ordinal.checked_add(1).ok_or_else(|| {
            CosmosError::builder()
                .with_status(CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_WINDOW_TOO_LARGE)
                .with_message("non-streaming ORDER BY row ordinal overflowed")
                .build()
        })?;

        if self.retention_limit == 0 {
            return Ok(());
        }

        let candidate = RetainedRow { row, ordinal };
        if self.retained.len() < self.retention_limit {
            let directions = &self.directions;
            binary_heap::push_by(&mut self.retained, candidate, |left, right| {
                compare_key_tuples(left.row.keys.as_ref(), right.row.keys.as_ref(), directions)
                    .then_with(|| left.ordinal.cmp(&right.ordinal))
                    == Ordering::Greater
            });
        } else if self.compare_rows(&candidate, &self.retained[0]) == Ordering::Less {
            let directions = &self.directions;
            binary_heap::replace_root_by(&mut self.retained, candidate, |left, right| {
                compare_key_tuples(left.row.keys.as_ref(), right.row.keys.as_ref(), directions)
                    .then_with(|| left.ordinal.cmp(&right.ordinal))
                    == Ordering::Greater
            });
        }
        Ok(())
    }

    fn finish_buffering(&mut self) {
        let mut retained = mem::take(&mut self.retained);
        let directions = &self.directions;
        retained.sort_by(|left, right| {
            compare_key_tuples(left.row.keys.as_ref(), right.row.keys.as_ref(), directions)
                .then_with(|| left.ordinal.cmp(&right.ordinal))
        });

        self.results = retained
            .into_iter()
            .skip(self.skip)
            .take(self.take)
            .map(|retained| retained.row.payload)
            .collect();
        self.session_token = self
            .aggregator
            .as_ref()
            .and_then(PageAggregator::session_token)
            .cloned();
        self.buffering_complete = true;
    }

    fn emit_page(&mut self) -> crate::error::Result<PageResult> {
        let count = self.page_size.min(self.results.len());
        if self.aggregator.is_none() {
            let mut aggregator = PageAggregator::new(self.emit_binary);
            aggregator.seed_session_token(self.session_token.clone());
            self.aggregator = Some(aggregator);
        }
        let aggregator = self
            .aggregator
            .as_ref()
            .expect("aggregator was initialized");
        let items = self
            .results
            .iter()
            .take(count)
            .enumerate()
            .map(|(index, payload)| aggregator.encode_item(index, payload))
            .collect::<crate::error::Result<Vec<_>>>()?;
        self.results.drain(..count);

        let aggregator = self.aggregator.take().expect("aggregator was initialized");
        let response = aggregator.build_page(items);
        let is_terminal = self.results.is_empty();
        self.exhausted = is_terminal;
        Ok(PageResult::Page {
            response,
            is_terminal,
        })
    }
}

#[async_trait]
impl PipelineNode for NonStreamingOrderedMerge {
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<PageResult> {
        if self.exhausted {
            return Ok(PageResult::Drained);
        }

        while !self.buffering_complete {
            match self.child.next_page(context).await? {
                PageResult::Page {
                    response,
                    is_terminal,
                } => {
                    let rows = parse_envelope_page(response.body(), self.directions.len())?;
                    self.aggregator
                        .as_mut()
                        .expect("aggregator exists while buffering")
                        .absorb(&response)?;
                    for row in rows {
                        self.retain(row)?;
                    }
                    if is_terminal {
                        self.finish_buffering();
                    }
                }
                PageResult::Drained => self.finish_buffering(),
                PageResult::SplitRequired { .. } => {
                    return Err(CosmosError::builder()
                        .with_status(CosmosStatus::CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT)
                        .with_message(
                            "non-streaming ORDER BY child unexpectedly requested split handling",
                        )
                        .build());
                }
            }
        }

        self.emit_page()
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        vec![self.child]
    }

    fn snapshot_state(&self) -> crate::error::Result<PipelineNodeState> {
        Err(CosmosError::builder()
            .with_status(CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_CONTINUATION_UNSUPPORTED)
            .with_message(
                "cross-partition non-streaming ORDER BY queries do not support continuation tokens",
            )
            .build())
    }

    fn topology_can_change(&self) -> bool {
        false
    }

    fn feed_range(&self) -> Option<&FeedRange> {
        self.child.feed_range()
    }

    fn fan_out_width(&self) -> usize {
        self.child.fan_out_width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        driver::dataflow::{
            mocks::{response_with_charge, MockLeaf, NoopRequestExecutor, NoopTopologyProvider},
            order_by::{OrderByItem, OrderByNumber},
            PipelineContext,
        },
        models::ResponseBody,
    };
    use serde_json::json;

    fn envelope(rows: &[(&str, f64, &str)]) -> Vec<u8> {
        serde_json::to_vec(&json!({
            "Documents": rows.iter().map(|(rid, score, id)| json!({
                "_rid": rid,
                "orderByItems": [{ "item": score }],
                "payload": { "id": id }
            })).collect::<Vec<_>>()
        }))
        .unwrap()
    }

    fn page(rows: &[(&str, f64, &str)], charge: f64, terminal: bool) -> PageResult {
        PageResult::Page {
            response: response_with_charge(&envelope(rows), charge),
            is_terminal: terminal,
        }
    }

    fn merge(
        pages: Vec<PageResult>,
        retention_limit: usize,
        skip: usize,
        take: usize,
        page_size: Option<u32>,
    ) -> NonStreamingOrderedMerge {
        NonStreamingOrderedMerge::new(
            Box::new(MockLeaf::with_pages(pages.into_iter().map(Ok).collect())),
            vec![SortOrder::Ascending],
            retention_limit,
            skip,
            take,
            page_size
                .map(|value| MaxItemCountHint::Limit(std::num::NonZeroU32::new(value).unwrap())),
            false,
        )
    }

    fn ids(response: &crate::models::CosmosResponse) -> Vec<String> {
        let ResponseBody::Items(items) = response.body() else {
            panic!("expected items response");
        };
        items
            .iter()
            .map(|item| {
                serde_json::from_slice::<serde_json::Value>(item).unwrap()["id"]
                    .as_str()
                    .unwrap()
                    .to_owned()
            })
            .collect()
    }

    fn context<'a>(
        executor: &'a mut NoopRequestExecutor,
        topology: &'a mut NoopTopologyProvider,
    ) -> PipelineContext<'a> {
        PipelineContext::new(executor, Some(topology))
    }

    #[tokio::test]
    async fn retains_global_window_and_paginates() {
        let mut node = merge(
            vec![
                page(&[("a", 0.0, "a"), ("e", 4.0, "e")], 1.25, false),
                page(
                    &[
                        ("b", 1.0, "b"),
                        ("c", 2.0, "c"),
                        ("d", 3.0, "d"),
                        ("f", 5.0, "f"),
                    ],
                    2.75,
                    true,
                ),
            ],
            4,
            1,
            3,
            Some(2),
        );
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = context(&mut executor, &mut topology);

        let PageResult::Page {
            response,
            is_terminal,
        } = node.next_page(&mut context).await.unwrap()
        else {
            panic!("expected page");
        };
        assert_eq!(ids(&response), ["b", "c"]);
        assert!(!is_terminal);
        assert_eq!(response.headers().request_charge.unwrap().value(), 4.0);

        let PageResult::Page {
            response,
            is_terminal,
        } = node.next_page(&mut context).await.unwrap()
        else {
            panic!("expected page");
        };
        assert_eq!(ids(&response), ["d"]);
        assert!(is_terminal);
        assert_eq!(response.headers().request_charge.unwrap().value(), 0.0);
    }

    #[tokio::test]
    async fn preserves_arrival_order_for_equal_keys() {
        let mut node = merge(
            vec![page(
                &[("a", 1.0, "a"), ("b", 1.0, "b"), ("c", 1.0, "c")],
                1.0,
                true,
            )],
            2,
            0,
            2,
            None,
        );
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = context(&mut executor, &mut topology);

        let PageResult::Page { response, .. } = node.next_page(&mut context).await.unwrap() else {
            panic!("expected page");
        };
        assert_eq!(ids(&response), ["a", "b"]);
    }

    #[tokio::test]
    async fn emits_terminal_empty_page_with_charge() {
        let mut node = merge(vec![page(&[], 3.5, true)], 1, 0, 1, None);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = context(&mut executor, &mut topology);

        let PageResult::Page {
            response,
            is_terminal,
        } = node.next_page(&mut context).await.unwrap()
        else {
            panic!("expected page");
        };
        assert!(ids(&response).is_empty());
        assert!(is_terminal);
        assert_eq!(response.headers().request_charge.unwrap().value(), 3.5);
    }

    #[tokio::test]
    async fn emits_binary_items_when_negotiated() {
        let mut node = NonStreamingOrderedMerge::new(
            Box::new(MockLeaf::with_pages(vec![Ok(page(
                &[("a", 1.0, "a")],
                1.0,
                true,
            ))])),
            vec![SortOrder::Ascending],
            1,
            0,
            1,
            None,
            true,
        );
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = context(&mut executor, &mut topology);

        let PageResult::Page { response, .. } = node.next_page(&mut context).await.unwrap() else {
            panic!("expected page");
        };
        let ResponseBody::Items(items) = response.body() else {
            panic!("expected items response");
        };
        assert_eq!(items.len(), 1);
        assert!(crate::binary_json::is_binary(&items[0]));
        let item: serde_json::Value = crate::binary_json::from_slice(&items[0]).unwrap();
        assert_eq!(item["id"], "a");
    }

    #[tokio::test]
    async fn binary_encode_failure_does_not_consume_results() {
        let mut deep = json!(1);
        for _ in 0..(crate::binary_json::reader::MAX_DEPTH + 8) {
            deep = serde_json::Value::Array(vec![deep]);
        }
        let body = serde_json::to_vec(&json!({
            "Documents": [{
                "_rid": "a",
                "orderByItems": [{ "item": 1.0 }],
                "payload": { "id": "a", "deep": deep }
            }]
        }))
        .unwrap();
        let mut node = NonStreamingOrderedMerge::new(
            Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                response: response_with_charge(&body, 1.0),
                is_terminal: true,
            })])),
            vec![SortOrder::Ascending],
            1,
            0,
            1,
            None,
            true,
        );
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = context(&mut executor, &mut topology);

        let error = node.next_page(&mut context).await.unwrap_err();
        assert_eq!(
            error.status(),
            CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID
        );
        assert_eq!(node.results.len(), 1);
    }

    #[test]
    fn continuation_is_always_rejected() {
        let node = merge(Vec::new(), 1, 0, 1, None);
        assert_eq!(
            node.snapshot_state().unwrap_err().status(),
            CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_CONTINUATION_UNSUPPORTED
        );
    }

    #[test]
    fn comparison_uses_all_order_by_items() {
        let left = RetainedRow {
            row: EnvelopeRow {
                keys: vec![
                    OrderByItem::Number(OrderByNumber::from(1.0)),
                    OrderByItem::String("a".to_owned()),
                ],
                rid: "left".to_owned(),
                payload: RawValue::from_string("{}".to_owned()).unwrap(),
            },
            ordinal: 0,
        };
        let right = RetainedRow {
            row: EnvelopeRow {
                keys: vec![
                    OrderByItem::Number(OrderByNumber::from(1.0)),
                    OrderByItem::String("b".to_owned()),
                ],
                rid: "right".to_owned(),
                payload: RawValue::from_string("{}".to_owned()).unwrap(),
            },
            ordinal: 1,
        };
        let node = NonStreamingOrderedMerge::new(
            Box::new(MockLeaf::with_pages(Vec::new())),
            vec![SortOrder::Ascending, SortOrder::Ascending],
            1,
            0,
            1,
            None,
            false,
        );
        assert_eq!(node.compare_rows(&left, &right), Ordering::Less);
    }
}
