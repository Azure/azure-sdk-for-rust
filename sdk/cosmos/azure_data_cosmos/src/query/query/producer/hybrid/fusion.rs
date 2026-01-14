// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::collections::{BTreeSet, VecDeque};

use crate::{
    query::{
        producer::hybrid::{
            component_state::ComponentQueryState, models::ComponentQueryResult,
            PaginationParameters,
        },
        query_result::FeedResponse,
        QueryResult, SortOrder,
    },
    ErrorKind,
};

pub enum QueryResultCollector {
    /// Collects results from a single component query.
    /// There's no need to de-duplicate results in this case.
    Singleton(Vec<ComponentQueryResult>),

    /// Collects results from multiple component queries.
    /// Results must be de-duplicated based on their RID.
    Multiple(BTreeSet<ComponentQueryResult>),
}

impl QueryResultCollector {
    pub fn singleton() -> Self {
        QueryResultCollector::Singleton(Vec::new())
    }

    pub fn multiple() -> Self {
        QueryResultCollector::Multiple(BTreeSet::new())
    }

    pub fn len(&self) -> usize {
        match self {
            QueryResultCollector::Singleton(v) => v.len(),
            QueryResultCollector::Multiple(s) => s.len(),
        }
    }

    pub fn provide_data(&mut self, data: &[u8]) -> crate::Result<()> {
        let result: FeedResponse<ComponentQueryResult> =
            serde_json::from_slice(data).map_err(|e| {
                ErrorKind::DeserializationError.with_message(format!(
                    "failed to deserialize component query result: {}",
                    e
                ))
            })?;

        match self {
            QueryResultCollector::Singleton(v) => v.extend(result.documents),
            QueryResultCollector::Multiple(s) => {
                for item in result.documents {
                    #[cfg(debug_assertions)]
                    let scores = item.payload.component_scores.clone();

                    #[allow(unused_variables, reason = "used in debug assertions")]
                    if let Some(old) = s.replace(item) {
                        // Check that all the component scores are the same for duplicate items
                        // We do have to include `#[cfg(debug_assertions)]` here, even though `debug_assert_eq!` exists.
                        // `debug_assert_eq!` works using 'if cfg!(debug_assertions)', which means the code is still type-checked in release builds.
                        // And it would error out because it can't find `scores`, which is only defined in debug builds.
                        #[cfg(debug_assertions)]
                        assert_eq!(
                            old.payload.component_scores, scores,
                            "mismatched component scores for duplicate hybrid search result"
                        );
                    }
                }
            }
        }
        Ok(())
    }

