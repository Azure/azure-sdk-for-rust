// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    http::{headers, RawResponse, StatusCode},
    Error, ErrorKind,
};
use bytes::Bytes;
use serde::Deserialize;
use std::{collections::HashMap, fmt};
#[cfg(feature = "json")]
use typespec::json::from_json;

/// An HTTP error response.
#[derive(Debug)]
pub struct HttpError {
    status: StatusCode,
    details: ErrorDetails,
    headers: HashMap<String, String>,
    body: Bytes,
}

impl HttpError {
    /// Create an error from an HTTP response.
    ///
    /// This does not check whether the response was successful and should only be used with unsuccessful responses.
    pub async fn new(response: RawResponse) -> Self {
        let status = response.status();
        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(name, value)| (name.as_str().to_owned(), value.as_str().to_owned()))
            .collect();
        let body = response
            .into_body()
            .collect()
            .await
            .unwrap_or_else(|_| Bytes::from_static(b"<ERROR COLLECTING BODY>"));
        let details = ErrorDetails::new(&headers, &body);
        HttpError {
            status,
            details,
            headers,
            body,
        }
    }

    /// Try to create an HTTP error from an [`Error`].
    ///
    /// This searches the entire ["source" chain](https://doc.rust-lang.org/std/error/trait.Error.html#method.source)
    /// looking for an `HttpError`.
    pub fn try_from(error: &Error) -> Option<&Self> {
        let mut error = error.get_ref()? as &(dyn std::error::Error);
        loop {
            match error.downcast_ref::<Self>() {
                Some(e) => return Some(e),
                None => error = error.source()?,
            }
        }
    }

    /// Get the status code for the HTTP error.
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get a reference to the HTTP error's error code.
    pub fn error_code(&self) -> Option<&str> {
        self.details.code.as_deref()
    }

    /// Get a reference to the HTTP error's error message.
    pub fn error_message(&self) -> Option<&str> {
        self.details.message.as_deref()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let newline = if f.alternate() { "\n" } else { " " };
        let tab = if f.alternate() { "\t" } else { " " };
        write!(f, "HttpError {{{newline}")?;
        write!(f, "{tab}Status: {},{newline}", self.status)?;
        write!(
            f,
            "{tab}Error Code: {},{newline}",
            self.details
                .code
                .as_deref()
                .unwrap_or("<unknown error code>")
        )?;
        // TODO: sanitize body
        write!(f, "{tab}Body: \"{:?}\",{newline}", self.body)?;
        write!(f, "{tab}Headers: [{newline}")?;
        // TODO: sanitize headers
        for (k, v) in &self.headers {
            write!(f, "{tab}{tab}{k}:{v}{newline}")?;
        }
        write!(f, "{tab}],{newline}}}{newline}")?;

        Ok(())
    }
}

impl std::error::Error for HttpError {}

#[derive(Debug)]
struct ErrorDetails {
    code: Option<String>,
    message: Option<String>,
}

impl ErrorDetails {
    fn new(headers: &HashMap<String, String>, body: &[u8]) -> Self {
        let mut code = get_error_code_from_header(headers);
        code = code.or_else(|| get_error_code_from_body(body));
        let message = get_error_message_from_body(body);
        Self { code, message }
    }
}

/// Gets the error code if it's present in the headers.
///
/// For more info, see [guidelines](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors).
fn get_error_code_from_header(headers: &HashMap<String, String>) -> Option<String> {
    headers.get(headers::ERROR_CODE.as_str()).cloned()
}

#[derive(Deserialize)]
struct NestedError {
    message: Option<String>,
    code: Option<String>,
}

#[derive(Deserialize)]
struct ErrorBody {
    error: Option<NestedError>,
    message: Option<String>,
    code: Option<String>,
}

/// Create an [`ErrorKind`] from an HTTP response with response content.
pub fn http_response_from_body(status: StatusCode, body: &[u8]) -> ErrorKind {
    let error_code = get_error_code_from_body(body);
    ErrorKind::http_response(status, error_code)
}

/// Gets the error code if it's present in the body.
///
/// For more info, see [guidelines](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors)
fn get_error_code_from_body(body: &[u8]) -> Option<String> {
    let decoded: ErrorBody = from_json(body).ok()?;
    decoded.error.and_then(|e| e.code).or(decoded.code)
}

/// Gets the error message if it's present in the body.
///
/// For more info, see [guidelines](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors)
fn get_error_message_from_body(body: &[u8]) -> Option<String> {
    let decoded: ErrorBody = from_json(body).ok()?;
    decoded.error.and_then(|e| e.message).or(decoded.message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_against_http_error() {
        let kind = http_response_from_body(StatusCode::ImATeapot, b"{}");

        assert!(matches!(
            kind,
            ErrorKind::HttpResponse {
                status: StatusCode::ImATeapot,
                error_code: None
            }
        ));

        let kind =
            http_response_from_body(StatusCode::ImATeapot, br#"{"error": {"code":"teapot"}}"#);

        assert!(matches!(
            kind,
            ErrorKind::HttpResponse {
                status: StatusCode::ImATeapot,
                error_code
            }
            if error_code.as_deref() == Some("teapot")
        ));
    }
}
