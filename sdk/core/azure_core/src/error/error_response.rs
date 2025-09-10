// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore innererror

use crate::{
    error::{Error, ErrorKind},
    http::{headers::ERROR_CODE, BufResponse},
};
use serde::Deserialize;
use std::{collections::HashMap, str};

/// An HTTP error response.
///
/// Implements a standard "ErrorResponse" as described in the [API guidelines](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors).
///
/// Can be converted from an `[Error]` if it is of kind `[ErrorKind::HttpResponse]` and has a raw response.
///
/// # Example
///
/// Converting an `Error` to an `ErrorResponse`:
///
///``` no_run
/// use azure_core::error::ErrorResponse;
/// # let err = azure_core::Error::from(azure_core::error::ErrorKind::DataConversion);
/// let error_response = ErrorResponse::try_from(err).expect("expected an ErrorResponse");
///```
///
///
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    /// The error details.
    pub error: Option<ErrorDetail>,
}

impl TryFrom<Error> for ErrorResponse {
    type Error = Error;

    fn try_from(value: Error) -> Result<Self, Self::Error> {
        match value.kind() {
            ErrorKind::HttpResponse { raw_response, .. } => {
                let error_response: Option<crate::Result<ErrorResponse>> = raw_response
                    .as_ref()
                    .map(|raw| serde_json::from_slice(raw.body()).map_err(Error::from));
                match error_response {
                    Some(result) => Ok(result?),
                    None => Err(value),
                }
            }
            _ => Err(value),
        }
    }
}

/// Details about an error returned from a service.
///
/// Implements a standard "ErrorDetails" as described in the [API guidelines](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetail {
    /// The error code. A machine readable error code defined by the service.
    pub code: Option<String>,

    /// A human-readable error message describing the error.
    pub message: Option<String>,

    /// The target of the error (for example, the name of the property in error).
    pub target: Option<String>,

    /// Additional details about the error.
    #[serde(default)]
    pub details: Vec<ErrorDetail>,

    /// An inner error that may have more specific information about the root cause of the error.
    #[serde(rename = "innererror")]
    pub inner_error: Option<InnerError>,

    /// Additional properties that may be returned with the error.
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