    pub fn compute_final_results(
        self,
        pagination: PaginationParameters,
        component_queries: &[ComponentQueryState],
    ) -> crate::Result<VecDeque<QueryResult>> {
        match self {
            QueryResultCollector::Singleton(results) => Ok(pagination.paginate(results)),
            QueryResultCollector::Multiple(results) => {
                let scores = get_scores(component_queries, &results)?;
                let ranks = scores.into_ranks();
                let fused = ranks.into_fused_results(component_queries, results);
                Ok(pagination.paginate(fused))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScoreTuple {
    pub score: f64,
    pub document_index: usize,
}

pub struct ScoreListBuilder {
    scores: Vec<Vec<ScoreTuple>>,
    component_sort_orders: Vec<SortOrder>,
    document_count: usize,
}

impl ScoreListBuilder {
    pub fn new(component_sort_orders: Vec<SortOrder>, document_count: usize) -> Self {
        let lists = vec![Vec::with_capacity(document_count); component_sort_orders.len()];
        Self {
            scores: lists,
            component_sort_orders,
            document_count,
        }
    }

    pub fn push_score(
        &mut self,
        component_index: usize,
        score: f64,
        document_index: usize,
    ) -> crate::Result<()> {
        if let Some(list) = self.scores.get_mut(component_index) {
            list.push(ScoreTuple {
                score,
                document_index,
            });
            Ok(())
        } else {
            Err(ErrorKind::InternalError.with_message(format!(
                "component index {} out of bounds (max {})",
                component_index,
                self.scores.len()
            )))
        }
    }

    pub fn build(mut self) -> ScoreList {
        // Sort each component's scores according to its sort order
        for (index, tuples) in self.scores.iter_mut().enumerate() {
            let sort_order = self.component_sort_orders[index];
            tuples.sort_by(|a, b| {
                let ordering = a
                    .score
                    .partial_cmp(&b.score)
                    .unwrap_or(std::cmp::Ordering::Equal);
                match sort_order {
                    SortOrder::Ascending => ordering,
                    SortOrder::Descending => ordering.reverse(),
                }
            });
        }
        ScoreList {
            scores: self.scores,
            result_count: self.document_count,
        }
    }
}

pub struct ScoreList {
    scores: Vec<Vec<ScoreTuple>>,
    result_count: usize,
}

impl ScoreList {
    /// Converts the scores into ranks for each component.
    pub fn into_ranks(self) -> RankList {
        let mut ranks = vec![vec![0; self.result_count]; self.scores.len()];

        // The scores are in order, so all we have to do is assign ranks based on position.
        // But, two identical scores should receive the same rank.
        for (component_index, score_list) in self.scores.iter().enumerate() {
            let mut current_rank = 1;
            for i in 0..score_list.len() {
                const ERROR_MARGIN: f64 = 1e-10;
                if i > 0 && (score_list[i].score - score_list[i - 1].score).abs() > ERROR_MARGIN {
                    current_rank += 1;
                }
                ranks[component_index][score_list[i].document_index] = current_rank;
            }
        }

        RankList(ranks)
    }
}

pub struct RankList(Vec<Vec<usize>>);

impl RankList {
    const RRF_CONSTANT: f64 = 60.0;

    pub fn into_fused_results(
        self,
        components: &[ComponentQueryState],
        results: impl IntoIterator<Item = ComponentQueryResult>,
    ) -> Vec<ComponentQueryResult> {
        debug_assert_eq!(self.0.len(), components.len());
        let mut fused_results = Vec::new();

        for (index, result) in results.into_iter().enumerate() {
            let mut fused_score = 0.0;
            for (component_index, rank_list) in self.0.iter().enumerate() {
                let rank = rank_list[index] as f64;
                let weight = components[component_index].weight;
                fused_score += weight / (Self::RRF_CONSTANT + rank);
            }
            fused_results.push(RankFusionResult {
                fused_score,
                result,
            })
        }

        fused_results.sort_by(|a, b| {
            a.fused_score
                .partial_cmp(&b.fused_score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .reverse()
        });
        fused_results.into_iter().map(|r| r.result).collect()
    }
}

#[derive(Debug)]
struct RankFusionResult {
    fused_score: f64,
    result: ComponentQueryResult,
}

impl PartialEq for RankFusionResult {
    fn eq(&self, other: &Self) -> bool {
        self.fused_score == other.fused_score
    }
}

impl Eq for RankFusionResult {}

impl PartialOrd for RankFusionResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RankFusionResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Only compare by fused score - stable sort will handle ties
        other
            .fused_score
            .partial_cmp(&self.fused_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn get_scores(
    component_queries: &[ComponentQueryState],
    results: &BTreeSet<ComponentQueryResult>,
) -> crate::Result<ScoreList> {
    let sort_orders = component_queries
        .iter()
        .map(|cq| {
            cq.query_info
                .order_by
                .get(0)
                .copied()
                .unwrap_or(SortOrder::Descending)
        })
        .collect();

    let mut score_list = ScoreListBuilder::new(sort_orders, results.len());
    for (index, result) in results.iter().enumerate() {
        if result.payload.component_scores.len() != component_queries.len() {
            return Err(ErrorKind::InternalError.with_message(format!(
                "mismatched number of component scores in hybrid search result: expected {}, got {}",
                component_queries.len(),
                result.payload.component_scores.len()
            )));
        }

        for (component_index, score) in result.payload.component_scores.iter().copied().enumerate()
        {
            score_list.push_score(component_index, score, index)?;
        }
    }

    Ok(score_list.build())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use serde_json::value::RawValue;

    // Helper functions
    fn create_raw_payload(data: &str) -> Box<RawValue> {
        RawValue::from_string(data.to_string()).unwrap()
    }

    fn create_test_result(rid: &str, scores: Vec<f64>) -> ComponentQueryResult {
        ComponentQueryResult {
            rid: rid.to_string(),
            payload: crate::query::producer::hybrid::models::ComponentQueryPayload {
                component_scores: scores,
                user_payload: create_raw_payload(r#"{"test": "data"}"#),
            },
        }
    }

    fn create_mock_component_state(weight: f64) -> ComponentQueryState {
        // Create a minimal component state for testing
        // We only need the weight field for RRF calculations
        let query_info = crate::query::QueryInfo::default();

        ComponentQueryState::new(0, query_info, weight, &["partition1".to_string()])
    }

    // Priority 1: Score-to-rank conversion and RRF calculation
    #[test]
    fn test_score_list_into_ranks_with_ties() {
        let mut builder = ScoreListBuilder::new(
            vec![
                SortOrder::Descending,
                SortOrder::Ascending,
                SortOrder::Ascending,
            ],
            5,
        );

        // Component 0: Descending with ties.
        // Inverted order to test that the ranks are in document order.
        builder.push_score(0, 100.0, 4).unwrap();
        builder.push_score(0, 90.0, 3).unwrap();
        builder.push_score(0, 90.0, 2).unwrap();
        builder.push_score(0, 80.0, 1).unwrap();
        builder.push_score(0, 70.0, 0).unwrap();

        // Component 1: Ascending without ties.
        builder.push_score(1, 10.0, 4).unwrap();
        builder.push_score(1, 20.0, 3).unwrap();
        builder.push_score(1, 30.0, 2).unwrap();
        builder.push_score(1, 40.0, 1).unwrap();
        builder.push_score(1, 50.0, 0).unwrap();

        // Component 2: Ascending with ties.
        builder.push_score(2, 5.0, 4).unwrap();
        builder.push_score(2, 5.0, 3).unwrap();
        builder.push_score(2, 15.0, 2).unwrap();
        builder.push_score(2, 25.0, 1).unwrap();
        builder.push_score(2, 35.0, 0).unwrap();

        let score_list = builder.build();
        let rank_list = score_list.into_ranks();
        assert_eq!(
            vec![
                vec![4, 3, 2, 2, 1], // Component 0 ranks
                vec![5, 4, 3, 2, 1], // Component 1 ranks
                vec![4, 3, 2, 1, 1], // Component 2 ranks
            ],
            rank_list.0
        );
    }

    #[test]
    fn test_rrf_calculation() {
        let components = vec![
            create_mock_component_state(1.0),
            create_mock_component_state(2.0),
            create_mock_component_state(0.0),
            create_mock_component_state(0.5),
        ];

        // Create rank lists for 4 components and 5 documents which demonstrate that each component's ranks are used correctly.
        // The rankings here should result in an inverse ordering of the documents
        let rank_list = RankList(vec![
            vec![1, 2, 3, 4, 5], // Component 0 ranks
            vec![5, 4, 3, 2, 1], // Component 1 ranks
            vec![1, 1, 1, 1, 1], // Component 2 ranks (should not affect fused score)
            vec![2, 2, 2, 2, 2], // Component 3 ranks
        ]);

        // Create the 5 results. The scores are irrelevant here because we've already computed artificial ranks
        let results = vec![
            create_test_result("doc1", vec![100.0, 80.0, 50.0, 70.0]),
            create_test_result("doc2", vec![90.0, 85.0, 60.0, 65.0]),
            create_test_result("doc3", vec![70.0, 70.0, 70.0, 60.0]),
            create_test_result("doc4", vec![60.0, 75.0, 80.0, 55.0]),
            create_test_result("doc5", vec![50.0, 90.0, 90.0, 50.0]),
        ];

        let fused = rank_list.into_fused_results(&components, results);

        // Validate the ordering of fused results based on expected fused scores
        assert_eq!(
            vec!["doc5", "doc4", "doc3", "doc2", "doc1"],
            fused.iter().map(|r| r.rid.as_str()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_score_tuple_ordering() {
        // Create a score list with out-of-order scores and validate the final list is ordered by score
        let mut builder = ScoreListBuilder::new(vec![SortOrder::Descending], 5);
        builder.push_score(0, 50.0, 0).unwrap();
        builder.push_score(0, 70.0, 1).unwrap();
        builder.push_score(0, 30.0, 2).unwrap();
        builder.push_score(0, 100.0, 3).unwrap();
        builder.push_score(0, 90.0, 4).unwrap();

        let score_list = builder.build();

        assert_eq!(
            vec![vec![
                ScoreTuple {
                    score: 100.0,
                    document_index: 3
                },
                ScoreTuple {
                    score: 90.0,
                    document_index: 4
                },
                ScoreTuple {
                    score: 70.0,
                    document_index: 1
                },
                ScoreTuple {
                    score: 50.0,
                    document_index: 0
                },
                ScoreTuple {
                    score: 30.0,
                    document_index: 2
                },
            ]],
            score_list.scores
        )
    }
}
