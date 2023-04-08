//! Error types for AMQP operations

use std::marker::PhantomData;

use fe2o3_amqp::{
    connection::{self, OpenError},
    session::{self, BeginError}, link::{SenderAttachError, ReceiverAttachError, DetachError},
};
use fe2o3_amqp_management::error::Error as ManagementError;

use crate::{consumer::error::OffsetIsEmpty, util::IntoAzureCoreError};

impl IntoAzureCoreError for ManagementError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            ManagementError::Send(_)
            | ManagementError::Recv(_) => azure_core::Error::new(ErrorKind::Io, self),
            _ => azure_core::Error::new(ErrorKind::Other, self),
        }
    }
}

/// The value exceeds the maximum length allowed
#[derive(Debug)]
pub struct MaxLengthExceededError {
    pub(crate) message: String,
}

impl MaxLengthExceededError {
    pub(crate) fn new(actual_length: usize, max_length: usize) -> Self {
        Self {
            message: format!(
                "The actual length {} exceeds the maximum length {}",
                actual_length, max_length
            ),
        }
    }
}

impl std::fmt::Display for MaxLengthExceededError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaxLengthExceededError: {}", self.message)
    }
}

impl std::error::Error for MaxLengthExceededError {}

/// Error setting the message ID
#[derive(Debug, thiserror::Error)]
pub enum SetMessageIdError {
    /// Value cannot be empty
    #[error("Value cannot be empty")]
    Empty,

    /// Max allowed length exceeded
    #[error(transparent)]
    MaxLengthExceeded(#[from] MaxLengthExceededError),
}

///
#[derive(Debug)]
pub struct MaxAllowedTtlExceededError {}

impl std::fmt::Display for MaxAllowedTtlExceededError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MaxAllowedTtlExceededError: The maximum allowed TTL is u32::MAX milliseconds"
        )
    }
}

impl std::error::Error for MaxAllowedTtlExceededError {}

/// The message carried in `ServiceBusReceivedMessage` or `ServiceBusPeekedMessage` is a raw AMQP
/// message. Please use [`ServiceBusReceivedMessage::raw_amqp_message`] or
/// [`ServiceBusPeekedMessage::raw_amqp_message`] to access the raw AMQP message body.
#[derive(Debug, Clone)]
pub struct RawAmqpMessageError {}

impl std::fmt::Display for RawAmqpMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawAmqpMessageError: The message is a raw AMQP message")
    }
}

impl std::error::Error for RawAmqpMessageError {}

