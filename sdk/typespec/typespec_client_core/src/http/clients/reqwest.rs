// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{
    headers::{HeaderName, HeaderValue, Headers},
    Body, HttpClient, Method, PinnedStream, Request, Response, StatusCode,
};
use async_trait::async_trait;
use futures::TryStreamExt;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tracing::{debug, warn};
use typespec::error::{Error, ErrorKind, Result, ResultExt};

/// Create a new [`HttpClient`] with the `reqwest` backend.
pub fn new_reqwest_client() -> Arc<dyn HttpClient> {
    debug!("instantiating an http client using the reqwest backend");

    // Set `pool_max_idle_per_host` to `0` to avoid an issue in the underlying
    // `hyper` library that causes the `reqwest` client to hang in some cases.
    //
    // See <https://github.com/hyperium/hyper/issues/2312> for more details.
    #[cfg(not(target_arch = "wasm32"))]
    let client = ::reqwest::ClientBuilder::new()
        .pool_max_idle_per_host(0)
        .build()
        .expect("failed to build `reqwest` client");

    // `reqwest` does not implement `pool_max_idle_per_host()` on WASM.
    #[cfg(target_arch = "wasm32")]
    let client = ::reqwest::ClientBuilder::new()
        .build()
        .expect("failed to build `reqwest` client");

    Arc::new(client)
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HttpClient for ::reqwest::Client {
    async fn execute_request(&self, request: &Request) -> Result<Response> {
        let url = request.url().clone();
        let method = request.method();
        let mut req = self.request(try_from_method(*method)?, url.clone());
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
        .context(ErrorKind::Other, "failed to build `reqwest` request")?;

        debug!("performing request {method} '{url}' with `reqwest`");
        let rsp = self
            .execute(reqwest_request)
            .await
            .context(ErrorKind::Io, "failed to execute `reqwest` request")?;

        let status = rsp.status();
        let headers = to_headers(rsp.headers());

        let body: PinnedStream = Box::pin(rsp.bytes_stream().map_err(|error| {
            Error::full(
                ErrorKind::Io,
                error,
                "error converting `reqwest` request into a byte stream",
            )
        }));

        Ok(Response::new(try_from_status(status)?, headers, body))
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

fn try_from_method(method: Method) -> Result<::reqwest::Method> {
    match method {
        Method::Connect => Ok(::reqwest::Method::CONNECT),
        Method::Delete => Ok(::reqwest::Method::DELETE),
        Method::Get => Ok(::reqwest::Method::GET),
        Method::Head => Ok(::reqwest::Method::HEAD),
        Method::Options => Ok(::reqwest::Method::OPTIONS),
        Method::Patch => Ok(::reqwest::Method::PATCH),
        Method::Post => Ok(::reqwest::Method::POST),
        Method::Put => Ok(::reqwest::Method::PUT),
        Method::Trace => Ok(::reqwest::Method::TRACE),
        _ => ::reqwest::Method::from_str(method.as_ref()).map_kind(ErrorKind::DataConversion),
    }
}

fn try_from_status(status: ::reqwest::StatusCode) -> Result<StatusCode> {
    let status = u16::from(status);
    StatusCode::try_from(status).map_err(|_| {
        Error::with_message(ErrorKind::DataConversion, || {
            format!("invalid status code {status}")
        })
    })
}
