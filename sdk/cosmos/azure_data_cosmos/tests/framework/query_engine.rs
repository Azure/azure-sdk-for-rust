// With the optional Send/Sync bounds, it's just easiest to (FOR NOW) have the tests only run on non-wasm32 targets.
#![cfg(not(target_arch = "wasm32"))]

use std::{collections::VecDeque, sync::Mutex};

use serde::{Deserialize, Serialize};

use azure_data_cosmos::query::{PipelineResult, QueryEngine, QueryPipeline};
use serde_json::value::RawValue;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartitionKeyRange {
    pub id: String,
}

#[derive(Deserialize)]
struct PkRanges {
    #[serde(rename = "PartitionKeyRanges")]
    pub ranges: Vec<PartitionKeyRange>,
}

#[derive(Deserialize)]
struct DocumentPayload<T> {
    #[serde(rename = "Documents")]
    pub documents: Vec<T>,
}

/// Represents a single item in the mock engine.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MockItem {
    /// The ID of the item.
    pub id: String,
    /// The partition key of the item.
    pub partition_key: String,
    /// The global merge order of the item, which will be used by the mock pipeline to sort items.
    pub merge_order: usize,
}

/// A mock query engine that can be used for testing.
pub struct MockQueryEngine {
    /// An error to return when creating a pipeline.
    pub create_error: Mutex<Option<azure_core::Error>>,
}

impl MockQueryEngine {
    /// Creates a new `MockQueryEngine`.
    pub fn new() -> Self {
        Self {
            create_error: Mutex::new(None),
        }
    }

    /// Sets an error to be returned when creating a pipeline.
    pub fn with_error(error: azure_core::Error) -> Self {
        Self {
            create_error: Mutex::new(Some(error)),
        }
    }
}

impl QueryEngine for MockQueryEngine {
    fn create_pipeline(
        &self,
        query: &str,
        _plan: &[u8],
        pkranges: &[u8],
    ) -> azure_core::Result<Box<dyn QueryPipeline + Send>> {
        {
            if let Some(err) = self.create_error.lock().unwrap().take() {
                return Err(err);
            }
        }

        // Deserialize the partition key ranges.
        let pkranges: PkRanges = serde_json::from_slice(pkranges)?;

        // Create a mock pipeline with the partition key ranges.
        let pipeline = MockQueryPipeline::new(query.to_string(), pkranges.ranges);

        Ok(Box::new(pipeline))
    }

    fn supported_features(&self) -> azure_core::Result<&str> {
        Ok("OrderBy")
    }
}

struct PartitionState {
    range: PartitionKeyRange,
    started: bool,
    queue: VecDeque<MockItem>,
    next_continuation: Option<String>,
}

impl PartitionState {
    pub fn exhausted(&self) -> bool {
        self.queue.is_empty() && self.started && self.next_continuation.is_none()
    }

    pub fn provide_data(&mut self, items: Vec<MockItem>, continuation: Option<String>) {
        self.started = true;
        self.queue.extend(items);
        self.next_continuation = continuation;
    }

    pub fn pop_item(&mut self) -> azure_core::Result<Option<Box<RawValue>>> {
        match self.queue.pop_front() {
            Some(item) => {
                let item = serde_json::value::to_raw_value(&item)?;
                Ok(Some(item))
            }
            None => Ok(None),
        }
    }
}

struct MockQueryPipeline {
    query: String,
    partitions: Vec<PartitionState>,
    completed: bool,
}

impl MockQueryPipeline {
    pub fn new(query: String, pkranges: Vec<PartitionKeyRange>) -> Self {
        let partitions = pkranges
            .into_iter()
            .map(|range| PartitionState {
                range,
                started: false,
                queue: VecDeque::new(),
                next_continuation: None,
            })
            .collect();

        Self {
            query,
            partitions,
            completed: false,
        }
    }

    fn get_requests(&self) -> Vec<azure_data_cosmos::query::QueryRequest> {
        self.partitions
            .iter()
            .filter(|state| !state.exhausted())
            .map(|state| azure_data_cosmos::query::QueryRequest {
                partition_key_range_id: state.range.id.clone(),
                continuation: if state.started {
                    state.next_continuation.clone()
                } else {
                    None
                },
            })
            .collect()
    }
}

impl QueryPipeline for MockQueryPipeline {
    fn query(&self) -> &str {
        &self.query
    }

    fn complete(&self) -> bool {
        self.completed
    }

    fn run(&mut self) -> azure_core::Result<azure_data_cosmos::query::PipelineResult> {
        let mut items = Vec::new();
        loop {
            let mut state = None;
            for (index, partition) in self.partitions.iter().enumerate() {
                if !partition.started {
                    // If any partition hasn't started, just return the requests with no items.
                    return Ok(PipelineResult {
                        is_completed: false,
                        items: vec![],
                        requests: self.get_requests(),
                    });
                }

                if partition.exhausted() {
                    // No need to check exhausted partitions.
                    continue;
                }

                // Peek the next item in the partition.
                let item = partition.queue.front();
                match (item, state) {
                    (Some(item), None) => {
                        state = Some((index, item.merge_order));
                    }
                    (Some(item), Some((_, lowest_merge_order))) => {
                        if item.merge_order < lowest_merge_order {
                            state = Some((index, item.merge_order));
                        }
                    }
                    _ => panic!("Unexpected state"),
                }
            }

            if let Some((index, _)) = state {
                // Add this item to the result.
                if let Some(item) = self.partitions[index].pop_item()? {
                    items.push(item);
                }
            } else {
                // All partitions are exhausted, or have no items to produce
                break;
            }

            // We've added an item, so we loop back around to check for more.
        }

        let requests = self.get_requests();

        if items.is_empty() && requests.is_empty() {
            // If there are no items and no requests, we are done.
            self.completed = true;
        }

        Ok(PipelineResult {
            is_completed: self.completed,
            items,
            requests,
        })
    }

    fn provide_data(
        &mut self,
        data: azure_data_cosmos::query::QueryResult,
    ) -> azure_core::Result<()> {
        let payload: DocumentPayload<MockItem> =
            serde_json::from_slice(data.result).map_err(|_| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "Failed to deserialize payload",
                )
            })?;

        let partition_state = self
            .partitions
            .iter_mut()
            .find(|state| state.range.id == data.partition_key_range_id);
        if let Some(partition_state) = partition_state {
            partition_state.provide_data(payload.documents, data.next_continuation);
            Ok(())
        } else {
            Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "Partition key range {} not found",
                    data.partition_key_range_id
                ),
            ))
        }
    }
}
