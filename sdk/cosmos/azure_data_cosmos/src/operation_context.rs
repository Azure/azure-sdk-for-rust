// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{request::options::ContentType, Method};

use crate::constants;

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
    /// Returns the content type of the body for this operation type, if any.
    ///
    /// If `None` is returned, no body content type header should be set and the request is expected to have no body.
    pub fn body_content_type(&self) -> Option<ContentType> {
        match self {
            OperationType::Query | OperationType::SqlQuery => Some(constants::QUERY_CONTENT_TYPE),
            OperationType::Create
            | OperationType::Replace
            | OperationType::Upsert
            | OperationType::Batch
            | OperationType::Patch
            | OperationType::Execute => Some(ContentType::APPLICATION_JSON),
            OperationType::Read
            | OperationType::ReadFeed
            | OperationType::Delete
            | OperationType::Head
            | OperationType::HeadFeed
            | OperationType::QueryPlan => None,
        }
    }

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
