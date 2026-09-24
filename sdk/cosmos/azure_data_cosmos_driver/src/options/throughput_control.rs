// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Resolved per-request throughput control values.

use crate::options::PriorityLevel;

/// Fully resolved throughput-control header inputs for a single request.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ResolvedThroughputControl {
    /// Value for the `x-ms-cosmos-throughput-bucket` header, if any.
    pub(crate) throughput_bucket: Option<u32>,
    /// Value for the `x-ms-cosmos-priority-level` header, if any.
    pub(crate) priority_level: Option<PriorityLevel>,
}
