// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines fault injection rules that combine conditions and results.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::Instant;

use super::condition::FaultInjectionCondition;
use super::result::FaultInjectionResult;

/// A fault injection rule that defines when and how to inject faults.
#[derive(Debug)]
#[non_exhaustive]
pub struct FaultInjectionRule {
    condition: FaultInjectionCondition,
    result: FaultInjectionResult,
    id: String,
    enabled: AtomicBool,
    hit_count: AtomicU32,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    hit_limit: Option<u32>,
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

    /// Returns the unique identifier for the fault injection scenario.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns whether the rule is currently enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    /// Enables the rule.
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::SeqCst);
    }

    /// Disables the rule.
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::SeqCst);
    }

    /// Returns the number of times this rule has been applied.
    pub fn hit_count(&self) -> u32 {
        self.hit_count.load(Ordering::SeqCst)
    }

    /// Increments the hit count by one.
    pub(crate) fn increment_hit_count(&self) {
        self.hit_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Returns the absolute time at which the rule becomes active, if set.
    pub fn start_time(&self) -> Option<Instant> {
        self.start_time
    }

    /// Returns the absolute time at which the rule expires, if set.
    pub fn end_time(&self) -> Option<Instant> {
        self.end_time
    }

    /// Returns the total hit limit of the rule, if set.
    pub fn hit_limit(&self) -> Option<u32> {
        self.hit_limit
    }
}

/// Builder for creating a fault injection rule.
pub struct FaultInjectionRuleBuilder {
    condition: FaultInjectionCondition,
    result: FaultInjectionResult,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    hit_limit: Option<u32>,
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
            start_time: None,
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
        self.start_time = Some(start_time);
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
            hit_count: AtomicU32::new(0),
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
        let rule = FaultInjectionRuleBuilder::new("test-rule", create_test_error()).build();

        assert_eq!(rule.id(), "test-rule");
        assert!(rule.start_time().is_none());
        assert!(rule.end_time().is_none());
        assert!(rule.hit_limit().is_none());
        assert!(rule.condition().operation_type().is_none());
        assert!(rule.is_enabled());
        assert_eq!(rule.hit_count(), 0);
    }

    #[test]
    fn hit_count_increments() {
        let rule = FaultInjectionRuleBuilder::new("hit-test", create_test_error()).build();

        assert_eq!(rule.hit_count(), 0);
        rule.increment_hit_count();
        assert_eq!(rule.hit_count(), 1);
        rule.increment_hit_count();
        assert_eq!(rule.hit_count(), 2);
    }

    #[test]
    fn enable_disable() {
        let rule = FaultInjectionRuleBuilder::new("toggle-test", create_test_error()).build();

        assert!(rule.is_enabled());
        rule.disable();
        assert!(!rule.is_enabled());
        rule.enable();
        assert!(rule.is_enabled());
    }

    #[test]
    fn builder_with_start_time() {
        let start = Instant::now();
        let rule = FaultInjectionRuleBuilder::new("start-test", create_test_error())
            .with_start_time(start)
            .build();

        assert_eq!(rule.start_time(), Some(start));
    }
}
