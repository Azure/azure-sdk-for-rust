// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{clients, policies::Policy, AsyncRawResponse, Context, HttpClient, Request};
use std::sync::Arc;
use typespec::error::Result;

/// The HTTP transport.
#[derive(Clone, Debug)]
pub struct Transport {
    inner: TransportImpl,
}

#[derive(Clone, Debug)]
enum TransportImpl {
    HttpClient {
        /// The HTTP client implementation to use for requests.
        http_client: Arc<dyn HttpClient>,
    },
    Policy(Arc<dyn Policy>),
}

impl Transport {
    /// Creates a new `Transport` using the given [`HttpClient`].
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            inner: TransportImpl::HttpClient { http_client },
        }
    }

    /// Creates a new `Transport` using the custom policy.
    ///
    /// This policy is expected to be the last policy in the pipeline.
    pub fn with_policy(policy: Arc<dyn Policy>) -> Self {
        Self {
            inner: TransportImpl::Policy(policy),
        }
    }

    /// Use these options to send a request.
    pub async fn send(&self, ctx: &Context<'_>, request: &mut Request) -> Result<AsyncRawResponse> {
        use TransportImpl as I;
        match &self.inner {
            I::HttpClient { http_client } => http_client.execute_request(request).await,
            I::Policy(s) => s.send(ctx, request, &[]).await,
        }
    }
}

impl Default for Transport {
    /// Creates an instance of the `Transport` using the default [`HttpClient`].
    fn default() -> Self {
        Self::new(clients::new_http_client())
    }
}
