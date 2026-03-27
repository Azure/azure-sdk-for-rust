// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-owned HTTP transport abstraction for Cosmos DB.
//!
//! This module defines [`CosmosTransportClient`], the internal transport trait
//! that replaces direct use of `azure_core::http::HttpClient`. The key
//! motivation is **per-request timeout support**: `HttpClient` only allows a
//! single client-level timeout, but Cosmos DB operations need distinct timeouts
//! for metadata vs. data-plane calls and for individual retry attempts.
//!
//! All types in this module are `pub(crate)` — they are implementation details
//! of the driver and are not exposed in the public API.

use std::fmt;
use std::time::Duration;

use azure_core::http::headers::Headers;
use azure_core::http::Method;
use url::Url;

use crate::diagnostics::RequestSentStatus;

// ----------------------------------------------------------------------------
// Request
// ----------------------------------------------------------------------------

/// An HTTP request with optional per-request timeout.
///
/// Unlike `azure_core::http::Request`, this type carries a [`timeout`](Self::timeout)
/// field so the transport layer can enforce a deadline that differs from the
/// client-level default.
#[derive(Clone, Debug)]
pub(crate) struct CosmosHttpRequest {
    pub url: Url,
    pub method: Method,
    pub headers: Headers,
    pub body: Option<bytes::Bytes>,

    /// Per-request timeout. When set, overrides the client-level timeout.
    pub timeout: Option<Duration>,
}

// ----------------------------------------------------------------------------
// Response
// ----------------------------------------------------------------------------

/// A fully-buffered HTTP response.
///
/// Cosmos DB enforces a maximum response payload of 4 MB (extendable to 16 MB
/// via configuration), so buffering the entire body is safe and simplifies
/// downstream processing.
#[derive(Clone, Debug)]
pub(crate) struct CosmosHttpResponse {
    pub status: u16,
    pub headers: Headers,
    pub body: Vec<u8>,
}

// ----------------------------------------------------------------------------
// Error
// ----------------------------------------------------------------------------

/// Transport-level error with metadata for retry classification.
///
/// Wraps the underlying `azure_core::Error` and adds flags that the retry
/// layer uses to decide whether and how to retry:
///
/// * [`request_sent`](Self::request_sent) — tri-state indicator of whether the
///   request reached the wire.
/// * [`is_connect_error`](Self::is_connect_error) — `true` when the failure
///   occurred during TCP/TLS connection establishment.
/// * [`is_timeout_error`](Self::is_timeout_error) — `true` when the operation
///   exceeded its deadline.
pub(crate) struct TransportError {
    /// The underlying error, preserved as `azure_core::Error` for public API
    /// compatibility.
    pub error: azure_core::Error,

    /// Whether the request was definitely sent, not sent, or unknown.
    pub request_sent: RequestSentStatus,

    /// Whether this was a connection establishment failure.
    pub is_connect_error: bool,

    /// Whether this was a timeout.
    pub is_timeout_error: bool,
}

impl TransportError {
    /// Creates a new [`TransportError`] with the given classification flags.
    pub fn new(
        error: azure_core::Error,
        request_sent: RequestSentStatus,
        is_connect_error: bool,
        is_timeout_error: bool,
    ) -> Self {
        Self {
            error,
            request_sent,
            is_connect_error,
            is_timeout_error,
        }
    }
}

impl fmt::Debug for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportError")
            .field("error", &self.error.to_string())
            .field("request_sent", &self.request_sent)
            .field("is_connect_error", &self.is_connect_error)
            .field("is_timeout_error", &self.is_timeout_error)
            .finish()
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for TransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error.source()
    }
}

// ----------------------------------------------------------------------------
// Trait
// ----------------------------------------------------------------------------

/// Async transport trait for dispatching HTTP requests to Cosmos DB.
///
/// Implementations handle the actual network I/O, including enforcing the
/// per-request [`timeout`](CosmosHttpRequest::timeout) when present.
#[async_trait::async_trait]
pub(crate) trait CosmosTransportClient: Send + Sync + fmt::Debug {
    /// Sends an HTTP request and returns a buffered response.
    ///
    /// # Errors
    ///
    /// Returns a [`TransportError`] on network failures, timeouts, or other
    /// I/O errors. The error carries classification flags that the retry layer
    /// uses to decide whether to retry.
    async fn send(&self, request: &CosmosHttpRequest)
        -> Result<CosmosHttpResponse, TransportError>;
}

// ----------------------------------------------------------------------------
// Bridge helpers (temporary — will be removed once the transport pipeline
// builds CosmosHttpRequest directly instead of azure_core::http::Request)
// ----------------------------------------------------------------------------

/// Converts an [`azure_core::http::Request`] into a [`CosmosHttpRequest`],
/// dispatches it through the given [`CosmosTransportClient`], and converts the
/// [`CosmosHttpResponse`] back into an [`azure_core::http::AsyncRawResponse`].
///
/// This exists solely to bridge the gap while consumers still build
/// `azure_core::http::Request` objects. Once every call-site constructs
/// `CosmosHttpRequest` directly this function should be deleted.
pub(crate) async fn bridge_send(
    client: &dyn CosmosTransportClient,
    request: &azure_core::http::Request,
) -> azure_core::Result<azure_core::http::AsyncRawResponse> {
    let body = match request.body() {
        azure_core::http::request::Body::Bytes(b) => {
            if b.is_empty() {
                None
            } else {
                Some(bytes::Bytes::copy_from_slice(b))
            }
        }
        // SeekableStream bodies are not used in Cosmos DB requests.
        // Fail loudly rather than silently dropping the body.
        azure_core::http::request::Body::SeekableStream(_) => {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "bridge_send does not support SeekableStream bodies",
            ));
        }
    };

    let cosmos_request = CosmosHttpRequest {
        url: request.url().clone(),
        method: request.method(),
        headers: request.headers().clone(),
        body,
        timeout: None,
    };

    match client.send(&cosmos_request).await {
        Ok(response) => {
            let status = azure_core::http::StatusCode::from(response.status);
            Ok(azure_core::http::AsyncRawResponse::from_bytes(
                status,
                response.headers,
                response.body,
            ))
        }
        Err(transport_err) => Err(transport_err.error),
    }
}
