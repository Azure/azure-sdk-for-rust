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
    error::ErrorKind,
    fmt::SafeDebug,
    http::{
        headers::{Headers, CONTENT_TYPE},
        response::PinnedStream,
        AsyncRawResponse, AsyncResponse, Body, Context, RawResponse, Request, RequestContent,
        Response, StatusCode,
    },
    json,
    tracing::PublicApiInstrumentationInformation,
    Bytes, Error, Result,
};
use futures::io::AsyncReadExt as _;
use percent_encoding::percent_decode_str;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::BTreeMap;

/// Decoded arguments from a fake HTTP request.
#[derive(SafeDebug)]
pub struct Arguments {
    path: Vec<String>,
    query: Query,
}

impl Arguments {
    /// Extracts arguments, optionally stripping an encoded endpoint path prefix.
    ///
    /// Prefixes are stripped at segment boundaries. Path segments are decoded
    /// individually, preserving encoded slashes within a segment.
    ///
    /// # Panics
    ///
    /// Panics if the prefix does not match or a path segment is not valid UTF-8.
    pub fn from_request(request: &Request, strip_prefix: Option<&str>) -> Self {
        let segments = |path: &str| -> Vec<String> {
            let path = path.trim_start_matches('/');
            if path.is_empty() {
                Vec::new()
            } else {
                path.split('/').map(str::to_owned).collect()
            }
        };
        let path = segments(request.url().path());
        let mut prefix = segments(strip_prefix.unwrap_or_default());
        while prefix.last().is_some_and(String::is_empty) {
            prefix.pop();
        }
        let path = path
            .strip_prefix(prefix.as_slice())
            .expect("fake request path must start with the endpoint prefix")
            .iter()
            .map(|segment| {
                percent_decode_str(segment)
                    .decode_utf8()
                    .expect("fake request path segment must be valid UTF-8")
                    .into_owned()
            })
            .collect();
        let mut query = BTreeMap::<String, Vec<String>>::new();
        for (name, value) in request.url().query_pairs() {
            query
                .entry(name.into_owned())
                .or_default()
                .push(value.into_owned());
        }
        Self {
            path,
            query: Query(query),
        }
    }

    /// Returns decoded relative path segments, including empty trailing segments.
    pub fn path_segments(&self) -> impl Iterator<Item = &str> {
        self.path.iter().map(String::as_str)
    }

    /// Returns the decoded query parameters.
    pub fn query(&self) -> &Query {
        &self.query
    }
}

/// Case-sensitive, decoded query parameters, including repeated values.
///
/// Values for each name retain their order in the request URL. URL form decoding
/// treats `+` as a space.
#[derive(SafeDebug)]
pub struct Query(BTreeMap<String, Vec<String>>);

impl Query {
    /// Returns all decoded values for `name`, or an empty iterator if absent.
    pub fn get(&self, name: &str) -> impl Iterator<Item = &str> {
        self.0.get(name).into_iter().flatten().map(String::as_str)
    }
}

/// Reads and validates a fake request's JSON body as request content.
///
/// Reads streams from their current position without resetting them.
///
/// # Errors
///
/// Returns an error if reading the body or deserializing it as `T` fails.
pub async fn request_content<T: DeserializeOwned>(request: &Request) -> Result<RequestContent<T>> {
    let bytes = match request.body() {
        Body::Bytes(bytes) => bytes.clone(),
        Body::SeekableStream(stream) => {
            let mut stream = stream.clone();
            let mut bytes = Vec::new();
            stream.read_to_end(&mut bytes).await.map_err(|err| {
                Error::with_error(ErrorKind::Io, err, "failed to read fake request body")
            })?;
            bytes.into()
        }
    };
    json::from_json::<_, T>(&bytes)?;
    Ok(RequestContent::from(bytes.to_vec()))
}

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
    /// response's format marker. For response-only fields marked `skip_serializing`,
    /// pass a JSON value or use [`fake_with_bytes()`](Self::fake_with_bytes) instead.
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
