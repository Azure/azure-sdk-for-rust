// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::collections::VecDeque;

use serde::Deserialize;

mod component_state;
mod fusion;
mod models;

use crate::{
    query::{
        node::PipelineNodeResult, plan::HybridSearchQueryInfo, DataRequest, PartitionKeyRange,
        QueryResult,
    },
    ErrorKind,
};

use component_state::ComponentQueryState;
use fusion::QueryResultCollector;
use models::{ComponentQueryResult, GlobalStatistics, HybridRequestId};

enum HybridSearchPhase {
    IssuingGlobalStatisticsQuery,
    AwaitingGlobalStatistics {
        aggregated_global_statistics: Option<GlobalStatistics>,
        remaining_partitions: usize,
    },
    ComponentQueries {
        remaining_component_queries: usize,
        results: QueryResultCollector,
    },
    ResultProduction(VecDeque<QueryResult>),
}

impl HybridSearchPhase {
    pub fn for_component_queries(count: usize) -> Self {
        if count == 1 {
            HybridSearchPhase::ComponentQueries {
                remaining_component_queries: 1,
                results: QueryResultCollector::singleton(),
            }
        } else {
            HybridSearchPhase::ComponentQueries {
                remaining_component_queries: count,
                results: QueryResultCollector::multiple(),
            }
        }
    }
}

