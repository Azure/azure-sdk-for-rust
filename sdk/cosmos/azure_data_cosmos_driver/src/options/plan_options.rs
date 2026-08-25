// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Plan-time options for [`CosmosDriver::plan_operation`](crate::driver::CosmosDriver::plan_operation).

/// Default maximum fan-out for a fresh cross-partition operation.
///
/// A plan that would fan out to more than this many leaf request nodes is
/// rejected unless the caller raises [`PlanOptions::max_fan_out`].
pub const DEFAULT_MAX_FAN_OUT: u32 = 100;

/// Default maximum number of candidate rows retained by a non-streaming
/// cross-partition `ORDER BY` query.
pub const DEFAULT_MAX_NON_STREAMING_ORDER_BY_BUFFERED_ITEMS: u32 = 50_000;

/// Options that shape how an operation is planned into a dataflow pipeline.
///
/// Unlike [`OperationOptions`](crate::options::OperationOptions), which controls
/// per-request behavior (consistency, routing, retries), `PlanOptions` controls
/// the *shape* of the plan itself. For example, how many partitions a plan
/// may fan out to.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PlanOptions {
    /// Maximum number of leaf request nodes a fresh cross-partition plan may
    /// fan out to.
    ///
    /// Cross-partition operations are expensive by design; an accidental broad
    /// query can span a very large number of physical partitions. When a fresh
    /// plan would exceed this limit, planning fails with
    /// [`CosmosStatus::CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED`](crate::error::CosmosStatus::CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED).
    ///
    /// The limit is enforced **only at initial plan time**, against the fan-out
    /// computed from the current partition topology. It is not a runtime cap: if
    /// a partition splits mid-execution and pushes the effective fan-out above
    /// this value, the operation keeps running and is not aborted. Likewise,
    /// resuming from a continuation token does not re-check the limit — the
    /// caller already opted in when the operation was first planned.
    ///
    /// Defaults to [`DEFAULT_MAX_FAN_OUT`].
    pub max_fan_out: u32,

    /// Maximum number of candidate rows a non-streaming cross-partition
    /// `ORDER BY` query may retain in memory.
    ///
    /// Vector searches require a finite `TOP` or `OFFSET`/`LIMIT` window and
    /// buffer that window before returning the first page. Planning fails when
    /// the required candidate count exceeds this value.
    ///
    /// Defaults to [`DEFAULT_MAX_NON_STREAMING_ORDER_BY_BUFFERED_ITEMS`].
    pub max_non_streaming_order_by_buffered_items: u32,
}

impl Default for PlanOptions {
    fn default() -> Self {
        Self {
            max_fan_out: DEFAULT_MAX_FAN_OUT,
            max_non_streaming_order_by_buffered_items:
                DEFAULT_MAX_NON_STREAMING_ORDER_BY_BUFFERED_ITEMS,
        }
    }
}

impl PlanOptions {
    /// Sets the maximum fan-out for a fresh cross-partition plan.
    pub fn with_max_fan_out(mut self, max_fan_out: u32) -> Self {
        self.max_fan_out = max_fan_out;
        self
    }

    /// Sets the maximum number of candidate rows retained by a non-streaming
    /// cross-partition `ORDER BY` query.
    pub fn with_max_non_streaming_order_by_buffered_items(
        mut self,
        max_buffered_items: u32,
    ) -> Self {
        self.max_non_streaming_order_by_buffered_items = max_buffered_items;
        self
    }
}
