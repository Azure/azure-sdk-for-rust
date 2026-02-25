// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Builder for creating a fault injection client.

use std::sync::Arc;

use azure_core::http::Transport;

use super::http_client::FaultClient;
use super::rule::FaultInjectionRule;

/// Builder for creating a fault injection client.
///
/// This builder creates a [`Transport`] that can be used with [`CosmosClientBuilder`](crate::CosmosClientBuilder)
/// to inject faults for testing purposes.
///
/// # Example
///
/// ```rust,no_run
/// use azure_data_cosmos::{
///     CosmosClientBuilder, CosmosAccountEndpoint,
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
///     .build((endpoint, Secret::from("my_account_key")))
///     .await
///     .unwrap();
/// # }
/// ```
pub struct FaultInjectionClientBuilder {
    /// The fault injection rules to apply.
    /// First valid rule will be applied.
    rules: Vec<Arc<FaultInjectionRule>>,
    /// Optional custom inner HTTP client (for testing with custom transports).
    inner_client: Option<Arc<dyn azure_core::http::HttpClient>>,
}

impl FaultInjectionClientBuilder {
    /// Creates a new FaultInjectionClientBuilder.
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            inner_client: None,
        }
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

    /// Sets a custom inner HTTP client to wrap with fault injection.
    ///
    /// This is useful when you need fault injection combined with other transport
    /// customizations, such as accepting invalid certificates for emulator testing.
    ///
    /// If not set, a default HTTP client will be created.
    pub fn with_inner_client(mut self, client: Arc<dyn azure_core::http::HttpClient>) -> Self {
        self.inner_client = Some(client);
        self
    }

    /// Builds the fault injection transport.
    ///
    /// Returns a [`Transport`] that wraps the inner HTTP client with fault injection capabilities.
    ///
    /// Note: When using [`CosmosClientBuilder::with_fault_injection()`](crate::CosmosClientBuilder::with_fault_injection()),
    /// this method is called internally. You only need to call `build()` directly if constructing
    /// the transport for use outside of `CosmosClientBuilder`.
    pub fn build(self) -> Transport {
        let inner_client = self
            .inner_client
            .unwrap_or_else(|| azure_core::http::new_http_client());

        let fault_client = FaultClient::new(inner_client, self.rules);
        Transport::new(Arc::new(fault_client))
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
    fn builder_default() {
        let builder = FaultInjectionClientBuilder::default();
        let _transport = builder.build();
        // Transport is created successfully - that's all we can verify without more setup
    }
}
