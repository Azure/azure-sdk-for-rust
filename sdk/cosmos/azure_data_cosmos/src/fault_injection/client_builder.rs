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
/// ```rust,ignore
/// use azure_data_cosmos::{CosmosClientBuilder, fault_injection::{FaultInjectionClientBuilder, FaultInjectionRule}};
/// use std::sync::Arc;
///
/// let rule = Arc::new(FaultInjectionRule::builder()
///     // configure rule...
///     .build());
///
/// let transport = FaultInjectionClientBuilder::new()
///     .with_rule(rule)
///     .build();
///
/// let client = CosmosClientBuilder::new()
///     .endpoint("https://myaccount.documents.azure.com/")
///     .credential(credential)
///     .fault_injection(true)
///     .transport(transport)
///     .build()
///     .unwrap();
/// ```
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

    /// Adds a fault injection rule to the builder.
    ///
    /// The rule is wrapped in an [`Arc`] so it can be shared with the caller,
    /// allowing runtime changes such as [`FaultInjectionRule::enable`] and
    /// [`FaultInjectionRule::disable`].
    pub fn with_rule(mut self, rule: Arc<FaultInjectionRule>) -> Self {
        self.rules.push(rule);
        self
    }

    /// Builds the fault injection transport.
    ///
    /// Returns a [`Transport`] that wraps a default HTTP client with fault injection capabilities.
    /// Use this transport with [`CosmosClientBuilder::transport()`](crate::CosmosClientBuilder::transport())
    /// and enable fault injection with [`CosmosClientBuilder::fault_injection(true)`](crate::CosmosClientBuilder::fault_injection()).
    pub fn build(self) -> Transport {
        let inner_client: Arc<dyn azure_core::http::HttpClient> =
            azure_core::http::new_http_client();

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
