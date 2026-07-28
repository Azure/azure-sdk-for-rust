// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod instrumentation;
mod user_agent;

pub use instrumentation::*;
use std::{borrow::Cow, collections::HashSet, sync::Arc};
use typespec_client_core::http::{policies::Policy, DEFAULT_ALLOWED_QUERY_PARAMETERS};
pub use typespec_client_core::http::{
    ClientMethodOptions, ExponentialRetryOptions, FixedRetryOptions, LoggingOptions,
    PipelineOptions, RetryOptions, Transport,
};
pub use user_agent::*;

use crate::cloud::CloudConfiguration;

/// Client options allow customization of general client policies, retry options, and more.
#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    /// Policies called per call.
    pub per_call_policies: Vec<Arc<dyn Policy>>,

    /// Policies called per try.
    pub per_try_policies: Vec<Arc<dyn Policy>>,

    /// Retry options.
    pub retry: RetryOptions,

    /// Transport options.
    pub transport: Option<Transport>,

    /// User-Agent telemetry options.
    pub user_agent: UserAgentOptions,

    /// Options for request instrumentation, such as distributed tracing.
    ///
    /// If not specified, defaults to no instrumentation.
    ///
    pub instrumentation: InstrumentationOptions,

    /// Logging options
    ///
    /// Specifies which headers and query parameters should be logged. All headers and query parameters not in the allow list will be redacted.
    pub logging: LoggingOptions,

    /// Cloud configuration for the client. If None, the client will default to Azure Public Cloud.
    pub cloud: Option<Arc<CloudConfiguration>>,
}

pub(crate) struct CoreClientOptions {
    pub(crate) user_agent: UserAgentOptions,
    pub(crate) instrumentation: InstrumentationOptions,
    pub(crate) allowed_query_params: HashSet<Cow<'static, str>>,
}

impl ClientOptions {
    /// Efficiently deconstructs into owned [`typespec_client_core::http::ClientOptions`] as well as unwrapped or default Azure-specific options.
    ///
    /// If instead we implemented [`Into`], we'd have to clone Azure-specific options instead of moving memory of [`Some`] values.
    pub(in crate::http) fn deconstruct(
        self,
    ) -> (CoreClientOptions, typespec_client_core::http::ClientOptions) {
        // Merge the default allowed query parameters with any additional ones from logging options.
        // This merged set is shared by both the logging policy and the request instrumentation policy
        // to sanitize query parameters in logs and traced URLs.
        let mut allowed_query_params = (*DEFAULT_ALLOWED_QUERY_PARAMETERS).clone();
        allowed_query_params.extend(self.logging.additional_allowed_query_params.iter().cloned());

        let options = typespec_client_core::http::ClientOptions {
            per_call_policies: self.per_call_policies,
            per_try_policies: self.per_try_policies,
            retry: self.retry,
            transport: self.transport,
            logging: self.logging,
        };

        (
            CoreClientOptions {
                user_agent: self.user_agent,
                instrumentation: self.instrumentation,
                allowed_query_params,
            },
            options,
        )
    }
}
