// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore innererror

use crate::{
    error::{Error, ErrorKind},
    http::{headers::ERROR_CODE, RawResponse},
};
use bytes::Bytes;
use serde::Deserialize;
use std::{collections::HashMap, str};

/// An HTTP error response.
///
/// Implements a standard "ErrorResponse" as described in the [API guidelines]
/// (https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    error: Option<ErrorDetail>,
}

impl ErrorResponse {
    /// The error details.
    pub fn error(&self) -> Option<&ErrorDetail> {
        self.error.as_ref()
    }
}

impl TryFrom<Error> for ErrorResponse {
    type Error = Error;

    fn try_from(value: Error) -> Result<Self, Self::Error> {
        match value.kind() {
            ErrorKind::HttpResponse { .. } => todo!(),
            _ => Err(value),
        }
    }
}

/// Details about an error returned from a service.
///
/// Implements a standard "ErrorDetails" as described in the [API guidelines]
/// (https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetail {
    /// The error code.
    code: Option<String>,

    /// A human-readable error message describing the error.
    message: Option<String>,

    /// The target of the error (for example, the name of the property in error).
    target: Option<String>,

    /// Additional details about the error.
    #[serde(default)]
    details: Vec<ErrorDetail>,

    /// An inner error that may have more specific information about the root cause of the error.
    #[serde(rename = "innererror")]
    inner_error: Option<InnerError>,

    /// Additional properties that may be returned with the error.
    #[serde(flatten)]
    additional_properties: HashMap<String, serde_json::Value>,
}

impl ErrorDetail {
    /// The error code.
    ///
    /// This is a machine-readable error code, typically derived from the x-ms-error-code response header.
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// The error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// The target of the error (for example, the name of the property in error).
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// Additional details about the error.
    pub fn details(&self) -> &[ErrorDetail] {
        &self.details
    }

    /// An inner error that may have more specific information about the root cause of the error.
    pub fn inner_error(&self) -> Option<&InnerError> {
        self.inner_error.as_ref()
    }

    /// Additional properties that may be returned with the error.
    pub fn additional_properties(&self) -> &HashMap<String, serde_json::Value> {
        &self.additional_properties
    }
}

/// Inner error information about an error returned from a service.
///
/// Implements a standard "InnerError" as described in the [API guidelines]
/// (https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerError {
    /// A more specific error than was contained in the containing error.
    code: Option<String>,

    /// An object containing more specific information than the current object about the error.
    #[serde(rename = "innererror")]
    inner_error: Option<Box<InnerError>>,
}

impl InnerError {
    /// The error code.
    ///
    /// This is a machine-readable error code, typically derived from the x-ms-error-code response header.
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// An inner error that may have more specific information about the root cause of the error.
    pub fn inner_error(&self) -> Option<&InnerError> {
        self.inner_error.as_deref()
    }
}

/// Internal struct to help with deserialization without allocating Strings.
#[derive(Debug, Deserialize)]
struct ErrorResponseInternal<'a> {
    #[serde(borrow)]
    error: ErrorDetailsInternal<'a>,
}

#[derive(Debug, Deserialize)]
struct ErrorDetailsInternal<'a> {
    code: Option<&'a str>,
    message: Option<&'a str>,
}

