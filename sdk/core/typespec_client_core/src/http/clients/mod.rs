// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Built-in HTTP clients.

#[cfg(not(feature = "reqwest"))]
mod noop;
#[cfg(feature = "reqwest")]
mod reqwest;

#[cfg(not(feature = "reqwest"))]
use self::noop::new_noop_client;
#[cfg(feature = "reqwest")]
use self::reqwest::new_reqwest_client;

use crate::http::{BufResponse, Request};
use async_trait::async_trait;
use std::sync::Arc;
use typespec::error::Result;

/// Create a new [`HttpClient`].
pub fn new_http_client() -> Arc<dyn HttpClient> {
    #[cfg(feature = "reqwest")]
    {
        new_reqwest_client()
    }
    #[cfg(not(feature = "reqwest"))]
    {
        new_noop_client()
    }
}

/// An HTTP client which can send requests.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HttpClient: Send + Sync + std::fmt::Debug {
    /// Send a request to the service.
    ///
    /// It does not consume the request. Implementors are expected to clone the necessary parts
    /// of the request and pass them to the underlying transport.
    ///
    /// # Errors
    ///
    /// The built-in [`RetryPolicy`](crate::http::policies::RetryPolicy) will resend the [`Request`]
    /// for some [`ErrorKind::HttpResponse`](crate::error::ErrorKind::HttpResponse) status codes e.g., [`StatusCode::TooManyRequests`](crate::http::StatusCode::TooManyRequests) and
    /// for [`ErrorKind::Io`](crate::error::ErrorKind::Io) returned by your `HttpClient` for situations like connection resets.
    async fn execute_request(&self, request: &Request) -> Result<BufResponse>;
}
