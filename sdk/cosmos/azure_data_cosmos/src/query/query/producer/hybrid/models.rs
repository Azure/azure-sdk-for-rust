// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// cSpell:ignore formattablehybridsearchquery formattableorderbyquery hitcountsarray totalwordcount totaldocumentcount

use serde::{Deserialize, Serialize};

use crate::{query::QueryInfo, ErrorKind};

/// A unique identifier for a hybrid search query request.
///
/// In order to correlate incoming responses to the appropriate query, we encode both the component query index and the page number
/// into a single u64 value. We start the page number at 1 to distinguish between
/// global statistics queries (which have an index of 0) and component queries.
///
/// We use the high 32 bits for the partition key range index and the low 32 bits for the component query index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HybridRequestId(u64);

impl From<u64> for HybridRequestId {
    fn from(value: u64) -> Self {
        HybridRequestId(value)
    }
}

impl Into<u64> for HybridRequestId {
    fn into(self) -> u64 {
        self.0
    }
}

impl HybridRequestId {
    pub const GLOBAL_STATISTICS_QUERY_ID: HybridRequestId = HybridRequestId(0);

    /// Creates a request ID for a component query.
    pub fn for_component_query(query_index: u32, page_number: u32) -> Self {
        if page_number == u32::MAX {
            // We can't represent u32::MAX + 1 in the low 32 bits, as it would wrap to 0.
            panic!("page_number cannot be u32::MAX");
        }

        let id = ((query_index as u64) << 32) | (page_number as u64 + 1);
        HybridRequestId(id)
    }

    /// Gets the query index from the request ID, if applicable.
    pub fn query_index(&self) -> Option<u32> {
        if self.0 == 0 {
            None
        } else {
            Some((self.0 >> 32) as u32)
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalStatistics {
    pub document_count: u64,
    pub full_text_statistics: Vec<FullTextStatistics>,
}

const TOTAL_DOCUMENT_COUNT: &str = "{documentdb-formattablehybridsearchquery-totaldocumentcount}";
const FORMATTABLE_ORDER_BY: &str = "{documentdb-formattableorderbyquery-filter}";

impl GlobalStatistics {
    pub fn aggregate_with(mut self, stats: GlobalStatistics) -> crate::Result<GlobalStatistics> {
        self.document_count += stats.document_count;
        if self.full_text_statistics.len() != stats.full_text_statistics.len() {
            return Err(ErrorKind::InvalidGatewayResponse
                .with_message("mismatched full text statistics length during aggregation"));
        }
        for (a, b) in self
            .full_text_statistics
            .iter_mut()
            .zip(stats.full_text_statistics.iter())
        {
            a.total_word_count += b.total_word_count;
            if a.hit_counts.len() != b.hit_counts.len() {
                return Err(ErrorKind::InvalidGatewayResponse
                    .with_message("mismatched hit counts length during aggregation"));
            }
            for (hit_a, hit_b) in a.hit_counts.iter_mut().zip(b.hit_counts.iter()) {
                *hit_a += *hit_b;
            }
        }
        Ok(self)
    }

    pub fn rewrite_component_query(&self, query: &mut QueryInfo) -> crate::Result<()> {
        for order_by_expression in &mut query.order_by_expressions {
            *order_by_expression = self.apply_to_query_template(order_by_expression)?;
        }
        query.rewritten_query = self.apply_to_query_template(&query.rewritten_query)?;
        Ok(())
    }

    fn apply_to_query_template(&self, query: &str) -> crate::Result<String> {
        // Shortcut for empty query
        if query.is_empty() {
            return Ok(String::new());
        }

        let mut rewritten_query = None;
        for (i, stats) in self.full_text_statistics.iter().enumerate() {
            let total_word_count_placeholder = format!(
                "{{documentdb-formattablehybridsearchquery-totalwordcount-{}}}",
                i
            );
            let hit_counts_array_placeholder = format!(
                "{{documentdb-formattablehybridsearchquery-hitcountsarray-{}}}",
                i
            );

            let hit_counts = stats
                .hit_counts
                .iter()
                .map(|count| count.to_string())
                .collect::<Vec<_>>()
                .join(",");

            let input_query = rewritten_query.as_deref().unwrap_or(query);
            let new_query = input_query
                .replace(
                    &total_word_count_placeholder,
                    &stats.total_word_count.to_string(),
                )
                .replace(&hit_counts_array_placeholder, &format!("[{}]", hit_counts));
            rewritten_query = Some(new_query);
        }

        let input_query = rewritten_query.as_deref().unwrap_or(query);
        let final_query = input_query
            .replace(TOTAL_DOCUMENT_COUNT, &self.document_count.to_string())
            .replace(FORMATTABLE_ORDER_BY, "true");
        tracing::trace!(final_query = ?final_query, "rewrote hybrid search query to incorporate global statistics");
        Ok(final_query)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullTextStatistics {
    pub total_word_count: u64,
    pub hit_counts: Vec<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentQueryResult {
    #[serde(rename = "_rid")]
    pub rid: String,
    pub payload: ComponentQueryPayload,
}

// Implement ordering, and equality based on the rid field only.
impl PartialOrd for ComponentQueryResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.rid.cmp(&other.rid))
    }
}

impl Ord for ComponentQueryResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rid.cmp(&other.rid)
    }
}

impl PartialEq for ComponentQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.rid == other.rid
    }
}

