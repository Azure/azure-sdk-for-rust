//! Error types for AMQP operations

use fe2o3_amqp::{
    connection::{self},
    session::{self},
};

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

/// Error setting the partition key
#[derive(Debug, thiserror::Error)]
pub enum SetPartitionKeyError {
    /// Max allowed length exceeded
    #[error(transparent)]
    MaxLengthExceeded(#[from] MaxLengthExceededError),

    /// PartitionKey cannot be set to a different value from SessionId
    #[error("PartitionKey cannot be set to a different value from SessionId")]
    PartitionKeyAndSessionIdAreDifferent,
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

/// The requested message batch size is out of range
#[derive(Debug)]
pub struct RequestedSizeOutOfRange {}

impl std::fmt::Display for RequestedSizeOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Requested size is out of range")
    }
}

impl std::error::Error for RequestedSizeOutOfRange {}

/// The correlation rule filter must have at least one non-empty entry
#[derive(Debug, Clone, thiserror::Error)]
pub enum CorrelationFilterError {
    /// The correlation filter must have at least one non-empty entry
    #[error("Correlation filter must include at least one entry")]
    EmptyFilter,
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
