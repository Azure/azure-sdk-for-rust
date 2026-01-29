// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Builder for creating a fault injection client.

use std::sync::Arc;

use azure_core::http::Transport;

use crate::options::CosmosClientOptions;

use super::fault_http_client::FaultClient;
use super::fault_injection_rule::FaultInjectionRule;

/// Builder for creating a fault injection client.
pub struct FaultInjectionClientBuilder {
    /// The fault injection rules to apply.
    /// Rules will be applied from first to last.
    rules: Vec<FaultInjectionRule>,
}

impl FaultInjectionClientBuilder {
    /// Creates a new FaultInjectionClientBuilder.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Injects the fault injection client into the CosmosClientOptions.
    /// Called after building the fault conditions.
    ///
    /// This wraps the existing transport (or creates a default one) with the fault injection client.
    pub fn inject(&self, mut options: CosmosClientOptions) -> CosmosClientOptions {
        // Create a default http client
        let inner_client: Arc<dyn azure_core::http::HttpClient> = azure_core::http::new_http_client();

        let fault_client = FaultClient::new(inner_client, self.rules.clone());
        options.client_options.transport = Some(Transport::new(Arc::new(fault_client)));
        options.fault_injection_enabled = true;

        options
    }

    /// Adds a fault injection rule to the builder.
    pub fn with_rule(&mut self, rule: FaultInjectionRule) -> &mut Self {
        self.rules.push(rule);
        self
    }
}

impl Default for FaultInjectionClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

