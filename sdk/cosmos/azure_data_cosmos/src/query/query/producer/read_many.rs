// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::collections::VecDeque;

use crate::{
    query::{
        node::PipelineNodeResult, query_result::QueryResultShape, DataRequest, QueryChunk,
        QueryResult,
    },
    ErrorKind,
};

use super::{create_query_chunk_states, state::QueryChunkState};

#[derive(Debug)]
pub struct ReadManyStrategy {
    pub query_chunk_states: Vec<QueryChunkState>,
    pub items: VecDeque<QueryResult>,
    remaining_chunks: usize,
}

impl ReadManyStrategy {
    pub fn new(query_chunks: Vec<QueryChunk>, pk_paths: Vec<String>) -> Self {
        let query_chunk_states = create_query_chunk_states(&query_chunks, pk_paths);
        let num_chunks = query_chunk_states.len();
        Self {
            query_chunk_states,
            items: VecDeque::new(),
            remaining_chunks: num_chunks,
        }
    }

    pub fn requests(&mut self) -> Vec<DataRequest> {
        self.query_chunk_states
            .iter()
            .filter_map(|query_chunk_states| query_chunk_states.request())
            .collect()
    }

    pub fn provide_data(
        &mut self,
        request_id: u64,
        data: &[u8],
        continuation: Option<String>,
    ) -> crate::Result<()> {
        // Parse the raw bytes using the result shape
        let parsed_data = QueryResultShape::RawPayload.results_from_slice(data)?;
        // Add the data to the items queue.
        self.items.extend(parsed_data);

        // Find the query chunk state by request_id (which matches the chunk's index)
        let query_chunk_state = self
            .query_chunk_states
            .get_mut(request_id as usize)
            .ok_or_else(|| {
                ErrorKind::InternalError.with_message(format!(
                    "no query chunk state found for request_id/index {}",
                    request_id
                ))
            })?;
        // Update the state and verify the done status before dropping the mutable borrow
        query_chunk_state.update_state(continuation);
        if query_chunk_state.done() {
            self.remaining_chunks -= 1;
        }

        Ok(())
    }

    pub fn produce_item(&mut self) -> crate::Result<PipelineNodeResult> {
        let value = self.items.pop_front();
        let terminated = self.items.is_empty() && self.remaining_chunks == 0;
        Ok(PipelineNodeResult { value, terminated })
    }
}