/// Checks if the response is a success and if not, creates an appropriate error.
///
/// # Arguments
/// * `response` - The HTTP response to check.
///
/// # Returns
/// * `Ok(RawResponse)` if the response is a success.
/// * `Err(Error)` if the response is an error, with details extracted from the response
///   body if possible.
///
pub async fn check_success(response: RawResponse) -> crate::Result<RawResponse> {
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }

    let (status, headers, body) = response.deconstruct();
    let body = body
        .collect()
        .await
        .unwrap_or_else(|_| Bytes::from_static(b"(error reading body)"));

    // If there's no body, we can't extract any more information.
    if body.is_empty() {
        let code = headers.get_optional_str(&ERROR_CODE);
        let error_kind = ErrorKind::http_response(status, code.map(str::to_owned));
        return Err(Error::message(error_kind, status.to_string()));
    }
    let internal_response =
        serde_json::de::from_slice::<ErrorResponseInternal>(body.as_ref()).map_err(Error::from);

    let internal_response = match internal_response {
        Ok(r) => r,
        Err(_) => {
            // If we can't parse the body, return a generic error with the status code and body
            let code = headers.get_optional_str(&ERROR_CODE);
            let error_kind = ErrorKind::http_response(status, code.map(str::to_owned));
            return Err(Error::message(
                error_kind,
                format!(
                    "{}: {}",
                    status,
                    str::from_utf8(&body).unwrap_or("(invalid utf-8 in body)")
                ),
            ));
        }
    };

    // We give priority to the error code in the header, and try the body version if it's not present.
    let code = headers.get_optional_str(&ERROR_CODE);
    let code = code.or(internal_response.error.code);

    let error_kind = ErrorKind::http_response(status, code.map(str::to_owned));

    Err(Error::message(
        error_kind,
        internal_response
            .error
            .message
            .map_or_else(|| status.to_string(), |m| m.to_owned()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{headers, headers::Headers, StatusCode};

    #[tokio::test]
    async fn matching_against_http_error() {
        let mut headers = Headers::new();
        headers.insert(headers::CONTENT_TYPE, "application/json".to_string());
        let response = RawResponse::from_bytes(
            StatusCode::ImATeapot,
            headers,
            Bytes::from_static(br#"{"error": {"code":"teapot","message":"I'm a teapot"}}"#),
        );

        let err = check_success(response).await.unwrap_err();
        let kind = err.kind();
        assert!(matches!(
            kind,
            ErrorKind::HttpResponse {
                status: StatusCode::ImATeapot,
                error_code
            }
            if error_code.as_deref() == Some("teapot")
        ));
    }

    #[tokio::test]
    async fn matching_against_http_error_no_body() {
        let mut headers = Headers::new();
        headers.insert(headers::ERROR_CODE, "testError".to_string());
        let response = RawResponse::from_bytes(StatusCode::ImATeapot, headers, Bytes::new());

        let err = check_success(response).await.unwrap_err();
        let kind = err.kind();
        assert_eq!(
            *kind,
            ErrorKind::HttpResponse {
                status: StatusCode::ImATeapot,
                error_code: Some("testError".to_string())
            }
        );
    }

    #[tokio::test]
    async fn matching_against_http_error_invalid_body() {
        let mut headers = Headers::new();
        headers.insert(headers::ERROR_CODE, "testError".to_string());
        let response = RawResponse::from_bytes(
            StatusCode::ImATeapot,
            headers,
            Bytes::from_static(br#"{"json": "error"}"#),
        );

        let err = check_success(response).await.unwrap_err();
        let kind = err.kind();
        assert_eq!(
            *kind,
            ErrorKind::HttpResponse {
                status: StatusCode::ImATeapot,
                error_code: Some("testError".to_string())
            }
        );
        assert!(err.to_string().contains(r#"{"json": "error"}"#));
    }

    #[test]
    fn deserialize_to_error_response() {
        let err : ErrorResponse = serde_json::from_slice (br#"{"error":{"code":"InvalidRequest","message":"The request object is not recognized.","innererror":{"code":"InvalidKey"},"key":"foo"}}"#)
            .expect("Parse success.");
        err.error().expect("error should be set");

        println!("{:?}", &err);
        assert_eq!(err.error().unwrap().code(), Some("InvalidRequest"));
        assert_eq!(
            err.error().unwrap().message(),
            Some("The request object is not recognized.")
        );
        assert!(err.error().unwrap().inner_error().is_some());
        assert_eq!(
            err.error().unwrap().inner_error().as_ref().unwrap().code(),
            Some("InvalidKey")
        );
        assert!(err
            .error()
            .unwrap()
            .additional_properties()
            .contains_key("key"));
    }

    #[tokio::test]
    async fn deserialize_to_error_response_internal() {
        let err :ErrorResponseInternal = serde_json::from_slice (br#"{"error":{"code":"InvalidRequest","message":"The request object is not recognized.","innererror":{"code":"InvalidKey","key":"foo"}}}"#)
            .expect("Parse success.");
        println!("{:?}", &err);

        assert_eq!(err.error.code, Some("InvalidRequest"));
        assert_eq!(
            err.error.message,
            Some("The request object is not recognized.")
        );
    }
}
