use bytes::Bytes;
use futures::Stream;
use futures::StreamExt;
use http::{HeaderMap, StatusCode};
use std::pin::Pin;

type PinnedStream = Pin<Box<dyn Stream<Item = crate::error::Result<Bytes>> + Send + Sync>>;

#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct ResponseBuilder {
    status: StatusCode,
    headers: HeaderMap,
}

#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
#[cfg(not(target_arch = "wasm32"))]
impl ResponseBuilder {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn with_header(
        &mut self,
        key: &http::header::HeaderName,
        value: http::HeaderValue,
    ) -> &mut Self {
        self.headers.append(key, value);
        self
    }

    pub fn with_pinned_stream(self, response: PinnedStream) -> Response {
        Response::new(self.status, self.headers, response)
    }
}

/// An HTTP Response.
pub struct Response {
    status: StatusCode,
    headers: HeaderMap,
    body: PinnedStream,
}

impl Response {
    #[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn new(status: StatusCode, headers: HeaderMap, body: PinnedStream) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    /// Get the status code from the response.
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the headers from the response.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Deconstruct the HTTP response into its components.
    pub fn deconstruct(self) -> (StatusCode, HeaderMap, PinnedStream) {
        (self.status, self.headers, self.body)
    }

    /// Consume the HTTP response and return the HTTP body bytes.
    pub async fn into_body(self) -> Bytes {
        collect_pinned_stream(self.body)
            .await
            .unwrap_or_else(|_| Bytes::from_static(b"<INVALID BODY>"))
    }

    /// Consume the HTTP response and read the HTTP body into a string.
    pub async fn into_body_string(self) -> String {
        std::str::from_utf8(&self.into_body().await)
            .unwrap_or("<NON-UTF8 BODY>")
            .to_owned()
    }
}

impl std::fmt::Debug for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &"<BODY>")
            .finish()
    }
}

/// Convenience function that transforms a `PinnedStream` in a `bytes::Bytes` struct by collecting all the chunks. It consumes the response stream.
pub async fn collect_pinned_stream(mut pinned_stream: PinnedStream) -> crate::error::Result<Bytes> {
    let mut final_result = Vec::new();

    while let Some(res) = pinned_stream.next().await {
        let res = res?;
        final_result.extend(&res);
    }

    Ok(final_result.into())
}
