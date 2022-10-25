use fe2o3_amqp::{connection::OpenError, link::SenderAttachError, session::BeginError};
use tokio::time::error::Elapsed;

use crate::{
    authorization::shared_access_signature::SasSignatureError,
    primitives::service_bus_connection_string_properties::FormatError,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Argument error: {}", .0)]
    ArgumentError(String),

    #[error(transparent)]
    FormatError(#[from] FormatError),

    #[error(transparent)]
    SasSignatureError(#[from] SasSignatureError),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    Open(#[from] OpenError),

    #[error(transparent)]
    WebSocket(#[from] fe2o3_amqp_ws::Error),

    #[error(transparent)]
    TimeoutElapsed(#[from] Elapsed),

    #[error(transparent)]
    Begin(#[from] BeginError),

    #[error(transparent)]
    SenderAttach(#[from] SenderAttachError),
}

impl From<crate::primitives::service_bus_connection::Error> for Error {
    fn from(error: crate::primitives::service_bus_connection::Error) -> Error {
        match error {
            crate::primitives::service_bus_connection::Error::ArgumentError(value) => {
                Error::ArgumentError(value)
            }
            crate::primitives::service_bus_connection::Error::FormatError(value) => {
                Error::FormatError(value)
            }
            crate::primitives::service_bus_connection::Error::SasSignatureError(value) => {
                Error::SasSignatureError(value)
            }
            crate::primitives::service_bus_connection::Error::UrlParseError(value) => {
                Error::UrlParseError(value)
            }
            crate::primitives::service_bus_connection::Error::Open(value) => Error::Open(value),
            crate::primitives::service_bus_connection::Error::WebSocket(value) => {
                Error::WebSocket(value)
            }
            crate::primitives::service_bus_connection::Error::TimeoutElapsed(value) => {
                Error::TimeoutElapsed(value)
            }
            crate::primitives::service_bus_connection::Error::Begin(value) => Error::Begin(value),
            crate::primitives::service_bus_connection::Error::SenderAttach(value) => {
                Error::SenderAttach(value)
            }
        }
    }
}
