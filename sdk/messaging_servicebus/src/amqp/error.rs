//! Error types for AMQP operations

use fe2o3_amqp::{
    connection::{self, OpenError},
    link::{
        DetachError, DetachThenResumeReceiverError, DetachThenResumeSenderError,
        IllegalLinkStateError, LinkStateError, ReceiverAttachError, ReceiverResumeErrorKind,
        RecvError, SendError, SenderAttachError, SenderResumeErrorKind,
    },
    session::{self, BeginError},
};
use fe2o3_amqp_management::error::{AttachError, Error as ManagementError};
use fe2o3_amqp_types::messaging::{Modified, Rejected, Released};
use timer_kit::error::Elapsed;

use crate::{
    primitives::{
        error::ClientDisposedError,
        service_bus_retry_policy::{
            should_try_recover_from_management_error, ServiceBusRetryPolicyError,
        },
    },
    util::IntoAzureCoreError,
    ServiceBusMessage,
};

// Conditional import for docs.rs
#[cfg(docsrs)]
use crate::{ServiceBusPeekedMessage, ServiceBusReceivedMessage};

#[derive(Debug, thiserror::Error)]
pub(crate) enum AmqpConnectionScopeError {
    #[error(transparent)]
    Open(#[from] OpenError),

    #[error(transparent)]
    WebSocket(#[from] fe2o3_amqp_ws::Error),

    #[error(transparent)]
    Elapsed(#[from] Elapsed),

    #[error(transparent)]
    Begin(#[from] BeginError),

    #[error(transparent)]
    SenderAttach(#[from] SenderAttachError),

    #[error(transparent)]
    ReceiverAttach(#[from] ReceiverAttachError),

    #[error("The connection scope is disposed")]
    ScopeDisposed,
}

/// Error with AMQP client
#[derive(Debug, thiserror::Error)]
pub(crate) enum AmqpClientError {
    /// Error parsing the URL
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    /// Error with opening the connection
    #[error(transparent)]
    Open(#[from] OpenError),

    /// Error with establishing the WebSocket transport
    #[error(transparent)]
    WebSocket(#[from] fe2o3_amqp_ws::Error),

    /// Operation timed out
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
    ReceiverAttach(#[from] ReceiverAttachError),

    /// Error closing the AMQP client
    #[error(transparent)]
    Dispose(#[from] DisposeError),

    /// Client is already disposed
    #[error("The client is disposed")]
    ClientDisposed(#[from] ClientDisposedError),
}

impl From<AmqpClientError> for azure_core::Error {
    fn from(value: AmqpClientError) -> Self {
        match value {
            AmqpClientError::UrlParseError(error) => error.into(),
            AmqpClientError::Open(error) => error.into_azure_core_error(),
            AmqpClientError::WebSocket(error) => error.into_azure_core_error(),
            AmqpClientError::Elapsed(error) => error.into_azure_core_error(),
            AmqpClientError::Begin(error) => error.into_azure_core_error(),
            AmqpClientError::SenderAttach(error) => error.into_azure_core_error(),
            AmqpClientError::ReceiverAttach(error) => error.into_azure_core_error(),
            AmqpClientError::Dispose(error) => error.into(),
            AmqpClientError::ClientDisposed(error) => error.into(),
        }
    }
}

impl From<AmqpConnectionScopeError> for AmqpClientError {
    fn from(err: AmqpConnectionScopeError) -> Self {
        match err {
            AmqpConnectionScopeError::Open(err) => Self::Open(err),
            AmqpConnectionScopeError::WebSocket(err) => Self::WebSocket(err),
            AmqpConnectionScopeError::Elapsed(err) => Self::Elapsed(err),
            AmqpConnectionScopeError::Begin(err) => Self::Begin(err),
            AmqpConnectionScopeError::SenderAttach(err) => Self::SenderAttach(err),
            AmqpConnectionScopeError::ReceiverAttach(err) => Self::ReceiverAttach(err),
            AmqpConnectionScopeError::ScopeDisposed => Self::ClientDisposed(ClientDisposedError),
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

/// Error closing the AMQP connection and AMQP session
#[derive(Debug, thiserror::Error)]
pub(crate) enum DisposeError {
    /// Error closing the AMQP session
    #[error(transparent)]
    SessionCloseError(#[from] session::Error),

    /// Error closing the AMQP connection
    #[error(transparent)]
    ConnectionCloseError(#[from] connection::Error),
}

impl From<DisposeError> for azure_core::Error {
    fn from(value: DisposeError) -> Self {
        match value {
            DisposeError::SessionCloseError(error) => error.into_azure_core_error(),
            DisposeError::ConnectionCloseError(error) => error.into_azure_core_error(),
        }
    }
}

/// Error opening a management link
#[derive(Debug, thiserror::Error)]
pub(crate) enum OpenMgmtLinkError {
    /// Connection scope is disposed
    #[error("Scope is disposed")]
    ConnectionScopeDisposed,

    /// Error attaching the management link
    #[error(transparent)]
    Attach(#[from] AttachError),

    /// Error with CBS auth the management link
    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),
}

/// Error opening a sender link
#[derive(Debug, thiserror::Error)]
pub(crate) enum OpenSenderError {
    /// Connection scope is disposed
    #[error("The connection scope is disposed")]
    ConnectionScopeDisposed,

    /// Error attaching the management link
    #[error(transparent)]
    ManagementLinkAttach(#[from] AttachError),

    /// Error attaching the sender link
    #[error(transparent)]
    SenderAttach(#[from] SenderAttachError),

    /// Error with CBS auth the sender link
    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),
}

impl From<OpenSenderError> for azure_core::Error {
    fn from(value: OpenSenderError) -> Self {
        match value {
            OpenSenderError::ConnectionScopeDisposed => ClientDisposedError.into(),
            OpenSenderError::ManagementLinkAttach(error) => error.into_azure_core_error(),
            OpenSenderError::SenderAttach(error) => error.into_azure_core_error(),
            OpenSenderError::CbsAuth(error) => error.into(),
        }
    }
}

impl From<OpenMgmtLinkError> for OpenSenderError {
    fn from(err: OpenMgmtLinkError) -> Self {
        match err {
            OpenMgmtLinkError::ConnectionScopeDisposed => OpenSenderError::ConnectionScopeDisposed,
            OpenMgmtLinkError::Attach(err) => OpenSenderError::ManagementLinkAttach(err),
            OpenMgmtLinkError::CbsAuth(err) => OpenSenderError::CbsAuth(err),
        }
    }
}

/// Error recovering a sender link
#[derive(Debug, thiserror::Error)]
pub(crate) enum RecoverSenderError {
    /// Connection scope is disposed
    #[error("The connection scope is disposed")]
    ConnectionScopeDisposed,

    /// Error attaching the management link
    #[error(transparent)]
    ManagementLinkAttach(#[from] AttachError),

    /// Error detaching the current sender link
    #[error(transparent)]
    SenderDetach(#[from] DetachError),

    /// Error attaching the sender link to new session
    #[error(transparent)]
    SenderResume(#[from] SenderResumeErrorKind),

    /// Error with CBS auth the recovering sender link
    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),
}

impl ServiceBusRetryPolicyError for RecoverSenderError {
    fn should_try_recover(&self) -> bool {
        // This error is only returned if the recover operation fails.
        false
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, RecoverSenderError::ConnectionScopeDisposed)
    }
}

impl From<DetachThenResumeSenderError> for RecoverSenderError {
    fn from(value: DetachThenResumeSenderError) -> Self {
        match value {
            DetachThenResumeSenderError::Detach(err) => RecoverSenderError::SenderDetach(err),
            DetachThenResumeSenderError::Resume(err) => RecoverSenderError::SenderResume(err),
        }
    }
}

impl From<OpenMgmtLinkError> for RecoverSenderError {
    fn from(err: OpenMgmtLinkError) -> Self {
        match err {
            OpenMgmtLinkError::ConnectionScopeDisposed => {
                RecoverSenderError::ConnectionScopeDisposed
            }
            OpenMgmtLinkError::Attach(err) => RecoverSenderError::ManagementLinkAttach(err),
            OpenMgmtLinkError::CbsAuth(err) => RecoverSenderError::CbsAuth(err),
        }
    }
}

/// Error opening a receiver link
#[derive(Debug, thiserror::Error)]
pub(crate) enum OpenReceiverError {
    /// Connection scope is disposed
    #[error("The connection scope is disposed")]
    ConnectionScopeDisposed,

    /// Error attaching the management link
    #[error(transparent)]
    ManagementLinkAttach(#[from] AttachError),

    /// Error attaching the receiver link
    #[error(transparent)]
    ReceiverAttach(#[from] ReceiverAttachError),

    /// Error with CBS auth the receiver link
    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),
}

impl From<OpenReceiverError> for azure_core::Error {
    fn from(value: OpenReceiverError) -> Self {
        match value {
            OpenReceiverError::ConnectionScopeDisposed => ClientDisposedError.into(),
            OpenReceiverError::ManagementLinkAttach(error) => error.into_azure_core_error(),
            OpenReceiverError::ReceiverAttach(error) => error.into_azure_core_error(),
            OpenReceiverError::CbsAuth(error) => error.into(),
        }
    }
}

impl From<OpenMgmtLinkError> for OpenReceiverError {
    fn from(err: OpenMgmtLinkError) -> Self {
        match err {
            OpenMgmtLinkError::ConnectionScopeDisposed => {
                OpenReceiverError::ConnectionScopeDisposed
            }
            OpenMgmtLinkError::Attach(err) => OpenReceiverError::ManagementLinkAttach(err),
            OpenMgmtLinkError::CbsAuth(err) => OpenReceiverError::CbsAuth(err),
        }
    }
}

/// Error recovering a receiver link
#[derive(Debug, thiserror::Error)]
pub(crate) enum RecoverReceiverError {
    /// Connection scope is disposed
    #[error("The connection scope is disposed")]
    ConnectionScopeDisposed,

    /// Error attaching the management link
    #[error(transparent)]
    ManagementLinkAttach(#[from] AttachError),

    /// Error detaching the current receiver link
    #[error(transparent)]
    ReceiverDetach(#[from] DetachError),

    /// Error attaching the receiver link to new session
    #[error(transparent)]
    ReceiverResume(#[from] ReceiverResumeErrorKind),

    /// Error with CBS auth the recovering receiver link
    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),
}

impl ServiceBusRetryPolicyError for RecoverReceiverError {
    fn should_try_recover(&self) -> bool {
        // This error is only returned if the recover operation fails.
        false
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, RecoverReceiverError::ConnectionScopeDisposed)
    }
}

impl From<DetachThenResumeReceiverError> for RecoverReceiverError {
    fn from(value: DetachThenResumeReceiverError) -> Self {
        match value {
            DetachThenResumeReceiverError::Detach(err) => RecoverReceiverError::ReceiverDetach(err),
            DetachThenResumeReceiverError::Resume(err) => RecoverReceiverError::ReceiverResume(err),
        }
    }
}

impl From<OpenMgmtLinkError> for RecoverReceiverError {
    fn from(err: OpenMgmtLinkError) -> Self {
        match err {
            OpenMgmtLinkError::ConnectionScopeDisposed => {
                RecoverReceiverError::ConnectionScopeDisposed
            }
            OpenMgmtLinkError::Attach(err) => RecoverReceiverError::ManagementLinkAttach(err),
            OpenMgmtLinkError::CbsAuth(err) => RecoverReceiverError::CbsAuth(err),
        }
    }
}

/// Error opening a rule manager
#[derive(Debug, thiserror::Error)]
pub(crate) enum OpenRuleManagerError {
    /// Connection scope is disposed
    #[error("The connection scope is disposed")]
    ConnectionScopeDisposed,

    /// Error attaching the management link
    #[error(transparent)]
    ManagementLinkAttach(#[from] AttachError),

    /// Error with CBS auth the rule manager
    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),
}

impl From<OpenRuleManagerError> for azure_core::Error {
    fn from(value: OpenRuleManagerError) -> Self {
        match value {
            OpenRuleManagerError::ConnectionScopeDisposed => ClientDisposedError.into(),
            OpenRuleManagerError::ManagementLinkAttach(error) => error.into_azure_core_error(),
            OpenRuleManagerError::CbsAuth(error) => error.into(),
        }
    }
}

impl ServiceBusRetryPolicyError for OpenRuleManagerError {
    fn should_try_recover(&self) -> bool {
        false
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, OpenRuleManagerError::ConnectionScopeDisposed)
    }
}

impl From<OpenMgmtLinkError> for OpenRuleManagerError {
    fn from(err: OpenMgmtLinkError) -> Self {
        match err {
            OpenMgmtLinkError::ConnectionScopeDisposed => {
                OpenRuleManagerError::ConnectionScopeDisposed
            }
            OpenMgmtLinkError::Attach(err) => OpenRuleManagerError::ManagementLinkAttach(err),
            OpenMgmtLinkError::CbsAuth(err) => OpenRuleManagerError::CbsAuth(err),
        }
    }
}

/// Error sending message to the service
#[derive(Debug, thiserror::Error)]
pub(crate) enum AmqpSendError {
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

impl ServiceBusRetryPolicyError for LinkStateError {
    fn should_try_recover(&self) -> bool {
        matches!(
            self,
            LinkStateError::IllegalState
                | LinkStateError::IllegalSessionState
                | LinkStateError::ExpectImmediateDetach
                | LinkStateError::RemoteDetached
        )
    }

    fn is_scope_disposed(&self) -> bool {
        false
    }
}

impl ServiceBusRetryPolicyError for DetachError {
    fn should_try_recover(&self) -> bool {
        matches!(
            self,
            DetachError::IllegalState
                | DetachError::IllegalSessionState
                | DetachError::RemoteDetachedWithError(_)
                | DetachError::DetachedByRemote
        )
    }

    fn is_scope_disposed(&self) -> bool {
        false
    }
}

impl ServiceBusRetryPolicyError for SendError {
    fn should_try_recover(&self) -> bool {
        match self {
            SendError::LinkStateError(err) => err.should_try_recover(),
            SendError::Detached(err) => err.should_try_recover(),
            _ => false,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        false
    }
}

impl ServiceBusRetryPolicyError for AmqpSendError {
    fn should_try_recover(&self) -> bool {
        match self {
            Self::Send(err) => err.should_try_recover(),
            Self::Elapsed(_) => true,
            _ => false,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        false
    }
}

/// Error receiving message from the service
#[derive(Debug, thiserror::Error)]
pub(crate) enum AmqpRecvError {
    /// Error with receiving the message
    #[error(transparent)]
    Recv(#[from] RecvError),

    /// Wrong link state
    #[error(transparent)]
    LinkState(#[from] IllegalLinkStateError),

    /// The operation timed out
    #[error(transparent)]
    Elapsed(#[from] Elapsed),

    /// The lock token is not found in the message
    #[error("A valid lock token was not found in the message")]
    LockTokenNotFound,
}

impl ServiceBusRetryPolicyError for AmqpRecvError {
    fn should_try_recover(&self) -> bool {
        matches!(
            self,
            Self::Recv(RecvError::LinkStateError(
                LinkStateError::IllegalState
                    | LinkStateError::IllegalSessionState
                    | LinkStateError::ExpectImmediateDetach
                    | LinkStateError::RemoteDetached
            )) | Self::LinkState(_)
        )
    }

    fn is_scope_disposed(&self) -> bool {
        false
    }
}

/// Error with message disposition
#[derive(Debug, thiserror::Error)]
pub(crate) enum AmqpDispositionError {
    /// Error with the link state
    #[error(transparent)]
    IllegalState(#[from] IllegalLinkStateError),

    /// Error with the request-response operation on the management link
    #[error(transparent)]
    RequestResponse(#[from] ManagementError),

    /// The operation timed out
    #[error(transparent)]
    Elapsed(#[from] Elapsed),
}

impl ServiceBusRetryPolicyError for AmqpDispositionError {
    fn should_try_recover(&self) -> bool {
        match self {
            Self::IllegalState(IllegalLinkStateError::IllegalSessionState) => true,
            Self::IllegalState(IllegalLinkStateError::IllegalState) => false,
            Self::RequestResponse(err) => should_try_recover_from_management_error(err),
            Self::Elapsed(_) => false,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        false
    }
}

/// Error with request-response operation
#[derive(Debug, thiserror::Error)]
pub(crate) enum AmqpRequestResponseError {
    /// Error with the request-response operation on the management link
    #[error(transparent)]
    RequestResponse(#[from] ManagementError),

    /// The operation timed out
    #[error(transparent)]
    Elapsed(#[from] Elapsed),
}

impl ServiceBusRetryPolicyError for AmqpRequestResponseError {
    fn should_try_recover(&self) -> bool {
        match self {
            Self::RequestResponse(err) => should_try_recover_from_management_error(err),
            Self::Elapsed(_) => false,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        false
    }
}

impl From<serde_amqp::Error> for AmqpRequestResponseError {
    fn from(_: serde_amqp::Error) -> Self {
        Self::RequestResponse(ManagementError::DecodeError(None))
    }
}

/// Error with CBS auth
#[derive(Debug, thiserror::Error)]
pub(crate) enum CbsAuthError {
    /// Error with the token provider
    #[error(transparent)]
    TokenCredential(#[from] azure_core::Error),

    /// Error with the CBS link
    #[error(transparent)]
    Cbs(#[from] ManagementError),
}

impl From<CbsAuthError> for azure_core::Error {
    fn from(value: CbsAuthError) -> Self {
        match value {
            CbsAuthError::TokenCredential(error) => error,
            CbsAuthError::Cbs(error) => error.into_azure_core_error(),
        }
    }
}

/// Error with adding a message to a batch
#[derive(Debug, thiserror::Error)]
pub enum TryAddMessageError {
    /// The message is too large to fit in a batch
    #[error("Message is too large to fit in a batch")]
    BatchFull(ServiceBusMessage),

    /// The message cannot be serialized
    #[error("Cannot serialize message")]
    Codec {
        /// The error from the codec
        source: serde_amqp::Error,
        /// The message that could not be serialized
        message: ServiceBusMessage,
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

/// Error with creating a rule
#[derive(Debug, thiserror::Error)]
pub enum CreateRuleError {
    /// The correlation filter must have at least one entry
    #[error("The correlation filter must have at least one entry")]
    EmptyCorrelationFilter,

    /// Error while performing request/response operation
    #[error(transparent)]
    RequestResponse(#[from] ManagementError),

    /// Operation timed out
    #[error(transparent)]
    Elapsed(#[from] Elapsed),

    /// Connection scope is disposed
    #[error("Connection scope is disposed")]
    ConnectionScopeDisposed,
}

impl From<CorrelationFilterError> for CreateRuleError {
    fn from(err: CorrelationFilterError) -> Self {
        match err {
            CorrelationFilterError::EmptyFilter => Self::EmptyCorrelationFilter,
        }
    }
}

impl From<AmqpRequestResponseError> for CreateRuleError {
    fn from(err: AmqpRequestResponseError) -> Self {
        match err {
            AmqpRequestResponseError::RequestResponse(err) => Self::RequestResponse(err),
            AmqpRequestResponseError::Elapsed(err) => Self::Elapsed(err),
        }
    }
}

impl ServiceBusRetryPolicyError for CreateRuleError {
    fn should_try_recover(&self) -> bool {
        match self {
            Self::RequestResponse(err) => should_try_recover_from_management_error(err),
            Self::Elapsed(_) => false,
            Self::ConnectionScopeDisposed => false,
            Self::EmptyCorrelationFilter => false,
        }
    }

    fn is_scope_disposed(&self) -> bool {
        matches!(self, Self::ConnectionScopeDisposed)
    }
}
