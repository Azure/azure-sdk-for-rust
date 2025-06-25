// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod user_agent;

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
}

impl From<ClientOptions> for typespec_client_core::http::ClientOptions {
    fn from(value: ClientOptions) -> Self {
        Self {
            per_call_policies: value.per_call_policies,
            per_try_policies: value.per_try_policies,
            retry: value.retry,
            transport: value.transport,
        }
    }
}
