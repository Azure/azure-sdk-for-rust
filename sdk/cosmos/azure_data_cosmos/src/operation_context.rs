// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // For the variants. Can be removed when we have them all implemented.
pub enum OperationType {
    Create,
    Read,
    ReadFeed,
    Replace,
    Delete,
    Upsert,
    Query,
    SqlQuery,
    QueryPlan,
    Batch,
    Patch,
    Head,
    HeadFeed,
}

impl OperationType {
    /// Returns true if the operation does not modify server state and can be
    /// treated as read-only for caching / retry heuristics.
    pub fn is_read_only(&self) -> bool {
        matches!(
            self,
            OperationType::Read
                | OperationType::Query
                | OperationType::SqlQuery
                | OperationType::QueryPlan
                | OperationType::Head
                | OperationType::HeadFeed
        )
    }
}
