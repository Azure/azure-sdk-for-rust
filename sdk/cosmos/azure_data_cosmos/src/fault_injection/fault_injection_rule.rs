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
    pub result: Box<FaultInjectionResult>,
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
    result: Box<FaultInjectionResult>,
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
    pub fn with_result(mut self, result: FaultInjectionResult) -> Self {
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

#[cfg(test)]
mod tests {
    use super::FaultInjectionRuleBuilder;
    use crate::fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultOperationType,
    };
    use crate::regions;
    use std::time::Duration;

    fn create_test_error() -> crate::fault_injection::FaultInjectionResult {
        FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::Timeout)
            .build()
    }

    #[test]
    fn builder_default_values() {
        let rule = FaultInjectionRuleBuilder::new("test-rule", create_test_error()).build();

        assert_eq!(rule.id, "test-rule");
        assert_eq!(rule.duration, Duration::MAX);
        assert_eq!(rule.start_delay, Duration::ZERO);
        assert!(rule.hit_limit.is_none());
        assert!(rule.condition.operation_type.is_none());
    }

    #[test]
    fn builder_with_condition() {
        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::CreateItem)
            .with_region(regions::EAST_US)
            .build();

        let rule = FaultInjectionRuleBuilder::new("rule-1", create_test_error())
            .with_condition(condition)
            .build();

        assert_eq!(
            rule.condition.operation_type,
            Some(FaultOperationType::CreateItem)
        );
        assert_eq!(rule.condition.region, Some(regions::EAST_US));
    }

    #[test]
    fn builder_with_duration() {
        let rule = FaultInjectionRuleBuilder::new("rule-2", create_test_error())
            .with_duration(Duration::from_secs(60))
            .build();

        assert_eq!(rule.duration, Duration::from_secs(60));
    }

    #[test]
    fn builder_with_start_delay() {
        let rule = FaultInjectionRuleBuilder::new("rule-3", create_test_error())
            .with_start_delay(Duration::from_secs(10))
            .build();

        assert_eq!(rule.start_delay, Duration::from_secs(10));
    }

    #[test]
    fn builder_with_hit_limit() {
        let rule = FaultInjectionRuleBuilder::new("rule-4", create_test_error())
            .with_hit_limit(5)
            .build();

        assert_eq!(rule.hit_limit, Some(5));
    }

    #[test]
    fn builder_with_result() {
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::TooManyRequests)
            .with_times(3)
            .build();

        let rule = FaultInjectionRuleBuilder::new("rule-5", create_test_error())
            .with_result(error)
            .build();

        assert_eq!(
            rule.result.error_type,
            Some(FaultInjectionErrorType::TooManyRequests)
        );
        assert_eq!(rule.result.times, Some(3));
    }

    #[test]
    fn builder_chained() {
        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::DeleteItem)
            .build();

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .build();

        let rule = FaultInjectionRuleBuilder::new("full-rule", error)
            .with_condition(condition)
            .with_duration(Duration::from_secs(120))
            .with_start_delay(Duration::from_secs(5))
            .with_hit_limit(10)
            .build();

        assert_eq!(rule.id, "full-rule");
        assert_eq!(
            rule.condition.operation_type,
            Some(FaultOperationType::DeleteItem)
        );
        assert_eq!(rule.duration, Duration::from_secs(120));
        assert_eq!(rule.start_delay, Duration::from_secs(5));
        assert_eq!(rule.hit_limit, Some(10));
        assert_eq!(
            rule.result.error_type,
            Some(FaultInjectionErrorType::ServiceUnavailable)
        );
    }

    #[test]
    fn rule_clone() {
        let rule = FaultInjectionRuleBuilder::new("clone-test", create_test_error())
            .with_hit_limit(3)
            .build();

        let cloned = rule.clone();

        assert_eq!(cloned.id, rule.id);
        assert_eq!(cloned.hit_limit, rule.hit_limit);
        assert_eq!(cloned.duration, rule.duration);
    }

    #[test]
    fn id_accepts_string_types() {
        let rule1 = FaultInjectionRuleBuilder::new("static-str", create_test_error()).build();
        assert_eq!(rule1.id, "static-str");

        let rule2 =
            FaultInjectionRuleBuilder::new(String::from("owned-string"), create_test_error())
                .build();
        assert_eq!(rule2.id, "owned-string");
    }
}
