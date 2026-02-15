// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines fault injection rules that combine conditions and results.

use std::sync::atomic::AtomicBool;
use std::time::Instant;

use super::condition::FaultInjectionCondition;
use super::result::FaultInjectionResult;

/// A fault injection rule that defines when and how to inject faults.
#[non_exhaustive]
#[derive(Debug)]
pub struct FaultInjectionRule {
    /// The condition under which to inject the fault.
    pub(crate) condition: FaultInjectionCondition,
    /// The result to inject when the condition is met.
    pub(crate) result: FaultInjectionResult,
    /// The absolute time at which the rule becomes active.
    pub(crate) start_time: Instant,
    /// The absolute time at which the rule expires, if set.
    pub(crate) end_time: Option<Instant>,
    /// The total hit limit of the rule.
    pub(crate) hit_limit: Option<u32>,
    /// Unique identifier for the fault injection scenario.
    pub(crate) id: String,
    /// Whether the rule is currently enabled.
    enabled: AtomicBool,
}

impl Clone for FaultInjectionRule {
    fn clone(&self) -> Self {
        Self {
            condition: self.condition.clone(),
            result: self.result.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
            hit_limit: self.hit_limit,
            id: self.id.clone(),
            enabled: AtomicBool::new(self.enabled.load(std::sync::atomic::Ordering::SeqCst)),
        }
    }
}

impl FaultInjectionRule {
    /// Returns the condition under which to inject the fault.
    pub fn condition(&self) -> &FaultInjectionCondition {
        &self.condition
    }

    /// Returns the result to inject when the condition is met.
    pub fn result(&self) -> &FaultInjectionResult {
        &self.result
    }

    /// Returns the absolute time at which the rule becomes active.
    pub fn start_time(&self) -> Instant {
        self.start_time
    }

    /// Returns the absolute time at which the rule expires, if set.
    pub fn end_time(&self) -> Option<Instant> {
        self.end_time
    }

    /// Returns the total hit limit of the rule.
    pub fn hit_limit(&self) -> Option<u32> {
        self.hit_limit
    }

    /// Returns the unique identifier for the fault injection scenario.
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
#[non_exhaustive]
pub struct FaultInjectionRuleBuilder {
    /// The condition under which to inject the fault.
    condition: FaultInjectionCondition,
    /// The result to inject when the condition is met.
    result: FaultInjectionResult,
    /// The absolute time at which the rule becomes active.
    start_time: Instant,
    /// The absolute time at which the rule expires.
    end_time: Option<Instant>,
    /// The total hit limit of the rule.
    hit_limit: Option<u32>,
    /// Unique identifier for the fault injection scenario.
    id: String,
}

impl FaultInjectionRuleBuilder {
    /// Creates a new FaultInjectionRuleBuilder with default values.
    ///
    /// By default the rule starts immediately and never expires.
    pub fn new(id: impl Into<String>, result: FaultInjectionResult) -> Self {
        Self {
            condition: FaultInjectionCondition::default(),
            result,
            start_time: Instant::now(),
            end_time: None,
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

    /// Sets the absolute time at which the rule becomes active.
    pub fn with_start_time(mut self, start_time: Instant) -> Self {
        self.start_time = start_time;
        self
    }

    /// Sets the absolute time at which the rule expires.
    pub fn with_end_time(mut self, end_time: Instant) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Sets the total hit limit of the rule.
    pub fn with_hit_limit(mut self, hit_limit: u32) -> Self {
        self.hit_limit = Some(hit_limit);
        self
    }

    /// Builds the FaultInjectionRule.
    pub fn build(self) -> FaultInjectionRule {
        FaultInjectionRule {
            condition: self.condition,
            result: self.result,
            start_time: self.start_time,
            end_time: self.end_time,
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
    use std::time::Instant;

    fn create_test_error() -> crate::fault_injection::FaultInjectionResult {
        FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::Timeout)
            .build()
    }

    #[test]
    fn builder_default_values() {
        let before = Instant::now();
        let rule = FaultInjectionRuleBuilder::new("test-rule", create_test_error()).build();

        assert_eq!(rule.id, "test-rule");
        assert!(rule.start_time >= before);
        assert!(rule.start_time <= Instant::now());
        assert!(rule.end_time.is_none());
        assert!(rule.hit_limit.is_none());
        assert!(rule.condition.operation_type.is_none());
        assert!(rule.is_enabled());
    }
}
