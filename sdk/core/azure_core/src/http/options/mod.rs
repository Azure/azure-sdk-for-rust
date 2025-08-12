// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod instrumentation;
mod user_agent;

pub use instrumentation::*;
use crate::cloud::CloudConfiguration;
use std::sync::Arc;
use typespec_client_core::http::policies::Policy;
pub use typespec_client_core::http::{
    ClientMethodOptions, ExponentialRetryOptions, FixedRetryOptions, RetryOptions, TransportOptions,
};
pub use user_agent::*;

/// Client options allow customization of general client policies, retry options, and more.
#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    /// Policies called per call.
    pub per_call_policies: Vec<Arc<dyn Policy>>,

    /// Policies called per try.
    pub per_try_policies: Vec<Arc<dyn Policy>>,

    /// Retry options.
    pub retry: Option<RetryOptions>,

    /// Transport options.
    pub transport: Option<TransportOptions>,

    /// User-Agent telemetry options.
    pub user_agent: Option<UserAgentOptions>,

    /// Options for request instrumentation, such as distributed tracing.
    ///
    /// If not specified, defaults to no instrumentation.
    ///
    pub instrumentation: Option<InstrumentationOptions>,

    /// Cloud configuration for determining endpoints and audiences.
    ///
    /// If not specified, defaults to Azure Public Cloud.
    pub cloud_config: Option<&'static CloudConfiguration>,

    /// Service audience for token requests.
    ///
    /// This is typically the base URI of the service being accessed.
    /// If not specified, the audience will be derived from the cloud configuration
    /// for known services, or default to the resource manager audience.
    pub audience: Option<String>,
}

pub(crate) struct CoreClientOptions {
    pub(crate) user_agent: UserAgentOptions,
    pub(crate) instrumentation: InstrumentationOptions,
    pub(crate) cloud_config: &'static CloudConfiguration,
    pub(crate) audience: Option<String>,
}

impl ClientOptions {
    /// Set the cloud configuration.
    ///
    /// This determines the endpoints and audiences used for authentication
    /// and service requests.
    pub fn with_cloud_config(mut self, cloud_config: &'static CloudConfiguration) -> Self {
        self.cloud_config = Some(cloud_config);
        self
    }

    /// Set the service audience for token requests.
    ///
    /// The audience should be the base URI of the service being accessed.
    /// For example, for Azure Storage, use "https://storage.azure.com".
    pub fn with_audience(mut self, audience: impl Into<String>) -> Self {
        self.audience = Some(audience.into());
        self
    }

    /// Efficiently deconstructs into owned [`typespec_client_core::http::ClientOptions`] as well as unwrapped or default Azure-specific options.
    ///
    /// If instead we implemented [`Into`], we'd have to clone Azure-specific options instead of moving memory of [`Some`] values.
    pub(in crate::http) fn deconstruct(
        self,
    ) -> (CoreClientOptions, typespec_client_core::http::ClientOptions) {
        let options = typespec_client_core::http::ClientOptions {
            per_call_policies: self.per_call_policies,
            per_try_policies: self.per_try_policies,
            retry: self.retry,
            transport: self.transport,
        };

        (
            CoreClientOptions {
                user_agent: self.user_agent.unwrap_or_default(),
                instrumentation: self.instrumentation.unwrap_or_default(),
                cloud_config: self.cloud_config.unwrap_or_else(|| crate::cloud::configurations::azure_public_cloud()),
                audience: self.audience,
            },
            options,
        )
    }
}
