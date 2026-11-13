// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Short constructors for client-side fault injection rules.
//!
//! ```rust,ignore
//! let slow = faults::delay(Op::ReadItem, Duration::from_millis(500)).in_region(PrimaryReadWrite);
//! let client = container.client_with_faults([slow]).await?;
//! ```

use crate::{RegionAlias, RegionMap, SpecResult};
use azure_data_cosmos::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};
use std::{sync::Arc, time::Duration};

pub use azure_data_cosmos::fault_injection::FaultOperationType as Op;

/// A fault under construction. Pass it to
/// [`ExclusiveContainer::client_with_faults`](crate::ExclusiveContainer::client_with_faults)
/// or [`SharedContainer::client_with_faults`](crate::SharedContainer::client_with_faults).
#[derive(Clone, Debug)]
pub struct Fault {
    operation: FaultOperationType,
    error: Option<FaultInjectionErrorType>,
    delay: Option<Duration>,
    region: Option<RegionAlias>,
    times: Option<u32>,
}

/// Every matching request waits `delay` before it is sent.
pub fn delay(operation: FaultOperationType, delay: Duration) -> Fault {
    Fault {
        operation,
        error: None,
        delay: Some(delay),
        region: None,
        times: None,
    }
}

/// Every matching request fails with `error` instead of reaching the service.
pub fn error(operation: FaultOperationType, error: FaultInjectionErrorType) -> Fault {
    Fault {
        operation,
        error: Some(error),
        delay: None,
        region: None,
        times: None,
    }
}

/// Every matching request is throttled with `429 Too Many Requests`.
pub fn throttle(operation: FaultOperationType) -> Fault {
    error(operation, FaultInjectionErrorType::TooManyRequests)
}

impl Fault {
    /// Only matches requests sent to the region behind `region`.
    pub fn in_region(mut self, region: RegionAlias) -> Self {
        self.region = Some(region);
        self
    }

    /// Stops matching after `n` hits.
    pub fn times(mut self, n: u32) -> Self {
        self.times = Some(n);
        self
    }

    /// Builds the driver rule, resolving the region alias with `regions`.
    pub(crate) fn build(self, regions: &RegionMap) -> SpecResult<Arc<FaultInjectionRule>> {
        let mut result = FaultInjectionResultBuilder::new().with_probability(1.0);
        if let Some(error) = self.error {
            result = result.with_error(error);
        }
        if let Some(delay) = self.delay {
            result = result.with_delay(delay);
        }
        let mut condition =
            FaultInjectionConditionBuilder::new().with_operation_type(self.operation);
        if let Some(alias) = self.region {
            condition = condition.with_region(regions.region(alias)?);
        }
        let id = format!("spec-{}", uuid::Uuid::new_v4());
        let mut rule =
            FaultInjectionRuleBuilder::new(id, result.build()).with_condition(condition.build());
        if let Some(n) = self.times {
            rule = rule.with_hit_limit(n);
        }
        Ok(Arc::new(rule.build()))
    }
}