impl Eq for ComponentQueryResult {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentQueryPayload {
    pub component_scores: Vec<f64>,
    #[serde(rename = "payload")]
    pub user_payload: Box<serde_json::value::RawValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::QueryInfo;

    // Helper functions
    fn assert_request_id_encoding(query_index: u32, page_number: u32, expected_u64: u64) {
        let request_id = HybridRequestId::for_component_query(query_index, page_number);
        let actual_u64: u64 = request_id.into();
        assert_eq!(actual_u64, expected_u64);
        assert_eq!(request_id.query_index(), Some(query_index));
    }

    fn assert_request_id_decoding(encoded_u64: u64, expected_query_index: u32) {
        let request_id = HybridRequestId::from(encoded_u64);
        assert_eq!(request_id.query_index(), Some(expected_query_index));
    }

    fn create_test_payload(data: &str) -> ComponentQueryPayload {
        ComponentQueryPayload {
            component_scores: vec![1.0],
            user_payload: serde_json::value::RawValue::from_string(format!(
                r#"{{"data": "{}"}}"#,
                data
            ))
            .unwrap(),
        }
    }

    fn create_test_result(rid: &str, data: &str) -> ComponentQueryResult {
        ComponentQueryResult {
            rid: rid.to_string(),
            payload: create_test_payload(data),
        }
    }

    #[test]
    fn test_hybrid_request_id_basic_encoding() {
        assert_request_id_encoding(5, 2, (5u64 << 32) | 3u64);
        assert_request_id_encoding(0, 0, 1u64);
        assert_request_id_encoding(255, 100, (255u64 << 32) | 101u64);
    }

    #[test]
    fn test_hybrid_request_id_edge_case_encoding() {
        // u32::MAX + 1 wraps to 0, so low 32 bits = 0
        assert_request_id_encoding(u32::MAX, u32::MAX - 1, u64::MAX);
    }

    #[test]
    fn test_hybrid_request_id_global_statistics() {
        let global_id_u64: u64 = HybridRequestId::GLOBAL_STATISTICS_QUERY_ID.into();
        assert_eq!(global_id_u64, 0u64);
        assert_eq!(
            HybridRequestId::GLOBAL_STATISTICS_QUERY_ID.query_index(),
            None
        );
    }

    #[test]
    fn test_hybrid_request_id_known_value_decoding() {
        assert_request_id_decoding(0x0000002A00000008u64, 42); // query_index=42, page_number=7
        assert_request_id_decoding(0x0000000100000001u64, 1); // query_index=1, page_number=0
    }

