// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP responses.

use crate::{
    http::{Headers, StatusCode},
    Bytes,
};
#[cfg(any(feature = "json", feature = "xml"))]
use serde::de::DeserializeOwned;

/// A raw HTTP response with status, headers, and body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawResponse {
    status: StatusCode,
    headers: Headers,
    body: Bytes,
}

impl RawResponse {
    /// Create an HTTP response from raw bytes.
    pub fn from_bytes(status: StatusCode, headers: Headers, body: impl Into<Bytes>) -> Self {
        Self {
            status,
            headers,
            body: body.into(),
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

    /// Get the response body.
    pub fn body(&self) -> &Bytes {
        &self.body
    }

    /// Get the response body.
    pub fn into_body(self) -> Bytes {
        self.body
    }

    /// Deserialize the JSON stream into type `T`.
    #[cfg(feature = "json")]
    pub fn json<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        crate::json::from_json(self.body)
    }

    /// Deserialize the XML stream into type `T`.
    #[cfg(feature = "xml")]
    pub fn xml<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        crate::xml::read_xml(&self.body)
    }
}
