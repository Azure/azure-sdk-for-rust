//! Error types for the service bus primitives.

use fe2o3_amqp::{connection::OpenError, link::SenderAttachError, session::BeginError};
use timer_kit::error::Elapsed;

use crate::{
    amqp::error::{AmqpClientError, DisposeError},
    authorization::shared_access_signature::SasSignatureError,
    util::IntoAzureCoreError,
};

use super::service_bus_connection_string_properties::FormatError;

/// Argument error
#[derive(Debug)]
pub struct ArgumentError(pub String);

impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Argument error: {}", self.0.as_str())
    }
}

impl std::error::Error for ArgumentError {}

impl From<ArgumentError> for azure_core::Error {
    fn from(value: ArgumentError) -> Self {
        azure_core::Error::new(azure_core::error::ErrorKind::Other, value)
    }
}

/// The client is already disposed
#[derive(Debug)]
pub struct ClientDisposedError;

impl std::fmt::Display for ClientDisposedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Client is disposed")
    }
}

impl std::error::Error for ClientDisposedError {}

impl From<ClientDisposedError> for azure_core::Error {
    fn from(value: ClientDisposedError) -> Self {
        azure_core::Error::new(azure_core::error::ErrorKind::Other, value)
    }
}

// TODO: split this into a few different error types
//
/// Error with service bus connection
#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    /// Argument error
    #[error("Argument error: {}", .0)]
    ArgumentError(#[from] ArgumentError),

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
    ClientDisposed(#[from] ClientDisposedError),
}

impl From<Error> for azure_core::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::ArgumentError(error) => error.into(),
            Error::FormatError(error) => error.into(),
            Error::SasSignatureError(error) => error.into(),
            Error::UrlParseError(error) => error.into(),
            Error::Open(error) => error.into_azure_core_error(),
            Error::WebSocket(error) => error.into_azure_core_error(),
            Error::Elapsed(error) => error.into_azure_core_error(),
            Error::Begin(error) => error.into_azure_core_error(),
            Error::SenderAttach(error) => error.into_azure_core_error(),
            Error::ReceiverAttach(error) => error.into_azure_core_error(),
            Error::Dispose(error) => error.into(),
            Error::ClientDisposed(error) => error.into(),
        }
    }
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
            AmqpClientError::ClientDisposed(err) => Self::ClientDisposed(err),
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

impl<E> From<RetryError<E>> for azure_core::Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(value: RetryError<E>) -> Self {
        azure_core::Error::new(azure_core::error::ErrorKind::Other, value)
    }
}
