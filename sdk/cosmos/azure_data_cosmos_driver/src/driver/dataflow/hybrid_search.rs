// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Buffered, cross-partition full-text and hybrid ranking.

use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::value::RawValue;

use crate::{
    error::{status_codes, CosmosError},
    models::{CosmosOperation, FeedRange, MaxItemCountHint, ResponseBody, SessionToken},
    options::FullTextScoreScope,
};

use super::{
    query_plan::{HybridSearchQueryInfo, SortOrder},
    query_response::{self, normalize_page_body, PageAggregator},
    PageResult, PipelineContext, PipelineNode, PipelineNodeState, Request, RequestTarget,
    SequentialDrain,
};

const RRF_CONSTANT: f64 = 60.0;
const DEFAULT_PAGE_SIZE: usize = 100;
const DOCUMENT_COUNT: &str = "{documentdb-formattablehybridsearchquery-totaldocumentcount}";

fn invalid_plan(message: impl Into<std::borrow::Cow<'static, str>>) -> CosmosError {
    CosmosError::builder()
        .with_status(status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
        .with_message(message)
        .build()
}

fn invalid_page(message: impl Into<std::borrow::Cow<'static, str>>) -> CosmosError {
    CosmosError::builder()
        .with_status(status_codes::SERVICE_ORDER_BY_ENVELOPE_INVALID)
        .with_message(message)
        .build()
}

fn invalid_page_json(message: &'static str, error: serde_json::Error) -> CosmosError {
    CosmosError::builder()
        .with_status(status_codes::SERVICE_ORDER_BY_ENVELOPE_INVALID)
        .with_message(message)
        .with_source(error)
        .build()
}

/// Enforces the same finite-window policy as other buffered query stages.
pub(super) fn hybrid_window(
    info: &HybridSearchQueryInfo,
    maximum: u64,
) -> crate::error::Result<(usize, usize)> {
    let skip = info.skip.unwrap_or(0);
    let window = info
        .take
        .and_then(|take| skip.checked_add(take))
        .filter(|window| *window <= maximum)
        .ok_or_else(|| {
            CosmosError::builder()
                .with_status(status_codes::CLIENT_BUFFERED_QUERY_REQUIRES_FINITE_WINDOW)
                .with_message(format!(
                    "cross-partition ranked full-text and hybrid queries require a finite global TOP or LIMIT and OFFSET plus take at most max_buffered_query_window ({maximum})"
                ))
                .build()
        })?;
    let skip =
        usize::try_from(skip).map_err(|_| invalid_plan("hybrid search OFFSET is too large"))?;
    let window =
        usize::try_from(window).map_err(|_| invalid_plan("hybrid search window is too large"))?;
    Ok((skip, window - skip))
}

#[derive(Deserialize)]
struct FeedPage {
    #[serde(alias = "Documents")]
    documents: Vec<Box<RawValue>>,
}

fn parse_feed(body: &ResponseBody) -> crate::error::Result<Vec<Box<RawValue>>> {
    match body {
        ResponseBody::NoPayload => Ok(Vec::new()),
        ResponseBody::Items(_) => Err(invalid_page(
            "hybrid search backend returned split items instead of a raw feed page",
        )),
        ResponseBody::Bytes(bytes) => {
            let bytes = normalize_page_body(bytes)?;
            let page: FeedPage = serde_json::from_slice(&bytes).map_err(|error| {
                invalid_page_json("failed to parse hybrid search backend feed page", error)
            })?;
            Ok(page.documents)
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FullTextStatistics {
    total_word_count: u64,
    hit_counts: Vec<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GlobalStatistics {
    document_count: u64,
    full_text_statistics: Vec<FullTextStatistics>,
}

impl GlobalStatistics {
    fn add(&mut self, other: Self) -> crate::error::Result<()> {
        if self.full_text_statistics.len() != other.full_text_statistics.len() {
            return Err(invalid_page(
                "hybrid search statistics disagree on the number of full-text components",
            ));
        }
        self.document_count = self
            .document_count
            .checked_add(other.document_count)
            .ok_or_else(|| invalid_page("hybrid search document count overflowed"))?;
        for (total, part) in self
            .full_text_statistics
            .iter_mut()
            .zip(other.full_text_statistics)
        {
            if total.hit_counts.len() != part.hit_counts.len() {
                return Err(invalid_page(
                    "hybrid search statistics disagree on the number of hit counts",
                ));
            }
            total.total_word_count = total
                .total_word_count
                .checked_add(part.total_word_count)
                .ok_or_else(|| invalid_page("hybrid search word count overflowed"))?;
            for (count, addition) in total.hit_counts.iter_mut().zip(part.hit_counts) {
                *count = count
                    .checked_add(addition)
                    .ok_or_else(|| invalid_page("hybrid search hit count overflowed"))?;
            }
        }
        Ok(())
    }
}

struct Component {
    query: String,
    direction: SortOrder,
}

struct RankedRow {
    rid: String,
    scores: Vec<Option<f64>>,
    payload: Box<RawValue>,
    rank_score: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResultEnvelope {
    #[serde(rename = "_rid")]
    rid: String,
    component_scores: Option<Vec<Option<f64>>>,
    payload: Option<Box<RawValue>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NestedResult {
    component_scores: Vec<Option<f64>>,
    payload: Box<RawValue>,
}

fn parse_result(raw: &RawValue, component_count: usize) -> crate::error::Result<RankedRow> {
    let result: ResultEnvelope = serde_json::from_str(raw.get()).map_err(|error| {
        invalid_page_json("failed to parse hybrid search result envelope", error)
    })?;
    if result.rid.is_empty() {
        return Err(invalid_page(
            "hybrid search result is missing a non-empty _rid",
        ));
    }
    let (scores, payload) = match (result.component_scores, result.payload) {
        (Some(scores), Some(payload)) => (scores, payload),
        (None, Some(outer)) => {
            let inner: NestedResult = serde_json::from_str(outer.get()).map_err(|error| {
                invalid_page_json("failed to parse nested hybrid search result", error)
            })?;
            (inner.component_scores, inner.payload)
        }
        _ => return Err(invalid_page("hybrid search result is missing its payload")),
    };
    let scores = if scores.is_empty() {
        vec![None; component_count]
    } else if scores.len() == component_count {
        scores
    } else {
        return Err(invalid_page(
            "hybrid search result has an unexpected number of component scores",
        ));
    };
    if scores.iter().flatten().any(|score| !score.is_finite()) {
        return Err(invalid_page("hybrid search result has a non-finite score"));
    }
    Ok(RankedRow {
        rid: result.rid,
        scores,
        payload,
        rank_score: 0.0,
    })
}

fn rank_results(rows: &mut [RankedRow], directions: &[SortOrder], weights: &[f64]) {
    for (component, (direction, weight)) in directions.iter().zip(weights).enumerate() {
        let mut scores: Vec<(f64, usize)> = rows
            .iter()
            .enumerate()
            .filter_map(|(index, row)| row.scores[component].map(|score| (score, index)))
            .collect();
        scores.sort_unstable_by(|left, right| {
            let order = left.0.partial_cmp(&right.0).unwrap_or(Ordering::Equal);
            match direction {
                SortOrder::Ascending => order,
                SortOrder::Descending => order.reverse(),
            }
            .then_with(|| rows[left.1].rid.cmp(&rows[right.1].rid))
        });
        let missing_rank = scores.len() + 1;
        for row in rows.iter_mut() {
            if row.scores[component].is_none() {
                row.rank_score += weight / (RRF_CONSTANT + missing_rank as f64);
            }
        }
        let mut rank = 1;
        for index in 0..scores.len() {
            if index > 0 && scores[index].0 != scores[index - 1].0 {
                rank = index + 1;
            }
            rows[scores[index].1].rank_score += weight / (RRF_CONSTANT + rank as f64);
        }
    }
    rows.sort_unstable_by(|left, right| {
        right
            .rank_score
            .partial_cmp(&left.rank_score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| left.rid.cmp(&right.rid))
    });
}

fn replace_statistics(
    query: &str,
    statistics: Option<&GlobalStatistics>,
    indices: &[Option<usize>],
) -> crate::error::Result<String> {
    let mut query = query_response::rewritten_query_from_beginning(query)?;
    if let Some(statistics) = statistics {
        query = query.replace(DOCUMENT_COUNT, &statistics.document_count.to_string());
        for (component, index) in indices.iter().enumerate() {
            let Some(index) = index else { continue };
            let values = &statistics.full_text_statistics[*index];
            query = query.replace(
                &format!("{{documentdb-formattablehybridsearchquery-totalwordcount-{component}}}"),
                &values.total_word_count.to_string(),
            );
            let hits = serde_json::to_string(&values.hit_counts).map_err(|error| {
                CosmosError::builder()
                    .with_status(status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("failed to serialize hybrid search hit counts")
                    .with_source(error)
                    .build()
            })?;
            query = query.replace(
                &format!("{{documentdb-formattablehybridsearchquery-hitcountsarray-{component}}}"),
                &hits,
            );
        }
    }
    if query.contains("documentdb-formattablehybridsearchquery-") {
        return Err(invalid_plan(
            "hybrid search component query has unresolved statistics placeholders",
        ));
    }
    Ok(query)
}

fn request_fanout(operation: Arc<CosmosOperation>, targets: &[RequestTarget]) -> SequentialDrain {
    SequentialDrain::new(
        targets
            .iter()
            .cloned()
            .map(|target| {
                Box::new(Request::new(Arc::clone(&operation), target, None))
                    as Box<dyn PipelineNode>
            })
            .collect(),
    )
}

/// Executes the statistics and component phases, then emits the fused window.
pub(crate) struct HybridSearch {
    operation: Arc<CosmosOperation>,
    targets: Vec<RequestTarget>,
    statistics_targets: Vec<RequestTarget>,
    statistics_child: Option<SequentialDrain>,
    component_child: Option<SequentialDrain>,
    components: Vec<Component>,
    component_weights: Vec<f64>,
    statistic_indices: Vec<Option<usize>>,
    statistics: Option<GlobalStatistics>,
    next_component: usize,
    rows: HashMap<String, RankedRow>,
    candidate_count: usize,
    component_limits: Vec<usize>,
    max_candidates: usize,
    skip: usize,
    take: usize,
    page_size: usize,
    emit_binary: bool,
    results: VecDeque<Box<RawValue>>,
    aggregator: Option<PageAggregator>,
    session_token: Option<SessionToken>,
    buffered: bool,
    exhausted: bool,
}

impl HybridSearch {
    pub(crate) fn new(
        operation: Arc<CosmosOperation>,
        info: &HybridSearchQueryInfo,
        targets: Vec<RequestTarget>,
        statistics_targets: Vec<RequestTarget>,
        scope: FullTextScoreScope,
        skip: usize,
        take: usize,
    ) -> crate::error::Result<Self> {
        if info.component_query_infos.is_empty() {
            return Err(invalid_plan("hybrid search plan has no component queries"));
        }
        if !info.component_weights.is_empty()
            && (info.component_weights.len() != info.component_query_infos.len()
                || info
                    .component_weights
                    .iter()
                    .any(|value| !value.is_finite()))
        {
            return Err(invalid_plan(
                "hybrid search plan has invalid component weights",
            ));
        }
        let components: Vec<Component> = info
            .component_query_infos
            .iter()
            .map(|component| {
                let query = component
                    .rewritten_query
                    .as_ref()
                    .filter(|query| !query.is_empty())
                    .ok_or_else(|| invalid_plan("hybrid search component has no rewrittenQuery"))?;
                Ok(Component {
                    query: query.clone(),
                    direction: component
                        .order_by
                        .first()
                        .copied()
                        .unwrap_or(SortOrder::Descending),
                })
            })
            .collect::<crate::error::Result<_>>()?;
        let statistic_indices = (0..components.len())
            .scan(0, |next, component| {
                let placeholder =
                    format!("documentdb-formattablehybridsearchquery-totalwordcount-{component}");
                let index = components
                    .iter()
                    .any(|entry| entry.query.contains(&placeholder))
                    .then(|| {
                        let index = *next;
                        *next += 1;
                        index
                    });
                Some(index)
            })
            .collect();
        let weights = if info.component_weights.is_empty() {
            vec![1.0; components.len()]
        } else {
            info.component_weights.clone()
        };
        let component_limits: Vec<usize> = info
            .component_query_infos
            .iter()
            .map(|query| {
                let bound = match (query.top, query.limit) {
                    (Some(top), Some(limit)) => top.min(limit),
                    (Some(top), None) => top,
                    (None, Some(limit)) => limit,
                    (None, None) => {
                        return Err(invalid_plan(
                            "hybrid search component query has no finite candidate limit",
                        ));
                    }
                };
                usize::try_from(bound)
                    .map_err(|_| invalid_plan("hybrid search candidate bound is too large"))
            })
            .collect::<crate::error::Result<_>>()?;
        let max_candidates = component_limits
            .iter()
            .try_fold(0_usize, |total, bound| {
                total
                    .checked_add(*bound)
                    .ok_or_else(|| invalid_plan("hybrid search candidate bound is too large"))
            })?
            .checked_mul(targets.len())
            .ok_or_else(|| invalid_plan("hybrid search candidate bound is too large"))?;
        let statistics_child = if info.requires_global_statistics {
            let container = operation
                .container()
                .ok_or_else(|| invalid_plan("hybrid search requires a container reference"))?;
            if info.global_statistics_query.is_empty() {
                return Err(invalid_plan(
                    "hybrid search plan has no globalStatisticsQuery",
                ));
            }
            let body = query_response::rewrite_query_body(
                operation.body(),
                &info.global_statistics_query,
            )?;
            let statistics_operation = match scope {
                FullTextScoreScope::Global => {
                    CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
                        .with_request_headers(operation.request_headers().clone())
                        .with_absolute_deadline(operation.absolute_deadline())
                        .with_body(body)
                }
                FullTextScoreScope::Local => operation.as_ref().clone().with_body(body),
            };
            Some(request_fanout(
                Arc::new(statistics_operation),
                &statistics_targets,
            ))
        } else {
            None
        };
        let page_size = match operation.request_headers().max_item_count {
            Some(MaxItemCountHint::Limit(count)) => count.get() as usize,
            _ => DEFAULT_PAGE_SIZE,
        };
        let emit_binary = operation.emits_binary_payload();
        Ok(Self {
            operation,
            targets,
            statistics_targets,
            statistics_child,
            component_child: None,
            components,
            component_weights: weights,
            statistic_indices,
            statistics: None,
            next_component: 0,
            rows: HashMap::new(),
            candidate_count: 0,
            component_limits,
            max_candidates,
            skip,
            take,
            page_size,
            emit_binary,
            results: VecDeque::new(),
            aggregator: Some(PageAggregator::new(emit_binary)),
            session_token: None,
            buffered: false,
            exhausted: false,
        })
    }

    fn collect_statistics(&mut self, body: &ResponseBody) -> crate::error::Result<()> {
        for raw in parse_feed(body)? {
            let part: GlobalStatistics = serde_json::from_str(raw.get()).map_err(|error| {
                invalid_page_json("failed to parse hybrid search statistics", error)
            })?;
            if let Some(total) = &mut self.statistics {
                total.add(part)?;
            } else {
                self.statistics = Some(part);
            }
        }
        Ok(())
    }

    fn start_component(&mut self) -> crate::error::Result<()> {
        if !self.statistics_targets.is_empty() && self.statistics.is_none() {
            return Err(invalid_page(
                "hybrid search statistics query returned no rows",
            ));
        }
        if let Some(statistics) = &self.statistics {
            if self.statistic_indices.iter().flatten().count()
                != statistics.full_text_statistics.len()
            {
                return Err(invalid_page(
                    "hybrid search statistics do not match the component queries",
                ));
            }
        }
        let query = replace_statistics(
            &self.components[self.next_component].query,
            self.statistics.as_ref(),
            &self.statistic_indices,
        )?;
        let body = query_response::rewrite_query_body(self.operation.body(), &query)?;
        let operation = Arc::new(self.operation.as_ref().clone().with_body(body));
        self.component_child = Some(request_fanout(operation, &self.targets));
        Ok(())
    }

    fn collect_results(&mut self, body: &ResponseBody) -> crate::error::Result<()> {
        for raw in parse_feed(body)? {
            self.candidate_count += 1;
            if self.candidate_count > self.max_candidates {
                return Err(invalid_page(format!(
                    "hybrid search component returned more than {} candidates for the planned component limits",
                    self.max_candidates
                )));
            }
            let row = parse_result(&raw, self.components.len())?;
            match self.rows.entry(row.rid.clone()) {
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(row);
                }
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    for (previous, current) in entry.get_mut().scores.iter_mut().zip(row.scores) {
                        if current.is_some() {
                            *previous = current;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn account_for_split_children(&mut self, count: usize) -> crate::error::Result<()> {
        let additional = count
            .checked_mul(self.component_limits[self.next_component])
            .ok_or_else(|| invalid_plan("hybrid search candidate bound is too large"))?;
        self.max_candidates = self
            .max_candidates
            .checked_add(additional)
            .ok_or_else(|| invalid_plan("hybrid search candidate bound is too large"))?;
        Ok(())
    }

    fn finish_buffering(&mut self) {
        let mut rows: Vec<RankedRow> = self.rows.drain().map(|(_, row)| row).collect();
        let directions: Vec<_> = self
            .components
            .iter()
            .map(|entry| entry.direction)
            .collect();
        rank_results(&mut rows, &directions, &self.component_weights);
        self.results = rows
            .into_iter()
            .skip(self.skip)
            .take(self.take)
            .map(|row| row.payload)
            .collect();
        self.session_token = self
            .aggregator
            .as_ref()
            .and_then(PageAggregator::session_token)
            .cloned();
        self.buffered = true;
    }

    fn emit_page(&mut self) -> crate::error::Result<PageResult> {
        if self.aggregator.is_none() {
            let mut aggregator = PageAggregator::new(self.emit_binary);
            aggregator.seed_session_token(self.session_token.clone());
            self.aggregator = Some(aggregator);
        }
        let aggregator = self.aggregator.as_ref().expect("aggregator initialized");
        let count = self.page_size.min(self.results.len());
        let items = self
            .results
            .iter()
            .take(count)
            .enumerate()
            .map(|(index, payload)| aggregator.encode_item(index, payload))
            .collect::<crate::error::Result<Vec<_>>>()?;
        self.results.drain(..count);
        let response = self
            .aggregator
            .take()
            .expect("aggregator initialized")
            .build_page(items);
        self.exhausted = self.results.is_empty();
        Ok(PageResult::Page {
            response,
            is_terminal: self.exhausted,
        })
    }
}

#[async_trait]
impl PipelineNode for HybridSearch {
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<PageResult> {
        if self.exhausted {
            return Ok(PageResult::Drained);
        }

        while !self.buffered {
            if let Some(child) = &mut self.statistics_child {
                match child.next_page(context).await? {
                    PageResult::Page {
                        response,
                        is_terminal,
                    } => {
                        self.collect_statistics(response.body())?;
                        self.aggregator
                            .as_mut()
                            .expect("aggregator initialized")
                            .absorb(&response)?;
                        if is_terminal {
                            self.statistics_child = None;
                        }
                    }
                    PageResult::Drained => self.statistics_child = None,
                    PageResult::SplitRequired { .. } => {
                        return Err(invalid_page(
                            "hybrid search statistics child unexpectedly requested a split",
                        ));
                    }
                }
                continue;
            }
            if self.next_component == self.components.len() {
                self.finish_buffering();
                break;
            }
            if self.component_child.is_none() {
                self.start_component()?;
            }
            let (result, spawned_children) = {
                let child = self
                    .component_child
                    .as_mut()
                    .expect("component initialized");
                let result = child.next_page(context).await?;
                (result, child.take_spawned_children())
            };
            self.account_for_split_children(spawned_children)?;
            match result {
                PageResult::Page {
                    response,
                    is_terminal,
                } => {
                    self.collect_results(response.body())?;
                    self.aggregator
                        .as_mut()
                        .expect("aggregator initialized")
                        .absorb(&response)?;
                    if is_terminal {
                        self.component_child = None;
                        self.next_component += 1;
                    }
                }
                PageResult::Drained => {
                    self.component_child = None;
                    self.next_component += 1;
                }
                PageResult::SplitRequired { .. } => {
                    return Err(invalid_page(
                        "hybrid search component child unexpectedly requested a split",
                    ));
                }
            }
        }
        self.emit_page()
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        self.statistics_child
            .into_iter()
            .map(|child| Box::new(child) as Box<dyn PipelineNode>)
            .chain(
                self.component_child
                    .map(|child| Box::new(child) as Box<dyn PipelineNode>),
            )
            .collect()
    }

    fn snapshot_state(&self) -> crate::error::Result<PipelineNodeState> {
        Err(CosmosError::builder()
            .with_status(status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED)
            .with_message("cross-partition ranked full-text and hybrid queries do not support continuation tokens")
            .build())
    }

    fn topology_can_change(&self) -> bool {
        false
    }

    fn feed_range(&self) -> Option<&FeedRange> {
        None
    }

    fn fan_out_width(&self) -> usize {
        self.targets
            .len()
            .saturating_mul(self.components.len())
            .saturating_add(self.statistics_targets.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        driver::dataflow::{
            mocks::{
                epk_range_target, gone_error, response_with_charge, response_with_continuation,
                MockRequestExecutor, MockTopologyProvider, NoopTopologyProvider,
            },
            query_plan::QueryInfo,
            PartitionRoutingRefresh, PipelineContext, ResolvedRange,
        },
        models::{
            effective_partition_key::EffectivePartitionKey, AccountReference, ContainerProperties,
            ContainerReference, FeedRange, MaxItemCountHint, PartitionKeyDefinition,
            SystemProperties,
        },
    };
    use serde_json::{json, Value};
    use std::num::NonZeroU32;

    fn operation() -> Arc<CosmosOperation> {
        let account = AccountReference::with_master_key(
            url::Url::parse("https://example.documents.azure.com/").unwrap(),
            "dGVzdA==",
        );
        let properties = ContainerProperties {
            id: "coll".into(),
            partition_key: PartitionKeyDefinition::new(vec!["/pk".into()]),
            system_properties: SystemProperties::default(),
        };
        let container =
            ContainerReference::new(account, "db", "db-rid", "coll", "coll-rid", &properties);
        Arc::new(
            CosmosOperation::query_items(container, Some(FeedRange::full()))
                .with_body(br#"{"query":"SELECT * FROM c WHERE c.name = @term","parameters":[{"name":"@term","value":"rust"}]}"#.to_vec())
                .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap())),
        )
    }

    fn plan() -> HybridSearchQueryInfo {
        HybridSearchQueryInfo {
            global_statistics_query: "SELECT VALUE GlobalStats(c) FROM c".to_owned(),
            component_query_infos: vec![
                QueryInfo {
                    top: Some(2),
                    rewritten_query: Some(
                        "SELECT TOP 2 c FROM c WHERE c.name = @term AND \
                         {documentdb-formattablehybridsearchquery-totaldocumentcount} > 0 \
                         AND {documentdb-formattablehybridsearchquery-totalwordcount-0} > 0 \
                         AND ARRAY_LENGTH({documentdb-formattablehybridsearchquery-hitcountsarray-0}) > 0"
                            .to_owned(),
                    ),
                    order_by: vec![SortOrder::Descending],
                    ..Default::default()
                },
                QueryInfo {
                    top: Some(2),
                    rewritten_query: Some("SELECT TOP 2 c FROM c WHERE c.name = @term".to_owned()),
                    order_by: vec![SortOrder::Descending],
                    ..Default::default()
                },
            ],
            component_weights: vec![1.0, 2.0],
            skip: Some(1),
            take: Some(2),
            requires_global_statistics: true,
        }
    }

    fn single_component_plan() -> HybridSearchQueryInfo {
        let mut info = plan();
        info.component_query_infos = vec![QueryInfo {
            top: Some(2),
            rewritten_query: Some("SELECT TOP 2 c FROM c".to_owned()),
            ..Default::default()
        }];
        info.component_weights.clear();
        info.skip = Some(0);
        info.take = Some(2);
        info.requires_global_statistics = false;
        info
    }

    fn page(documents: Value, charge: f64) -> crate::models::CosmosResponse {
        response_with_charge(
            &serde_json::to_vec(&json!({"Documents": documents})).unwrap(),
            charge,
        )
    }

    fn ids(response: &crate::models::CosmosResponse) -> Vec<String> {
        let ResponseBody::Items(items) = response.body() else {
            panic!("expected split items");
        };
        items
            .iter()
            .map(|item| {
                serde_json::from_slice::<Value>(item).unwrap()["id"]
                    .as_str()
                    .unwrap()
                    .to_owned()
            })
            .collect()
    }

    #[tokio::test]
    async fn collects_statistics_and_fuses_weighted_components_in_pages() {
        let info = plan();
        let target = epk_range_target().unwrap();
        let mut node = HybridSearch::new(
            operation(),
            &info,
            vec![target.clone()],
            vec![target.clone(), target],
            FullTextScoreScope::Global,
            1,
            2,
        )
        .unwrap();
        let stats = |document_count, total_word_count, hits| {
            json!([{
                "documentCount": document_count,
                "fullTextStatistics": [{"totalWordCount":total_word_count,"hitCounts":[hits]}]
            }])
        };
        let responses = vec![
            page(stats(2, 10, 2), 1.0),
            page(stats(3, 5, 1), 2.0),
            page(
                json!([
                    {"_rid":"a","payload":{"componentScores":[0.9,0.3],"payload":{"id":"a"}}},
                    {"_rid":"b","payload":{"componentScores":[0.8,0.7],"payload":{"id":"b"}}}
                ]),
                3.0,
            ),
            page(
                json!([
                    {"_rid":"b","componentScores":[0.8,0.7],"payload":{"id":"b"}},
                    {"_rid":"c","componentScores":[0.1,0.9],"payload":{"id":"c"}}
                ]),
                4.0,
            ),
        ];
        let mut executor = MockRequestExecutor::new(responses.into_iter().map(Ok).collect());
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let PageResult::Page {
            response,
            is_terminal,
        } = node.next_page(&mut context).await.unwrap()
        else {
            panic!("expected first page");
        };
        assert_eq!(ids(&response), ["b"]);
        assert!(!is_terminal);
        assert_eq!(response.headers().request_charge.unwrap().value(), 10.0);

        let PageResult::Page {
            response,
            is_terminal,
        } = node.next_page(&mut context).await.unwrap()
        else {
            panic!("expected second page");
        };
        assert_eq!(ids(&response), ["a"]);
        assert!(is_terminal);
        assert_eq!(response.headers().request_charge.unwrap().value(), 0.0);
        assert!(matches!(
            node.next_page(&mut context).await.unwrap(),
            PageResult::Drained
        ));
        assert_eq!(
            node.snapshot_state().unwrap_err().status(),
            status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED
        );
        assert_eq!(executor.query_bodies.len(), 4);
        let component: Value =
            serde_json::from_slice(&executor.query_bodies[2].as_ref().unwrap()).unwrap();
        let query = component["query"].as_str().unwrap();
        assert!(query.contains("5 > 0"));
        assert!(query.contains("15 > 0"));
        assert!(query.contains("ARRAY_LENGTH([3])"));
        assert_eq!(component["parameters"][0]["value"], "rust");
    }

    #[tokio::test]
    async fn allows_candidates_from_split_children_after_parent_emits_results() {
        let target = epk_range_target().unwrap();
        let mut node = HybridSearch::new(
            operation(),
            &single_component_plan(),
            vec![target],
            vec![],
            FullTextScoreScope::Local,
            0,
            2,
        )
        .unwrap();
        let row =
            |id, score| json!({"_rid": id, "componentScores": [score], "payload": {"id": id}});
        let parent = response_with_continuation(
            &serde_json::to_vec(&json!({"Documents": [row("a", 0.9)]})).unwrap(),
            Some("resume"),
        );
        let mut executor = MockRequestExecutor::new(vec![
            Ok(parent),
            Err(gone_error()),
            Ok(page(json!([row("b", 0.8), row("c", 0.7)]), 1.0)),
            Ok(page(json!([row("d", 0.6), row("e", 0.5)]), 1.0)),
        ]);
        let middle = EffectivePartitionKey::try_from("40").unwrap();
        let end = EffectivePartitionKey::try_from("80").unwrap();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            ResolvedRange {
                partition_key_range_id: "1".to_owned(),
                parents: vec!["0".to_owned()],
                range: FeedRange::new(EffectivePartitionKey::MIN, middle.clone()).unwrap(),
            },
            ResolvedRange {
                partition_key_range_id: "2".to_owned(),
                parents: vec!["0".to_owned()],
                range: FeedRange::new(middle, end).unwrap(),
            },
        ])]);
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let PageResult::Page { response, .. } = node.next_page(&mut context).await.unwrap() else {
            panic!("expected first ranked page");
        };
        assert_eq!(ids(&response), ["a"]);
        let PageResult::Page {
            response,
            is_terminal,
        } = node.next_page(&mut context).await.unwrap()
        else {
            panic!("expected second ranked page");
        };
        assert_eq!(ids(&response), ["b"]);
        assert!(is_terminal);
        assert_eq!(node.candidate_count, 5);
        assert_eq!(node.max_candidates, 6);
        assert_eq!(
            executor.continuation_calls,
            [
                None,
                Some("resume".to_owned()),
                Some("resume".to_owned()),
                Some("resume".to_owned())
            ]
        );
        assert_eq!(
            topology.refresh_calls,
            [PartitionRoutingRefresh::ForceRefresh]
        );
    }

    #[test]
    fn rejects_excess_candidates_without_a_split() {
        let mut node = HybridSearch::new(
            operation(),
            &single_component_plan(),
            vec![epk_range_target().unwrap()],
            vec![],
            FullTextScoreScope::Local,
            0,
            2,
        )
        .unwrap();
        let response = page(
            json!([
                {"_rid":"a", "componentScores":[0.9], "payload":{"id":"a"}},
                {"_rid":"b", "componentScores":[0.8], "payload":{"id":"b"}},
                {"_rid":"c", "componentScores":[0.7], "payload":{"id":"c"}}
            ]),
            1.0,
        );
        assert_eq!(
            node.collect_results(response.body()).unwrap_err().status(),
            status_codes::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }

    #[test]
    fn sparse_text_statistics_map_to_their_component_indices() {
        let info = HybridSearchQueryInfo {
            component_query_infos: vec![
                QueryInfo {
                    top: Some(2),
                    rewritten_query: Some("SELECT * FROM c".to_owned()),
                    ..Default::default()
                },
                QueryInfo {
                    top: Some(2),
                    rewritten_query: Some(
                        "SELECT {documentdb-formattablehybridsearchquery-totalwordcount-1} FROM c"
                            .to_owned(),
                    ),
                    ..Default::default()
                },
            ],
            ..plan()
        };
        let node = HybridSearch::new(
            operation(),
            &info,
            vec![epk_range_target().unwrap()],
            vec![epk_range_target().unwrap()],
            FullTextScoreScope::Local,
            0,
            2,
        )
        .unwrap();
        assert_eq!(node.statistic_indices, [None, Some(0)]);
        let stats = GlobalStatistics {
            document_count: 3,
            full_text_statistics: vec![FullTextStatistics {
                total_word_count: 7,
                hit_counts: vec![1],
            }],
        };
        assert_eq!(
            replace_statistics(
                &node.components[1].query,
                Some(&stats),
                &node.statistic_indices
            )
            .unwrap(),
            "SELECT 7 FROM c"
        );
    }

    #[test]
    fn fusion_uses_competition_ranks_for_ties_and_missing_scores() {
        let mut rows: Vec<RankedRow> = [
            ("a", vec![Some(1.0), Some(1.0)]),
            ("b", vec![Some(1.0), Some(0.0)]),
            ("c", vec![None, Some(1.0)]),
        ]
        .into_iter()
        .map(|(rid, scores)| RankedRow {
            rid: rid.to_owned(),
            scores,
            payload: serde_json::value::to_raw_value(&json!({"id": rid})).unwrap(),
            rank_score: 0.0,
        })
        .collect();
        rank_results(
            &mut rows,
            &[SortOrder::Descending, SortOrder::Descending],
            &[1.0, 2.0],
        );
        assert_eq!(
            rows.iter().map(|row| row.rid.as_str()).collect::<Vec<_>>(),
            ["a", "c", "b"]
        );
        assert!((rows[0].rank_score - 3.0 / (RRF_CONSTANT + 1.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn hybrid_admission_and_malformed_responses_fail_explicitly() {
        let mut info = plan();
        for (take, skip, maximum, valid) in [
            (Some(2), Some(1), 3, true),
            (Some(2), Some(2), 3, false),
            (None, None, u64::MAX, false),
            (Some(u64::MAX), Some(1), u64::MAX, false),
        ] {
            info.take = take;
            info.skip = skip;
            assert_eq!(hybrid_window(&info, maximum).is_ok(), valid);
        }
        assert_eq!(
            parse_result(
                serde_json::value::to_raw_value(&json!({
                    "_rid":"a", "componentScores":[0.5], "payload":{"id":"a"}
                }))
                .unwrap()
                .as_ref(),
                2
            )
            .err()
            .unwrap()
            .status(),
            status_codes::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
        assert_eq!(
            replace_statistics(
                "SELECT {documentdb-formattablehybridsearchquery-totalwordcount-0} FROM c",
                None,
                &[Some(0)]
            )
            .unwrap_err()
            .status(),
            status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
        );
        info.component_query_infos[0].top = None;
        assert!(HybridSearch::new(
            operation(),
            &info,
            vec![epk_range_target().unwrap()],
            vec![epk_range_target().unwrap()],
            FullTextScoreScope::Global,
            0,
            2,
        )
        .is_err());
    }

    #[test]
    fn statistics_reject_inconsistent_component_shapes() {
        let mut statistics = GlobalStatistics {
            document_count: 2,
            full_text_statistics: vec![FullTextStatistics {
                total_word_count: 5,
                hit_counts: vec![1, 2],
            }],
        };
        let other = GlobalStatistics {
            document_count: 1,
            full_text_statistics: vec![FullTextStatistics {
                total_word_count: 3,
                hit_counts: vec![1],
            }],
        };
        assert_eq!(
            statistics.add(other).unwrap_err().status(),
            status_codes::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }
}
