// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::query::{
    node::PipelineNodeResult, plan::HybridSearchQueryInfo, query_result::QueryResultShape,
    DataRequest, PartitionKeyRange, QueryChunk, QueryChunkItem, SortOrder,
};

mod hybrid;
mod non_streaming;
mod read_many;
mod sorting;
mod state;
mod streaming;
mod unordered;

use hybrid::HybridSearchStrategy;
use non_streaming::NonStreamingStrategy;
use read_many::ReadManyStrategy;
use state::PartitionState;
use state::QueryChunkState;
use streaming::StreamingStrategy;
use unordered::UnorderedStrategy;

/// An item producer handles merging results from several partitions into a single stream of results.
///
/// The single-partition result streams are merged according to the variant of the producer selected when the producer is initialized.
/// The producer is only responsible for handling ordering the results, other query operations like aggregations or offset/limit
/// are handled by the pipeline that runs after a specific item has been produced.
/// Ordering can't really be done by the pipeline though, since it may require buffering results from some or all partitions.
/// So, before the pipeline runs, the producer is responsible for actually organizing the initial set of results in the correct order.
// --
// This uses a common Rust pattern for internal-only "dynamic dispatch" called "enum dispatch".
// True dynamic dispatch, using `dyn` has an increased runtime cost and hides information from the optimizer leading to even more performance loss.
// Since this is an internal API, we can use an enum to select the strategy at runtime and delegate methods to the appropriate concrete strategy type.
// This dispatch should be no worse than a virtual function call, and is often quite a lot better.
// See https://crates.io/crates/enum_dispatch for more on this pattern (we're not using that crate, but we're doing what it does manually).
#[derive(Debug)]
pub enum ItemProducer {
    /// Results are not re-ordered by the query and should be ordered by the partition key range minimum.
    Unordered(UnorderedStrategy),
    /// Results should be merged by comparing the sort order of the `ORDER BY` items. Results can be streamed, because each partition will provide data in a global order.
    Streaming(StreamingStrategy),
    /// Results should be merged by comparing the sort order of the `ORDER BY` items. Results cannot be streamed, because each partition will provide data in a local order.
    NonStreaming(NonStreamingStrategy),
    /// Results should be merged by reading each partition individually, exhausting one partition before moving to the next.
    /// Final ordering should happen once all results are available.
    ReadMany(ReadManyStrategy),
    /// The query is a hybrid search query.
    Hybrid(HybridSearchStrategy),
}

pub fn create_partition_state(
    pkranges: impl IntoIterator<Item = PartitionKeyRange>,
) -> Vec<PartitionState> {
    let mut partitions = pkranges
        .into_iter()
        .enumerate()
        .map(|(i, p)| PartitionState::new(i, p))
        .collect::<Vec<_>>();
    partitions.sort();
    partitions
}

pub fn create_query_chunk_states(
    query_chunks: &Vec<QueryChunk>,
    pk_paths: Vec<String>,
) -> Vec<QueryChunkState> {
    let mut chunk_states = Vec::with_capacity(query_chunks.len());

    for i in 0..query_chunks.len() {
        let query = create_query_chunk_query(&query_chunks[i].items, &pk_paths);
        // For QueryChunkState, we will use the index as the indentifier as opposed to pkrange ID.
        let chunk = QueryChunkState::new(i, query_chunks[i].pk_range_id.clone(), query);
        tracing::debug!("created query chunk state: {:?}", chunk);
        chunk_states.push(chunk);
    }
    chunk_states
}

fn create_query_chunk_query(
    query_chunk_items: &Vec<QueryChunkItem>,
    pk_paths: &Vec<String>,
) -> String {
    if query_chunk_items.is_empty() {
        return "SELECT * FROM c WHERE 1 = 0".to_string();
    }

    if pk_paths.len() == 1 {
        // strip the leading "/" to get just the partition key property name
        let pk_path = pk_paths[0].trim_start_matches('/');

        let conditions = query_chunk_items
            .iter()
            .map(|item| {
                format!(
                    "(c.id='{}' AND c.{}='{}')",
                    item.id, pk_path, item.partition_key_value
                )
            })
            .collect::<Vec<_>>()
            .join(" OR ");

        let query = format!("SELECT * FROM c WHERE ( {conditions} )");
        tracing::debug!(query_len = query.len(), "generated query length");
        query
    } else {
        // here we could have logic for HPK later down the line - for now we just do queries with only ID values
        let conditions = query_chunk_items
            .iter()
            .map(|item| format!("(c.id = '{}')", item.id))
            .collect::<Vec<_>>()
            .join(" OR ");

        let query = format!("SELECT * FROM c WHERE ( {conditions} )");
        tracing::debug!(query_len = query.len(), "generated query");
        query
    }
}

