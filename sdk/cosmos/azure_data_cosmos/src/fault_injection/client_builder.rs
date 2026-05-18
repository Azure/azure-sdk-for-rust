// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Builder for collecting fault injection rules to register with a Cosmos client.

use std::sync::Arc;

use super::FaultInjectionRule;

/// Builder for assembling the fault injection rule set for a Cosmos client.
///
/// This builder is a thin SDK-side container for [`FaultInjectionRule`]s.
/// Pass it to
/// [`CosmosClientBuilder::with_fault_injection`](crate::CosmosClientBuilder::with_fault_injection),
/// which forwards the rules into the driver runtime; the driver's
/// fault-injection transport client then evaluates the rules on every
/// in-flight request.
///
/// # Example
///
/// ```rust,no_run
/// use azure_data_cosmos::{
///     CosmosClientBuilder, CosmosAccountEndpoint, Region, RoutingStrategy,
///     fault_injection::{
///         FaultInjectionClientBuilder, FaultInjectionErrorType,
///         FaultInjectionResultBuilder, FaultInjectionRuleBuilder,
///     },
/// };
/// use azure_core::credentials::Secret;
/// use std::sync::Arc;
///
/// # async fn doc() {
/// let result = FaultInjectionResultBuilder::new()
///     .with_error(FaultInjectionErrorType::ServiceUnavailable)
///     .build();
///
/// let rule = Arc::new(FaultInjectionRuleBuilder::new("my-rule", result).build());
///
/// let fault_builder = FaultInjectionClientBuilder::new()
///     .with_rule(rule);
///
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let client = CosmosClientBuilder::new()
///     .with_fault_injection(fault_builder)
///     .build(
///         (endpoint, Secret::from("my_account_key")),
///         RoutingStrategy::ProximityTo(Region::EAST_US),
///     )
///     .await
///     .unwrap();
/// # }
/// ```
pub struct FaultInjectionClientBuilder {
    /// The fault injection rules to apply. The first matching rule is applied
    /// at the driver transport layer.
    rules: Vec<Arc<FaultInjectionRule>>,
}

impl FaultInjectionClientBuilder {
    /// Creates a new FaultInjectionClientBuilder.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Adds a fault injection rule to the builder.
    ///
    /// The rule is wrapped in an [`Arc`] so it can be shared with the caller,
    /// allowing runtime changes such as [`FaultInjectionRule::enable`] and
    /// [`FaultInjectionRule::disable`].
    pub fn with_rule(mut self, rule: Arc<FaultInjectionRule>) -> Self {
        self.rules.push(rule);
        self
    }

    /// Returns a reference to the current fault injection rules.
    pub(crate) fn rules(&self) -> &[Arc<FaultInjectionRule>] {
        &self.rules
    }
}

impl Default for FaultInjectionClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::FaultInjectionClientBuilder;

    #[test]
    fn builder_default_is_empty() {
        let builder = FaultInjectionClientBuilder::default();
        assert!(
            builder.rules().is_empty(),
            "default builder should hold zero rules"
        );
    }
}
