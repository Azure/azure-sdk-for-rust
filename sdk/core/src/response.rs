use crate::error::{ErrorKind, ResultExt};
use crate::headers::Headers;
use crate::StatusCode;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use std::fmt::Debug;
use std::pin::Pin;

pub(crate) type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>> + Send + Sync>>;

/// An HTTP Response.
pub struct Response {
    status: StatusCode,
    headers: Headers,
    body: ResponseBody,
}

impl Response {
    pub fn new(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self {
        Self {
            status,
            headers,
            body: ResponseBody::new(stream),
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
    pub fn deconstruct(self) -> (StatusCode, Headers, ResponseBody) {
        (self.status, self.headers, self.body)
    }

    /// Consume the HTTP response and return the HTTP body bytes.
    pub fn into_body(self) -> ResponseBody {
        self.body
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

    /// From a response
    pub async fn from_response(response: Response) -> crate::Result<Self> {
        let (status, headers, body) = response.deconstruct();
        let body = body.collect().await?;
        Ok(Self::new(status, headers, body))
    }
}

/// A response body stream
///
/// This body can either be streamed or collected into `Bytes`
#[pin_project::pin_project]
pub struct ResponseBody(#[pin] PinnedStream);

impl ResponseBody {
    fn new(stream: PinnedStream) -> Self {
        Self(stream)
    }

    /// Collect the stream into a `Bytes` collection
    pub async fn collect(mut self) -> crate::Result<Bytes> {
        let mut final_result = Vec::new();

        while let Some(res) = self.0.next().await {
            final_result.extend(&res?);
        }

        Ok(final_result.into())
    }

    /// Collect the stream into a `String`
    pub async fn collect_string(self) -> crate::Result<String> {
        std::str::from_utf8(&self.collect().await?)
            .context(
                ErrorKind::DataConversion,
                "response body was not utf-8 like expected",
            )
            .map(ToOwned::to_owned)
    }
}

impl Stream for ResponseBody {
    type Item = crate::Result<Bytes>;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();
        this.0.poll_next(cx)
    }
}

impl Debug for ResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ResonseBody")
    }
}
