// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP responses.

use crate::{
    error::{ErrorKind, ResultExt as _},
    http::{headers::Headers, StatusCode},
    Bytes,
};
#[cfg(any(feature = "json", feature = "xml"))]
use serde::de::DeserializeOwned;
use std::{fmt, ops::Deref};

/// A raw HTTP response with status, headers, and body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawResponse {
    status: StatusCode,
    headers: Headers,
    body: ResponseBody,
}

impl RawResponse {
    /// Create an HTTP response from raw bytes.
    pub fn from_bytes(status: StatusCode, headers: Headers, body: impl Into<Bytes>) -> Self {
        Self {
            status,
            headers,
            body: ResponseBody::from_bytes(body),
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

    /// Get the [`ResponseBody`].
    pub fn body(&self) -> &ResponseBody {
        &self.body
    }

    /// Get the [`ResponseBody`].
    pub fn into_body(self) -> ResponseBody {
        self.body
    }

    /// Deconstruct the HTTP response into its components.
    pub fn deconstruct(self) -> (StatusCode, Headers, ResponseBody) {
        (self.status, self.headers, self.body)
    }
}

/// A response body.
///
/// This body has already been collected.
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseBody(Bytes);

impl ResponseBody {
    /// Create a new [`ResponseBody`] from a byte slice.
    pub fn from_bytes(bytes: impl Into<Bytes>) -> Self {
        Self(bytes.into())
    }

    /// Collect the stream into a [`String`].
    pub fn collect_string(self) -> crate::Result<String> {
        std::str::from_utf8(&self.0)
            .with_context(
                ErrorKind::DataConversion,
                "response body was not utf-8 like expected",
            )
            .map(ToOwned::to_owned)
    }

    /// Deserialize the JSON stream into type `T`.
    #[cfg(feature = "json")]
    pub fn json<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        crate::json::from_json(&self.0)
    }

    /// Deserialize the XML stream into type `T`.
    #[cfg(feature = "xml")]
    pub fn xml<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        crate::xml::read_xml(&self.0)
    }
}

impl AsRef<[u8]> for ResponseBody {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for ResponseBody {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ResponseBody")
    }
}

impl From<ResponseBody> for Bytes {
    fn from(body: ResponseBody) -> Self {
        body.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Employee {
        id: u32,
        name: String,
    }

    #[test]
    fn as_ref() {
        let resp =
            RawResponse::from_bytes(StatusCode::Ok, Headers::new(), r#"{"id":12,"name":"Bob"}"#);
        assert_eq!(resp.body().as_ref(), br#"{"id":12,"name":"Bob"}"#);
    }

    #[test]
    fn deref() {
        let resp =
            RawResponse::from_bytes(StatusCode::Ok, Headers::new(), r#"{"id":12,"name":"Bob"}"#);
        assert_eq!(&*resp.into_body(), br#"{"id":12,"name":"Bob"}"#);
    }

    #[cfg(feature = "json")]
    #[test]
    fn deserializes_json() {
        let resp =
            RawResponse::from_bytes(StatusCode::Ok, Headers::new(), r#"{"id":12,"name":"Bob"}"#);
        let e: Employee = resp.into_body().json().expect("deserialize Employee");
        assert_eq!(e.id, 12);
        assert_eq!(e.name, "Bob");
    }

    #[cfg(feature = "xml")]
    #[test]
    fn deserializes_xml() {
        let resp = RawResponse::from_bytes(
            StatusCode::Ok,
            Headers::new(),
            r#"<Employee><id>34</id><name>Maria</name></Employee>"#,
        );
        let e: Employee = resp.into_body().xml().expect("deserialize Employee");
        assert_eq!(e.id, 34);
        assert_eq!(e.name, "Maria");
    }
}