impl ItemProducer {
    /// Creates a producer for queries without ORDER BY clauses.
    ///
    /// This strategy processes partitions sequentially in partition key range order,
    /// exhausting one partition completely before moving to the next.
    ///
    /// Use this for queries that don't require global ordering across partitions.
    pub fn unordered(
        pkranges: impl IntoIterator<Item = PartitionKeyRange>,
        result_shape: QueryResultShape,
    ) -> Self {
        Self::Unordered(UnorderedStrategy::new(pkranges, result_shape))
    }

    /// Creates a producer for ORDER BY queries where each partition returns globally sorted results.
    ///
    /// This strategy can stream results immediately because it assumes each partition's results
    /// are already sorted in the global order. It maintains a small buffer per partition and
    /// continuously merges the "head" items to produce the next globally ordered result.
    ///
    /// Use this when:
    /// - The query has an ORDER BY clause
    /// - Each partition's results are sorted in the same global order
    /// - You want to stream results without waiting for all partitions to complete
    pub fn streaming(
        pkranges: impl IntoIterator<Item = PartitionKeyRange>,
        sorting: Vec<SortOrder>,
    ) -> Self {
        Self::Streaming(StreamingStrategy::new(pkranges, sorting))
    }

    /// Creates a producer for ORDER BY queries where partitions return locally sorted results.
    ///
    /// This strategy buffers ALL results from ALL partitions before returning any items.
    /// It uses a binary heap to sort results globally after collecting everything.
    /// No results can be streamed until all partitions are completely exhausted.
    ///
    /// Use this when:
    /// - The query has an ORDER BY clause
    /// - Each partition's results are only sorted locally (not in global order)
    /// - You can afford to buffer the entire result set in memory
    /// - Correctness is more important than streaming performance
    pub fn non_streaming(
        pkranges: impl IntoIterator<Item = PartitionKeyRange>,
        sorting: Vec<SortOrder>,
    ) -> Self {
        Self::NonStreaming(NonStreamingStrategy::new(pkranges, sorting))
    }

    /// Creates a producer for read many operations.
    ///
    /// This strategy processes query chunks sequentially, where each chunk will map to its own query and partition key range.
    pub fn read_many(query_chunks: Vec<QueryChunk>, pk_paths: Vec<String>) -> Self {
        Self::ReadMany(ReadManyStrategy::new(query_chunks, pk_paths))
    }

    /// Creates a producer for Hybrid search queries (which include Full-Text searches, and Rank Fusion operations)
    pub fn hybrid(
        pkranges: impl IntoIterator<Item = PartitionKeyRange>,
        hybrid_search_query_info: HybridSearchQueryInfo,
    ) -> crate::Result<Self> {
        Ok(Self::Hybrid(HybridSearchStrategy::new(
            pkranges,
            hybrid_search_query_info,
        )?))
    }

    /// Gets the [`DataRequest`]s that must be performed in order to add additional data to the partition buffers.
    pub fn data_requests(&mut self) -> crate::Result<Vec<DataRequest>> {
        // The default value for Vec is an empty vec, which doesn't allocate until items are added.
        match self {
            ItemProducer::Unordered(s) => Ok(s.requests()),
            ItemProducer::Streaming(s) => Ok(s.requests()),
            ItemProducer::NonStreaming(s) => Ok(s.requests()),
            ItemProducer::ReadMany(s) => Ok(s.requests()),
            ItemProducer::Hybrid(s) => s.requests(),
        }
    }

