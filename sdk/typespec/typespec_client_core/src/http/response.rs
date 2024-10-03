// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{headers::Headers, PinnedStream, ResponseBody, StatusCode};
use bytes::Bytes;
use std::fmt;

/// An HTTP response.
///
/// The type parameter `T` represents the type of the body.
/// Usually, this will be a concrete model type, specific to the API called.
/// In some cases, the body may be of type [`ResponseBody`], which represents a raw, unparsed, HTTP body.
pub struct Response<T = ResponseBody> {
    status: StatusCode,
    headers: Headers,
    body: T,
}

impl<T> Response<T> {
    /// Create an HTTP response wrapping the provided body.
    pub fn new(status: StatusCode, headers: Headers, body: T) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }
}

impl Response<ResponseBody> {
    /// Create an HTTP response from an asynchronous stream of bytes.
    pub fn from_stream(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self {
        Self::new(status, headers, ResponseBody::new(stream))
    }

    /// Create an HTTP response from raw bytes.
    pub fn from_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        Self::new(status, headers, ResponseBody::from_bytes(bytes))
    }
}

impl<T> Response<T> {
    /// Get the status code from the response.
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the headers from the response.
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Consumes the response and returns the body.
    pub fn into_body(self) -> T {
        self.body
    }

    /// Deconstruct the HTTP response into its components.
    pub fn deconstruct(self) -> (StatusCode, Headers, T) {
        (self.status, self.headers, self.body)
    }
}

impl<T> fmt::Debug for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &"(body)")
            .finish()
    }
}
