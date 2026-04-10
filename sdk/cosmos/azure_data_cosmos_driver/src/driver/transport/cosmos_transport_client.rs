// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-owned HTTP transport abstraction for Cosmos DB.
//!
//! This module defines [`TransportClient`], the internal transport trait
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

#[cfg(feature = "fault_injection")]
use crate::fault_injection::EvaluationCollector;

// ----------------------------------------------------------------------------
// Request
// ----------------------------------------------------------------------------

/// An HTTP request with optional per-request timeout.
///
/// Unlike `azure_core::http::Request`, this type carries a [`timeout`](Self::timeout)
/// field so the transport layer can enforce a deadline that differs from the
/// client-level default.
#[derive(Clone, Debug)]
pub(crate) struct HttpRequest {
    pub url: Url,
    pub method: Method,
    pub headers: Headers,
    pub body: Option<bytes::Bytes>,

    /// Per-request timeout. When set, overrides the client-level timeout.
    pub timeout: Option<Duration>,

    /// Collector for fault injection evaluations.
    ///
    /// When present, [`FaultClient`](crate::fault_injection::FaultClient) writes its
    /// rule evaluations into this collector so the transport pipeline can read
    /// them after the request completes — without a global store or
    /// header-based correlation.
    #[cfg(feature = "fault_injection")]
    pub evaluation_collector: Option<EvaluationCollector>,
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
pub(crate) struct HttpResponse {
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
pub(crate) struct TransportError {
    /// The underlying error, preserved as `azure_core::Error` for public API
    /// compatibility.
    pub error: azure_core::Error,

    /// Whether the request was definitely sent, not sent, or unknown.
    pub request_sent: RequestSentStatus,
}

impl TransportError {
    /// Creates a new [`TransportError`].
    pub fn new(error: azure_core::Error, request_sent: RequestSentStatus) -> Self {
        Self {
            error,
            request_sent,
        }
    }
}

impl fmt::Debug for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportError")
            .field("error", &self.error.to_string())
            .field("request_sent", &self.request_sent)
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
/// per-request [`timeout`](HttpRequest::timeout) when present.
#[async_trait::async_trait]
pub(crate) trait TransportClient: Send + Sync + fmt::Debug {
    /// Sends an HTTP request and returns a buffered response.
    ///
    /// # Errors
    ///
    /// Returns a [`TransportError`] on network failures, timeouts, or other
    /// I/O errors. The error carries classification flags that the retry layer
    /// uses to decide whether to retry.
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError>;
}

// (bridge_send removed — all callers now build HttpRequest directly)
