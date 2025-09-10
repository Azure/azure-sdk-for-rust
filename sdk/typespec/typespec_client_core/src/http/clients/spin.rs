//! Spin HTTP backend for wasm32-wasip2 environments.
#![cfg(all(target_arch = "wasm32", target_os = "wasi", feature = "spin"))]

use crate::http::{
    headers::{HeaderName, HeaderValue, Headers},
    request::{Body, Request},
    response::PinnedStream,
    HttpClient, Method, RawResponse, Sanitizer, DEFAULT_ALLOWED_QUERY_PARAMETERS,
};
use async_trait::async_trait;
use spin_sdk::http::Request as SpinRequest;
use std::{collections::HashMap, sync::Arc};
use tracing::debug;
use typespec::error::{ErrorKind, Result, ResultExt};

/// Create a new [`HttpClient`] with the `spin-sdk` backend.
pub fn new_spin_client() -> Arc<dyn HttpClient> {
    debug!("creating an http client using `spin-sdk`");
    Arc::new(SpinClient)
}

#[derive(Debug)]
struct SpinClient;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HttpClient for SpinClient {
    async fn execute_request(&self, request: &Request) -> Result<RawResponse> {
        let url = request.url().clone();
        let method = request.method();

        let body = request.body().clone();
        let mut built = make_spin_request(method, url.as_str(), &body)?;
        // Apply headers
        match &mut built {
            SpinBuiltRequest::Owned(req) => {
                for (name, value) in request.headers().iter() {
                    req.set_header(name.as_str(), value.as_str());
                }
            }
            SpinBuiltRequest::Builder(builder) => {
                for (name, value) in request.headers().iter() {
                    builder.header(name.as_str(), value.as_str());
                }
            }
        }

        debug!(
            "performing request {method} '{}' with `spin-sdk`",
            url.sanitize(&DEFAULT_ALLOWED_QUERY_PARAMETERS)
        );

        let request = match built {
            SpinBuiltRequest::Owned(req) => req,
            SpinBuiltRequest::Builder(mut builder) => builder.build(),
        };
        let response: spin_sdk::http::Response = spin_sdk::http::send(request)
            .await
            .context(ErrorKind::Io, "failed to execute `spin` request")?;

        let status = (*response.status() as u16).into();
        // Try to capture some common headers. If iteration isn't supported, leave empty.
        let headers = to_headers(&response);

        // Spin returns the full body as bytes slice
        let body_bytes = bytes::Bytes::copy_from_slice(response.body());
        let stream: PinnedStream = Box::pin(futures::stream::once(async move { Ok(body_bytes) }));
        Ok(RawResponse::new(status, headers, stream))
    }
}

fn to_headers(rsp: &spin_sdk::http::Response) -> Headers {
    // If spin exposes .headers() -> iterator, use it; otherwise, return empty.
    let mut map = HashMap::new();
    // Known header: content-type
    if let Some(ct) = rsp.header("content-type").and_then(|v| v.as_str()) {
        map.insert(
            HeaderName::from("content-type".to_string()),
            HeaderValue::from(ct.to_string()),
        );
    }
    Headers::from(map)
}

enum SpinBuiltRequest {
    Owned(SpinRequest),
    Builder(spin_sdk::http::RequestBuilder),
}

fn make_spin_request(method: Method, url: &str, body: &Body) -> Result<SpinBuiltRequest> {
    Ok(match method {
        Method::Get => {
            if !matches!(body, Body::Bytes(b) if b.is_empty()) {
                return Err(typespec::error::Error::message(
                    ErrorKind::Other,
                    "GET with body is not supported by spin backend",
                ));
            }
            SpinBuiltRequest::Builder(spin_sdk::http::Request::get(url))
        }
        Method::Delete => {
            if !matches!(body, Body::Bytes(b) if b.is_empty()) {
                return Err(typespec::error::Error::message(
                    ErrorKind::Other,
                    "DELETE with body is not supported by spin backend",
                ));
            }
            // In 3.1.1, delete returns a builder; use it and no body
            SpinBuiltRequest::Builder(spin_sdk::http::Request::delete(url))
        }
        Method::Head => {
            // Use Request::new to create a HEAD request
            SpinBuiltRequest::Owned(SpinRequest::new(spin_sdk::http::Method::Head, url))
        }
        Method::Post => {
            let bytes = match body.clone() {
                Body::Bytes(b) => b,
                #[cfg(not(target_arch = "wasm32"))]
                Body::SeekableStream(_) => {
                    return Err(typespec::error::Error::message(
                        ErrorKind::Other,
                        "streaming body not supported in spin backend",
                    ))
                }
            };
            SpinBuiltRequest::Builder(spin_sdk::http::Request::post(url, bytes.to_vec()))
        }
        Method::Put => {
            let bytes = match body.clone() {
                Body::Bytes(b) => b,
                #[cfg(not(target_arch = "wasm32"))]
                Body::SeekableStream(_) => {
                    return Err(typespec::error::Error::message(
                        ErrorKind::Other,
                        "streaming body not supported in spin backend",
                    ))
                }
            };
            SpinBuiltRequest::Builder(spin_sdk::http::Request::put(url, bytes.to_vec()))
        }
        Method::Patch => {
            let bytes = match body.clone() {
                Body::Bytes(b) => b,
                #[cfg(not(target_arch = "wasm32"))]
                Body::SeekableStream(_) => {
                    return Err(typespec::error::Error::message(
                        ErrorKind::Other,
                        "streaming body not supported in spin backend",
                    ))
                }
            };
            SpinBuiltRequest::Builder(spin_sdk::http::Request::patch(url, bytes.to_vec()))
        }
    })
}
