// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Helpers for testing client libraries with fake servers.
//!
//! Enable the client library's `test` and `unstable_fake` features to use its
//! experimental `clients::fake` module.
//! Fake servers return an [`AsyncRawResponse`], allowing tests to control the status,
//! headers, and body. Import [`ResponseExt`] to create responses from bytes,
//! and [`AsyncResponseExt`] to create responses from streams.

use crate::{
    http::{
        headers::{Headers, CONTENT_TYPE},
        response::PinnedStream,
        AsyncRawResponse, AsyncResponse, Context, RawResponse, Response, StatusCode,
    },
    json,
    tracing::PublicApiInstrumentationInformation,
    Bytes, Result,
};

/// Returns the borrowed API name from the context, or `None` if absent.
pub fn get_api_name<'a>(context: &'a Context<'_>) -> Option<&'a str> {
    context
        .value::<PublicApiInstrumentationInformation>()
        .map(PublicApiInstrumentationInformation::api_name)
}

/// Extension methods for creating fake HTTP responses.
///
/// Implemented for [`Response`], [`RawResponse`], [`AsyncResponse`], and [`AsyncRawResponse`].
/// These methods preserve the supplied status without checking whether it indicates success.
pub trait ResponseExt: Sized {
    /// Creates a JSON error response containing a service error code and message.
    ///
    /// Preserves the supplied status and headers. Adds `content-type:
    /// application/json` only if no content type is specified.
    ///
    /// # Errors
    ///
    /// Returns an error if serializing the error body fails.
    fn fake_with_error(
        status: StatusCode,
        mut headers: Headers,
        code: &str,
        message: &str,
    ) -> Result<Self> {
        let body = json::to_json(&serde_json::json!({
            "error": { "code": code, "message": message },
        }))?;
        if headers.get_optional_str(&CONTENT_TYPE).is_none() {
            headers.insert(CONTENT_TYPE, "application/json");
        }
        Ok(Self::fake_with_bytes(status, headers, body))
    }

    /// Creates a fake response from bytes without changing the status or headers.
    ///
    /// For JSON responses, pass a JSON string or the string representation of a
    /// [`serde_json::json!`] value.
    fn fake_with_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self;
}

impl ResponseExt for RawResponse {
    fn fake_with_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        Self::from_bytes(status, headers, bytes)
    }
}

impl<T, F> ResponseExt for Response<T, F> {
    fn fake_with_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        RawResponse::fake_with_bytes(status, headers, bytes).into()
    }
}

impl ResponseExt for AsyncRawResponse {
    fn fake_with_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        Self::from_bytes(status, headers, bytes)
    }
}

impl<T> ResponseExt for AsyncResponse<T> {
    fn fake_with_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self {
        AsyncRawResponse::fake_with_bytes(status, headers, bytes).into()
    }
}

/// Extension methods for creating fake streaming HTTP responses.
pub trait AsyncResponseExt: ResponseExt {
    /// Creates a fake response from a stream without changing the status or headers.
    ///
    /// Stream errors are returned when the response body is read.
    fn fake_with_stream(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self;
}

impl AsyncResponseExt for AsyncRawResponse {
    fn fake_with_stream(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self {
        Self::new(status, headers, stream)
    }
}

impl<T> AsyncResponseExt for AsyncResponse<T> {
    fn fake_with_stream(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self {
        AsyncRawResponse::fake_with_stream(status, headers, stream).into()
    }
}

#[cfg(test)]
mod tests;
