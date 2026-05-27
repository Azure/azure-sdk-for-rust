// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Default [`TransportClient`] implementation backed by `reqwest`.
//!
//! [`ReqwestTransportClient`] wraps a `reqwest::Client` and translates
//! between the driver's [`HttpRequest`]/[`HttpResponse`] types
//! and reqwest's native request/response types. Per-request timeouts are
//! applied via `reqwest::RequestBuilder::timeout()`.

use azure_core::http::headers::{HeaderName, HeaderValue, Headers};

use crate::diagnostics::RequestSentStatus;
use crate::models::CosmosStatus;

use super::cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError};

/// A [`TransportClient`] backed by `reqwest::Client`.
///
/// Each instance wraps a single `reqwest::Client` configured with
/// connection-pool settings, TLS, and HTTP version policy. Per-request
/// timeouts are applied via `reqwest::RequestBuilder::timeout()`,
/// overriding the client-level timeout for each call.
#[derive(Debug)]
pub(crate) struct ReqwestTransportClient {
    client: reqwest::Client,
}

impl ReqwestTransportClient {
    /// Wraps an existing `reqwest::Client`.
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl TransportClient for ReqwestTransportClient {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let method = to_reqwest_method(request.method);
        let mut builder = self.client.request(method, request.url.clone());

        for (name, value) in request.headers.iter() {
            builder = builder.header(name.as_str(), value.as_str());
        }

        if let Some(body) = &request.body {
            builder = builder.body(body.clone());
        }

        if let Some(timeout) = request.timeout {
            builder = builder.timeout(timeout);
        }

        let response = builder.send().await.map_err(|err| {
            let is_connect = err.is_connect();
            let request_sent = if is_connect {
                RequestSentStatus::NotSent
            } else if err.is_body() {
                RequestSentStatus::Sent
            } else {
                RequestSentStatus::Unknown
            };
            // Base status from the reqwest classification (connect vs. body/io),
            // refined via the source chain to upgrade to more specific Cosmos
            // statuses when the inner cause is recognizable (h2 protocol
            // incompatibility, DNS lookup failure, …).
            let base_status = if is_connect {
                CosmosStatus::TRANSPORT_CONNECTION_FAILED
            } else {
                CosmosStatus::TRANSPORT_IO_FAILED
            };
            let status = refine_status_from_source_chain(std::error::Error::source(&err))
                .unwrap_or(base_status);
            let message = err.to_string();
            let cosmos_err = crate::error::Error::transport(
                status,
                message,
                None,
                Some(std::sync::Arc::new(err)),
            );
            TransportError::new(cosmos_err, request_sent)
        })?;

        let status = response.status().as_u16();
        let headers = to_driver_headers(response.headers());

        let body = response.bytes().await.map_err(|err| {
            let message = err.to_string();
            let cosmos_err = crate::error::Error::transport(
                CosmosStatus::TRANSPORT_BODY_READ_FAILED,
                message,
                None,
                Some(std::sync::Arc::new(err)),
            );
            TransportError::new(cosmos_err, RequestSentStatus::Sent)
        })?;

        Ok(HttpResponse {
            status,
            headers,
            body: body.to_vec(),
        })
    }
}

fn to_reqwest_method(method: azure_core::http::Method) -> reqwest::Method {
    reqwest::Method::from_bytes(method.as_str().as_bytes())
        .expect("azure_core::http::Method should always be a valid HTTP method")
}

/// Maximum number of `.source()` frames walked by
/// [`refine_status_from_source_chain`]. Real Cosmos transport chains are
/// never deeper than ~5; the cap exists so a pathological or cyclic chain
/// cannot pin a thread on the transport hot path.
const MAX_SOURCE_CHAIN_DEPTH: usize = 64;

/// Walks the `.source()` chain of a `reqwest::Error` looking for
/// downcasts that map to a more specific [`CosmosStatus`] than reqwest's
/// own classification (`is_connect()` / `is_body()`) exposes \u2014 h2
/// protocol incompatibility and io DNS failures. Returns `None` if
/// nothing more specific is found, in which case the caller's base
/// classification stands. Bounded by [`MAX_SOURCE_CHAIN_DEPTH`].
fn refine_status_from_source_chain(
    start: Option<&(dyn std::error::Error + 'static)>,
) -> Option<CosmosStatus> {
    let mut cur = start;
    for _ in 0..MAX_SOURCE_CHAIN_DEPTH {
        let Some(e) = cur else { return None };
        if let Some(h2_err) = e.downcast_ref::<h2::Error>() {
            if matches!(
                h2_err.reason(),
                Some(
                    h2::Reason::HTTP_1_1_REQUIRED
                        | h2::Reason::PROTOCOL_ERROR
                        | h2::Reason::FRAME_SIZE_ERROR
                )
            ) {
                return Some(CosmosStatus::TRANSPORT_HTTP2_INCOMPATIBLE);
            }
        }
        if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
            // Best-effort DNS detection. `reqwest`/`hyper` typically surface
            // resolver failures as `io::ErrorKind::NotFound` /
            // `AddrNotAvailable`. TLS / generic socket I/O falls through to
            // the caller's base classification.
            if matches!(
                io_err.kind(),
                std::io::ErrorKind::NotFound | std::io::ErrorKind::AddrNotAvailable
            ) {
                return Some(CosmosStatus::TRANSPORT_DNS_FAILED);
            }
        }
        cur = e.source();
    }
    None
}

fn to_driver_headers(reqwest_headers: &reqwest::header::HeaderMap) -> Headers {
    let mut headers = Headers::new();
    for (name, value) in reqwest_headers.iter() {
        if let Ok(val) = value.to_str() {
            headers.insert(
                HeaderName::from(name.as_str().to_owned()),
                HeaderValue::from(val.to_owned()),
            );
        }
    }
    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_conversion_roundtrip() {
        assert_eq!(
            to_reqwest_method(azure_core::http::Method::Get),
            reqwest::Method::GET
        );
        assert_eq!(
            to_reqwest_method(azure_core::http::Method::Post),
            reqwest::Method::POST
        );
        assert_eq!(
            to_reqwest_method(azure_core::http::Method::Put),
            reqwest::Method::PUT
        );
        assert_eq!(
            to_reqwest_method(azure_core::http::Method::Delete),
            reqwest::Method::DELETE
        );
        assert_eq!(
            to_reqwest_method(azure_core::http::Method::Patch),
            reqwest::Method::PATCH
        );
        assert_eq!(
            to_reqwest_method(azure_core::http::Method::Head),
            reqwest::Method::HEAD
        );
    }
}
