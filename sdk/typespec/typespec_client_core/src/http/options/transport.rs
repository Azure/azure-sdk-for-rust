// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{clients, Context, HttpClient, Policy, Request, Response};
use std::sync::Arc;
use typespec::error::Result;

/// Transport options.
#[derive(Clone, Debug)]
pub struct TransportOptions {
    inner: TransportOptionsImpl,
}

#[derive(Clone, Debug)]
enum TransportOptionsImpl {
    Http {
        /// The HTTP client implementation to use for requests.
        http_client: Arc<dyn HttpClient>,
    },
    Custom(Arc<dyn Policy>),
}

impl TransportOptions {
    /// Creates a new `TransportOptions` using the given `HttpClient`.
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        let inner = TransportOptionsImpl::Http { http_client };
        Self { inner }
    }

    /// Creates a new `TransportOptions` using the custom policy.
    ///
    /// This policy is expected to be the last policy in the pipeline.
    pub fn new_custom_policy(policy: Arc<dyn Policy>) -> Self {
        let inner = TransportOptionsImpl::Custom(policy);
        Self { inner }
    }

    /// Use these options to send a request.
    pub async fn send<T>(&self, ctx: &Context<'_>, request: &mut Request) -> Result<Response<T>> {
        use TransportOptionsImpl as I;
        let raw_response = match &self.inner {
            I::Http { http_client } => http_client.execute_request(request).await,
            I::Custom(s) => s.send(ctx, request, &[]).await,
        };

        raw_response.map(|r| r.set_default_deserialize_type())
    }
}

impl Default for TransportOptions {
    /// Creates an instance of the `TransportOptions` using the default `HttpClient`.
    fn default() -> Self {
        Self::new(clients::new_http_client())
    }
}