    /// Provides additional data for the given partition.
    pub fn provide_data(
        &mut self,
        pkrange_id: &str,
        request_id: u64,
        data: &[u8],
        continuation: Option<String>,
    ) -> crate::Result<()> {
        match self {
            ItemProducer::Unordered(s) => s.provide_data(pkrange_id, data, continuation),
            ItemProducer::Streaming(s) => s.provide_data(pkrange_id, data, continuation),
            ItemProducer::NonStreaming(s) => s.provide_data(pkrange_id, data, continuation),
            ItemProducer::ReadMany(s) => s.provide_data(request_id, data, continuation),
            ItemProducer::Hybrid(s) => s.provide_data(pkrange_id, request_id, data, continuation),
        }
    }

    /// Requests the next item from the cross-partition result stream.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn produce_item(&mut self) -> crate::Result<PipelineNodeResult> {
        match self {
            ItemProducer::Unordered(s) => s.produce_item(),
            ItemProducer::Streaming(s) => s.produce_item(),
            ItemProducer::NonStreaming(s) => s.produce_item(),
            ItemProducer::ReadMany(s) => s.produce_item(),
            ItemProducer::Hybrid(s) => s.produce_item(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, VecDeque};

    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::{
        query::{
            query_result::{FeedResponse, QueryResultShape},
            PartitionKeyRange, QueryResult,
        },
        ErrorKind,
    };

    use super::*;

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Item {
        id: String,
        pk: String,
        title: String,
    }

    impl Item {
        pub fn new(id: impl Into<String>, pk: impl Into<String>, title: impl Into<String>) -> Self {
            Self {
                id: id.into(),
                pk: pk.into(),
                title: title.into(),
            }
        }
    }

    pub type TestPage = (Option<String>, Vec<QueryResult>);

    fn create_item(
        pkrange_id: &str,
        id: impl Into<String>,
        order_by_items: Vec<serde_json::Value>,
    ) -> QueryResult {
        let id = id.into();
        let item = Item::new(
            id.clone(),
            pkrange_id.to_string(),
            format!("{pkrange_id} / {id}"),
        );
        if order_by_items.is_empty() {
            QueryResult::RawPayload(serde_json::value::to_raw_value(&item).unwrap())
        } else {
            let order_by_items = order_by_items
                .into_iter()
                .map(|value| serde_json::from_value(value).unwrap())
                .collect();
            QueryResult::OrderBy {
                order_by_items,
                payload: serde_json::value::to_raw_value(&item).unwrap(),
            }
        }
    }

    /// Helper function to serialize QueryResult items back into the JSON format expected by the gateway
    fn serialize_query_results(results: &[QueryResult]) -> crate::Result<Vec<u8>> {
        let wrapper = FeedResponse {
            documents: results.to_vec(),
        };
        let json = serde_json::to_vec(&wrapper).map_err(|e| {
            ErrorKind::InternalError
                .with_message(format!("failed to serialize query results: {}", e))
        })?;
        Ok(json)
    }

    fn run_producer(
        producer: &mut ItemProducer,
        mut partitions: HashMap<String, VecDeque<TestPage>>,
    ) -> crate::Result<Vec<Item>> {
        let mut items = Vec::new();
        loop {
            let requests = producer.data_requests()?;
            for request in requests {
                let pkrange_id = request.pkrange_id.to_string();
                if let Some(pages) = partitions.get_mut(&pkrange_id) {
                    let (token, query_results) =
                        pages.pop_front().unwrap_or_else(|| (None, Vec::new()));
                    assert_eq!(
                        request.continuation, token,
                        "continuation token should match the one provided in the request"
                    );
                    let next_token = pages.front().and_then(|(t, _)| t.clone());

                    // Serialize QueryResult items to JSON bytes in the appropriate shape
                    let json_bytes = serialize_query_results(&query_results)?;
                    producer.provide_data(&pkrange_id, request.id, &json_bytes, next_token)?;
                } else {
                    return Err(ErrorKind::UnknownPartitionKeyRange
                        .with_message(format!("unknown partition key range ID: {pkrange_id}")));
                }
            }

            // Now drain the items from the producer.
            loop {
                let result = producer.produce_item()?;
                let has_value = result.value.is_some(); // Capture Some/None state before we consume it.
                if let Some(value) = result.value {
                    let payload = value.into_payload().unwrap();
                    let item = serde_json::from_str(payload.get()).unwrap();
                    items.push(item);
                }

                if result.terminated {
                    return Ok(items);
                }

                if !has_value {
                    break;
                }
            }

            // Loop back around to check for requests.
        }
    }

    #[test]
    pub fn unordered_strategy_orders_by_partition_key_minimum(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // The partitions are "Vec<(Option<String>, Vec<Item>)>", where the first element is the continuation token
        // and the second element is the list of items for that partition.
        let mut partition0: VecDeque<TestPage> = VecDeque::new();
        let mut partition1: VecDeque<TestPage> = VecDeque::new();

        /// Generates a page of test items for a given partition.
        fn fill_page(
            partition: &mut VecDeque<TestPage>,
            pkrange_id: &str,
            start_id: usize,
            count: usize,
            continuation: Option<String>,
        ) -> crate::Result<()> {
            // NOTE: A PKRange ID is NOT the same as a partition key, but in our testing it can serve that purpose.

            let mut page = Vec::new();
            for i in 0..count {
                let id = format!("item{}", start_id + i);
                page.push(create_item(pkrange_id, id, Vec::new()));
            }

            partition.push_back((continuation, page));
            Ok(())
        }

        // Two pages of 5 items for each partition
        fill_page(&mut partition0, "partition0", 0, 5, None)?;
        fill_page(
            &mut partition0,
            "partition0",
            5,
            5,
            Some("p0c0".to_string()),
        )?;
        fill_page(&mut partition1, "partition1", 0, 5, None)?;
        fill_page(
            &mut partition1,
            "partition1",
            5,
            5,
            Some("p1c0".to_string()),
        )?;

        let mut producer = ItemProducer::unordered(
            vec![
                PartitionKeyRange::new("partition0", "00", "99"),
                PartitionKeyRange::new("partition1", "99", "FF"),
            ],
            QueryResultShape::RawPayload,
        );

        let items = run_producer(
            &mut producer,
            HashMap::from([
                ("partition0".to_string(), partition0),
                ("partition1".to_string(), partition1),
            ]),
        )?;

        assert_eq!(
            vec![
                Item::new("item0", "partition0", "partition0 / item0"),
                Item::new("item1", "partition0", "partition0 / item1"),
                Item::new("item2", "partition0", "partition0 / item2"),
                Item::new("item3", "partition0", "partition0 / item3"),
                Item::new("item4", "partition0", "partition0 / item4"),
                Item::new("item5", "partition0", "partition0 / item5"),
                Item::new("item6", "partition0", "partition0 / item6"),
                Item::new("item7", "partition0", "partition0 / item7"),
                Item::new("item8", "partition0", "partition0 / item8"),
                Item::new("item9", "partition0", "partition0 / item9"),
                Item::new("item0", "partition1", "partition1 / item0"),
                Item::new("item1", "partition1", "partition1 / item1"),
                Item::new("item2", "partition1", "partition1 / item2"),
                Item::new("item3", "partition1", "partition1 / item3"),
                Item::new("item4", "partition1", "partition1 / item4"),
                Item::new("item5", "partition1", "partition1 / item5"),
                Item::new("item6", "partition1", "partition1 / item6"),
                Item::new("item7", "partition1", "partition1 / item7"),
                Item::new("item8", "partition1", "partition1 / item8"),
                Item::new("item9", "partition1", "partition1 / item9"),
            ],
            items
        );

        Ok(())
    }

    #[test]
    pub fn streaming_strategy_merges_ordered_streams_of_data(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut partition0: VecDeque<TestPage> = VecDeque::new();
        let mut partition1: VecDeque<TestPage> = VecDeque::new();

        // Partition 0 has two pages, but the second is empty (this can happen in real scenarios).
        partition0.push_back((
            None,
            vec![
                create_item(
                    "partition0",
                    "item0",
                    vec![json!({"item": 1}), json!({"item": "aaaa"})],
                ),
                create_item(
                    "partition0",
                    "item1",
                    vec![json!({"item": 2}), json!({"item": "yyyy"})],
                ),
                create_item(
                    "partition0",
                    "item2",
                    vec![json!({"item": 6}), json!({"item": "zzzz"})],
                ),
            ],
        ));
        partition0.push_back((Some("p0c0".to_string()), vec![]));

        // Partition 1 doesn't have a second page, so it will be exhausted after the first page.
        partition1.push_back((
            None,
            vec![
                create_item(
                    "partition1",
                    "item0",
                    vec![json!({"item": 1}), json!({"item": "zzzz"})],
                ),
                create_item(
                    "partition1",
                    "item1",
                    vec![json!({"item": 2}), json!({"item": "bbbb"})],
                ),
                create_item(
                    "partition1",
                    "item2",
                    vec![json!({"item": 3}), json!({"item": "zzzz"})],
                ),
                create_item(
                    "partition1",
                    "item3",
                    vec![json!({"item": 7}), json!({"item": "zzzz"})],
                ),
                create_item(
                    "partition1",
                    "item4",
                    vec![json!({"item": 8}), json!({"item": "zzzz"})],
                ),
                create_item(
                    "partition1",
                    "item5",
                    vec![json!({"item": 9}), json!({"item": "zzzz"})],
                ),
            ],
        ));

        let mut producer = ItemProducer::streaming(
            vec![
                PartitionKeyRange::new("partition0", "00", "99"),
                PartitionKeyRange::new("partition1", "99", "FF"),
            ],
            vec![SortOrder::Ascending, SortOrder::Descending],
        );

        // We should stop once any partition's queue is empty.
        let items = run_producer(
            &mut producer,
            HashMap::from([
                ("partition0".to_string(), partition0),
                ("partition1".to_string(), partition1),
            ]),
        )?;

        assert_eq!(
            vec![
                Item::new("item0", "partition1", "partition1 / item0"),
                Item::new("item0", "partition0", "partition0 / item0"),
                Item::new("item1", "partition0", "partition0 / item1"),
                Item::new("item1", "partition1", "partition1 / item1"),
                Item::new("item2", "partition1", "partition1 / item2"),
                Item::new("item2", "partition0", "partition0 / item2"),
                Item::new("item3", "partition1", "partition1 / item3"),
                Item::new("item4", "partition1", "partition1 / item4"),
                Item::new("item5", "partition1", "partition1 / item5"),
            ],
            items
        );

        Ok(())
    }

    #[test]
    pub fn nonstreaming_strategy_buffers_all_results_before_ordering(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut partition0: VecDeque<TestPage> = VecDeque::new();
        let mut partition1: VecDeque<TestPage> = VecDeque::new();

        // For this test, we basically use the same data as the streaming strategy, but each partition's results are not pre-sorted, in fact they're reverse-sorted.

        // Partition 0 has two pages, but the second is empty (this can happen in real scenarios).
        partition0.push_back((
            None,
            vec![
                create_item(
                    "partition0",
                    "item2",
                    vec![json!({"item": 6}), json!({"item": "zzzz"})],
                ),
                create_item(
                    "partition0",
                    "item1",
                    vec![json!({"item": 2}), json!({"item": "yyyy"})],
                ),
                create_item(
                    "partition0",
                    "item0",
                    vec![json!({"item": 1}), json!({"item": "aaaa"})],
                ),
            ],
        ));
        partition0.push_back((Some("p0c0".to_string()), vec![]));

        // Partition 1 doesn't have a second page, so it will be exhausted after the first page.
        partition1.push_back((
            None,
            vec![
                create_item(
                    "partition1",
                    "item5",
                    vec![json!({"item": 9}), json!({"item": "zzzz"})],
                ),
                create_item(
                    "partition1",
                    "item4",
                    vec![json!({"item": 8}), json!({"item": "zzzz"})],
                ),
                create_item(
                    "partition1",
                    "item3",
                    vec![json!({"item": 7}), json!({"item": "zzzz"})],
                ),
                create_item(
                    "partition1",
                    "item2",
                    vec![json!({"item": 3}), json!({"item": "zzzz"})],
                ),
                create_item(
                    "partition1",
                    "item1",
                    vec![json!({"item": 2}), json!({"item": "bbbb"})],
                ),
                create_item(
                    "partition1",
                    "item0",
                    vec![json!({"item": 1}), json!({"item": "zzzz"})],
                ),
            ],
        ));

        let mut producer = ItemProducer::non_streaming(
            vec![
                PartitionKeyRange::new("partition0", "00", "99"),
                PartitionKeyRange::new("partition1", "99", "FF"),
            ],
            vec![SortOrder::Ascending, SortOrder::Descending],
        );

        // We should stop once any partition's queue is empty.
        let items = run_producer(
            &mut producer,
            HashMap::from([
                ("partition0".to_string(), partition0),
                ("partition1".to_string(), partition1),
            ]),
        )?;

        assert_eq!(
            vec![
                Item::new("item0", "partition1", "partition1 / item0"),
                Item::new("item0", "partition0", "partition0 / item0"),
                Item::new("item1", "partition0", "partition0 / item1"),
                Item::new("item1", "partition1", "partition1 / item1"),
                Item::new("item2", "partition1", "partition1 / item2"),
                Item::new("item2", "partition0", "partition0 / item2"),
                Item::new("item3", "partition1", "partition1 / item3"),
                Item::new("item4", "partition1", "partition1 / item4"),
                Item::new("item5", "partition1", "partition1 / item5"),
            ],
            items
        );

        Ok(())
    }

    #[test]
    pub fn readmany_strategy_returns_items_in_original_order(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut partition0: VecDeque<TestPage> = VecDeque::new();
        let mut partition1: VecDeque<TestPage> = VecDeque::new();
        // partition0 will return: item2, item0, item4
        // partition1 will return: item1, item3, item5

        partition0.push_back((
            None,
            vec![
                create_item("partition0", "item2", vec![]),
                create_item("partition0", "item0", vec![]),
                create_item("partition0", "item4", vec![]),
            ],
        ));

        partition1.push_back((
            None,
            vec![
                create_item("partition1", "item1", vec![]),
                create_item("partition1", "item3", vec![]),
                create_item("partition1", "item5", vec![]),
            ],
        ));

        // The chunks will be distributed across partitions
        let query_chunks = vec![
            QueryChunk {
                pk_range_id: "partition0".to_string(),
                items: vec![
                    QueryChunkItem {
                        index: 0,
                        id: "item0".to_string(),
                        partition_key_value: "partition0".to_string(),
                    },
                    QueryChunkItem {
                        index: 2,
                        id: "item2".to_string(),
                        partition_key_value: "partition0".to_string(),
                    },
                    QueryChunkItem {
                        index: 4,
                        id: "item4".to_string(),
                        partition_key_value: "partition0".to_string(),
                    },
                ],
            },
            QueryChunk {
                pk_range_id: "partition1".to_string(),
                items: vec![
                    QueryChunkItem {
                        index: 1,
                        id: "item1".to_string(),
                        partition_key_value: "partition1".to_string(),
                    },
                    QueryChunkItem {
                        index: 3,
                        id: "item3".to_string(),
                        partition_key_value: "partition1".to_string(),
                    },
                    QueryChunkItem {
                        index: 5,
                        id: "item5".to_string(),
                        partition_key_value: "partition1".to_string(),
                    },
                ],
            },
        ];

        let mut producer = ItemProducer::read_many(query_chunks, vec!["/pk".to_string()]);

        let items = run_producer(
            &mut producer,
            HashMap::from([
                ("partition0".to_string(), partition0),
                ("partition1".to_string(), partition1),
            ]),
        )?;

        // partition0 items first, then partition1 items
        assert_eq!(
            vec![
                Item::new("item2", "partition0", "partition0 / item2"),
                Item::new("item0", "partition0", "partition0 / item0"),
                Item::new("item4", "partition0", "partition0 / item4"),
                Item::new("item1", "partition1", "partition1 / item1"),
                Item::new("item3", "partition1", "partition1 / item3"),
                Item::new("item5", "partition1", "partition1 / item5"),
            ],
            items
        );

        Ok(())
    }
}
