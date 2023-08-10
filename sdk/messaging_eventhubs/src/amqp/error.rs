//! Error types for AMQP operations

use fe2o3_amqp::{
    connection::{self, OpenError},
    link::{
        DetachError, DetachThenResumeReceiverError, DetachThenResumeSenderError, DispositionError,
        ReceiverAttachError, ReceiverResumeErrorKind, RecvError, SenderAttachError,
        SenderResumeErrorKind,
    },
    session::{self, BeginError},
};
use fe2o3_amqp_management::error::{DetachThenResumeError, Error as ManagementError};
use fe2o3_amqp_types::messaging::{Modified, Rejected, Released};
use timer_kit::error::Elapsed;

use crate::{
    consumer::error::OffsetIsEmpty, core::RecoverableError, util::IntoAzureCoreError, EventData,
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

/// The message carried in `ReceivedEvent` is a raw AMQP
/// message. Please use [`ReceivedEvent::raw_amqp_message`] or
/// [`ReceivedEvent::raw_amqp_message`] to access the raw AMQP message body.
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

impl From<AmqpConnectionScopeError> for azure_core::Error {
    fn from(err: AmqpConnectionScopeError) -> Self {
        use azure_core::error::ErrorKind;

        match err {
            AmqpConnectionScopeError::Open(err) => err.into_azure_core_error(),
            AmqpConnectionScopeError::WebSocket(err) => err.into_azure_core_error(),
            AmqpConnectionScopeError::Begin(err) => err.into_azure_core_error(),
            AmqpConnectionScopeError::SenderAttach(err) => err.into_azure_core_error(),
            AmqpConnectionScopeError::ReceiverAttach(err) => err.into_azure_core_error(),
            AmqpConnectionScopeError::ScopeDisposed => azure_core::Error::new(ErrorKind::Io, err),
            AmqpConnectionScopeError::Parse(err) => err.into(),
        }
    }
}

impl RecoverableError for AmqpConnectionScopeError {
    fn should_try_recover(&self) -> bool {
        // All variants indicate some sort of rejection from the server or network
        match self {
            AmqpConnectionScopeError::Open(_) => false,
            AmqpConnectionScopeError::WebSocket(_) => false,
            AmqpConnectionScopeError::Begin(_) => true,
            AmqpConnectionScopeError::SenderAttach(_) => true,
            AmqpConnectionScopeError::ReceiverAttach(_) => true,
            AmqpConnectionScopeError::ScopeDisposed => false,
            AmqpConnectionScopeError::Parse(_) => false,
        }
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

impl From<CbsAuthError> for azure_core::Error {
    fn from(err: CbsAuthError) -> Self {
        match err {
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
        match self {
            OpenMgmtLinkError::ConnectionScopeDisposed => false,
            OpenMgmtLinkError::Session(_) => true,
            OpenMgmtLinkError::Link(_) => true,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, OpenMgmtLinkError::ConnectionScopeDisposed)
    }
}

impl From<OpenMgmtLinkError> for azure_core::Error {
    fn from(err: OpenMgmtLinkError) -> Self {
        use azure_core::error::ErrorKind;

        match err {
            OpenMgmtLinkError::ConnectionScopeDisposed => {
                azure_core::Error::new(ErrorKind::Io, err)
            }
            OpenMgmtLinkError::Session(_) => azure_core::Error::new(ErrorKind::Other, err),
            OpenMgmtLinkError::Link(_) => azure_core::Error::new(ErrorKind::Other, err),
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

impl From<AmqpClientError> for azure_core::Error {
    fn from(err: AmqpClientError) -> Self {
        use azure_core::error::ErrorKind;

        match err {
            AmqpClientError::ParseUrl(err) => err.into(),
            AmqpClientError::ConnectionScope(err) => err.into(),
            AmqpClientError::ManagementLink(err) => err.into(),
            AmqpClientError::SetUrlScheme => azure_core::Error::new(ErrorKind::Other, err),
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
        match self {
            AmqpClientError::ConnectionScope(err) => err.is_scope_disposed(),
            AmqpClientError::ParseUrl(_) => false,
            AmqpClientError::SetUrlScheme => false,
            AmqpClientError::ManagementLink(err) => err.is_scope_disposed(),
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
    Elapsed(#[from] Elapsed),
}

impl From<OpenProducerError> for azure_core::Error {
    fn from(err: OpenProducerError) -> Self {
        use azure_core::error::ErrorKind;

        match err {
            OpenProducerError::ParseEndpoint(err) => err.into(),
            OpenProducerError::ConnectionScopeDisposed => azure_core::Error::new(ErrorKind::Io, err),
            OpenProducerError::CbsAuth(err) => err.into(),
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
    Elapsed(#[from] Elapsed),
}

impl From<OpenConsumerError> for azure_core::Error {
    fn from(err: OpenConsumerError) -> Self {
        use azure_core::error::ErrorKind;

        match err {
            OpenConsumerError::ParseEndpoint(err) => err.into(),
            OpenConsumerError::ConnectionScopeDisposed => azure_core::Error::new(ErrorKind::Io, err),
            OpenConsumerError::CbsAuth(err) => err.into(),
            OpenConsumerError::Session(err) => err.into_azure_core_error(),
            OpenConsumerError::ReceiverLink(err) => err.into_azure_core_error(),
            OpenConsumerError::ConsumerFilter(err) => err.into(),
            OpenConsumerError::Elapsed(err) => err.into_azure_core_error(),
        }
    }
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

impl From<DisposeError> for azure_core::Error {
    fn from(err: DisposeError) -> Self {
        match err {
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

impl From<DisposeProducerError> for azure_core::Error {
    fn from(err: DisposeProducerError) -> Self {
        match err {
            DisposeProducerError::Sender(err) => err.into_azure_core_error(),
            DisposeProducerError::Session(err) => err.into_azure_core_error(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DisposeConsumerError {
    #[error(transparent)]
    Receiver(#[from] DetachError),

    #[error(transparent)]
    Session(#[from] fe2o3_amqp::session::Error),
}

impl From<DisposeConsumerError> for azure_core::Error {
    fn from(err: DisposeConsumerError) -> Self {
        match err {
            DisposeConsumerError::Receiver(err) => err.into_azure_core_error(),
            DisposeConsumerError::Session(err) => err.into_azure_core_error(),
        }
    }
}

/// Error with adding an event to a batch
#[derive(Debug, thiserror::Error)]
pub enum TryAddError {
    /// The message is too large to fit in a batch
    #[error("Message is too large to fit in a batch")]
    BatchFull(EventData),

    /// The message cannot be serialized
    #[error("Cannot serialize message")]
    Codec {
        /// The error from the codec
        source: serde_amqp::Error,
        /// The message that could not be serialized
        event: EventData,
    },
}

impl From<TryAddError> for azure_core::Error {
    fn from(err: TryAddError) -> Self {
        match err {
            TryAddError::BatchFull(_) => {
                azure_core::Error::new(azure_core::error::ErrorKind::Other, err)
            }
            TryAddError::Codec { source, .. } => source.into_azure_core_error(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateBatchError {
    /// The requested message batch size is out of range
    #[error("Requested size is out of range")]
    RequestedSizeOutOfRange,

    /// The sent message is not accepted by the service
    #[error(transparent)]
    Codec(#[from] serde_amqp::Error),
}

impl From<CreateBatchError> for azure_core::Error {
    fn from(err: CreateBatchError) -> Self {
        match err {
            CreateBatchError::RequestedSizeOutOfRange => {
                azure_core::Error::new(azure_core::error::ErrorKind::Other, err)
            }
            CreateBatchError::Codec(err) => err.into_azure_core_error(),
        }
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

impl From<NotAcceptedError> for azure_core::Error {
    fn from(err: NotAcceptedError) -> Self {
        use azure_core::error::ErrorKind;

        azure_core::Error::new(ErrorKind::Other, err)
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
}

impl From<AmqpSendError> for azure_core::Error {
    fn from(err: AmqpSendError) -> azure_core::Error {
        match err {
            AmqpSendError::Send(err) => err.into_azure_core_error(),
            AmqpSendError::NotAccepted(err) => err.into(),
            AmqpSendError::Elapsed(err) => err.into_azure_core_error(),
        }
    }
}

impl RecoverableError for AmqpSendError {
    fn should_try_recover(&self) -> bool {
        match self {
            AmqpSendError::Send(_) => true,
            AmqpSendError::NotAccepted(_) => false,
            AmqpSendError::Elapsed(_) => true,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        false
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecoverProducerError {
    #[error(transparent)]
    ParseEndpoint(#[from] url::ParseError),

    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),

    #[error(transparent)]
    SessionBegin(#[from] BeginError),

    #[error(transparent)]
    SenderAttach(#[from] SenderAttachError),

    #[error(transparent)]
    SenderDetach(#[from] DetachError),

    /// Error with resuming the sender
    #[error(transparent)]
    SenderResume(#[from] SenderResumeErrorKind),

    #[error("Connection scope is disposed")]
    ConnectionScopeDisposed,

    #[error(transparent)]
    Elapsed(#[from] Elapsed),
}

impl From<DetachThenResumeSenderError> for RecoverProducerError {
    fn from(err: DetachThenResumeSenderError) -> Self {
        match err {
            DetachThenResumeSenderError::Detach(err) => err.into(),
            DetachThenResumeSenderError::Resume(err) => err.into(),
        }
    }
}

impl From<OpenProducerError> for RecoverProducerError {
    fn from(value: OpenProducerError) -> Self {
        match value {
            OpenProducerError::ParseEndpoint(err) => err.into(),
            OpenProducerError::ConnectionScopeDisposed => Self::ConnectionScopeDisposed,
            OpenProducerError::CbsAuth(err) => err.into(),
            OpenProducerError::Session(err) => err.into(),
            OpenProducerError::SenderLink(err) => err.into(),
            OpenProducerError::Elapsed(err) => err.into(),
        }
    }
}

impl RecoverableError for RecoverProducerError {
    fn should_try_recover(&self) -> bool {
        match self {
            RecoverProducerError::SessionBegin(_) => true,
            RecoverProducerError::SenderDetach(_) => true,
            RecoverProducerError::SenderResume(_) => true,
            RecoverProducerError::ConnectionScopeDisposed => false,
            RecoverProducerError::ParseEndpoint(_) => false,
            RecoverProducerError::CbsAuth(_) => true,
            RecoverProducerError::SenderAttach(_) => true,
            RecoverProducerError::Elapsed(_) => true,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, RecoverProducerError::ConnectionScopeDisposed)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecoverAndSendError {
    #[error(transparent)]
    ParseEndpoint(#[from] url::ParseError),

    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),

    #[error(transparent)]
    SessionBegin(#[from] BeginError),

    #[error(transparent)]
    SenderAttach(#[from] SenderAttachError),

    #[error(transparent)]
    SenderDetach(#[from] DetachError),

    /// Error with resuming the sender
    #[error(transparent)]
    SenderResume(#[from] SenderResumeErrorKind),

    #[error("Connection scope is disposed")]
    ConnectionScopeDisposed,

    /// Error with sending the message
    #[error(transparent)]
    Send(#[from] fe2o3_amqp::link::SendError),

    /// The sent message is not accepted by the service
    #[error(transparent)]
    NotAccepted(#[from] NotAcceptedError),

    /// The operation timed out
    #[error(transparent)]
    Elapsed(#[from] Elapsed),
}

impl From<RecoverProducerError> for RecoverAndSendError {
    fn from(value: RecoverProducerError) -> Self {
        match value {
            RecoverProducerError::SessionBegin(err) => err.into(),
            RecoverProducerError::SenderDetach(err) => err.into(),
            RecoverProducerError::SenderResume(err) => err.into(),
            RecoverProducerError::ConnectionScopeDisposed => {
                RecoverAndSendError::ConnectionScopeDisposed
            }
            RecoverProducerError::ParseEndpoint(err) => err.into(),
            RecoverProducerError::CbsAuth(err) => err.into(),
            RecoverProducerError::SenderAttach(err) => err.into(),
            RecoverProducerError::Elapsed(err) => err.into(),
        }
    }
}

impl From<AmqpSendError> for RecoverAndSendError {
    fn from(value: AmqpSendError) -> Self {
        match value {
            AmqpSendError::Send(err) => err.into(),
            AmqpSendError::NotAccepted(err) => err.into(),
            AmqpSendError::Elapsed(err) => err.into(),
        }
    }
}

impl RecoverableError for RecoverAndSendError {
    fn should_try_recover(&self) -> bool {
        match self {
            RecoverAndSendError::SessionBegin(_) => true,
            RecoverAndSendError::SenderAttach(_) => true,
            RecoverAndSendError::SenderDetach(_) => true,
            RecoverAndSendError::SenderResume(_) => true,
            RecoverAndSendError::ConnectionScopeDisposed => false,
            RecoverAndSendError::Send(_) => true,
            RecoverAndSendError::NotAccepted(_) => false,
            // The first time we try to send after a forced closure may fail with timeout
            RecoverAndSendError::Elapsed(_) => true,
            RecoverAndSendError::ParseEndpoint(_) => false,
            RecoverAndSendError::CbsAuth(_) => true,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, RecoverAndSendError::ConnectionScopeDisposed)
    }
}

impl From<RecoverAndSendError> for azure_core::Error {
    fn from(value: RecoverAndSendError) -> Self {
        match value {
            RecoverAndSendError::SessionBegin(err) => err.into_azure_core_error(),
            RecoverAndSendError::SenderDetach(err) => err.into_azure_core_error(),
            RecoverAndSendError::SenderResume(err) => err.into_azure_core_error(),
            RecoverAndSendError::ConnectionScopeDisposed => {
                azure_core::Error::new(azure_core::error::ErrorKind::Other, value)
            }
            RecoverAndSendError::Send(err) => err.into_azure_core_error(),
            RecoverAndSendError::NotAccepted(err) => err.into(),
            RecoverAndSendError::Elapsed(err) => err.into_azure_core_error(),
            RecoverAndSendError::ParseEndpoint(err) => err.into(),
            RecoverAndSendError::CbsAuth(err) => err.into(),
            RecoverAndSendError::SenderAttach(err) => err.into_azure_core_error(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecoverConsumerError {
    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),

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

impl From<DetachThenResumeReceiverError> for RecoverConsumerError {
    fn from(err: DetachThenResumeReceiverError) -> Self {
        match err {
            DetachThenResumeReceiverError::Detach(err) => err.into(),
            DetachThenResumeReceiverError::Resume(err) => err.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecoverManagementLinkError {
    #[error(transparent)]
    SessionBegin(#[from] BeginError),

    #[error(transparent)]
    LinkDetach(#[from] DetachError),

    /// Error with resuming the sender
    #[error(transparent)]
    SenderResume(#[from] SenderResumeErrorKind),

    /// Error with resuming the sender
    #[error(transparent)]
    ReceiverResume(#[from] ReceiverResumeErrorKind),

    #[error("Connection scope is disposed")]
    ConnectionScopeDisposed,
}

impl From<DetachThenResumeSenderError> for RecoverManagementLinkError {
    fn from(err: DetachThenResumeSenderError) -> Self {
        match err {
            DetachThenResumeSenderError::Detach(err) => err.into(),
            DetachThenResumeSenderError::Resume(err) => err.into(),
        }
    }
}

impl From<DetachThenResumeReceiverError> for RecoverManagementLinkError {
    fn from(err: DetachThenResumeReceiverError) -> Self {
        match err {
            DetachThenResumeReceiverError::Detach(err) => err.into(),
            DetachThenResumeReceiverError::Resume(err) => err.into(),
        }
    }
}

impl From<DetachThenResumeError> for RecoverManagementLinkError {
    fn from(err: DetachThenResumeError) -> Self {
        match err {
            DetachThenResumeError::Sender(err) => err.into(),
            DetachThenResumeError::Receiver(err) => err.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecoverTransportClientError {
    #[error(transparent)]
    Parse(#[from] url::ParseError),

    #[error(transparent)]
    Open(#[from] OpenError),

    #[error(transparent)]
    WebSocket(#[from] fe2o3_amqp_ws::Error),

    #[error(transparent)]
    SessionBegin(#[from] BeginError),

    #[error(transparent)]
    LinkDetach(#[from] DetachError),

    /// Error with resuming the sender
    #[error(transparent)]
    SenderResume(#[from] SenderResumeErrorKind),

    /// Error with resuming the sender
    #[error(transparent)]
    ReceiverResume(#[from] ReceiverResumeErrorKind),

    #[error("Connection scope is disposed")]
    ConnectionScopeDisposed,
}

impl From<AmqpConnectionScopeError> for RecoverTransportClientError {
    fn from(value: AmqpConnectionScopeError) -> Self {
        match value {
            AmqpConnectionScopeError::Parse(err) => err.into(),
            AmqpConnectionScopeError::Open(err) => err.into(),
            AmqpConnectionScopeError::WebSocket(err) => err.into(),
            AmqpConnectionScopeError::Begin(err) => err.into(),
            AmqpConnectionScopeError::SenderAttach(err) => {
                SenderResumeErrorKind::AttachError(err).into()
            }
            AmqpConnectionScopeError::ReceiverAttach(err) => {
                ReceiverResumeErrorKind::AttachError(err).into()
            }
            AmqpConnectionScopeError::ScopeDisposed => Self::ConnectionScopeDisposed,
        }
    }
}

impl From<RecoverManagementLinkError> for RecoverTransportClientError {
    fn from(value: RecoverManagementLinkError) -> Self {
        match value {
            RecoverManagementLinkError::SessionBegin(err) => err.into(),
            RecoverManagementLinkError::LinkDetach(err) => err.into(),
            RecoverManagementLinkError::SenderResume(err) => err.into(),
            RecoverManagementLinkError::ReceiverResume(err) => err.into(),
            RecoverManagementLinkError::ConnectionScopeDisposed => {
                RecoverTransportClientError::ConnectionScopeDisposed
            }
        }
    }
}

impl RecoverableError for RecoverTransportClientError {
    fn should_try_recover(&self) -> bool {
        match self {
            RecoverTransportClientError::Parse(_) => false,
            RecoverTransportClientError::Open(_) => false,
            RecoverTransportClientError::WebSocket(_) => false,
            RecoverTransportClientError::SessionBegin(_) => true, // The connection should be recovered
            RecoverTransportClientError::LinkDetach(_) => true,   // The session should be recovered
            RecoverTransportClientError::SenderResume(_) => true,
            RecoverTransportClientError::ReceiverResume(_) => true,
            RecoverTransportClientError::ConnectionScopeDisposed => false,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, RecoverTransportClientError::ConnectionScopeDisposed)
    }
}

impl From<RecoverTransportClientError> for azure_core::Error {
    fn from(err: RecoverTransportClientError) -> Self {
        match err {
            RecoverTransportClientError::Parse(err) => err.into(),
            RecoverTransportClientError::Open(err) => err.into_azure_core_error(),
            RecoverTransportClientError::WebSocket(err) => err.into_azure_core_error(),
            RecoverTransportClientError::SessionBegin(err) => err.into_azure_core_error(),
            RecoverTransportClientError::LinkDetach(err) => err.into_azure_core_error(),
            RecoverTransportClientError::SenderResume(err) => err.into_azure_core_error(),
            RecoverTransportClientError::ReceiverResume(err) => err.into_azure_core_error(),
            RecoverTransportClientError::ConnectionScopeDisposed => {
                azure_core::Error::new(azure_core::error::ErrorKind::Other, err)
            }
        }
    }
}

impl RecoverableError for ManagementError {
    fn should_try_recover(&self) -> bool {
        match self {
            ManagementError::Send(_) => true,
            ManagementError::Recv(_) => true,
            ManagementError::Disposition(_) => true,
            ManagementError::CorrelationIdAndMessageIdAreNone => false,
            ManagementError::StatusCodeNotFound => false,
            ManagementError::DecodeError(_) => false,
            ManagementError::Status(_) => false,
            ManagementError::NotAccepted(_) => false,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        false
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecoverAndCallError {
    #[error(transparent)]
    RecoverClient(#[from] RecoverTransportClientError),

    #[error(transparent)]
    Management(#[from] ManagementError),

    #[error(transparent)]
    Elapsed(#[from] Elapsed),
}

impl From<RecoverAndCallError> for azure_core::Error {
    fn from(err: RecoverAndCallError) -> Self {
        match err {
            RecoverAndCallError::RecoverClient(err) => err.into(),
            RecoverAndCallError::Management(err) => err.into_azure_core_error(),
            RecoverAndCallError::Elapsed(err) => err.into_azure_core_error(),
        }
    }
}

impl RecoverableError for RecoverAndCallError {
    fn should_try_recover(&self) -> bool {
        match self {
            RecoverAndCallError::RecoverClient(err) => err.should_try_recover(),
            RecoverAndCallError::Management(err) => err.should_try_recover(),
            RecoverAndCallError::Elapsed(_) => true,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        match self {
            RecoverAndCallError::RecoverClient(err) => err.is_scope_disposed(),
            RecoverAndCallError::Management(_) => false,
            RecoverAndCallError::Elapsed(_) => false,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecoverAndReceiveError {
    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),

    #[error(transparent)]
    Receive(#[from] RecvError),

    #[error(transparent)]
    SessionBegin(#[from] BeginError),

    /// Error with resuming the sender
    #[error(transparent)]
    ReceiverResume(#[from] ReceiverResumeErrorKind),

    #[error("Connection scope is disposed")]
    ConnectionScopeDisposed,

    #[error(transparent)]
    Parse(#[from] url::ParseError),

    #[error(transparent)]
    Open(#[from] OpenError),

    #[error(transparent)]
    WebSocket(#[from] fe2o3_amqp_ws::Error),

    #[error(transparent)]
    LinkDetach(#[from] DetachError),

    /// Error with resuming the sender
    #[error(transparent)]
    SenderResume(#[from] SenderResumeErrorKind),

    #[error(transparent)]
    Disposition(#[from] DispositionError),

    #[error(transparent)]
    SessionEnd(#[from] fe2o3_amqp::session::Error),

    #[error(transparent)]
    Elapsed(#[from] Elapsed),
}

impl From<RecoverTransportClientError> for RecoverAndReceiveError {
    fn from(value: RecoverTransportClientError) -> Self {
        match value {
            RecoverTransportClientError::Parse(err) => err.into(),
            RecoverTransportClientError::Open(err) => err.into(),
            RecoverTransportClientError::WebSocket(err) => err.into(),
            RecoverTransportClientError::SessionBegin(err) => err.into(),
            RecoverTransportClientError::LinkDetach(err) => err.into(),
            RecoverTransportClientError::SenderResume(err) => err.into(),
            RecoverTransportClientError::ReceiverResume(err) => err.into(),
            RecoverTransportClientError::ConnectionScopeDisposed => {
                RecoverAndReceiveError::ConnectionScopeDisposed
            }
        }
    }
}

impl From<RecoverConsumerError> for RecoverAndReceiveError {
    fn from(value: RecoverConsumerError) -> Self {
        match value {
            RecoverConsumerError::SessionBegin(err) => err.into(),
            RecoverConsumerError::ConnectionScopeDisposed => {
                RecoverAndReceiveError::ConnectionScopeDisposed
            }
            RecoverConsumerError::ReceiverDetach(err) => err.into(),
            RecoverConsumerError::ReceiverResume(err) => err.into(),
            RecoverConsumerError::CbsAuth(err) => err.into(),
        }
    }
}

impl From<DisposeConsumerError> for RecoverAndReceiveError {
    fn from(value: DisposeConsumerError) -> Self {
        match value {
            DisposeConsumerError::Receiver(err) => err.into(),
            DisposeConsumerError::Session(err) => err.into(),
        }
    }
}

impl From<RecoverAndReceiveError> for azure_core::Error {
    fn from(value: RecoverAndReceiveError) -> Self {
        match value {
            RecoverAndReceiveError::CbsAuth(err) => err.into(),
            RecoverAndReceiveError::Receive(err) => err.into_azure_core_error(),
            RecoverAndReceiveError::SessionBegin(err) => err.into_azure_core_error(),
            RecoverAndReceiveError::ReceiverResume(err) => err.into_azure_core_error(),
            RecoverAndReceiveError::ConnectionScopeDisposed => {
                azure_core::Error::new(azure_core::error::ErrorKind::Other, value)
            }
            RecoverAndReceiveError::Parse(err) => err.into(),
            RecoverAndReceiveError::Open(err) => err.into_azure_core_error(),
            RecoverAndReceiveError::WebSocket(err) => err.into_azure_core_error(),
            RecoverAndReceiveError::LinkDetach(err) => err.into_azure_core_error(),
            RecoverAndReceiveError::SenderResume(err) => err.into_azure_core_error(),
            RecoverAndReceiveError::Disposition(err) => err.into_azure_core_error(),
            RecoverAndReceiveError::SessionEnd(err) => err.into_azure_core_error(),
            RecoverAndReceiveError::Elapsed(err) => err.into_azure_core_error(),
        }
    }
}

impl RecoverableError for RecoverAndReceiveError {
    fn should_try_recover(&self) -> bool {
        match self {
            RecoverAndReceiveError::Receive(_) => true,
            RecoverAndReceiveError::SessionBegin(_) => true,
            RecoverAndReceiveError::ReceiverResume(_) => true,
            RecoverAndReceiveError::ConnectionScopeDisposed => false,
            RecoverAndReceiveError::Parse(_) => false,
            RecoverAndReceiveError::Open(_) => false,
            RecoverAndReceiveError::WebSocket(_) => false,
            RecoverAndReceiveError::LinkDetach(_) => true,
            RecoverAndReceiveError::SenderResume(_) => true,
            RecoverAndReceiveError::Disposition(_) => true,
            RecoverAndReceiveError::Elapsed(_) => true,
            RecoverAndReceiveError::SessionEnd(_) => true, // TODO: should this be true?
            RecoverAndReceiveError::CbsAuth(_) => true,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, RecoverAndReceiveError::ConnectionScopeDisposed)
    }
}
