use crate::errors::StreamError;
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
    response: PinnedStream,
}

impl Response {
    fn new(status: StatusCode, headers: HeaderMap, response: PinnedStream) -> Self {
        Self {
            status,
            headers,
            response,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn deconstruct(self) -> (StatusCode, HeaderMap, PinnedStream) {
        (self.status, self.headers, self.response)
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
