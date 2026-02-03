// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::generated::models::StorageErrorCode;
use azure_core::{error::ErrorKind, http::RawResponse, xml};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// A specialized `Result` type for Azure Storage Blob operations.
pub type Result<T> = std::result::Result<T, StorageError>;

/// Represents an error response from Azure Storage services.
#[derive(Debug, Clone)]
pub struct StorageError {
    /// The HTTP status code.
    pub status_code: azure_core::http::StatusCode,
    /// The Storage error code, if available.
    pub error_code: Option<StorageErrorCode>,
    /// The error message, if available.
    pub message: Option<String>,
    /// The request ID from the x-ms-request-id header, if available.
    pub request_id: Option<String>,
    /// A general reason for the error, if available.
    pub reason: Option<String>,
    /// Additional authentication error details, if available.
    pub authentication_error_detail: Option<String>,
    /// The HTTP status code from the copy source, if available.
    pub copy_source_status_code: Option<azure_core::http::StatusCode>,
    /// The error code from the copy source, if available.
    pub copy_source_error_code: Option<String>,
    /// The error message from the copy source, if available.
    pub copy_source_error_message: Option<String>,
    /// Additional fields from the error response that weren't explicitly mapped.
    pub additional_error_info: HashMap<String, String>,
}

impl StorageError {
    /// Converts a `serde_json::Value` to a String representation, handling nested XML structures.
    fn value_to_string(value: &Value) -> String {
        match value {
            // Primitive types: extract directly
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),

            // Special case: XML elements with $text field - extract the text content
            Value::Object(obj) if obj.len() == 1 && obj.contains_key("$text") => obj
                .get("$text")
                .and_then(|v| v.as_str())
                .map(String::from)
                .unwrap_or_default(),

            // Arrays and complex objects: use serde_json's compact representation
            _ => serde_json::to_string(value).unwrap_or_default(),
        }
    }

    /// Deserializes a `StorageError` from XML body with HTTP response metadata.
    fn from_xml(
        status_code: azure_core::http::StatusCode,
        error_code: Option<StorageErrorCode>,
        request_id: Option<String>,
        raw_response: RawResponse,
    ) -> std::result::Result<Self, azure_core::Error> {
        #[derive(Deserialize)]
        #[serde(rename = "Error")]
        struct StorageErrorXml {
            #[serde(rename = "Code")]
            code: Option<String>,
            #[serde(rename = "Message")]
            message: Option<String>,
            #[serde(rename = "Reason")]
            reason: Option<String>,
            #[serde(rename = "AuthenticationErrorDetail")]
            authentication_error_detail: Option<String>,
            #[serde(rename = "CopySourceStatusCode")]
            copy_source_status_code: Option<String>,
            #[serde(rename = "CopySourceErrorCode")]
            copy_source_error_code: Option<String>,
            #[serde(rename = "CopySourceErrorMessage")]
            copy_source_error_message: Option<String>,
            #[serde(flatten)]
            additional_fields: HashMap<String, Value>,
        }

        let xml_fields = xml::from_xml::<_, StorageErrorXml>(raw_response.body())?;

        // Convert additional fields from HashMap<String, Value> to HashMap<String, String>
        let additional_error_info = xml_fields
            .additional_fields
            .iter()
            .map(|(k, v)| (k.clone(), Self::value_to_string(v)))
            .collect();

        Ok(StorageError {
            status_code,
            error_code,
            message: xml_fields.message,
            request_id,
            reason: xml_fields.reason,
            authentication_error_detail: xml_fields.authentication_error_detail,
            copy_source_status_code: xml_fields
                .copy_source_status_code
                .and_then(|s| s.parse::<u16>().ok())
                .map(azure_core::http::StatusCode::from),
            copy_source_error_code: xml_fields.copy_source_error_code,
            copy_source_error_message: xml_fields.copy_source_error_message,
            additional_error_info,
        })
    }
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "HTTP Status Code: {}", self.status_code)?;

        if let Some(request_id) = &self.request_id {
            writeln!(f, "Request ID: {}", request_id)?;
        }

        if let Some(error_code) = &self.error_code {
            writeln!(f, "Storage Error Code: {}", error_code)?;
        }

        if let Some(message) = &self.message {
            writeln!(f, "Error Message: {}", message)?;
        }

        if let Some(reason) = &self.reason {
            writeln!(f, "Reason: {}", reason)?;
        }

        if let Some(detail) = &self.authentication_error_detail {
            writeln!(f, "Authentication Error Detail: {}", detail)?;
        }

        if let Some(status) = &self.copy_source_status_code {
            writeln!(f, "Copy Source Status Code: {}", status)?;
        }

        if let Some(code) = &self.copy_source_error_code {
            writeln!(f, "Copy Source Error Code: {}", code)?;
        }

        if let Some(message) = &self.copy_source_error_message {
            writeln!(f, "Copy Source Error Message: {}", message)?;
        }

        if !self.additional_error_info.is_empty() {
            writeln!(f, "\nAdditional Error Info:")?;
            for (key, value) in &self.additional_error_info {
                writeln!(f, "{}: {}", key, value)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for StorageError {}

impl TryFrom<azure_core::Error> for StorageError {
    type Error = azure_core::Error;

    fn try_from(error: azure_core::Error) -> std::result::Result<Self, Self::Error> {
        match error.kind() {
            ErrorKind::HttpResponse {
                status,
                raw_response: Some(raw_response),
                ..
            } => {
                let headers = raw_response.headers();
                let body = raw_response.body();

                let error_code = headers
                    .get_optional_string(&azure_core::http::headers::HeaderName::from_static(
                        "x-ms-error-code",
                    ))
                    .and_then(|code| {
                        code.parse()
                            .ok()
                            .or(Some(StorageErrorCode::UnknownValue(code)))
                    });

                let request_id = headers.get_optional_string(
                    &azure_core::http::headers::HeaderName::from_static("x-ms-request-id"),
                );

                if body.is_empty() {
                    // For bodiless responses, use the canonical reason phrase as a fallback message
                    let message = Some(status.canonical_reason().to_string());

                    return Ok(StorageError {
                        status_code: *status,
                        error_code,
                        message,
                        request_id,
                        reason: None,
                        authentication_error_detail: None,
                        copy_source_status_code: None,
                        copy_source_error_code: None,
                        copy_source_error_message: None,
                        additional_error_info: HashMap::new(),
                    });
                }

                StorageError::from_xml(
                    *status,
                    error_code,
                    request_id,
                    raw_response.as_ref().clone(),
                )
            }
            // Return a minimal StorageError if raw_response is missing
            ErrorKind::HttpResponse {
                status,
                raw_response: None,
                ..
            } => {
                let message = Some(status.canonical_reason().to_string());
                Ok(StorageError {
                    status_code: *status,
                    error_code: None,
                    message,
                    request_id: None,
                    reason: None,
                    authentication_error_detail: None,
                    copy_source_status_code: None,
                    copy_source_error_code: None,
                    copy_source_error_message: None,
                    additional_error_info: HashMap::new(),
                })
            }
            _ => Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                "ErrorKind was not HttpResponse and could not be parsed.",
            )),
        }
    }
}
