use crate::error::{ErrorKind, ResultExt};
use crate::{Body, HttpClient, PinnedStream};

use async_trait::async_trait;
use futures::TryStreamExt;
use std::{collections::HashMap, str::FromStr};

/// Construct a new `HttpClient` with the `reqwest` backend.
pub fn new_reqwest_client() -> std::sync::Arc<dyn HttpClient> {
    log::debug!("instantiating an http client using the reqwest backend");
    std::sync::Arc::new(::reqwest::Client::new())
}

#[async_trait]
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
            Body::SeekableStream(mut seekable_stream) => {
                seekable_stream.reset().await.context(
                    ErrorKind::Other,
                    "failed to reset body stream when building request",
                )?;
                req.body(::reqwest::Body::wrap_stream(seekable_stream))
                    .build()
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