#[derive(Debug, thiserror::Error)]
pub(crate) enum AmqpConnectionScopeError {
    #[error(transparent)]
    Parse(#[from] url::ParseError),

    #[error(transparent)]
    Open(#[from] OpenError),

    #[error(transparent)]
    WebSocket(#[from] fe2o3_amqp_ws::Error),

    #[error(transparent)]
    Begin(#[from] BeginError),

    #[error(transparent)]
    SenderAttach(#[from] SenderAttachError),

    #[error(transparent)]
    ReceiverAttach(#[from] ReceiverAttachError),

    #[error("The connection scope is disposed")]
    ScopeDisposed,
}

impl IntoAzureCoreError for AmqpConnectionScopeError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            AmqpConnectionScopeError::Open(err) => err.into_azure_core_error(),
            AmqpConnectionScopeError::WebSocket(err) => err.into_azure_core_error(),
            AmqpConnectionScopeError::Begin(err) => err.into_azure_core_error(),
            AmqpConnectionScopeError::SenderAttach(err) => err.into_azure_core_error(),
            AmqpConnectionScopeError::ReceiverAttach(err) => err.into_azure_core_error(),
            AmqpConnectionScopeError::ScopeDisposed => azure_core::Error::new(ErrorKind::Io, self),
            AmqpConnectionScopeError::Parse(err) => err.into(),
        }
    }
}

/// The CBS event loop has stopped
#[derive(Debug)]
pub(crate) struct AmqpCbsEventLoopStopped {}

impl std::fmt::Display for AmqpCbsEventLoopStopped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The CBS event loop has stopped")
    }
}

impl std::error::Error for AmqpCbsEventLoopStopped {}

/// Error with CBS auth
#[derive(Debug, thiserror::Error)]
pub enum CbsAuthError {
    /// Error with the token provider
    #[error(transparent)]
    TokenCredential(#[from] azure_core::Error),

    /// Error with the CBS link
    #[error(transparent)]
    Cbs(#[from] ManagementError),
}

impl IntoAzureCoreError for CbsAuthError {
    fn into_azure_core_error(self) -> azure_core::Error {
        match self {
            CbsAuthError::TokenCredential(err) => err,
            CbsAuthError::Cbs(err) => err.into_azure_core_error(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OpenMgmtLinkError {
    #[error("The connection scope is disposed")]
    ConnectionScopeDisposed,

    #[error(transparent)]
    Session(#[from] BeginError),

    #[error(transparent)]
    Link(#[from] fe2o3_amqp_management::error::AttachError),
}

impl IntoAzureCoreError for OpenMgmtLinkError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            OpenMgmtLinkError::ConnectionScopeDisposed => azure_core::Error::new(ErrorKind::Io, self),
            OpenMgmtLinkError::Session(_) => azure_core::Error::new(ErrorKind::Other, self),
            OpenMgmtLinkError::Link(_) => azure_core::Error::new(ErrorKind::Other, self),
        }
    }
}

/// Error opening a producer
#[derive(Debug, thiserror::Error)]
pub enum OpenProducerError {
    #[error(transparent)]
    ParseEndpoint(#[from] url::ParseError),

    #[error("The connection scope is disposed")]
    ConnectionScopeDisposed,

    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),

    #[error(transparent)]
    Session(#[from] BeginError),

    #[error(transparent)]
    SenderLink(#[from] SenderAttachError),

    #[error(transparent)]
    Elapsed(#[from] timer_kit::error::Elapsed),
}

impl IntoAzureCoreError for OpenProducerError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            OpenProducerError::ParseEndpoint(err) => err.into(),
            OpenProducerError::ConnectionScopeDisposed => azure_core::Error::new(ErrorKind::Io, self),
            OpenProducerError::CbsAuth(err) => err.into_azure_core_error(),
            OpenProducerError::Session(err) => err.into_azure_core_error(),
            OpenProducerError::SenderLink(err) => err.into_azure_core_error(),
            OpenProducerError::Elapsed(err) => err.into_azure_core_error(),
        }
    }
}

/// Error opening a consumer
#[derive(Debug, thiserror::Error)]
pub enum OpenConsumerError {
    #[error(transparent)]
    ParseEndpoint(#[from] url::ParseError),

    #[error("The connection scope is disposed")]
    ConnectionScopeDisposed,

    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),

    #[error(transparent)]
    Session(#[from] BeginError),

    #[error(transparent)]
    ReceiverLink(#[from] ReceiverAttachError),

    #[error(transparent)]
    ConsumerFilter(#[from] OffsetIsEmpty),

    #[error(transparent)]
    Elapsed(#[from] timer_kit::error::Elapsed),
}

/// Error closing the AMQP connection and AMQP session
#[derive(Debug, thiserror::Error)]
pub enum DisposeError {
    /// Error closing the AMQP session
    #[error(transparent)]
    SessionCloseError(#[from] session::Error),

    /// Error closing the AMQP connection
    #[error(transparent)]
    ConnectionCloseError(#[from] connection::Error),
}

impl IntoAzureCoreError for DisposeError {
    fn into_azure_core_error(self) -> azure_core::Error {
        match self {
            DisposeError::SessionCloseError(err) => err.into_azure_core_error(),
            DisposeError::ConnectionCloseError(err) => err.into_azure_core_error(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DisposeProducerError {
    #[error(transparent)]
    Sender(#[from] DetachError),

    #[error(transparent)]
    Session(#[from] fe2o3_amqp::session::Error),
}

impl IntoAzureCoreError for DisposeProducerError {
    fn into_azure_core_error(self) -> azure_core::Error {
        match self {
            DisposeProducerError::Sender(err) => err.into_azure_core_error(),
            DisposeProducerError::Session(err) => err.into_azure_core_error(),
        }
    }
}
