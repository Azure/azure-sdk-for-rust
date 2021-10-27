#![deny(missing_docs)]
//! The IoT Hub crate contains a client that can be used to manage the IoT Hub.

/// The service module contains the IoT Hub Service Client that can be used to manage the IoT Hub.
pub mod service;

/// A specialized Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// A general error having to do with the IoTHub.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("etag is not set")]
    EtagNotSet,

    #[error("From connection string error: {0}")]
    FromConnectionStringError(#[from] service::FromConnectionStringError),
    #[error("Generate SAS token error: {0}")]
    GenerateSasTokenError(#[from] service::GenerateSasTokenError),

    #[error("core error: {0}")]
    CoreError(#[from] azure_core::Error),
    #[error("core http error: {0}")]
    CoreHttpError(#[from] azure_core::HttpError),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("http error: {0}")]
    HttpError(#[from] http::Error),
}
