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

use crate::{cloud::CloudConfiguration, http::headers::DEFAULT_ALLOWED_AZURE_HEADER_NAMES};

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
        // Merge query parameters into a HashSet up front because request instrumentation
        // needs the final lookup set on the hot path. Unlike the Azure-specific header
        // allowlist below, building an expanded Vec for LoggingOptions would duplicate
        // the shared defaults without avoiding this HashSet allocation.
        let mut allowed_query_params = (*DEFAULT_ALLOWED_QUERY_PARAMETERS).clone();
        allowed_query_params.extend(self.logging.additional_allowed_query_params.iter().cloned());

        // Merge the small Azure-specific allowlist after the shared defaults and before
        // customer headers. Keep this as a static slice rather than a lazy set because
        // it is only copied into the final HashSet once and is not used for per-request
        // lookups on the hot path.
        let mut additional_allowed_header_names: Vec<Cow<'static, str>> =
            DEFAULT_ALLOWED_AZURE_HEADER_NAMES
                .iter()
                .map(|s| Cow::Borrowed(*s))
                .collect();
        additional_allowed_header_names.extend(self.logging.additional_allowed_header_names);

        let options = typespec_client_core::http::ClientOptions {
            per_call_policies: self.per_call_policies,
            per_try_policies: self.per_try_policies,
            retry: self.retry,
            transport: self.transport,
            logging: LoggingOptions {
                additional_allowed_header_names,
                additional_allowed_query_params: self.logging.additional_allowed_query_params,
            },
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

#[cfg(test)]
mod tests {
    use super::*;
    use typespec_client_core::http::DEFAULT_ALLOWED_HEADER_NAMES;

    #[test]
    fn client_options_includes_azure_deprecating_header() {
        let options = ClientOptions::default();
        let (_core, tsc_options) = options.deconstruct();

        // Build the full set as LoggingPolicy would: typespec defaults + additional.
        let mut allowed: std::collections::HashSet<std::borrow::Cow<'static, str>> =
            (*DEFAULT_ALLOWED_HEADER_NAMES).clone();
        allowed.extend(tsc_options.logging.additional_allowed_header_names);

        assert!(
            allowed.contains("azure-deprecating"),
            "`azure-deprecating` must be in the allowed header set"
        );
    }
}
