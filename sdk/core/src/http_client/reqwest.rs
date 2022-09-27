use crate::error::{ErrorKind, ResultExt};
use crate::{Body, HttpClient, PinnedStream};

use async_trait::async_trait;
#[cfg(not(target_arch = "wasm32"))]
use futures::TryStreamExt;
use std::{collections::HashMap, str::FromStr};

/// Construct a new `HttpClient` with the `reqwest` backend.
pub fn new_reqwest_client() -> std::sync::Arc<dyn HttpClient> {
    log::debug!("instantiating an http client using the reqwest backend");
    std::sync::Arc::new(::reqwest::Client::new())
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HttpClient for ::reqwest::Client {
    async fn execute_request(&self, request: &crate::Request) -> crate::Result<crate::Response> {
        let url = request.url().clone();
        let method = request.method();
        let mut req = self.request(try_from_method(method)?, url.clone());
        for (name, value) in request.headers().iter() {
            req = req.header(name.as_str(), value.as_str());
        }
        let body = request.body().clone();

        let reqwest_request = match body {
            Body::Bytes(bytes) => req.body(bytes).build(),
            // Hack... Temporarily remove for wasm32
            // Need to implement properly when reqwest has support for streaming body.
            // https://github.com/seanmonstar/reqwest/pull/1576
            #[cfg(not(target_arch = "wasm32"))]
            Body::SeekableStream(mut seekable_stream) => {
                seekable_stream.reset().await.context(
                    ErrorKind::Other,
                    "failed to reset body stream when building request",
                )?;
                req.body(::reqwest::Body::wrap_stream(seekable_stream))
                    .build()
            }
            #[cfg(target_arch = "wasm32")]
            _ => {
                return Err(crate::error::Error::message(
                    ErrorKind::Io,
                    "error converting `reqwest` request into a byte stream",
                ))
            }
        }
        .context(ErrorKind::Other, "failed to build `reqwest` request")?;

        log::debug!("performing request {method} '{url}' with `reqwest`");
        let rsp = self
            .execute(reqwest_request)
            .await
            .context(ErrorKind::Io, "failed to execute `reqwest` request")?;

        let status = rsp.status();
        let headers = to_headers(rsp.headers());

        // Hack... Temporarily workaround for wasm32
        // Need to implement properly when reqwest has support for streaming body.
        // https://github.com/seanmonstar/reqwest/pull/1576
        #[cfg(target_arch = "wasm32")]
        let body: PinnedStream = Box::pin(futures::stream::once(futures::future::ok(
            rsp.bytes().await.map_err(|error| {
                crate::error::Error::full(
                    ErrorKind::Io,
                    error,
                    "error converting `reqwest` request into a byte stream",
                )
            })?,
        )));

        #[cfg(not(target_arch = "wasm32"))]
        let body: PinnedStream = Box::pin(rsp.bytes_stream().map_err(|error| {
            crate::error::Error::full(
                ErrorKind::Io,
                error,
                "error converting `reqwest` request into a byte stream",
            )
        }));

        Ok(crate::Response::new(
            try_from_status(status)?,
            headers,
            body,
        ))
    }
}

fn to_headers(map: &::reqwest::header::HeaderMap) -> crate::headers::Headers {
    let map = map
        .iter()
        .filter_map(|(k, v)| {
            let key = k.as_str();
            match std::str::from_utf8(v.as_bytes()) {
                Ok(value) => Some((
                    crate::headers::HeaderName::from(key.to_owned()),
                    crate::headers::HeaderValue::from(value.to_owned()),
                )),
                Err(_) => {
                    log::warn!("header value for `{key}` is not utf8");
                    None
                }
            }
        })
        .collect::<HashMap<_, _>>();
    crate::headers::Headers::from(map)
}

fn try_from_method(method: &crate::Method) -> crate::Result<::reqwest::Method> {
    match method {
        crate::Method::Connect => Ok(::reqwest::Method::CONNECT),
        crate::Method::Delete => Ok(::reqwest::Method::DELETE),
        crate::Method::Get => Ok(::reqwest::Method::GET),
        crate::Method::Head => Ok(::reqwest::Method::HEAD),
        crate::Method::Options => Ok(::reqwest::Method::OPTIONS),
        crate::Method::Patch => Ok(::reqwest::Method::PATCH),
        crate::Method::Post => Ok(::reqwest::Method::POST),
        crate::Method::Put => Ok(::reqwest::Method::PUT),
        crate::Method::Trace => Ok(::reqwest::Method::TRACE),
        _ => ::reqwest::Method::from_str(method.as_ref()).map_kind(ErrorKind::DataConversion),
    }
}

fn try_from_status(status: ::reqwest::StatusCode) -> crate::Result<crate::StatusCode> {
    let status = u16::from(status);
    crate::StatusCode::try_from(status).map_err(|_| {
        crate::error::Error::with_message(ErrorKind::DataConversion, || {
            format!("invalid status code {status}")
        })
    })
}
