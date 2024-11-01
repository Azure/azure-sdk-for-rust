// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(feature = "json")]
use crate::json::from_json;
use crate::{
    error::ErrorKind,
    http::{headers, Response, StatusCode},
    Error,
};
use bytes::Bytes;
use serde::Deserialize;
use std::{collections::HashMap, fmt};

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
    pub async fn new(response: Response<()>) -> Self {
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

impl fmt::Debug for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Elide potential PII since it's too easily to accidentally leak through Debug or Display.
        f.debug_struct("HttpError")
            .field("status", &self.status)
            .field("details", &self.details)
            .finish_non_exhaustive()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Unquote<'a>(&'a str);
        impl<'a> fmt::Debug for Unquote<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.0)
            }
        }

        // Elide potential PII since it's too easily to accidentally leak through Debug or Display.
        f.debug_struct("HttpError")
            .field("Status", &Unquote(&self.status.to_string()))
            .field(
                "Error Code",
                &Unquote(
                    self.details
                        .code
                        .as_deref()
                        .unwrap_or("(unknown error code)"),
                ),
            )
            .finish_non_exhaustive()
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
        let actual = format!("{err:?}");
        assert_eq!(
            actual,
            r#"HttpError { status: NotFound, details: ErrorDetails { code: Some("Not Found"), message: Some("Resource not found") }, .. }"#
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
        let actual = format!("{err}");
        assert_eq!(
            actual,
            r#"HttpError { Status: 404, Error Code: (unknown error code), .. }"#
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

        let response: Response<()> = Response::from_bytes(
            StatusCode::BadRequest,
            Headers::new(),
            Bytes::from_static(br#"{"error":{"code":"InvalidRequest","message":"The request object is not recognized.","innererror":{"code":"InvalidKey","key":"foo"}}}"#),
        );
        let err = HttpError::new(response).await;

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
