// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::Method;

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
    Execute,
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

    /// Maps the `OperationType` to its corresponding HTTP verb.
    pub fn http_method(self) -> Method {
        match self {
            OperationType::Create
            | OperationType::Upsert
            | OperationType::Query
            | OperationType::SqlQuery
            | OperationType::Batch
            | OperationType::QueryPlan
            | OperationType::Execute => Method::Post,
            OperationType::Delete => Method::Delete,
            OperationType::Read => Method::Get,
            OperationType::ReadFeed => Method::Get,
            OperationType::Replace => Method::Put,
            OperationType::Patch => Method::Patch,
            OperationType::Head | OperationType::HeadFeed => Method::Head,
        }
    }
}
