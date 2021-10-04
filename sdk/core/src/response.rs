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

    pub async fn validate(self, expected_status: StatusCode) -> Result<Self, crate::HttpError> {
        let status = self.status();
        if expected_status != status {
            let body = collect_pinned_stream(self.body)
                .await
                .unwrap_or_else(|_| Bytes::from_static("<INVALID BODY>".as_bytes()));
            Err(crate::HttpError::new_unexpected_status_code(
                expected_status,
                status,
                std::str::from_utf8(&body as &[u8]).unwrap_or("<NON-UTF8 BODY>"),
            ))
        } else {
            Ok(self)
        }
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
