// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::cmp::Ordering;

use crate::query::{DataRequest, PartitionKeyRange};

/// Represents the current stage of pagination for a partition.
#[derive(Debug, Clone)]
pub enum PaginationState {
    /// The partition is ready for the first data request. There should be no data in the queue yet.
    Initial,

    /// The partition has a pending continuation. When the current queue is exhausted, the continuation can be used to fetch more data.
    Continuing { token: String, next_page_index: u32 },

    /// The partition has been exhausted. When the current queue is exhausted, the partition is done.
    Done,
}

impl PaginationState {
    pub fn update(&mut self, continuation: Option<String>) {
        let next_page_index = match &self {
            PaginationState::Initial => 1,
            PaginationState::Continuing {
                next_page_index, ..
            } => *next_page_index + 1,
            PaginationState::Done => 0, // Doesn't matter, we're done
        };
        match continuation {
            Some(token) => {
                *self = PaginationState::Continuing {
                    token,
                    next_page_index,
                };
            }
            None => {
                *self = PaginationState::Done;
            }
        }
    }
}

#[derive(Debug)]
pub struct PartitionState {
    /// The index of the partition in the pkranges list used by the pipeline.
    pub index: usize,
    /// The partition key range this state is for.
    pub pkrange: PartitionKeyRange,
    /// The current stage of pagination for this partition.
    pub stage: PaginationState,
}

impl PartialEq for PartitionState {
    fn eq(&self, other: &Self) -> bool {
        self.pkrange.id == other.pkrange.id
    }
}

impl Eq for PartitionState {}

impl PartialOrd for PartitionState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PartitionState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pkrange.min_inclusive.cmp(&other.pkrange.min_inclusive)
    }
}

impl PartitionState {
    /// Initializes a partition state for the given partition key range.
    pub fn new(index: usize, pkrange: PartitionKeyRange) -> Self {
        Self {
            index,
            pkrange,
            stage: PaginationState::Initial,
        }
    }

    /// Gets the next [`DataRequest`] for this partition, if one is needed.
    pub fn request(&self) -> Option<DataRequest> {
        match &self.stage {
            PaginationState::Initial => {
                Some(DataRequest::new(0, self.pkrange.id.clone(), None, None))
            }
            PaginationState::Continuing {
                next_page_index,
                token,
            } => Some(DataRequest::new(
                *next_page_index as u64,
                self.pkrange.id.clone(),
                Some(token.clone()),
                None,
            )),
            PaginationState::Done => None,
        }
    }

    pub fn update_state(&mut self, continuation: Option<String>) {
        self.stage.update(continuation);
    }

    pub fn started(&self) -> bool {
        !matches!(self.stage, PaginationState::Initial)
    }

    pub fn done(&self) -> bool {
        matches!(self.stage, PaginationState::Done)
    }
}

#[derive(Debug)]
pub struct QueryChunkState {
    /// The index of the query chunks in the list used by the pipeline.
    pub index: usize,
    /// The partition key range id this state is for.
    pub pkrange_id: String,
    /// The query that belongs to this set of item identities.
    pub query: String,
    /// The current stage of pagination for this partition.
    pub stage: PaginationState,
}

impl QueryChunkState {
    /// Initializes a partition state for the given partition key range.
    pub fn new(index: usize, pkrange_id: String, query: String) -> Self {
        Self {
            index: index,
            pkrange_id: pkrange_id,
            query: query,
            stage: PaginationState::Initial,
        }
    }

    /// Gets the next [`DataRequest`] for this partition, if one is needed.
    pub fn request(&self) -> Option<DataRequest> {
        match &self.stage {
            PaginationState::Initial => Some(DataRequest::new(
                self.index as u64,
                self.pkrange_id.clone(),
                None,
                Some(self.query.clone()),
            )),
            PaginationState::Continuing {
                next_page_index: _,
                token,
            } => Some(DataRequest::new(
                self.index as u64,
                self.pkrange_id.clone(),
                Some(token.clone()),
                Some(self.query.clone()),
            )),
            PaginationState::Done => None,
        }
    }

    pub fn update_state(&mut self, continuation: Option<String>) {
        self.stage.update(continuation);
    }

    pub fn started(&self) -> bool {
        !matches!(self.stage, PaginationState::Initial)
    }

    pub fn done(&self) -> bool {
        matches!(self.stage, PaginationState::Done)
    }
}
