//! Errors specific to identity services.
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(untagged)]
/// An unrecognized error response from an identity service.
pub enum ErrorResponse {
    #[error("Unrecognized Azure error response:\n{}\n", error_description)]
    GenericError { error_description: String },
}
