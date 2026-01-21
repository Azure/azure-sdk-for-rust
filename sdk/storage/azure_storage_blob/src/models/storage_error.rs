// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::generated::models::StorageErrorCode;
use azure_core::{error::ErrorKind, http::RawResponse, xml};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// Represents an error response from Azure Storage services.
#[derive(Debug, Clone)]
pub struct StorageError {
    /// The HTTP status code.
    status_code: azure_core::http::StatusCode,
    /// The Storage error code, if available.
    error_code: Option<StorageErrorCode>,
    /// The error message, if available.
    message: Option<String>,
    /// The request ID from the x-ms-request-id header, if available.
    request_id: Option<String>,
    /// Additional fields from the error response that weren't explicitly mapped.
    additional_error_info: HashMap<String, String>,
}

impl StorageError {
    pub fn status_code(&self) -> azure_core::http::StatusCode {
        self.status_code
    }

    pub fn error_code(&self) -> Option<&StorageErrorCode> {
        self.error_code.as_ref()
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    pub fn additional_error_info(&self) -> &HashMap<String, String> {
        &self.additional_error_info
    }

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
    ) -> Result<Self, azure_core::Error> {
        #[derive(Deserialize)]
        #[serde(rename = "Error")]
        struct StorageErrorXml {
            #[serde(rename = "Code")]
            code: Option<String>,
            #[serde(rename = "Message")]
            message: Option<String>,
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

    fn try_from(error: azure_core::Error) -> Result<Self, Self::Error> {
        match error.kind() {
            ErrorKind::HttpResponse {
                status,
                error_code: _,
                raw_response,
            } => {
                let raw_response = raw_response.as_ref().ok_or_else(|| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        "Cannot convert to StorageError: raw_response is missing.",
                    )
                })?;

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
            _ => Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                "ErrorKind was not HttpResponse and could not be parsed.",
            )),
        }
    }
}
