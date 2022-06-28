use crate::headers::Headers;
use crate::StatusCode;
use bytes::Bytes;
use futures::Stream;
use futures::StreamExt;
use std::pin::Pin;

pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>> + Send + Sync>>;

/// An HTTP Response.
pub struct Response {
    status: StatusCode,
    headers: Headers,
    body: PinnedStream,
}

impl Response {
    #[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn new(status: StatusCode, headers: Headers, body: PinnedStream) -> Self {
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
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Deconstruct the HTTP response into its components.
    pub fn deconstruct(self) -> (StatusCode, Headers, PinnedStream) {
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

/// A response with the body collected as bytes
#[derive(Debug, Clone)]
pub struct CollectedResponse {
    status: StatusCode,
    headers: Headers,
    body: Bytes,
}

impl CollectedResponse {
    /// Create a new instance
    pub fn new(status: StatusCode, headers: Headers, body: Bytes) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    /// Get the status
    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    /// Get the headers
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Get the body
    pub fn body(&self) -> &Bytes {
        &self.body
    }

    /// Into the body
    pub fn into_body(self) -> Bytes {
        self.body
    }

    /// From a response
    pub async fn from_response(response: Response) -> crate::Result<Self> {
        let (status, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;
        Ok(Self::new(status, headers, body))
    }
}