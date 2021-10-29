use crate::bytes_response::BytesResponse;
use crate::BytesStream;
use crate::StreamError;
use bytes::Bytes;
use futures::Stream;
use futures::StreamExt;
use http::{header::HeaderName, HeaderMap, HeaderValue, StatusCode};
use std::pin::Pin;

type PinnedStream = Pin<Box<dyn Stream<Item = Result<Bytes, StreamError>> + Send + Sync>>;

#[allow(dead_code)]
pub(crate) struct ResponseBuilder {
    status: StatusCode,
    headers: HeaderMap,
}

impl ResponseBuilder {
    #[allow(dead_code)]
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_header(&mut self, key: &HeaderName, value: HeaderValue) -> &mut Self {
        self.headers.append(key, value);
        self
    }

    #[allow(dead_code)]
    pub fn with_pinned_stream(self, response: PinnedStream) -> Response {
        Response::new(self.status, self.headers, response)
    }
}

// An HTTP Response
pub struct Response {
    status: StatusCode,
    headers: HeaderMap,
    body: PinnedStream,
}

impl Response {
    pub(crate) fn new(status: StatusCode, headers: HeaderMap, body: PinnedStream) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn deconstruct(self) -> (StatusCode, HeaderMap, PinnedStream) {
        (self.status, self.headers, self.body)
    }

    pub async fn validate(self) -> Result<Self, crate::HttpError> {
        let status = self.status();
        if (200..400).contains(&status.as_u16()) {
            let body = self.into_body_string().await;
            Err(crate::HttpError::ErrorStatusCode { status, body })
        } else {
            Ok(self)
        }
    }

    pub async fn into_body_string(self) -> String {
        pinned_stream_into_utf8_string(self.body).await
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
pub async fn collect_pinned_stream(mut pinned_stream: PinnedStream) -> Result<Bytes, StreamError> {
    let mut final_result = Vec::new();

    while let Some(res) = pinned_stream.next().await {
        let res = res?;
        final_result.extend(&res);
    }

    Ok(final_result.into())
}

/// Collects a `PinnedStream` into a utf8 String
///
/// If the stream cannot be collected or is not utf8, a placeholder string
/// will be returned.
pub async fn pinned_stream_into_utf8_string(stream: PinnedStream) -> String {
    let body = collect_pinned_stream(stream)
        .await
        .unwrap_or_else(|_| Bytes::from_static("<INVALID BODY>".as_bytes()));
    let body = std::str::from_utf8(&body)
        .unwrap_or("<NON-UTF8 BODY>")
        .to_owned();
    body
}

impl From<BytesResponse> for Response {
    fn from(bytes_response: BytesResponse) -> Self {
        let (status, headers, body) = bytes_response.deconstruct();

        let bytes_stream: BytesStream = body.into();

        Self {
            status,
            headers,
            body: Box::pin(bytes_stream),
        }
    }
}
