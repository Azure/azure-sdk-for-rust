// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    http::{headers::Headers, StatusCode},
    json::from_json,
};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use std::{fmt, marker::PhantomData, pin::Pin};
use typespec::error::{ErrorKind, ResultExt};

#[cfg(not(target_arch = "wasm32"))]
pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>> + Send + Sync>>;
#[cfg(target_arch = "wasm32")]
pub type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>>>>;

/// An HTTP response.
pub struct RawResponse {
    status: StatusCode,
    headers: Headers,
    body: ResponseBody,
}

impl RawResponse {
    /// Create an HTTP response.
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

    /// Get the response body as the specified type from JSON.
    pub async fn json<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        self.into_body().json().await
    }

    /// Get the response body as the specified type from XML.
    #[cfg(feature = "xml")]
    pub async fn xml<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        self.into_body().xml().await
    }
}

impl fmt::Debug for RawResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &"(body)")
            .finish()
    }
}

impl<T> From<RawResponse> for Response<T> {
    fn from(response: RawResponse) -> Self {
        Self {
            response,
            phantom: PhantomData,
        }
    }
}

pub struct Response<T> {
    response: RawResponse,
    phantom: PhantomData<T>,
}

impl<T> Response<T> {
    /// Create an HTTP response.
    pub fn new(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self {
        Self {
            response: RawResponse::new(status, headers, stream),
            phantom: PhantomData,
        }
    }

    /// Get the status code from the response.
    pub fn status(&self) -> StatusCode {
        self.response.status
    }

    /// Get the headers from the response.
    pub fn headers(&self) -> &Headers {
        &self.response.headers
    }

    /// Deconstruct the HTTP response into its components.
    pub fn deconstruct(self) -> (StatusCode, Headers, ResponseBody) {
        (
            self.response.status,
            self.response.headers,
            self.response.body,
        )
    }

    /// Consume the HTTP response and return the HTTP body bytes.
    pub fn into_body(self) -> ResponseBody {
        self.response.body
    }

    /// Get the response body as the specified type from JSON.
    pub async fn json(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        self.into_body().json().await
    }

    /// Get the response body as the specified type from XML.
    #[cfg(feature = "xml")]
    pub async fn xml(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        self.into_body().xml().await
    }
}

impl<T> fmt::Debug for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.response.status)
            .field("headers", &self.response.headers)
            .field("body", &"(body)")
            .finish()
    }
}

/// An HTTP response with the body collected as bytes.
#[derive(Debug, Clone)]
pub struct CollectedResponse<T> {
    status: StatusCode,
    headers: Headers,
    body: Bytes,
    phantom: PhantomData<T>,
}

impl<T> CollectedResponse<T> {
    /// Create a collected HTTP response.
    pub fn new(status: StatusCode, headers: Headers, body: Bytes) -> Self {
        Self {
            status,
            headers,
            body,
            phantom: PhantomData,
        }
    }

    /// Get the status code from the response.
    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    /// Get the headers from the response.
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Get the collected body from the response.
    pub fn body(&self) -> &Bytes {
        &self.body
    }

    /// Create a collected HTTP response from a [`Response`].
    pub async fn from_response(response: RawResponse) -> crate::Result<Self> {
        let (status, headers, body) = response.deconstruct();
        let body = body.collect().await?;
        Ok(Self::new(status, headers, body))
    }

    pub fn json(&self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        from_json(&self.body)
    }

    #[cfg(feature = "xml")]
    pub fn xml(&self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        crate::xml::read_xml(&self.body)
    }
}

impl<T> AsRef<[u8]> for CollectedResponse<T> {
    fn as_ref(&self) -> &[u8] {
        self.body.as_ref()
    }
}

/// A response body stream.
///
/// This body can either be streamed or collected into [`Bytes`].
#[pin_project::pin_project]
pub struct ResponseBody(#[pin] PinnedStream);

impl ResponseBody {
    fn new(stream: PinnedStream) -> Self {
        Self(stream)
    }

    /// Collect the stream into a [`Bytes`] collection.
    pub async fn collect(mut self) -> crate::Result<Bytes> {
        let mut final_result = Vec::new();

        while let Some(res) = self.0.next().await {
            final_result.extend(&res?);
        }

        Ok(final_result.into())
    }

    /// Collect the stream into a [`String`].
    pub async fn collect_string(self) -> crate::Result<String> {
        std::str::from_utf8(&self.collect().await?)
            .context(
                ErrorKind::DataConversion,
                "response body was not utf-8 like expected",
            )
            .map(ToOwned::to_owned)
    }

    /// Deserialize the JSON stream into type `T`.
    pub async fn json<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        let body = self.collect().await?;
        from_json(body)
    }

    /// Deserialize the XML stream into type `T`.
    #[cfg(feature = "xml")]
    pub async fn xml<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        let body = self.collect().await?;
        crate::xml::read_xml(&body)
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

impl fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ResponseBody")
    }
}
