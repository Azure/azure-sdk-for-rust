// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(feature = "json")]
use crate::json::from_json;
use crate::{
    error::ErrorKind,
    http::{headers::HeaderName, BufResponse, StatusCode},
    Bytes, Error,
};
use serde::Deserialize;
use std::{collections::HashMap, fmt, str};

/// An HTTP error response.
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
    pub async fn new(response: BufResponse, header_name: Option<HeaderName>) -> Self {
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
            .unwrap_or_else(|_| Bytes::from_static(b"(error reading body)"));
        let details = ErrorDetails::new(&headers, header_name, &body);
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
        let mut error = error.get_ref()? as &dyn std::error::Error;
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

    /// Get a reference to the HTTP error's headers.
    ///
    /// You should not display these headers directly.
    /// Headers may contain Personally-Identifiable Information (PII) and need to be sanitized.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Get a reference to the HTTP error's body.
    ///
    /// You should not display the body directly.
    /// The body may contain Personally-Identifiable Information (PII) and need to be sanitized.
    pub fn body(&self) -> &Bytes {
        &self.body
    }
}

struct Unquote<'a>(&'a str);
impl fmt::Debug for Unquote<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl fmt::Debug for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Elide potential PII since it's too easily to accidentally leak through Debug or Display.
        let mut dbg = f.debug_struct("HttpError");

        #[cfg_attr(not(feature = "test"), allow(unused_mut))]
        let mut dbg = dbg
            .field("status", &self.status)
            .field("details", &self.details);

        #[cfg(feature = "test")]
        {
            dbg = dbg.field(
                "body",
                &Unquote(
                    String::from_utf8(self.body.to_vec())
                        .as_deref()
                        .unwrap_or("(bytes)"),
                ),
            );
        }

        dbg.finish_non_exhaustive()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Status(StatusCode);
        impl fmt::Debug for Status {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_fmt(format_args!(
                    "{} ({})",
                    std::convert::Into::<u16>::into(self.0),
                    self.0.canonical_reason()
                ))
            }
        }

        // Elide potential PII since it's too easily to accidentally leak through Debug or Display.
        let mut dbg = f.debug_struct("HttpError");

        #[cfg_attr(not(feature = "test"), allow(unused_mut))]
        let mut dbg = dbg
            .field("Status", &Status(self.status))
            .field(
                "Error Code",
                &Unquote(
                    self.details
                        .code
                        .as_deref()
                        .unwrap_or("(error code unavailable)"),
                ),
            )
            .field(
                "Message",
                &Unquote(
                    self.details
                        .message
                        .as_deref()
                        .unwrap_or("(message unavailable)"),
                ),
            );

        #[cfg(feature = "test")]
        {
            dbg = dbg.field(
                "Body",
                &Unquote(
                    String::from_utf8(self.body.to_vec())
                        .as_deref()
                        .unwrap_or("(bytes)"),
                ),
            );
        }

        dbg.finish_non_exhaustive()
    }
}

impl std::error::Error for HttpError {}

#[derive(Debug)]
struct ErrorDetails {
    code: Option<String>,
    message: Option<String>,
}

impl ErrorDetails {
    fn new(
        headers: &HashMap<String, String>,
        header_name: Option<HeaderName>,
        body: &[u8],
    ) -> Self {
        let mut code = get_error_code_from_header(headers, header_name);
        code = code.or_else(|| get_error_code_from_body(body));
        let message = get_error_message_from_body(body);
        Self { code, message }
    }
}

/// Gets the error code if it's present in the headers.
///
/// For more info, see [guidelines](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors).
fn get_error_code_from_header(
    headers: &HashMap<String, String>,
    error_header_name: Option<HeaderName>,
) -> Option<String> {
    if let Some(error_header_name) = error_header_name {
        headers.get(error_header_name.as_str()).cloned()
    } else {
        None
    }
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
                error_code: None,
                ..
            }
        ));

        let kind =
            http_response_from_body(StatusCode::ImATeapot, br#"{"error": {"code":"teapot"}}"#);

        assert!(matches!(
            kind,
            ErrorKind::HttpResponse {
                status: StatusCode::ImATeapot,
                error_code,
                ..
            }
            if error_code.as_deref() == Some("teapot")
        ));
    }

    #[test]
    fn debug_is_sanitized() {
        let err = HttpError {
            status: StatusCode::NotFound,
            details: ErrorDetails {
                code: Some("Not Found".to_string()),
                message: Some("Resource not found".to_string()),
            },
            body: Bytes::from_static(b"resource not found"),
            headers: HashMap::from([
                ("authorization".to_string(), "bearer *****".to_string()),
                ("x-ms-request-id".to_string(), "abcd1234".to_string()),
            ]),
        };
        #[cfg(not(feature = "test"))]
        assert_eq!(
            format!("{err:?}"),
            r#"HttpError { status: NotFound, details: ErrorDetails { code: Some("Not Found"), message: Some("Resource not found") }, .. }"#
        );
        #[cfg(feature = "test")]
        assert_eq!(
            format!("{err:?}"),
            r#"HttpError { status: NotFound, details: ErrorDetails { code: Some("Not Found"), message: Some("Resource not found") }, body: resource not found, .. }"#
        );
    }

    #[test]
    fn display_is_sanitized() {
        let err = HttpError {
            status: StatusCode::NotFound,
            details: ErrorDetails {
                code: None,
                message: None,
            },
            body: Bytes::from_static(b"resource not found"),
            headers: HashMap::from([
                ("authorization".to_string(), "bearer *****".to_string()),
                ("x-ms-request-id".to_string(), "abcd1234".to_string()),
            ]),
        };
        #[cfg(not(feature = "test"))]
        assert_eq!(
            format!("{err:}"),
            r#"HttpError { Status: 404 (Not Found), Error Code: (error code unavailable), Message: (message unavailable), .. }"#
        );

        #[cfg(feature = "test")]
        assert_eq!(
            format!("{err:}"),
            r#"HttpError { Status: 404 (Not Found), Error Code: (error code unavailable), Message: (message unavailable), Body: resource not found, .. }"#
        );
    }

    #[cfg(feature = "json")]
    #[tokio::test]
    async fn deserialize_body() {
        // cspell:ignore innererror
        use crate::{http::headers::Headers, json};

        #[derive(Deserialize)]
        struct ErrorResponse {
            error: ErrorDetail,
        }

        #[derive(Deserialize)]
        struct ErrorDetail {
            #[serde(rename = "innererror")]
            inner_error: Option<InnerError>,
        }

        #[derive(Deserialize)]
        struct InnerError {
            code: Option<String>,
        }

        let response = BufResponse::from_bytes(
            StatusCode::BadRequest,
            Headers::new(),
            Bytes::from_static(br#"{"error":{"code":"InvalidRequest","message":"The request object is not recognized.","innererror":{"code":"InvalidKey","key":"foo"}}}"#),
        );
        let err = HttpError::new(response, Some(HeaderName::from_static("x-ms-error-code"))).await;

        assert_eq!(err.status(), StatusCode::BadRequest);
        assert_eq!(err.error_code(), Some("InvalidRequest"));
        assert_eq!(
            err.error_message(),
            Some("The request object is not recognized.")
        );
        assert!(err.headers().is_empty());

        let details: ErrorResponse = json::from_json(err.body()).expect("JSON error response");
        assert_eq!(
            details.error.inner_error.expect("innererror code").code,
            Some("InvalidKey".to_string()),
        );
    }
}