    #[test]
    fn test_global_statistics_basic_aggregation() {
        let stats1 = GlobalStatistics {
            document_count: 100,
            full_text_statistics: vec![
                FullTextStatistics {
                    total_word_count: 50,
                    hit_counts: vec![10, 20, 30],
                },
                FullTextStatistics {
                    total_word_count: 25,
                    hit_counts: vec![5, 15],
                },
            ],
        };

        let stats2 = GlobalStatistics {
            document_count: 200,
            full_text_statistics: vec![
                FullTextStatistics {
                    total_word_count: 75,
                    hit_counts: vec![15, 25, 35],
                },
                FullTextStatistics {
                    total_word_count: 40,
                    hit_counts: vec![8, 12],
                },
            ],
        };

        let expected = GlobalStatistics {
            document_count: 300,
            full_text_statistics: vec![
                FullTextStatistics {
                    total_word_count: 125,
                    hit_counts: vec![25, 45, 65],
                },
                FullTextStatistics {
                    total_word_count: 65,
                    hit_counts: vec![13, 27],
                },
            ],
        };

        let result = stats1.aggregate_with(stats2).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_global_statistics_comprehensive_query_rewriting() {
        let stats = GlobalStatistics {
            document_count: 1000,
            full_text_statistics: vec![
                FullTextStatistics {
                    total_word_count: 500,
                    hit_counts: vec![10, 20, 30],
                },
                FullTextStatistics {
                    total_word_count: 300,
                    hit_counts: vec![5, 15],
                },
            ],
        };

        let mut query_info = QueryInfo {
            rewritten_query: "SELECT * FROM c WHERE c.docCount = {documentdb-formattablehybridsearchquery-totaldocumentcount} AND c.wordCount0 = {documentdb-formattablehybridsearchquery-totalwordcount-0} AND c.hits0 = {documentdb-formattablehybridsearchquery-hitcountsarray-0} AND c.wordCount1 = {documentdb-formattablehybridsearchquery-totalwordcount-1} AND c.hits1 = {documentdb-formattablehybridsearchquery-hitcountsarray-1} AND {documentdb-formattableorderbyquery-filter}".to_string(),
            order_by_expressions: vec![
                "c.score + {documentdb-formattablehybridsearchquery-totalwordcount-0}".to_string(),
                "c.relevance * {documentdb-formattablehybridsearchquery-hitcountsarray-1}".to_string(),
            ],
            ..Default::default()
        };

        stats.rewrite_component_query(&mut query_info).unwrap();

        let expected_rewritten_query = "SELECT * FROM c WHERE c.docCount = 1000 AND c.wordCount0 = 500 AND c.hits0 = [10,20,30] AND c.wordCount1 = 300 AND c.hits1 = [5,15] AND true";
        assert_eq!(query_info.rewritten_query, expected_rewritten_query);

        let expected_order_by = vec![
            "c.score + 500".to_string(),
            "c.relevance * [5,15]".to_string(),
        ];
        assert_eq!(query_info.order_by_expressions, expected_order_by);
    }

    #[test]
    fn test_global_statistics_empty_query_rewriting() {
        let stats = GlobalStatistics {
            document_count: 1000,
            full_text_statistics: vec![],
        };

        let mut empty_query = QueryInfo {
            rewritten_query: String::new(),
            ..Default::default()
        };
        stats.rewrite_component_query(&mut empty_query).unwrap();
        assert_eq!(empty_query.rewritten_query, String::new());
    }

    #[test]
    fn test_global_statistics_passthrough_query_rewriting() {
        let stats = GlobalStatistics {
            document_count: 1000,
            full_text_statistics: vec![],
        };

        let mut simple_query = QueryInfo {
            rewritten_query: "SELECT * FROM c".to_string(),
            ..Default::default()
        };
        stats.rewrite_component_query(&mut simple_query).unwrap();
        assert_eq!(simple_query.rewritten_query, "SELECT * FROM c");
    }

    // ComponentQueryResult tests
    #[test]
    fn test_component_query_result_equality_ignores_payload() {
        let result1a = create_test_result("doc1", "test1");
        let result1b = create_test_result("doc1", "test2"); // Different payload, same rid
        let result2 = create_test_result("doc2", "test3");

        assert_eq!(result1a, result1b);
        assert_ne!(result1a, result2);
    }

    #[test]
    fn test_component_query_result_ordering_by_rid() {
        let result1 = create_test_result("doc1", "test1");
        let result2 = create_test_result("doc2", "test2");

        assert!(result1 < result2);
        assert!(result2 > result1);
        assert_eq!(result1.cmp(&result1), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_component_query_result_alphabetical_sorting() {
        let result_alpha = create_test_result("aaa", "test1");
        let result_beta = create_test_result("zzz", "test2");
        let result_doc1 = create_test_result("doc1", "test3");
        let result_doc2 = create_test_result("doc2", "test4");

        let mut results = vec![result_doc2, result_doc1, result_beta, result_alpha];
        results.sort();

        let expected_rids = vec!["aaa", "doc1", "doc2", "zzz"];
        let actual_rids: Vec<&str> = results.iter().map(|r| r.rid.as_str()).collect();
        assert_eq!(actual_rids, expected_rids);
    }

    // GlobalStatistics error handling tests
    #[test]
    fn test_global_statistics_mismatched_statistics_length_error() {
        let stats1 = GlobalStatistics {
            document_count: 100,
            full_text_statistics: vec![FullTextStatistics {
                total_word_count: 50,
                hit_counts: vec![10, 20],
            }],
        };

        let stats2 = GlobalStatistics {
            document_count: 200,
            full_text_statistics: vec![
                FullTextStatistics {
                    total_word_count: 75,
                    hit_counts: vec![15, 25],
                },
                FullTextStatistics {
                    total_word_count: 40,
                    hit_counts: vec![8, 12],
                },
            ],
        };

        let result = stats1.aggregate_with(stats2);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(
            error.kind(),
            crate::ErrorKind::InvalidGatewayResponse
        ));
        assert!(error
            .to_string()
            .contains("mismatched full text statistics length"));
    }

    #[test]
    fn test_global_statistics_mismatched_hit_counts_length_error() {
        let stats1 = GlobalStatistics {
            document_count: 100,
            full_text_statistics: vec![FullTextStatistics {
                total_word_count: 50,
                hit_counts: vec![10, 20, 30],
            }],
        };

        let stats2 = GlobalStatistics {
            document_count: 200,
            full_text_statistics: vec![FullTextStatistics {
                total_word_count: 75,
                hit_counts: vec![15, 25], // Different length
            }],
        };

        let result = stats1.aggregate_with(stats2);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(
            error.kind(),
            crate::ErrorKind::InvalidGatewayResponse
        ));
        assert!(error.to_string().contains("mismatched hit counts length"));
    }
}
