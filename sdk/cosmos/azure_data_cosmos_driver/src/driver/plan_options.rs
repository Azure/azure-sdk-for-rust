// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Options for the planning phase of a Cosmos DB operation.

use crate::models::ContinuationToken;

/// Options passed to [`CosmosDriver::plan_operation`](crate::driver::CosmosDriver::plan_operation).
///
/// Group both continuation-token resumption and the fan-out cap into a single
/// struct so the signature of `plan_operation` can grow without forcing a
/// change at every call site. Callers that need no special behavior can pass
/// `None`; `plan_operation` treats `None` as `PlanOptions::default()`.
#[derive(Clone, Debug, Default)]
pub struct PlanOptions {
    /// Continuation token to resume a previous query from where it left off.
    ///
    /// When `None`, the query starts from the beginning.
    pub continuation: Option<ContinuationToken>,

    /// Maximum number of physical partitions a cross-partition query may fan
    /// out to.
    ///
    /// When `None`, a built-in default of 100 applies. Queries that would
    /// target more partitions than this limit fail with HTTP 400 /
    /// sub-status 20307.
    pub max_fan_out: Option<usize>,
}
