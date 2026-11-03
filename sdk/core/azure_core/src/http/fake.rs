// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Helpers for testing client libraries with fake servers.
//!
//! Enable the client library's `test` and `unstable_fake` features to use its
//! experimental `clients::fake` module.
//! Fake servers return an [`AsyncRawResponse`], allowing tests to control the status,
//! headers, and body. Import [`ResponseExt`] to create responses from models or
//! bytes, and [`AsyncResponseExt`] to create responses from streams.

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
use serde::Serialize;

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
        headers: Headers,
        code: &str,
        message: &str,
    ) -> Result<Self> {
        Self::fake_with_model(
            status,
            headers,
            serde_json::json!({
                "error": { "code": code, "message": message },
            }),
        )
    }

    /// Creates a fake response from bytes without changing the status or headers.
    fn fake_with_bytes(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self;

    /// Creates a fake response by serializing a model as JSON.
    ///
    /// Adds `content-type: application/json` only if a content type isn't specified.
    /// Serialization uses the model's [`Serialize`] implementation, regardless of the
    /// response's format marker. Client libraries with fake support enable serialization
    /// of read-only model fields when their `test` and `unstable_fake` features are enabled.
    /// For other models whose serializer omits fields, pass a JSON value or use
    /// [`fake_with_bytes()`](Self::fake_with_bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if `model` cannot be serialized as JSON.
    fn fake_with_model<T: Serialize>(
        status: StatusCode,
        mut headers: Headers,
        model: T,
    ) -> Result<Self> {
        let body = json::to_json(&model)?;
        if headers.get_optional_str(&CONTENT_TYPE).is_none() {
            headers.insert(CONTENT_TYPE, "application/json");
        }
        Ok(Self::fake_with_bytes(status, headers, body))
    }
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
