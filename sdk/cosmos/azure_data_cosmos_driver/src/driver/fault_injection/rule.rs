// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines fault injection rules that combine conditions and results.

use std::sync::atomic::AtomicBool;
use std::time::Instant;

use super::condition::FaultInjectionCondition;
use super::result::FaultInjectionResult;

/// A fault injection rule that defines when and how to inject faults.
#[derive(Debug)]
pub struct FaultInjectionRule {
    /// The condition under which to inject the fault.
    pub condition: FaultInjectionCondition,
    /// The result to inject when the condition is met.
    pub result: FaultInjectionResult,
    /// The absolute time at which the rule becomes active.
    pub start_time: Instant,
    /// The absolute time at which the rule expires, if set.
    pub end_time: Option<Instant>,
    /// The total hit limit of the rule.
    pub hit_limit: Option<u32>,
    /// Unique identifier for the fault injection scenario.
    pub id: String,
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
    /// The absolute time at which the rule becomes active.
    start_time: Instant,
    /// The absolute time at which the rule expires.
    end_time: Option<Instant>,
    /// The total hit limit of the rule.
    hit_limit: Option<u32>,
    /// Unique identifier for the fault injection scenario.
    id: String,
    /// Whether the rule starts enabled.
    enabled: bool,
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
            enabled: true,
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

    /// Sets the rule to start in a disabled state.
    ///
    /// By default, rules start enabled. Call this to create a rule that must be
    /// explicitly enabled via [`FaultInjectionRule::enable()`].
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
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
            enabled: AtomicBool::new(self.enabled),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FaultInjectionRuleBuilder;
    use crate::driver::fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultOperationType,
    };
    use std::time::{Duration, Instant};

    fn create_test_error() -> crate::driver::fault_injection::FaultInjectionResult {
        FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::Timeout)
            .build()
    }

    #[test]
    fn builder_default_values() {
        let before = Instant::now();
        let rule = FaultInjectionRuleBuilder::new("test-rule", create_test_error()).build();
        let after = Instant::now();

        assert_eq!(rule.id, "test-rule");
        assert!(rule.start_time >= before && rule.start_time <= after);
        assert_eq!(rule.end_time, None);
        assert_eq!(rule.hit_limit, None);
        assert_eq!(rule.condition.operation_type, None);
        assert_eq!(rule.condition.region, None);
        assert_eq!(rule.condition.container_id, None);
        assert_eq!(
            rule.result.error_type,
            Some(FaultInjectionErrorType::Timeout)
        );
        assert_eq!(rule.is_enabled(), true);
    }

    #[test]
    fn builder_with_all_fields() {
        let start = Instant::now();
        let end = start + Duration::from_secs(30);
        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::ReadItem)
            .build();
        let result = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .with_delay(Duration::from_millis(100))
            .build();

        let rule = FaultInjectionRuleBuilder::new("full-rule", result)
            .with_condition(condition)
            .with_start_time(start)
            .with_end_time(end)
            .with_hit_limit(5)
            .build();

        assert_eq!(rule.id, "full-rule");
        assert_eq!(rule.start_time, start);
        assert_eq!(rule.end_time, Some(end));
        assert_eq!(rule.hit_limit, Some(5));
        assert_eq!(
            rule.condition.operation_type,
            Some(FaultOperationType::ReadItem)
        );
        assert_eq!(
            rule.result.error_type,
            Some(FaultInjectionErrorType::ServiceUnavailable)
        );
        assert_eq!(rule.result.delay, Duration::from_millis(100));
        assert_eq!(rule.is_enabled(), true);
    }

    #[test]
    fn enable_disable_toggles_state() {
        let rule = FaultInjectionRuleBuilder::new("toggle", create_test_error()).build();
        assert_eq!(rule.is_enabled(), true);

        rule.disable();
        assert_eq!(rule.is_enabled(), false);

        rule.enable();
        assert_eq!(rule.is_enabled(), true);
    }

    #[test]
    fn disabled_builder_starts_rule_disabled() {
        let rule = FaultInjectionRuleBuilder::new("disabled-rule", create_test_error())
            .disabled()
            .build();
        assert_eq!(rule.is_enabled(), false);
        assert_eq!(rule.id, "disabled-rule");

        rule.enable();
        assert_eq!(rule.is_enabled(), true);
    }
}
