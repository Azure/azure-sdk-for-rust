// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // For the variants. Can be removed when we have them all implemented.
/// Placeholder for operation type.
pub enum OperationType {
    Invalid,
    Create,
    Read,
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
    // ... add other variants as needed
}

