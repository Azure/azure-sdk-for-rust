// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Error types for the query plan interop client.

use crate::query_plan_native::native::HResult;

/// Errors that can occur when interacting with the QueryPlanInterop library.
#[derive(Debug)]
pub enum QueryPlanError {
    /// The native library returned a failure HRESULT with no diagnostic payload.
    Unexpected {
        /// The raw HRESULT value.
        hresult: u32,
    },

    /// The native library returned a failure HRESULT together with a JSON
    /// diagnostic payload (e.g. a query-syntax error message).
    Expected {
        /// The raw HRESULT value.
        #[allow(dead_code)]
        hresult: u32,
        /// The UTF-8 JSON payload returned by the native library.
        message: String,
    },

    /// The JSON returned by the native library could not be deserialized
    /// into the expected Rust model.
    Deserialization { source: serde_json::Error },

    /// The native library could not be loaded (DLL/so not found on PATH).
    LibraryNotAvailable {
        /// The loading error message.
        message: String,
    },

    /// A supplied configuration string contained an interior null character.
    ConfigContainsNull,

    /// The native library returned invalid UTF-8 in its output.
    InvalidUtf8 {
        /// The UTF-8 conversion error details.
        message: String,
    },
}

impl std::fmt::Display for QueryPlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unexpected { hresult } => {
                write!(f, "query plan interop failed with HRESULT 0x{hresult:08X}")
            }
            Self::Expected { message, .. } => {
                write!(f, "query plan error: {message}")
            }
            Self::Deserialization { source } => {
                write!(f, "failed to deserialize query plan: {source}")
            }
            Self::LibraryNotAvailable { message } => {
                write!(f, "native query plan library not available: {message}")
            }
            Self::ConfigContainsNull => {
                write!(f, "configuration string contains interior null character")
            }
            Self::InvalidUtf8 { message } => {
                write!(f, "native library returned invalid UTF-8: {message}")
            }
        }
    }
}

impl std::error::Error for QueryPlanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Deserialization { source } => Some(source),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for QueryPlanError {
    fn from(source: serde_json::Error) -> Self {
        Self::Deserialization { source }
    }
}

impl QueryPlanError {
    /// Creates an [`Unexpected`](QueryPlanError::Unexpected) error from a raw HRESULT.
    pub(crate) fn from_hresult(hr: HResult) -> Self {
        Self::Unexpected { hresult: hr as u32 }
    }

    /// Creates an [`Expected`](QueryPlanError::Expected) error from a raw HRESULT
    /// and the diagnostic JSON payload.
    pub(crate) fn from_hresult_with_payload(hr: HResult, payload: String) -> Self {
        Self::Expected {
            hresult: hr as u32,
            message: payload,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unexpected_error_display() {
        let err = QueryPlanError::from_hresult(-2147467259); // E_FAIL
        assert!(format!("{err}").contains("HRESULT"));
    }

    #[test]
    fn expected_error_display() {
        let err = QueryPlanError::from_hresult_with_payload(-1, "syntax error".to_string());
        let msg = format!("{err}");
        assert!(msg.contains("syntax error"));
    }

    #[test]
    fn deserialization_error_from_serde() {
        let err: QueryPlanError =
            serde_json::from_str::<crate::driver::dataflow::query_plan::QueryPlan>("invalid")
                .unwrap_err()
                .into();
        assert!(matches!(err, QueryPlanError::Deserialization { .. }));
    }

    #[test]
    fn config_null_error() {
        let err = QueryPlanError::ConfigContainsNull;
        assert!(format!("{err}").contains("null"));
    }
}
