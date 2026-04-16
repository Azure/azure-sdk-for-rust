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

use crate::http::{AsyncRawResponse, Request};
use async_trait::async_trait;
use std::sync::Arc;
use typespec::error::Result;

/// Options to construct an [`HttpClient`] from [`new_http_client()`].
#[derive(Clone, Debug)]
pub struct HttpClientOptions {
    /// Automatically decompress responses if the `content-encoding` header indicates a supported compression encoding.
    /// Defaults to `true`.
    ///
    /// # Notes
    ///
    /// Only affects the built-in `reqwest::Client` if the `reqwest` feature is enabled,
    /// and only if either the `reqwest_deflate` or `reqwest_gzip` feature are enabled.
    pub automatic_decompression: bool,
}

impl Default for HttpClientOptions {
    fn default() -> Self {
        Self {
            automatic_decompression: true,
        }
    }
}

/// Create a new [`HttpClient`].
///
/// # Arguments
///
/// * `options` - Optional configuration for the [`Client`](::reqwest::Client).
///   Automatic decompression is enabled if `reqwest_gzip` or `reqwest_deflate` are enabled.
///   Client libraries can disable this without impacting other client libraries by disabling it
///   when calling this function.
#[cfg_attr(not(feature = "reqwest"), allow(unused_variables))]
pub fn new_http_client(options: Option<HttpClientOptions>) -> Arc<dyn HttpClient> {
    #[cfg(feature = "reqwest")]
    {
        new_reqwest_client(options)
    }
    #[cfg(not(feature = "reqwest"))]
    {
        new_noop_client()
    }
}

/// An HTTP client which can send requests.
#[async_trait]
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
    async fn execute_request(&self, request: &Request) -> Result<AsyncRawResponse>;
}
