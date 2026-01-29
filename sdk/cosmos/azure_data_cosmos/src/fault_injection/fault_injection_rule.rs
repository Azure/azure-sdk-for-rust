// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines fault injection rules that combine conditions and results.

use std::time::Duration;

use super::fault_injection_condition::FaultInjectionCondition;
use super::fault_injection_result::FaultInjectionResult;

/// A fault injection rule that defines when and how to inject faults.
#[derive(Clone, Debug)]
pub struct FaultInjectionRule {
    /// The condition under which to inject the fault.
    pub condition: FaultInjectionCondition,
    /// The result to inject when the condition is met.
    pub result: Box<dyn FaultInjectionResult>,
    /// Duration for which the fault injection is active.
    /// default is infinite duration.
    pub duration: Duration,
    /// Delay before starting the fault injection.
    /// Default is no delay.
    pub start_delay: Duration,
    /// Set the total hit limit of the rule. The rule will be not applicable anymore once it has applied hitLimit times.
    ///
    /// By default, there is no limit.
    pub hit_limit: Option<u32>,
    /// Unique identifier for the fault injection scenario.
    pub id: String,
}

/// Builder for creating a fault injection rule.
pub struct FaultInjectionRuleBuilder {
    /// The condition under which to inject the fault.
    condition: FaultInjectionCondition,
    /// The result to inject when the condition is met.
    result: Box<dyn FaultInjectionResult>,
    /// Duration for which the fault injection is active.
    /// default is infinite duration.
    duration: Duration,
    /// Delay before starting the fault injection.
    /// Default is no delay.
    start_delay: Duration,
    /// Set the total hit limit of the rule. The rule will be not applicable anymore once it has applied hitLimit times.
    ///
    /// By default, there is no limit.
    hit_limit: Option<u32>,
    /// Unique identifier for the fault injection scenario.
    id: String,
}

impl FaultInjectionRuleBuilder {
    /// Creates a new FaultInjectionRuleBuilder with default values.
    pub fn new(id: impl Into<String>, result: impl FaultInjectionResult + 'static) -> Self {
        Self {
            condition: FaultInjectionCondition::default(),
            result: Box::new(result),
            duration: Duration::MAX, // Infinite duration by default
            start_delay: Duration::ZERO,
            hit_limit: None,
            id: id.into(),
        }
    }

    /// Sets the condition for when to inject the fault.
    pub fn with_condition(mut self, condition: FaultInjectionCondition) -> Self {
        self.condition = condition;
        self
    }

    /// Sets the result to inject when the condition is met.
    pub fn with_result(mut self, result: impl FaultInjectionResult + 'static) -> Self {
        self.result = Box::new(result);
        self
    }

    /// Sets the duration for which the fault injection is active.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the delay before starting the fault injection.
    pub fn with_start_delay(mut self, start_delay: Duration) -> Self {
        self.start_delay = start_delay;
        self
    }

    /// Sets the total hit limit of the rule.
    pub fn with_hit_limit(mut self, hit_limit: u32) -> Self {
        self.hit_limit = Some(hit_limit);
        self
    }

    /// Builds the FaultInjectionRule.
    ///
    /// # Panics
    /// Panics if no result has been set.
    pub fn build(self) -> FaultInjectionRule {
        FaultInjectionRule {
            condition: self.condition,
            result: self.result,
            duration: self.duration,
            start_delay: self.start_delay,
            hit_limit: self.hit_limit,
            id: self.id,
        }
    }
}

