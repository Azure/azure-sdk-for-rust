//! Error types for the service bus primitives.

use fe2o3_amqp::{connection::OpenError, session::BeginError, link::SenderAttachError};
use tokio::time::error::Elapsed;

use crate::{authorization::shared_access_signature::SasSignatureError, amqp::error::{DisposeError, AmqpClientError}};

use super::service_bus_connection_string_properties::FormatError;

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

    #[error(transparent)]
    ReceiverAttach(#[from] fe2o3_amqp::link::ReceiverAttachError),

    #[error(transparent)]
    Rng(#[from] rand::Error),

    #[error("Cancelled")]
    Cancelled,

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
            AmqpClientError::Rng(err) => Self::Rng(err),
            AmqpClientError::Cancelled => Self::Cancelled,
            AmqpClientError::Dispose(err) => Self::Dispose(err),
            AmqpClientError::ReceiverAttach(err) => Self::ReceiverAttach(err),
        }
    }
}
