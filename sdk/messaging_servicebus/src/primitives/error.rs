//! Error types for the service bus primitives.

use fe2o3_amqp::{connection::OpenError, link::SenderAttachError, session::BeginError};
use tokio::time::error::Elapsed;

use crate::{
    amqp::error::{AmqpClientError, DisposeError},
    authorization::shared_access_signature::SasSignatureError,
};

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

    /// Error opening the connection over websocket
    #[error(transparent)]
    WebSocket(#[from] fe2o3_amqp_ws::Error),

    /// Opening the connection timed out
    #[error(transparent)]
    Elapsed(#[from] Elapsed),

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

    /// Client is disposed
    #[error("Client is disposed")]
    ClientDisposed,
}

impl From<AmqpClientError> for Error {
    fn from(err: AmqpClientError) -> Self {
        match err {
            AmqpClientError::UrlParseError(err) => Self::UrlParseError(err),
            AmqpClientError::Open(err) => Self::Open(err),
            AmqpClientError::WebSocket(err) => Self::WebSocket(err),
            AmqpClientError::Elapsed(err) => Self::Elapsed(err),
            AmqpClientError::Begin(err) => Self::Begin(err),
            AmqpClientError::SenderAttach(err) => Self::SenderAttach(err),
            AmqpClientError::Dispose(err) => Self::Dispose(err),
            AmqpClientError::ReceiverAttach(err) => Self::ReceiverAttach(err),
            AmqpClientError::ClientDisposed => Self::ClientDisposed,
        }
    }
}

/// Service bus retry policy error
#[derive(Debug, thiserror::Error)]
pub enum RetryError<E> {
    /// Retry policy exhausted
    #[error("Retry policy exhausted")]
    ServiceBusy,

    /// Error with the operation
    #[error(transparent)]
    Operation(E),
}
