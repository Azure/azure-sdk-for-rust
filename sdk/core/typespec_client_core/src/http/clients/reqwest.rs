// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{
    headers::{HeaderName, HeaderValue, Headers},
    request::{Body, Request},
    response::PinnedStream,
    AsyncRawResponse, HttpClient, Method, Sanitizer, DEFAULT_ALLOWED_QUERY_PARAMETERS,
};
use async_trait::async_trait;
use futures::TryStreamExt;
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, warn};
use typespec::error::{Error, ErrorKind, Result, ResultExt};

/// Create a new [`HttpClient`] with the `reqwest` backend.
pub fn new_reqwest_client() -> Arc<dyn HttpClient> {
    debug!("creating an http client using `reqwest`");

    // Some customers in the past have reported challenges associated with enabling
    // connection pooling in reqwest. See <https://github.com/hyperium/hyper/issues/2312>
    // for more details.
    //
    // Due to the significant performance impact when disabling connection pooling,
    // it is enabled here by default. See the `azure_core` troubleshooting guide to disable pooling.
    let client = ::reqwest::ClientBuilder::new()
        .build()
        .expect("failed to build `reqwest` client");

    Arc::new(client)
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HttpClient for ::reqwest::Client {
    async fn execute_request(&self, request: &Request) -> Result<AsyncRawResponse> {
        let url = request.url().clone();
        let method = request.method();
        let mut req = self.request(from_method(method), url.clone());
        for (name, value) in request.headers().iter() {
            req = req.header(name.as_str(), value.as_str());
        }
        let body = request.body().clone();

        let reqwest_request = match body {
            Body::Bytes(bytes) => req.body(bytes).build(),

            // We cannot currently implement `Body::SeekableStream` for WASM
            // because `reqwest::Body::wrap_stream()` is not implemented for WASM.
            #[cfg(not(target_arch = "wasm32"))]
            Body::SeekableStream(seekable_stream) => req
                .body(::reqwest::Body::wrap_stream(seekable_stream))
                .build(),
        }
        .with_context(ErrorKind::Other, "failed to build `reqwest` request")?;

        debug!(
            "performing request {method} '{}' with `reqwest`",
            url.sanitize(&DEFAULT_ALLOWED_QUERY_PARAMETERS)
        );
        let rsp = self
            .execute(reqwest_request)
            .await
            .with_context(ErrorKind::Io, "failed to execute `reqwest` request")?;

        let status = rsp.status();
        let headers = to_headers(rsp.headers());

        let body: PinnedStream = Box::pin(rsp.bytes_stream().map_err(|error| {
            Error::with_error(
                ErrorKind::Io,
                error,
                "error converting `reqwest` request into a byte stream",
            )
        }));

        Ok(AsyncRawResponse::new(status.as_u16().into(), headers, body))
    }
}

fn to_headers(map: &::reqwest::header::HeaderMap) -> Headers {
    let map = map
        .iter()
        .filter_map(|(k, v)| {
            let key = k.as_str();
            if let Ok(value) = v.to_str() {
                Some((
                    HeaderName::from(key.to_owned()),
                    HeaderValue::from(value.to_owned()),
                ))
            } else {
                warn!("header value for `{key}` is not utf8");
                None
            }
        })
        .collect::<HashMap<_, _>>();
    Headers::from(map)
}

fn from_method(method: Method) -> ::reqwest::Method {
    match method {
        Method::Delete => ::reqwest::Method::DELETE,
        Method::Get => ::reqwest::Method::GET,
        Method::Head => ::reqwest::Method::HEAD,
        Method::Patch => ::reqwest::Method::PATCH,
        Method::Post => ::reqwest::Method::POST,
        Method::Put => ::reqwest::Method::PUT,
    }
}
