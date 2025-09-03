// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod instrumentation;
mod user_agent;

pub use instrumentation::*;
use std::sync::Arc;
use typespec_client_core::http::policies::Policy;
pub use typespec_client_core::http::{
    ClientMethodOptions, ExponentialRetryOptions, FixedRetryOptions, LoggingOptions,
    PipelineOptions, RetryOptions, TransportOptions,
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

    /// Logging options
    ///
    /// Specifies which headers and query parameters should be logged. All headers and query parameters not in the allow list will be redacted.
    pub logging: LoggingOptions,
}

pub(crate) struct CoreClientOptions {
    pub(crate) user_agent: UserAgentOptions,
    pub(crate) instrumentation: InstrumentationOptions,
}

impl ClientOptions {
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
            logging: self.logging,
        };

        (
            CoreClientOptions {
                user_agent: self.user_agent.unwrap_or_default(),
                instrumentation: self.instrumentation.unwrap_or_default(),
            },
            options,
        )
    }
}
