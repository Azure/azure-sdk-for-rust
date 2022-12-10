//! Error types for the service bus primitives.

use fe2o3_amqp::{connection::OpenError, session::BeginError, link::SenderAttachError};
use tokio::time::error::Elapsed;

use crate::{authorization::shared_access_signature::SasSignatureError, amqp::error::{DisposeError, AmqpClientError}};

use super::service_bus_connection_string_properties::FormatError;

// TODO: split this into a few different error types
//
/// Error with service bus connection
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Argument error
    #[error("Argument error: {}", .0)]
    ArgumentError(String),

    /// Error with the connection string
    #[error(transparent)]
    FormatError(#[from] FormatError),

    /// Error with the SAS signature
    #[error(transparent)]
    SasSignatureError(#[from] SasSignatureError),

    /// Error parsing url from connection string
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    /// Error opening the connection
    #[error(transparent)]
    Open(#[from] OpenError),

    /// Error opening the connection over websockets
    #[error(transparent)]
    WebSocket(#[from] fe2o3_amqp_ws::Error),

    /// Opening the connection timed out
    #[error(transparent)]
    TimeoutElapsed(#[from] Elapsed),

    /// Error beginning the AMQP session
    #[error(transparent)]
    Begin(#[from] BeginError),

    /// Error attaching the sender
    #[error(transparent)]
    SenderAttach(#[from] SenderAttachError),

    /// Error attaching the receiver
    #[error(transparent)]
    ReceiverAttach(#[from] fe2o3_amqp::link::ReceiverAttachError),

    /// Error disposing the connection
    #[error(transparent)]
    Dispose(#[from] DisposeError),
}

impl From<AmqpClientError> for Error {
    fn from(err: AmqpClientError) -> Self {
        match err {
            AmqpClientError::UrlParseError(err) => Self::UrlParseError(err),
            AmqpClientError::Open(err) => Self::Open(err),
            AmqpClientError::WebSocket(err) => Self::WebSocket(err),
            AmqpClientError::TimeoutElapsed(err) => Self::TimeoutElapsed(err),
            AmqpClientError::Begin(err) => Self::Begin(err),
            AmqpClientError::SenderAttach(err) => Self::SenderAttach(err),
            AmqpClientError::Dispose(err) => Self::Dispose(err),
            AmqpClientError::ReceiverAttach(err) => Self::ReceiverAttach(err),
        }
    }
}
