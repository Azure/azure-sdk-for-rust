//! Error types for AMQP operations

use std::marker::PhantomData;

use fe2o3_amqp::{
    connection::{self, OpenError},
    link::{DetachError, ReceiverAttachError, SenderAttachError, DetachThenResumeSenderError, SenderResumeErrorKind, ReceiverResumeErrorKind, DetachThenResumeReceiverError},
    session::{self, BeginError},
};
use fe2o3_amqp_management::error::Error as ManagementError;
use fe2o3_amqp_types::messaging::{Modified, Rejected, Released};
use timer_kit::error::Elapsed;

use crate::{consumer::error::OffsetIsEmpty, util::IntoAzureCoreError, Event, core::RecoverableError};

impl IntoAzureCoreError for ManagementError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            ManagementError::Send(_) | ManagementError::Recv(_) => {
                azure_core::Error::new(ErrorKind::Io, self)
            }
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
pub enum AmqpConnectionScopeError {
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

impl RecoverableError for AmqpConnectionScopeError {
    fn should_try_recover(&self) -> bool {
        // All variants indicate some sort of rejection from the server or network
        false
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, AmqpConnectionScopeError::ScopeDisposed)
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

impl RecoverableError for OpenMgmtLinkError {
    fn should_try_recover(&self) -> bool {
        false
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, OpenMgmtLinkError::ConnectionScopeDisposed)
    }
}

impl IntoAzureCoreError for OpenMgmtLinkError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            OpenMgmtLinkError::ConnectionScopeDisposed => {
                azure_core::Error::new(ErrorKind::Io, self)
            }
            OpenMgmtLinkError::Session(_) => azure_core::Error::new(ErrorKind::Other, self),
            OpenMgmtLinkError::Link(_) => azure_core::Error::new(ErrorKind::Other, self),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AmqpClientError {
    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),

    #[error("Cannot set url scheme")]
    SetUrlScheme,

    #[error(transparent)]
    ConnectionScope(#[from] AmqpConnectionScopeError),

    #[error(transparent)]
    ManagementLink(#[from] OpenMgmtLinkError),
}

impl IntoAzureCoreError for AmqpClientError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            AmqpClientError::ParseUrl(err) => err.into(),
            AmqpClientError::ConnectionScope(err) => err.into_azure_core_error(),
            AmqpClientError::ManagementLink(err) => err.into_azure_core_error(),
            AmqpClientError::SetUrlScheme => azure_core::Error::new(ErrorKind::Other, self),
        }
    }
}

impl RecoverableError for AmqpClientError {
    fn should_try_recover(&self) -> bool {
        match self {
            AmqpClientError::ConnectionScope(err) => err.should_try_recover(),
            AmqpClientError::ParseUrl(_) => false,
            AmqpClientError::SetUrlScheme => false,
            AmqpClientError::ManagementLink(err) => err.should_try_recover(),
        }
    }

    fn is_scope_disposed(&self) -> bool {
        todo!()
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
            OpenProducerError::ConnectionScopeDisposed => {
                azure_core::Error::new(ErrorKind::Io, self)
            }
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

/// Error with adding an event to a batch
#[derive(Debug, thiserror::Error)]
pub enum TryAddError {
    /// The message is too large to fit in a batch
    #[error("Message is too large to fit in a batch")]
    BatchFull(Event),

    /// The message cannot be serialized
    #[error("Cannot serialize message")]
    Codec {
        /// The error from the codec
        source: serde_amqp::Error,
        /// The message that could not be serialized
        event: Event,
    },
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

impl IntoAzureCoreError for RequestedSizeOutOfRange {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        azure_core::Error::new(ErrorKind::Other, self)
    }
}

impl From<RequestedSizeOutOfRange> for azure_core::Error {
    fn from(err: RequestedSizeOutOfRange) -> Self {
        err.into_azure_core_error()
    }
}

/// The sent message is not accepted by the service
#[derive(Debug, thiserror::Error)]
pub enum NotAcceptedError {
    /// 3.4.2 Rejected
    #[error("Rejceted: {:?}", .0)]
    Rejected(Rejected),

    /// 3.4.4 Released
    #[error("Released: {:?}", .0)]
    Released(Released),

    /// 3.4.5 Modified
    #[error("Modified: {:?}", .0)]
    Modified(Modified),
}

impl IntoAzureCoreError for NotAcceptedError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        azure_core::Error::new(ErrorKind::Other, self)
    }
}

/// Error sending message to the service
#[derive(Debug, thiserror::Error)]
pub enum AmqpSendError {
    /// Error with sending the message
    #[error(transparent)]
    Send(#[from] fe2o3_amqp::link::SendError),

    /// The sent message is not accepted by the service
    #[error(transparent)]
    NotAccepted(#[from] NotAcceptedError),

    /// The operation timed out
    #[error(transparent)]
    Elapsed(#[from] Elapsed),

    /// Connection scope is disposed
    #[error("Connection scope is disposed")]
    ConnectionScopeDisposed,
}

impl IntoAzureCoreError for AmqpSendError {
    fn into_azure_core_error(self) -> azure_core::Error {
        match self {
            AmqpSendError::Send(err) => err.into_azure_core_error(),
            AmqpSendError::NotAccepted(err) => err.into_azure_core_error(),
            AmqpSendError::Elapsed(err) => err.into_azure_core_error(),
            AmqpSendError::ConnectionScopeDisposed => {
                azure_core::Error::new(azure_core::error::ErrorKind::Other, self)
            }
        }
    }
}

impl RecoverableError for AmqpSendError {
    fn should_try_recover(&self) -> bool {
        match self {
            AmqpSendError::Send(_) => true,
            AmqpSendError::NotAccepted(_) => false,
            AmqpSendError::Elapsed(_) => false,
            AmqpSendError::ConnectionScopeDisposed => false,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, AmqpSendError::ConnectionScopeDisposed)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecoverProducerError {
    #[error(transparent)]
    SessionBegin(#[from] BeginError),

    #[error(transparent)]
    SenderDetach(#[from] DetachError),

    /// Error with resuming the sender
    #[error(transparent)]
    SenderResume(#[from] SenderResumeErrorKind),

    #[error("Connection scope is disposed")]
    ConnectionScopeDisposed,
}

impl From<DetachThenResumeSenderError> for RecoverProducerError {
    fn from(err: DetachThenResumeSenderError) -> Self {
        match err {
            DetachThenResumeSenderError::Detach(err) => err.into(),
            DetachThenResumeSenderError::Resume(err) => err.into(),
        }
    }
}

impl RecoverableError for RecoverProducerError {
    fn should_try_recover(&self) -> bool {
        false
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, RecoverProducerError::ConnectionScopeDisposed)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecoverConsumeError {
    #[error(transparent)]
    SessionBegin(#[from] BeginError),

    #[error(transparent)]
    ReceiverDetach(#[from] DetachError),

    /// Error with resuming the sender
    #[error(transparent)]
    ReceiverResume(#[from] ReceiverResumeErrorKind),

    #[error("Connection scope is disposed")]
    ConnectionScopeDisposed,
}

impl From<DetachThenResumeReceiverError> for RecoverConsumeError {
    fn from(err: DetachThenResumeReceiverError) -> Self {
        match err {
            DetachThenResumeReceiverError::Detach(err) => err.into(),
            DetachThenResumeReceiverError::Resume(err) => err.into(),
        }
    }
}
