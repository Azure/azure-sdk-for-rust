//! Error types for AMQP operations

use std::marker::PhantomData;

use fe2o3_amqp::{
    connection::{self, OpenError},
    session::{self, BeginError}, link::{SenderAttachError, ReceiverAttachError},
};
use fe2o3_amqp_management::error::Error as ManagementError;

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
    Open(#[from] OpenError),

    #[error(transparent)]
    WebSocket(#[from] fe2o3_amqp_ws::Error),

    #[error("Timeout elapsed")]
    Elapsed,

    #[error(transparent)]
    Begin(#[from] BeginError),

    #[error(transparent)]
    SenderAttach(#[from] SenderAttachError),

    #[error(transparent)]
    ReceiverAttach(#[from] ReceiverAttachError),

    #[error("The connection scope is disposed")]
    ScopeDisposed,
}

impl From<timer_kit::error::Elapsed> for AmqpConnectionScopeError {
    fn from(_: timer_kit::error::Elapsed) -> Self {
        Self::Elapsed
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

/// Error opening a producer
#[derive(Debug, thiserror::Error)]
pub enum OpenProducerError {
    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),

    #[error(transparent)]
    Session(#[from] BeginError),

    #[error(transparent)]
    SenderLink(#[from] SenderAttachError),
}

