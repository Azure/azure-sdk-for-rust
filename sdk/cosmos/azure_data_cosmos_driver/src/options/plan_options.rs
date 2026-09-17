// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Plan-time options for [`CosmosDriver::plan_operation`](crate::driver::CosmosDriver::plan_operation).

use crate::options::QueryPlanMode;

/// Default maximum fan-out for a fresh cross-partition operation.
///
/// A plan that would fan out to more than this many leaf request nodes is
/// rejected unless the caller raises [`PlanOptions::max_fan_out`].
pub const DEFAULT_MAX_FAN_OUT: u32 = 100;

/// Default maximum global OFFSET plus take for client-buffered queries.
pub const DEFAULT_MAX_BUFFERED_QUERY_WINDOW: u64 = 1000;

/// Options that shape how an operation is planned into a dataflow pipeline.
///
/// Unlike [`OperationOptions`](crate::options::OperationOptions), which controls
/// per-request behavior (consistency, routing, retries), `PlanOptions` controls
/// the *shape* of the plan itself. For example, how many partitions a plan
/// may fan out to.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PlanOptions {
    /// Maximum global OFFSET plus effective take for client-buffered queries.
    ///
    /// Requires a finite TOP or LIMIT; when both exist, the smaller is used.
    /// Defaults to [`DEFAULT_MAX_BUFFERED_QUERY_WINDOW`]. Zero is a valid limit.
    pub max_buffered_query_window: u64,

    /// Query-plan provider selection for this query.
    ///
    /// Defaults to [`QueryPlanMode::LocalPreferred`].
    pub query_plan_mode: QueryPlanMode,

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
}

impl Default for PlanOptions {
    fn default() -> Self {
        Self {
            max_buffered_query_window: DEFAULT_MAX_BUFFERED_QUERY_WINDOW,
            query_plan_mode: QueryPlanMode::LocalPreferred,
            max_fan_out: DEFAULT_MAX_FAN_OUT,
        }
    }
}

impl PlanOptions {
    /// Sets the maximum global OFFSET plus effective take for client-buffered queries.
    pub fn with_max_buffered_query_window(mut self, max_buffered_query_window: u64) -> Self {
        self.max_buffered_query_window = max_buffered_query_window;
        self
    }

    /// Sets the query-plan provider selection for this query.
    pub fn with_query_plan_mode(mut self, mode: QueryPlanMode) -> Self {
        self.query_plan_mode = mode;
        self
    }

    /// Sets the maximum fan-out for a fresh cross-partition plan.
    pub fn with_max_fan_out(mut self, max_fan_out: u32) -> Self {
        self.max_fan_out = max_fan_out;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::PlanOptions;
    use crate::{
        driver::dataflow::{
            planner::validate_buffered_query,
            query_plan::{DistinctType, QueryInfo, QueryPlan},
        },
        error::CosmosStatus,
    };

    #[test]
    fn buffered_admission_uses_plan_default_and_explicit_values() {
        let mut query = QueryPlan {
            query_info: Some(QueryInfo {
                distinct_type: DistinctType::Unordered,
                top: Some(1000),
                ..Default::default()
            }),
            ..Default::default()
        };
        for (maximum, take, accepted) in [
            (None, Some(1000), true),
            (None, Some(1001), false),
            (Some(1001), Some(1001), true),
            (Some(999), Some(1000), false),
            (Some(0), Some(0), true),
            (Some(0), Some(1), false),
            (Some(u64::MAX), None, false),
        ] {
            let options = maximum.map_or_else(PlanOptions::default, |maximum| {
                PlanOptions::default().with_max_buffered_query_window(maximum)
            });
            query.query_info.as_mut().unwrap().top = take;
            let result = validate_buffered_query(&query, options.max_buffered_query_window);
            assert_eq!(
                result.is_ok(),
                accepted,
                "maximum={maximum:?}, take={take:?}"
            );
            if let Err(error) = result {
                assert_eq!(
                    error.status(),
                    CosmosStatus::CLIENT_BUFFERED_QUERY_REQUIRES_FINITE_WINDOW
                );
            }
        }
    }
}
