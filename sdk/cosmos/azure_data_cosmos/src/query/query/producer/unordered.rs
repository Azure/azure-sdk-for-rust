// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::collections::VecDeque;

use crate::{
    query::{
        node::PipelineNodeResult, query_result::QueryResultShape, DataRequest, PartitionKeyRange,
        QueryResult,
    },
    ErrorKind,
};

use super::{create_partition_state, state::PartitionState};

pub struct UnorderedStrategy {
    pub partitions: Vec<PartitionState>,
    pub current_partition_index: usize,
    pub current_pkrange_id: Option<String>,
    pub items: VecDeque<QueryResult>,
    pub result_shape: QueryResultShape,
}

impl std::fmt::Debug for UnorderedStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnorderedStrategy")
            .field("partitions", &self.partitions)
            .field("current_partition_index", &self.current_partition_index)
            .field("current_pkrange_id", &self.current_pkrange_id)
            .field("items_len", &self.items.len())
            .finish()
    }
}

impl UnorderedStrategy {
    pub fn new(
        pkranges: impl IntoIterator<Item = PartitionKeyRange>,
        result_shape: QueryResultShape,
    ) -> Self {
        let partitions = create_partition_state(pkranges);
        Self {
            current_partition_index: 0,
            current_pkrange_id: partitions.first().map(|p| p.pkrange.id.clone()),
            items: VecDeque::new(),
            partitions,
            result_shape,
        }
    }

    pub fn requests(&mut self) -> Vec<DataRequest> {
        // In the unordered strategy, we simply return the first partition key range's request.
        // Once that partition is exhausted, we remove it from the list and return the next one.
        let mut requests = Vec::new();
        while requests.is_empty() {
            // If there are no more partitions, we are done, return empty list.
            let Some(partition) = self.partitions.get(self.current_partition_index) else {
                break;
            };
            match partition.request() {
                Some(request) => {
                    tracing::trace!(pkrange_id = ?partition.pkrange.id, "requesting data for partition");
                    requests.push(request);
                }
                None => {
                    tracing::trace!(pkrange_id = ?partition.pkrange.id, "partition exhausted, removing from list");
                    self.current_partition_index += 1;
                    self.current_pkrange_id = self
                        .partitions
                        .get(self.current_partition_index)
                        .map(|p| p.pkrange.id.clone());
                }
            }
        }
        requests
    }

    pub fn provide_data(
        &mut self,
        pkrange_id: &str,
        data: &[u8],
        continuation: Option<String>,
    ) -> crate::Result<()> {
        match &self.current_pkrange_id {
            Some(id) => {
                if *id != pkrange_id {
                    // The caller provided data for a different partition key range ID before draining the current items queue.
                    return Err(ErrorKind::InternalError.with_message(format!(
                        "provided data for partition key range ID: {}, but current partition is: {}",
                        pkrange_id, id
                    )));
                }
            }
            None => {
                return Err(ErrorKind::InternalError.with_message(format!(
                    "provided data for partition key range ID: {}, but all partitions are exhausted",
                    pkrange_id
                )));
            }
        }

        // Parse the raw bytes using the result shape
        let parsed_data = self.result_shape.results_from_slice(data)?;

        // Add the data to the items queue. There's no ordering to worry about, so we just append the items.
        self.items.extend(parsed_data);

        // Update the partition state with the continuation token
        let partition = self
            .partitions
            .iter_mut()
            .find(|p| p.pkrange.id == pkrange_id)
            .ok_or_else(|| {
                ErrorKind::UnknownPartitionKeyRange
                    .with_message(format!("unknown partition key range ID: {pkrange_id}"))
            })?;
        partition.update_state(continuation);

        Ok(())
    }

    pub fn produce_item(&mut self) -> crate::Result<PipelineNodeResult> {
        let value = self.items.pop_front();
        let terminated = self.items.is_empty()
            && (self.current_partition_index == self.partitions.len() - 1)
            && self.partitions[self.current_partition_index].done();
        Ok(PipelineNodeResult { value, terminated })
    }
}
