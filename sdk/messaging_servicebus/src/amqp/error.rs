use fe2o3_amqp::{
    connection::{self, OpenError},
    link::{
        IllegalLinkStateError, LinkStateError, ReceiverAttachError, RecvError, SenderAttachError,
    },
    session::{self, BeginError},
};
use fe2o3_amqp_management::error::{AttachError, Error as ManagementError};
use fe2o3_amqp_types::messaging::{Modified, Rejected, Released};
use tokio::time::error::Elapsed;

use crate::{primitives::service_bus_retry_policy::ServiceBusRetryPolicyError, ServiceBusMessage};

#[derive(Debug, thiserror::Error)]
pub(crate) enum AmqpConnectionScopeError {
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
    ReceiverAttach(#[from] ReceiverAttachError),
}


#[derive(Debug, thiserror::Error)]
pub enum AmqpClientError {
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
    ReceiverAttach(#[from] ReceiverAttachError),

    #[error(transparent)]
    Dispose(#[from] DisposeError),
}

impl From<AmqpConnectionScopeError> for AmqpClientError {
    fn from(err: AmqpConnectionScopeError) -> Self {
        match err {
            AmqpConnectionScopeError::Open(err) => Self::Open(err),
            AmqpConnectionScopeError::WebSocket(err) => Self::WebSocket(err),
            AmqpConnectionScopeError::TimeoutElapsed(err) => Self::TimeoutElapsed(err),
            AmqpConnectionScopeError::Begin(err) => Self::Begin(err),
            AmqpConnectionScopeError::SenderAttach(err) => Self::SenderAttach(err),
            AmqpConnectionScopeError::ReceiverAttach(err) => Self::ReceiverAttach(err),
        }
    }
}

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

#[derive(Debug, thiserror::Error)]
pub enum SetMessageIdError {
    #[error("Value cannot be empty")]
    Empty,

    #[error(transparent)]
    MaxLengthExceeded(#[from] MaxLengthExceededError),
}

#[derive(Debug, thiserror::Error)]
pub enum SetPartitionKeyError {
    #[error(transparent)]
    MaxLengthExceeded(#[from] MaxLengthExceededError),

    #[error("PartitionKey cannot be set to a different value than SessionId")]
    PartitionKeyAndSessionIdAreDifferent,
}

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

#[derive(Debug, Clone)]
pub struct RawAmqpMessageError {}

impl std::fmt::Display for RawAmqpMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawAmqpMessageError: The message is a raw AMQP message")
    }
}

impl std::error::Error for RawAmqpMessageError {}

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

#[derive(Debug, thiserror::Error)]
pub enum DisposeError {
    #[error(transparent)]
    SessionCloseError(#[from] session::Error),

    #[error(transparent)]
    ConnectionCloseError(#[from] connection::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum OpenMgmtLinkError {
    #[error("Scope is disposed")]
    ScopeIsDisposed,

    #[error(transparent)]
    Attach(#[from] AttachError),

    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),
}

#[derive(Debug, thiserror::Error)]
pub enum OpenSenderError {
    #[error("The connection scope is disposed")]
    ScopeIsDisposed,

    #[error(transparent)]
    ManagemetnLinkAttach(#[from] AttachError),

    #[error(transparent)]
    SenderAttach(#[from] SenderAttachError),

    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),
}

impl From<OpenMgmtLinkError> for OpenSenderError {
    fn from(err: OpenMgmtLinkError) -> Self {
        match err {
            OpenMgmtLinkError::ScopeIsDisposed => OpenSenderError::ScopeIsDisposed,
            OpenMgmtLinkError::Attach(err) => OpenSenderError::ManagemetnLinkAttach(err),
            OpenMgmtLinkError::CbsAuth(err) => OpenSenderError::CbsAuth(err),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OpenReceiverError {
    #[error("The connection scope is disposed")]
    ScopeIsDisposed,

    #[error(transparent)]
    ManagementLinkAttach(#[from] AttachError),

    #[error(transparent)]
    ReceiverAttach(#[from] ReceiverAttachError),

    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),
}

impl From<OpenMgmtLinkError> for OpenReceiverError {
    fn from(err: OpenMgmtLinkError) -> Self {
        match err {
            OpenMgmtLinkError::ScopeIsDisposed => OpenReceiverError::ScopeIsDisposed,
            OpenMgmtLinkError::Attach(err) => OpenReceiverError::ManagementLinkAttach(err),
            OpenMgmtLinkError::CbsAuth(err) => OpenReceiverError::CbsAuth(err),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AmqpSendError {
    #[error(transparent)]
    Send(#[from] fe2o3_amqp::link::SendError),

    #[error(transparent)]
    NotAccepted(#[from] NotAcceptedError),

    #[error(transparent)]
    Elapsed(#[from] Elapsed),
}

impl ServiceBusRetryPolicyError for AmqpSendError {
    fn is_scope_disposed(&self) -> bool {
        use fe2o3_amqp::link::SendError;
        matches!(
            self,
            Self::Send(SendError::LinkStateError(
                LinkStateError::IllegalSessionState
            ))
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AmqpRecvError {
    #[error(transparent)]
    Recv(#[from] RecvError),

    #[error(transparent)]
    LinkState(#[from] IllegalLinkStateError),

    #[error(transparent)]
    Elapsed(#[from] Elapsed),

    #[error("A valid lock token was not found in the message")]
    LockTokenNotFound,
}

impl ServiceBusRetryPolicyError for AmqpRecvError {
    fn is_scope_disposed(&self) -> bool {
        match self {
            Self::Recv(err) => match err {
                RecvError::LinkStateError(LinkStateError::IllegalSessionState) => true,
                _ => false,
            },
            Self::LinkState(IllegalLinkStateError::IllegalSessionState) => true,
            _ => false,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AmqpDispositionError {
    #[error(transparent)]
    IllegalState(#[from] IllegalLinkStateError),

    #[error(transparent)]
    RequestResponse(#[from] ManagementError),

    #[error(transparent)]
    Elapsed(#[from] Elapsed),
}

impl ServiceBusRetryPolicyError for AmqpDispositionError {
    fn is_scope_disposed(&self) -> bool {
        match self {
            Self::IllegalState(IllegalLinkStateError::IllegalSessionState) => true,
            Self::RequestResponse(err) => err.is_scope_disposed(),
            _ => false,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AmqpRequestResponseError {
    #[error(transparent)]
    RequestResponse(#[from] ManagementError),

    #[error(transparent)]
    Elapsed(#[from] Elapsed),
}

impl ServiceBusRetryPolicyError for AmqpRequestResponseError {
    fn is_scope_disposed(&self) -> bool {
        match self {
            Self::RequestResponse(err) => err.is_scope_disposed(),
            _ => false,
        }
    }
}

impl From<serde_amqp::Error> for AmqpRequestResponseError {
    fn from(_: serde_amqp::Error) -> Self {
        Self::RequestResponse(ManagementError::DecodeError(None))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CbsAuthError {
    #[error(transparent)]
    TokenCredential(#[from] azure_core::Error),

    #[error(transparent)]
    Cbs(#[from] ManagementError),
}

#[derive(Debug, thiserror::Error)]
pub enum TryAddMessageError {
    #[error("Message is too large to fit in a batch")]
    BatchFull(ServiceBusMessage),

    #[error("Cannot serialize message")]
    Codec {
        source: serde_amqp::Error,
        message: ServiceBusMessage,
    },
}

#[derive(Debug)]
pub struct RequestedSizeOutOfRange {}

impl std::fmt::Display for RequestedSizeOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Requested size is out of range")
    }
}

impl std::error::Error for RequestedSizeOutOfRange {}

#[derive(Debug, Clone, thiserror::Error)]
pub enum CorrelationFilterError {
    #[error("Correlation filter must include at least one entry")]
    EmptyFilter,
}

#[derive(Debug)]
pub struct AmqpCbsEventLoopStopped {}

impl std::fmt::Display for AmqpCbsEventLoopStopped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The CBS event loop has stopped")
    }
}

impl std::error::Error for AmqpCbsEventLoopStopped {}
