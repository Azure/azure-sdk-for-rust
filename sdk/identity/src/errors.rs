//! Errors specific to identity services.
use serde::Deserialize;
use std::fmt;

/// Errors specific to identity services
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error requesting token: {0}")]
    Token(ErrorToken),
}

impl From<Error> for azure_core::error::Error {
    fn from(error: Error) -> Self {
        Self::new(azure_core::error::ErrorKind::Credential, error)
    }
}

/// Error Token
#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct ErrorToken {
    error: String,
    error_description: String,
    error_codes: Vec<i64>,
    timestamp: Option<String>,
    trace_id: Option<String>,
    correlation_id: Option<String>,
    suberror: Option<String>,
    claims: Option<String>,
}

impl fmt::Display for ErrorToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "error: {}", self.error)?;
        if let Some(suberror) = &self.suberror {
            writeln!(f, "suberror: {}", suberror)?;
        }
        writeln!(f, "description: {}", self.error_description)
    }
}