/// Inner error information about an error returned from a service.
///
/// Implements a standard "InnerError" as described in the [API guidelines](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerError {
    /// A more specific error than was contained in the containing error.
    pub code: Option<String>,

    /// An object containing more specific information than the current object about the error.
    #[serde(rename = "innererror")]
    pub inner_error: Option<Box<InnerError>>,
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
pub async fn check_success(response: BufResponse) -> crate::Result<BufResponse> {
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }

    let response = response.try_into_raw_response().await?;

    // If there's no body, we can't extract any more information.
    if response.body().is_empty() {
        let code = response.headers().get_optional_str(&ERROR_CODE);
        let error_kind = ErrorKind::http_response(status, code.map(str::to_owned));
        return Err(Error::message(error_kind, status.to_string()));
    }
    let internal_response =
        serde_json::de::from_slice::<ErrorResponseInternal>(response.body()).map_err(Error::from);

    let internal_response = match internal_response {
        Ok(r) => r,
        Err(_) => {
            // If we can't parse the body, return a generic error with the status code and body
            let code = response.headers().get_optional_str(&ERROR_CODE);
            let error_kind = ErrorKind::http_response(
                status,
                Some(code.map_or_else(|| response.status().to_string(), str::to_owned)),
            );
            return Err(Error::message(
                error_kind,
                format!(
                    "{}: {}",
                    status,
                    str::from_utf8(response.body()).unwrap_or("(invalid utf-8 in body)")
                ),
            ));
        }
    };

    // We give priority to the error code in the header, and try the body version if it's not present.
    let code = response.headers().get_optional_str(&ERROR_CODE);
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
    use crate::Bytes;

    #[tokio::test]
    async fn matching_against_http_error() {
        let mut headers = Headers::new();
        headers.insert(headers::CONTENT_TYPE, "application/json".to_string());
        let response = BufResponse::from_bytes(
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
                error_code,
                raw_response: None
            }
            if error_code.as_deref() == Some("teapot")
        ));
    }

    #[tokio::test]
    async fn matching_against_http_error_no_body() {
        let mut headers = Headers::new();
        headers.insert(headers::ERROR_CODE, "testError".to_string());
        let response = BufResponse::from_bytes(StatusCode::ImATeapot, headers, Bytes::new());

        let err = check_success(response).await.unwrap_err();
        let kind = err.kind();
        assert_eq!(
            *kind,
            ErrorKind::HttpResponse {
                status: StatusCode::ImATeapot,
                error_code: Some("testError".to_string()),
                raw_response: None
            }
        );
    }

    #[tokio::test]
    async fn matching_against_http_error_invalid_body() {
        let mut headers = Headers::new();
        headers.insert(headers::ERROR_CODE, "testError".to_string());
        let response = BufResponse::from_bytes(
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
                error_code: Some("testError".to_string()),
                raw_response: None
            }
        );
        assert!(err.to_string().contains(r#"{"json": "error"}"#));
    }

    #[test]
    fn deserialize_to_error_response() {
        let err : ErrorResponse = serde_json::from_slice (br#"{"error":{"code":"InvalidRequest","message":"The request object is not recognized.","innererror":{"code":"InvalidKey"},"key":"foo"}}"#)
            .expect("Parse success.");
        err.error.as_ref().expect("error should be set");

        println!("{:?}", &err);
        assert_eq!(
            err.error.as_ref().unwrap().code,
            Some("InvalidRequest".to_string())
        );
        assert_eq!(
            err.error.as_ref().unwrap().message,
            Some("The request object is not recognized.".to_string())
        );
        assert!(err.error.as_ref().unwrap().inner_error.is_some());
        assert_eq!(
            err.error
                .as_ref()
                .unwrap()
                .inner_error
                .as_ref()
                .unwrap()
                .code,
            Some("InvalidKey".to_string())
        );
        assert!(err
            .error
            .as_ref()
            .unwrap()
            .additional_properties
            .contains_key("key"));
    }

    #[tokio::test]
    async fn convert_error_to_error_response() -> crate::Result<()> {
        {
            let err: Error = Error::from(ErrorKind::HttpResponse {
                status: StatusCode::BadRequest,
                error_code: Some("testError".to_string()),
                raw_response: None,
            });
            let _error_response = ErrorResponse::try_from(err)
                .expect_err("expected an error because there is no raw_response");
        }
        {
            let buf_response = BufResponse::from_bytes(
                StatusCode::BadRequest,
                Headers::new(),
                Bytes::from_static(br#"{"error":{"code":"InvalidRequest","message":"The request object is not recognized.","innererror":{"code":"InvalidKey"},"key":"foo"}}"#),
            );
            let err: Error = Error::from(ErrorKind::HttpResponse {
                status: StatusCode::BadRequest,
                error_code: Some("testError".to_string()),
                raw_response: Some(Box::new(buf_response.try_into_raw_response().await?)),
            });
            let error_response = ErrorResponse::try_from(err).expect("expected an ErrorResponse");
            error_response.error.as_ref().expect("error should be set");
            println!("{:?}", &error_response);
            assert_eq!(
                error_response.error.as_ref().unwrap().code,
                Some("InvalidRequest".to_string())
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn convert_buf_response_to_error_response() -> crate::Result<()> {
        {
            let buf_response = BufResponse::from_bytes(
                StatusCode::BadRequest,
                Headers::new(),
                Bytes::from_static(br#"{"error":{"code":"InvalidRequest","message":"The request object is not recognized.","innererror":{"code":"InvalidKey"},"key":"foo"}}"#),
            );
            let error_response: ErrorResponse = buf_response
                .into_body()
                .json()
                .await
                .expect("expected an ErrorResponse");
            error_response.error.as_ref().expect("error should be set");
            println!("{:?}", &error_response);
            assert_eq!(
                error_response.error.as_ref().unwrap().code,
                Some("InvalidRequest".to_string())
            );
        }
        Ok(())
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
