// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{cmp::Ordering, collections::VecDeque};

use crate::{
    query::{
        node::PipelineNodeResult, query_result::QueryResultShape, DataRequest, PartitionKeyRange,
        QueryResult, SortOrder,
    },
    ErrorKind,
};

use super::{create_partition_state, sorting::Sorting, state::PartitionState};

pub struct StreamingStrategy {
    pub partitions: Vec<PartitionState>,
    pub sorting: Sorting,
    pub buffers: Vec<(String, VecDeque<QueryResult>)>,
}

impl std::fmt::Debug for StreamingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamingStrategy")
            .field("partitions", &self.partitions)
            .field("sorting", &self.sorting)
            .field(
                "buffers_len",
                &self
                    .buffers
                    .iter()
                    .map(|(_, b)| b.len())
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl StreamingStrategy {
    pub fn new(
        pkranges: impl IntoIterator<Item = PartitionKeyRange>,
        sorting: Vec<SortOrder>,
    ) -> Self {
        let partitions = create_partition_state(pkranges);
        let buffers = partitions
            .iter()
            .map(|p| (p.pkrange.id.clone(), VecDeque::new()))
            .collect();
        Self {
            partitions,
            sorting: Sorting::new(sorting),
            buffers,
        }
    }

    pub fn requests(&mut self) -> Vec<DataRequest> {
        self.partitions
            .iter()
            .filter_map(|partition| partition.request())
            .collect()
    }

    pub fn provide_data(
        &mut self,
        pkrange_id: &str,
        data: &[u8],
        continuation: Option<String>,
    ) -> crate::Result<()> {
        let partition_index = self
            .partitions
            .iter()
            .position(|p| p.pkrange.id == pkrange_id)
            .ok_or_else(|| {
                ErrorKind::UnknownPartitionKeyRange
                    .with_message(format!("unknown partition key range ID: {pkrange_id}"))
            })?;
        let (pkrange_id, buffer) = self.buffers.get_mut(partition_index).ok_or_else(|| {
            ErrorKind::InternalError.with_message(format!(
                "missing buffer for partition index: {}",
                partition_index
            ))
        })?;
        debug_assert_eq!(
            pkrange_id, &self.partitions[partition_index].pkrange.id,
            "buffer ID should match partition key range ID",
        );

        // Parse the raw bytes using the result shape
        let parsed_data = QueryResultShape::OrderBy.results_from_slice(data)?;

        // We assume the data is coming from the server pre-sorted, so we can just extend the buffer with the data.
        buffer.extend(parsed_data);

        self.partitions[partition_index].update_state(continuation);

        Ok(())
    }

    pub fn produce_item(&mut self) -> crate::Result<PipelineNodeResult> {
        // Scan through each partition to find the next item to produce.
        // We do the scan first with an immutable borrow of the buffers, and then end up with the index of the partition that has the next item to produce.
        // Then we can borrow the buffer mutably after the loop to pop the item out of it.
        let mut current_match = None;
        for (i, partition) in self.partitions.iter().enumerate() {
            let (pkrange_id, buffer) = self.buffers.get(i).ok_or_else(|| {
                ErrorKind::InternalError.with_message(format!(
                    "missing buffer for partition key range ID: {}",
                    partition.pkrange.id
                ))
            })?;
            debug_assert_eq!(pkrange_id, &partition.pkrange.id); // This should always be true, as the lists are initialized together.

            if !partition.started() {
                // If any partition hasn't started, we have to stop producing items.
                // A Partition is considered "started" when we've received at least one `provide_data` call referencing it.
                // For a streaming order by, we can't stream ANY results until we've received at least one set of results from each partition.
                // The missing partitions may contain values that sort BEFORE items in the partitions we've received.
                //
                // SDKs could optimize how they call the engine to avoid this scenario (by always making requests first, for example),
                // but we can't assume that will always be the case.
                tracing::debug!(pkrange_id = ?partition.pkrange.id, "partition not started, stopping item production");
                return Ok(PipelineNodeResult::NO_RESULT);
            }

            if partition.done() && buffer.is_empty() {
                // If the partition is done and the buffer is empty, we can skip it.
                // In fact, we NEED to skip it because we know it won't produce any more items and if we leave it in the set of partitions we consider,
                // we might end up trying to query it for more data.
                tracing::debug!(pkrange_id = ?partition.pkrange.id, "partition done and buffer empty, skipping");
                continue;
            }

            match current_match {
                None => {
                    // If we haven't found a match yet, we set the current match to this partition so that we always pick a partition.
                    current_match = Some((i, buffer.front()));
                }
                Some((current_index, current_item)) => {
                    let current_order_by_items =
                        current_item.and_then(|r| r.as_order_by()).map(|(i, _)| i);
                    let new_order_by_items =
                        buffer.front().and_then(|r| r.as_order_by()).map(|(i, _)| i);
                    match self
                        .sorting
                        .compare(current_order_by_items, new_order_by_items)?
                    {
                        Ordering::Greater => {
                            // The current item sorts higher than the new item, so we keep the current match.
                            continue;
                        }
                        Ordering::Less => {
                            // The new item sorts higher than the current item, so we update the current match to this partition.
                            // Note: This might be because the partition's buffer is currently empty.
                            // That can result in the selected partition being one with an empty buffer.
                            // This is intentional, see below where we return the item.
                            current_match = Some((i, buffer.front()));
                        }
                        Ordering::Equal => {
                            // Compare the index of the partitions to ensure we always return the first partition with the same item.
                            if i < current_index {
                                // The new item is equal to the current item, but the partition index is lower,
                                // so we update the current match to this partition.
                                current_match = Some((i, buffer.front()));
                            }
                        }
                    }
                }
            }
        }
        if let Some((i, _)) = current_match {
            // We found a match, pop the item out of the buffer and return it.
            debug_assert_eq!(
                self.buffers[i].0, self.partitions[i].pkrange.id,
                "buffer ID should match partition key range ID",
            );
            // If the buffer is empty, this may return `None`. That's by design!
            // It means the partition has an empty buffer, and we may need to fetch more data for it.
            // If it was fully exhausted, the check for `done() && buffer.is_empty()` would have excluded it.
            // Instead, we have an empty buffer AND the possibility for more data from this partition.
            // That means we WANT to return `None` here. We need to check this partition for more data before we can yield an item.
            let value = self.buffers[i].1.pop_front();
            let terminated = value.is_none() && self.partitions.iter().all(|p| p.done());
            Ok(PipelineNodeResult { value, terminated })
        } else {
            // No match found, meaning all partitions are either exhausted or waiting for data.
            let terminated = self.partitions.iter().all(|p| p.done());
            Ok(PipelineNodeResult {
                value: None,
                terminated,
            })
        }
    }
}
