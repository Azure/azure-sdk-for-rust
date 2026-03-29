// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Default [`CosmosTransportClient`] implementation backed by `reqwest`.
//!
//! [`ReqwestTransportClient`] wraps a `reqwest::Client` and translates
//! between the driver's [`CosmosHttpRequest`]/[`CosmosHttpResponse`] types
//! and reqwest's native request/response types. Per-request timeouts are
//! applied via `reqwest::RequestBuilder::timeout()`.

use std::sync::Arc;

use azure_core::http::headers::{HeaderName, HeaderValue, Headers};

use crate::diagnostics::RequestSentStatus;

use super::cosmos_transport_client::{
    CosmosHttpRequest, CosmosHttpResponse, CosmosTransportClient, TransportError,
};

/// A [`CosmosTransportClient`] backed by `reqwest::Client`.
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

    /// Wraps an existing `reqwest::Client` and returns it as an `Arc<dyn CosmosTransportClient>`.
    pub fn into_arc(client: reqwest::Client) -> Arc<dyn CosmosTransportClient> {
        Arc::new(Self::new(client))
    }
}

#[async_trait::async_trait]
impl CosmosTransportClient for ReqwestTransportClient {
    async fn send(
        &self,
        request: &CosmosHttpRequest,
    ) -> Result<CosmosHttpResponse, TransportError> {
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
            let is_timeout = err.is_timeout();
            let request_sent = if is_connect || err.is_request() {
                RequestSentStatus::NotSent
            } else if err.is_body() {
                RequestSentStatus::Sent
            } else {
                RequestSentStatus::Unknown
            };
            let kind = if is_connect {
                azure_core::error::ErrorKind::Connection
            } else {
                azure_core::error::ErrorKind::Io
            };
            TransportError::new(
                azure_core::Error::new(kind, err),
                request_sent,
                is_connect,
                is_timeout,
            )
        })?;

        let status = response.status().as_u16();
        let headers = to_driver_headers(response.headers());

        let body = response.bytes().await.map_err(|err| {
            TransportError::new(
                azure_core::Error::new(azure_core::error::ErrorKind::Io, err),
                RequestSentStatus::Sent,
                false,
                false,
            )
        })?;

        Ok(CosmosHttpResponse {
            status,
            headers,
            body: body.to_vec(),
        })
    }
}

fn to_reqwest_method(method: azure_core::http::Method) -> reqwest::Method {
    match method {
        azure_core::http::Method::Get => reqwest::Method::GET,
        azure_core::http::Method::Post => reqwest::Method::POST,
        azure_core::http::Method::Put => reqwest::Method::PUT,
        azure_core::http::Method::Delete => reqwest::Method::DELETE,
        azure_core::http::Method::Patch => reqwest::Method::PATCH,
        azure_core::http::Method::Head => reqwest::Method::HEAD,
        _ => reqwest::Method::GET, // Unreachable for Cosmos operations
    }
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