impl std::fmt::Debug for HybridSearchPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HybridSearchPhase::IssuingGlobalStatisticsQuery => {
                f.debug_struct("IssuingGlobalStatisticsQuery").finish()
            }
            HybridSearchPhase::AwaitingGlobalStatistics {
                aggregated_global_statistics,
                remaining_partitions,
            } => f
                .debug_struct("AwaitingGlobalStatistics")
                .field("aggregated_global_statistics", aggregated_global_statistics)
                .field("remaining_partitions", remaining_partitions)
                .finish(),
            HybridSearchPhase::ComponentQueries {
                remaining_component_queries,
                results,
            } => f
                .debug_struct("ComponentQueries")
                .field("remaining_component_queries", remaining_component_queries)
                .field("results_count", &results.len())
                .finish(),
            HybridSearchPhase::ResultProduction(results) => f
                .debug_struct("ResultProduction")
                .field("results_count", &results.len())
                .finish(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PaginationParameters {
    skip: u64,
    take: u64,
}

impl PaginationParameters {
    pub fn paginate(
        self,
        results: impl IntoIterator<Item = ComponentQueryResult>,
    ) -> VecDeque<QueryResult> {
        results
            .into_iter()
            .skip(self.skip as usize)
            .take(self.take as usize)
            .map(|r| QueryResult::RawPayload(r.payload.user_payload))
            .collect()
    }
}

#[derive(Debug)]
pub struct HybridSearchStrategy {
    global_statistics_query: String,
    phase: HybridSearchPhase,
    pkrange_ids: Vec<String>,
    component_queries: Vec<ComponentQueryState>,
    pagination: PaginationParameters,
}

impl HybridSearchStrategy {
    pub fn new(
        pkranges: impl IntoIterator<Item = PartitionKeyRange>,
        query_info: HybridSearchQueryInfo,
    ) -> crate::Result<Self> {
        let phase = if query_info.requires_global_statistics {
            HybridSearchPhase::IssuingGlobalStatisticsQuery
        } else {
            HybridSearchPhase::for_component_queries(query_info.component_query_infos.len())
        };
        let pkrange_ids: Vec<String> = pkranges.into_iter().map(|p| p.id).collect();

        let component_queries = query_info
            .component_query_infos
            .into_iter()
            .enumerate()
            .map(|(i, q)| {
                ComponentQueryState::new(
                    i as u32,
                    q,
                    query_info.component_weights.get(i).copied().unwrap_or(1.0),
                    &pkrange_ids,
                )
            })
            .collect();
        Ok(Self {
            global_statistics_query: query_info.global_statistics_query,
            phase,
            pkrange_ids,
            component_queries,
            pagination: PaginationParameters {
                skip: query_info.skip.unwrap_or(0),
                take: query_info.take.ok_or_else(|| {
                    ErrorKind::InvalidQuery
                        .with_message("hybrid search query must include take parameter")
                })?,
            },
        })
    }

    pub fn requests(&mut self) -> crate::Result<Vec<DataRequest>> {
        match self.phase {
            HybridSearchPhase::IssuingGlobalStatisticsQuery => {
                let requests = self
                    .pkrange_ids
                    .iter()
                    .map(|pkrange_id| {
                        DataRequest::with_query(
                            HybridRequestId::GLOBAL_STATISTICS_QUERY_ID.into(),
                            pkrange_id.clone(),
                            None,
                            self.global_statistics_query.clone(),
                            true,
                        )
                    })
                    .collect::<Vec<_>>();
                self.phase = HybridSearchPhase::AwaitingGlobalStatistics {
                    aggregated_global_statistics: None,
                    remaining_partitions: self.pkrange_ids.len(),
                };
                Ok(requests)
            }
            HybridSearchPhase::AwaitingGlobalStatistics { .. } => Err(ErrorKind::InternalError
                .with_message("no requests should be made in AwaitingGlobalStatistics phase")),
            HybridSearchPhase::ComponentQueries { .. } => {
                let mut requests = Vec::new();
                for query_state in &self.component_queries {
                    let query_requests = query_state.requests();
                    requests.extend(query_requests);
                }
                Ok(requests)
            }
            // No more requests should be made once we are producing results, but it's not an error to check for them.
            HybridSearchPhase::ResultProduction(_) => Ok(Vec::new()),
        }
    }

    pub fn provide_data(
        &mut self,
        pkrange_id: &str,
        request_id: u64,
        data: &[u8],
        continuation: Option<String>,
    ) -> Result<(), crate::Error> {
        let request_id = HybridRequestId::from(request_id);
        match self.phase {
            HybridSearchPhase::IssuingGlobalStatisticsQuery => Err(ErrorKind::InternalError
                .with_message(
                    "provide_data should not be called in IssuingGlobalStatisticsQuery phase",
                )),
            HybridSearchPhase::AwaitingGlobalStatistics {
                ref mut aggregated_global_statistics,
                ref mut remaining_partitions,
            } => {
                if request_id != HybridRequestId::GLOBAL_STATISTICS_QUERY_ID {
                    return Err(ErrorKind::InvalidGatewayResponse
                        .with_message("expected global statistics query response"));
                }

                #[derive(Deserialize)]
                struct GlobalStatisticsResult {
                    #[serde(rename = "Documents")]
                    documents: Vec<GlobalStatistics>,
                }
                let results =
                    serde_json::from_slice::<GlobalStatisticsResult>(data).map_err(|e| {
                        ErrorKind::DeserializationError.with_message(format!(
                            "failed to deserialize global statistics result: {}",
                            e
                        ))
                    })?;

                if results.documents.len() != 1 {
                    return Err(ErrorKind::InvalidGatewayResponse
                        .with_message("global statistics query should have only one item"));
                }
                let stats = results
                    .documents
                    .into_iter()
                    .next()
                    .expect("we just checked the length");
                tracing::trace!(
                    ?stats,
                    pkrange_id,
                    "received global statistics for hybrid search"
                );
                let global_statistics = match aggregated_global_statistics.take() {
                    None => stats,
                    Some(existing_stats) => existing_stats.aggregate_with(stats)?,
                };
                *remaining_partitions -= 1;
                if *remaining_partitions == 0 {
                    // We've received all the global statistics results.
                    // Rewrite component queries with aggregated global statistics
                    tracing::debug!(
                        "received all global statistics results, rewriting component queries"
                    );
                    self.phase =
                        HybridSearchPhase::for_component_queries(self.component_queries.len());

                    for query_state in &mut self.component_queries {
                        global_statistics.rewrite_component_query(&mut query_state.query_info)?;
                    }
                } else {
                    self.phase = HybridSearchPhase::AwaitingGlobalStatistics {
                        aggregated_global_statistics: Some(global_statistics),
                        remaining_partitions: *remaining_partitions,
                    };
                }
                Ok(())
            }
            HybridSearchPhase::ComponentQueries {
                ref mut remaining_component_queries,
                ref mut results,
            } => {
                let query_index = request_id.query_index().ok_or_else(|| {
                    ErrorKind::InvalidRequestId.with_message("expected component query request ID")
                })?;
                tracing::trace!(
                    query_index,
                    pkrange_id,
                    "providing data for component query"
                );

                let component_query = self
                    .component_queries
                    .get_mut(query_index as usize)
                    .ok_or_else(|| {
                        ErrorKind::InvalidRequestId
                            .with_message("invalid component query index in request ID")
                    })?;
                component_query.update_partition_state(pkrange_id, continuation)?;
                results.provide_data(data)?;
                if component_query.complete() {
                    *remaining_component_queries -= 1;
                }
                if *remaining_component_queries == 0 {
                    tracing::debug!("all component queries complete");

                    // Swap out the results collector to take ownership of the results.
                    // A brand new singleton collector contains only an empty vector (no heap allocation) so it's cheap to create.
                    let results = std::mem::replace(results, QueryResultCollector::singleton());

                    // Process the results and move to result production
                    let results =
                        results.compute_final_results(self.pagination, &self.component_queries)?;
                    self.phase = HybridSearchPhase::ResultProduction(results);
                }
                Ok(())
            }
            HybridSearchPhase::ResultProduction(_) => Err(ErrorKind::InternalError
                .with_message("provide_data should not be called in ResultProduction phase")),
        }
    }

    pub fn produce_item(&mut self) -> crate::Result<PipelineNodeResult> {
        if let HybridSearchPhase::ResultProduction(ref mut results) = self.phase {
            if let Some(item) = results.pop_front() {
                tracing::debug!("producing hybrid search result item");
                Ok(PipelineNodeResult::result(item.clone(), results.is_empty()))
            } else {
                tracing::debug!("no more hybrid search result items to produce");
                Ok(PipelineNodeResult {
                    value: None,
                    terminated: true,
                })
            }
        } else {
            tracing::debug!(
                "cannot produce items until in ResultProduction phase, current phase: {:?}",
                self.phase
            );
            Ok(PipelineNodeResult::NO_RESULT)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::{plan::HybridSearchQueryInfo, QueryInfo};
    use models::{FullTextStatistics, GlobalStatistics};
    use pretty_assertions::assert_eq;

    impl PartialEq for HybridSearchPhase {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::IssuingGlobalStatisticsQuery, Self::IssuingGlobalStatisticsQuery) => true,
                (
                    Self::AwaitingGlobalStatistics {
                        aggregated_global_statistics: a1,
                        remaining_partitions: r1,
                    },
                    Self::AwaitingGlobalStatistics {
                        aggregated_global_statistics: a2,
                        remaining_partitions: r2,
                    },
                ) => a1 == a2 && r1 == r2,
                (
                    Self::ComponentQueries {
                        remaining_component_queries: r1,
                        results: _,
                    },
                    Self::ComponentQueries {
                        remaining_component_queries: r2,
                        results: _,
                    },
                ) => r1 == r2,
                (Self::ResultProduction(r1), Self::ResultProduction(r2)) => r1.len() == r2.len(),
                _ => false,
            }
        }
    }

    impl PartialEq for PaginationParameters {
        fn eq(&self, other: &Self) -> bool {
            self.skip == other.skip && self.take == other.take
        }
    }

    fn create_test_query_info(query: &str) -> QueryInfo {
        QueryInfo {
            rewritten_query: query.to_string(),
            ..Default::default()
        }
    }

    fn create_hybrid_query_info(
        requires_global_stats: bool,
        component_count: usize,
        take: Option<u64>,
    ) -> HybridSearchQueryInfo {
        HybridSearchQueryInfo {
            global_statistics_query: "SELECT COUNT(c) as documentCount FROM c".to_string(),
            component_query_infos: (0..component_count)
                .map(|i| create_test_query_info(&format!(
                    "SELECT * FROM c WHERE c.type = {} AND c.docCount = {{documentdb-formattablehybridsearchquery-totaldocumentcount}}", 
                    i
                )))
                .collect(),
            component_weights: vec![1.0; component_count],
            skip: Some(0),
            take,
            requires_global_statistics: requires_global_stats,
        }
    }

    fn create_test_pkranges(count: usize) -> Vec<PartitionKeyRange> {
        (0..count)
            .map(|i| PartitionKeyRange {
                id: format!("partition_{}", i),
                min_inclusive: "00".to_string(),
                max_exclusive: "FF".to_string(),
            })
            .collect()
    }

    fn create_global_stats(doc_count: u64) -> GlobalStatistics {
        GlobalStatistics {
            document_count: doc_count,
            full_text_statistics: vec![FullTextStatistics {
                total_word_count: 100,
                hit_counts: vec![10, 20, 30],
            }],
        }
    }

    fn create_global_stats_response(stats: &GlobalStatistics) -> Vec<u8> {
        let response = serde_json::json!({
            "Documents": [stats]
        });
        serde_json::to_vec(&response).unwrap()
    }

    fn force_strategy_to_phase(strategy: &mut HybridSearchStrategy, phase: HybridSearchPhase) {
        strategy.phase = phase;
    }

    #[test]
    fn test_global_statistics_to_component_queries_transition() {
        let pkranges = create_test_pkranges(2);
        let query_info = create_hybrid_query_info(true, 2, Some(10));
        let mut strategy = HybridSearchStrategy::new(pkranges, query_info).unwrap();
        assert_eq!(
            strategy.phase,
            HybridSearchPhase::IssuingGlobalStatisticsQuery
        );

        let requests = strategy.requests().unwrap();
        assert_eq!(
            vec![
                DataRequest::with_query(
                    HybridRequestId::GLOBAL_STATISTICS_QUERY_ID.into(),
                    "partition_0".to_string(),
                    None,
                    strategy.global_statistics_query.clone(),
                    true,
                ),
                DataRequest::with_query(
                    HybridRequestId::GLOBAL_STATISTICS_QUERY_ID.into(),
                    "partition_1".to_string(),
                    None,
                    strategy.global_statistics_query.clone(),
                    true,
                ),
            ],
            requests
        );

        assert_eq!(
            strategy.phase,
            HybridSearchPhase::AwaitingGlobalStatistics {
                aggregated_global_statistics: None,
                remaining_partitions: 2
            }
        );

        let stats1 = create_global_stats(100);
        let response1 = create_global_stats_response(&stats1);
        strategy
            .provide_data("partition_0", 0, &response1, None)
            .unwrap();
        assert_eq!(
            strategy.phase,
            HybridSearchPhase::AwaitingGlobalStatistics {
                aggregated_global_statistics: Some(stats1.clone()),
                remaining_partitions: 1
            }
        );

        let stats2 = create_global_stats(200);
        let response2 = create_global_stats_response(&stats2);
        strategy
            .provide_data("partition_1", 0, &response2, None)
            .unwrap();
        assert_eq!(
            strategy.phase,
            HybridSearchPhase::ComponentQueries {
                remaining_component_queries: 2,
                results: QueryResultCollector::multiple()
            }
        );

        for component_query in &strategy.component_queries {
            assert_eq!(
                component_query.query_info.rewritten_query,
                format!(
                    "SELECT * FROM c WHERE c.type = {} AND c.docCount = 300",
                    component_query.query_index
                )
            )
            // doc_count = 100 + 200
        }
    }

    #[test]
    fn test_component_queries_to_result_production_transition() {
        let pkranges = create_test_pkranges(1);
        let query_info = create_hybrid_query_info(false, 2, Some(10));
        let mut strategy = HybridSearchStrategy::new(pkranges, query_info).unwrap();
        strategy.phase = HybridSearchPhase::ComponentQueries {
            remaining_component_queries: 2,
            results: QueryResultCollector::multiple(),
        };

        let component_response = serde_json::json!({
            "Documents": [{
                "_rid": "doc1",
                "payload": {
                    "componentScores": [0.5, 0.3],
                    "payload": {"test": "data1"}
                }
            }]
        });
        strategy
            .provide_data(
                "partition_0",
                (0u64 << 32) | 1u64,
                &serde_json::to_vec(&component_response).unwrap(),
                None,
            )
            .unwrap();

        assert_eq!(
            strategy.phase,
            HybridSearchPhase::ComponentQueries {
                remaining_component_queries: 1,
                results: QueryResultCollector::multiple()
            }
        );

        strategy
            .provide_data(
                "partition_0",
                (1u64 << 32) | 1u64,
                &serde_json::to_vec(&component_response).unwrap(),
                None,
            )
            .unwrap();

        if let HybridSearchPhase::ResultProduction(results) = &strategy.phase {
            assert_eq!(
                results.len(),
                1,
                "expected one result after processing both component queries"
            );
            assert_eq!(
                r#"{"test":"data1"}"#,
                results[0].as_raw_payload().unwrap().get().to_string()
            );
        } else {
            panic!("expected strategy to be in ResultProduction phase");
        }
        matches!(strategy.phase, HybridSearchPhase::ResultProduction(_));
    }

    #[test]
    fn test_skip_global_statistics_path() {
        let pkranges = create_test_pkranges(2);
        let query_info = create_hybrid_query_info(false, 1, Some(10));
        let strategy = HybridSearchStrategy::new(pkranges, query_info).unwrap();
        assert_eq!(
            strategy.phase,
            HybridSearchPhase::ComponentQueries {
                remaining_component_queries: 1,
                results: QueryResultCollector::singleton()
            }
        );

        // Query isn't rewritten. In our case this means the placeholders are still present because we always put placeholders in our test query.
        assert_eq!(
            vec![
                "SELECT * FROM c WHERE c.type = 0 AND c.docCount = {documentdb-formattablehybridsearchquery-totaldocumentcount}".to_string()
            ],
            strategy.component_queries.iter().map(|q| q.query_info.rewritten_query.clone()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_mixed_component_query_completion() {
        let pkranges = create_test_pkranges(1);
        let query_info = create_hybrid_query_info(false, 3, Some(10));
        let mut strategy = HybridSearchStrategy::new(pkranges, query_info).unwrap();

        // Force into component queries phase
        force_strategy_to_phase(
            &mut strategy,
            HybridSearchPhase::ComponentQueries {
                remaining_component_queries: 3,
                results: QueryResultCollector::multiple(),
            },
        );

        // Complete component queries in non-sequential order
        let response = serde_json::json!({
            "Documents": [{
                "_rid": "doc1",
                "payload": {
                    "componentScores": [0.5, 0.3, 0.8],
                    "payload": {"test": "data"}
                }
            }]
        });

        // Complete query 1 (middle)
        strategy
            .provide_data(
                "partition_0",
                (1u64 << 32) | 1u64,
                &serde_json::to_vec(&response).unwrap(),
                None,
            )
            .unwrap();
        assert_eq!(
            strategy.phase,
            HybridSearchPhase::ComponentQueries {
                remaining_component_queries: 2,
                results: QueryResultCollector::multiple()
            }
        );

        // Complete query 0 (first)
        strategy
            .provide_data(
                "partition_0",
                (0u64 << 32) | 1u64,
                &serde_json::to_vec(&response).unwrap(),
                None,
            )
            .unwrap();
        assert_eq!(
            strategy.phase,
            HybridSearchPhase::ComponentQueries {
                remaining_component_queries: 1,
                results: QueryResultCollector::multiple()
            }
        );

        // Complete query 2 (last)
        strategy
            .provide_data(
                "partition_0",
                (2u64 << 32) | 1u64,
                &serde_json::to_vec(&response).unwrap(),
                None,
            )
            .unwrap();

        // Should transition to result production
        matches!(strategy.phase, HybridSearchPhase::ResultProduction(_));
    }

    #[test]
    fn test_component_queries_request_generation() {
        let pkranges = create_test_pkranges(2);
        let query_info = create_hybrid_query_info(false, 2, Some(10));
        let mut strategy = HybridSearchStrategy::new(pkranges, query_info).unwrap();

        let requests = strategy.requests().unwrap();

        // Should generate 2 partitions × 2 component queries = 4 requests
        assert_eq!(
            vec![
                DataRequest::with_query(
                    HybridRequestId::for_component_query(0, 0).into(),
                    "partition_0".to_string(),
                    None,
                    strategy.component_queries[0]
                        .query_info
                        .rewritten_query
                        .clone(),
                    true,
                ),
                DataRequest::with_query(
                    HybridRequestId::for_component_query(0, 0).into(),
                    "partition_1".to_string(),
                    None,
                    strategy.component_queries[0]
                        .query_info
                        .rewritten_query
                        .clone(),
                    true,
                ),
                DataRequest::with_query(
                    HybridRequestId::for_component_query(1, 0).into(),
                    "partition_0".to_string(),
                    None,
                    strategy.component_queries[1]
                        .query_info
                        .rewritten_query
                        .clone(),
                    true,
                ),
                DataRequest::with_query(
                    HybridRequestId::for_component_query(1, 0).into(),
                    "partition_1".to_string(),
                    None,
                    strategy.component_queries[1]
                        .query_info
                        .rewritten_query
                        .clone(),
                    true,
                ),
            ],
            requests
        );
    }

    #[test]
    fn test_no_requests_in_result_production() {
        let pkranges = create_test_pkranges(1);
        let query_info = create_hybrid_query_info(false, 1, Some(10));
        let mut strategy = HybridSearchStrategy::new(pkranges, query_info).unwrap();
        strategy.phase = HybridSearchPhase::ResultProduction(VecDeque::new());

        let requests = strategy.requests().unwrap();
        assert_eq!(requests.len(), 0);
    }

    #[test]
    fn test_result_production_flow() {
        let pkranges = create_test_pkranges(1);
        let query_info = create_hybrid_query_info(false, 1, Some(10));
        let mut strategy = HybridSearchStrategy::new(pkranges, query_info).unwrap();

        let mut results = VecDeque::new();
        results.push_back(QueryResult::RawPayload(
            serde_json::value::RawValue::from_string(r#"{"data": "test1"}"#.to_string()).unwrap(),
        ));
        results.push_back(QueryResult::RawPayload(
            serde_json::value::RawValue::from_string(r#"{"data": "test2"}"#.to_string()).unwrap(),
        ));
        strategy.phase = HybridSearchPhase::ResultProduction(results);

        let result1 = strategy.produce_item().unwrap();
        assert_eq!(
            r#"{"data": "test1"}"#,
            result1
                .value
                .unwrap()
                .as_raw_payload()
                .unwrap()
                .get()
                .to_string()
        );
        assert!(!result1.terminated);

        let result2 = strategy.produce_item().unwrap();
        assert_eq!(
            r#"{"data": "test2"}"#,
            result2
                .value
                .unwrap()
                .as_raw_payload()
                .unwrap()
                .get()
                .to_string()
        );
        assert!(result2.terminated);

        let result3 = strategy.produce_item().unwrap();
        assert!(result3.value.is_none());
        assert!(result3.terminated);
    }

    #[test]
    fn test_item_production_outside_result_phase() {
        let pkranges = create_test_pkranges(1);
        let query_info = create_hybrid_query_info(false, 1, Some(10));
        let mut strategy = HybridSearchStrategy::new(pkranges, query_info).unwrap();

        let result = strategy.produce_item().unwrap();
        assert!(result.value.is_none());
        assert!(!result.terminated);
    }

    #[test]
    fn test_component_weight_defaults() {
        let pkranges = create_test_pkranges(1);
        let mut query_info = create_hybrid_query_info(false, 3, Some(10));
        query_info.component_weights = vec![2.0]; // Only one weight for three queries

        let strategy = HybridSearchStrategy::new(pkranges, query_info).unwrap();

        // First component should have explicit weight, others should default to 1.0
        assert_eq!(strategy.component_queries[0].weight, 2.0);
        assert_eq!(strategy.component_queries[1].weight, 1.0);
        assert_eq!(strategy.component_queries[2].weight, 1.0);
    }

    #[test]
    fn test_singleton_vs_multiple_collector() {
        let pkranges = create_test_pkranges(1);

        // Single component query should use singleton collector
        let query_info_single = create_hybrid_query_info(false, 1, Some(10));
        let strategy_single =
            HybridSearchStrategy::new(pkranges.clone(), query_info_single).unwrap();
        assert_eq!(
            strategy_single.phase,
            HybridSearchPhase::ComponentQueries {
                remaining_component_queries: 1,
                results: QueryResultCollector::singleton()
            }
        );

        // Multiple component queries should use multiple collector
        let query_info_multiple = create_hybrid_query_info(false, 2, Some(10));
        let strategy_multiple = HybridSearchStrategy::new(pkranges, query_info_multiple).unwrap();
        assert_eq!(
            strategy_multiple.phase,
            HybridSearchPhase::ComponentQueries {
                remaining_component_queries: 2,
                results: QueryResultCollector::multiple()
            }
        );
    }
}
