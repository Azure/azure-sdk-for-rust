// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines fault injection rules that combine conditions and results.

use std::sync::atomic::AtomicBool;
use std::time::Duration;

use super::fault_injection_condition::FaultInjectionCondition;
use super::fault_injection_result::FaultInjectionResult;

/// A fault injection rule that defines when and how to inject faults.
#[derive(Debug)]
pub struct FaultInjectionRule {
    /// The condition under which to inject the fault.
    condition: FaultInjectionCondition,
    /// The result to inject when the condition is met.
    result: FaultInjectionResult,
    /// Duration for which the fault injection is active.
    duration: Duration,
    /// Delay before starting the fault injection.
    start_delay: Duration,
    /// The total hit limit of the rule.
    hit_limit: Option<u32>,
    /// Unique identifier for the fault injection scenario.
    /// This can be used for logging and debugging purposes to identify which rule was applied.
    id: String,
    /// Whether the rule is currently enabled.
    enabled: AtomicBool,
}

impl Clone for FaultInjectionRule {
    fn clone(&self) -> Self {
        Self {
            condition: self.condition.clone(),
            result: self.result.clone(),
            duration: self.duration,
            start_delay: self.start_delay,
            hit_limit: self.hit_limit,
            id: self.id.clone(),
            enabled: AtomicBool::new(self.enabled.load(std::sync::atomic::Ordering::SeqCst)),
        }
    }
}

impl FaultInjectionRule {
    /// Returns the condition under which the fault is injected.
    pub fn condition(&self) -> &FaultInjectionCondition {
        &self.condition
    }

    /// Returns the result to inject when the condition is met.
    pub fn result(&self) -> &FaultInjectionResult {
        &self.result
    }

    /// Returns the duration for which the fault injection is active.
    pub fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the delay before starting the fault injection.
    pub fn start_delay(&self) -> Duration {
        self.start_delay
    }

    /// Returns the total hit limit of the rule, if set.
    pub fn hit_limit(&self) -> Option<u32> {
        self.hit_limit
    }

    /// Returns the unique identifier for this rule.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns whether the rule is currently enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Enables the rule.
    pub fn enable(&self) {
        self.enabled
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// Disables the rule.
    pub fn disable(&self) {
        self.enabled
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

/// Builder for creating a fault injection rule.
pub struct FaultInjectionRuleBuilder {
    /// The condition under which to inject the fault.
    condition: FaultInjectionCondition,
    /// The result to inject when the condition is met.
    result: FaultInjectionResult,
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
    pub fn new(id: impl Into<String>, result: FaultInjectionResult) -> Self {
        Self {
            condition: FaultInjectionCondition::default(),
            result,
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
    pub fn with_result(mut self, result: FaultInjectionResult) -> Self {
        self.result = result;
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
            enabled: AtomicBool::new(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FaultInjectionRuleBuilder;
    use crate::fault_injection::{FaultInjectionErrorType, FaultInjectionResultBuilder};
    use std::time::Duration;

    fn create_test_error() -> crate::fault_injection::FaultInjectionResult {
        FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::Timeout)
            .build()
    }

    #[test]
    fn builder_default_values() {
        let rule = FaultInjectionRuleBuilder::new("test-rule", create_test_error()).build();

        assert_eq!(rule.id(), "test-rule");
        assert_eq!(rule.duration(), Duration::MAX);
        assert_eq!(rule.start_delay(), Duration::ZERO);
        assert!(rule.hit_limit().is_none());
        assert!(rule.condition().operation_type().is_none());
        assert!(rule.is_enabled());
    }
}
