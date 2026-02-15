// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Builder for creating a fault injection client.

use std::sync::Arc;

use azure_core::http::Transport;

use crate::options::CosmosClientOptions;

use super::http_client::FaultClient;
use super::rule::FaultInjectionRule;

/// Builder for creating a fault injection client.
#[non_exhaustive]
pub struct FaultInjectionClientBuilder {
    /// The fault injection rules to apply.
    /// First valid rule will be applied.
    rules: Vec<Arc<FaultInjectionRule>>,
}

impl FaultInjectionClientBuilder {
    /// Creates a new FaultInjectionClientBuilder.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Injects the fault injection client into the CosmosClientOptions using the default HTTP client.
    ///
    /// This wraps a default HTTP client with the fault injection client and sets it as the transport.
    pub fn inject(self, options: CosmosClientOptions) -> CosmosClientOptions {
        self.inject_with_http_client(azure_core::http::new_http_client(), options)
    }

    /// Injects the fault injection client into the CosmosClientOptions using a custom HTTP client.
    ///
    /// This wraps the provided `inner_client` with the fault injection client and sets it as the
    /// transport. Use this when the inner HTTP client needs custom configuration (e.g., accepting
    /// invalid certificates for emulator tests).
    pub fn inject_with_http_client(
        self,
        inner_client: Arc<dyn azure_core::http::HttpClient>,
        mut options: CosmosClientOptions,
    ) -> CosmosClientOptions {
        let fault_client = FaultClient::new(inner_client, self.rules);
        options.client_options.transport = Some(Transport::new(Arc::new(fault_client)));
        options.fault_injection_enabled = true;

        options
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
}

impl Default for FaultInjectionClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::FaultInjectionClientBuilder;
    use crate::options::CosmosClientOptions;

    #[test]
    fn builder_default() {
        let builder = FaultInjectionClientBuilder::default();
        let options = builder.inject(CosmosClientOptions::default());

        assert!(options.fault_injection_enabled);
        assert!(options.client_options.transport.is_some());
    }
}
