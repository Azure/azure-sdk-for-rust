use fe2o3_amqp::{
    connection,
    link::{
        IllegalLinkStateError, LinkStateError, ReceiverAttachError, RecvError,
        SenderAttachError,
    },
    session,
};
use fe2o3_amqp_management::error::{AttachError, Error as ManagementError};
use fe2o3_amqp_types::messaging::{Modified, Rejected, Released};
use tokio::time::error::Elapsed;

use crate::{primitives::service_bus_retry_policy::ServiceBusRetryPolicyError, ServiceBusMessage};

use super::amqp_message_converter::InvalidLockTokenError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("PartitionKey cannot be set to a different value than SessionId")]
    PartitionKeyAndSessionIdAreDifferent,

    #[error("The message is a raw AMQP message")]
    RawAmqpMessage,
}

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
    ManagemetnLinkAttach(#[from] AttachError),

    #[error(transparent)]
    ReceiverAttach(#[from] ReceiverAttachError),

    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),
}

impl From<OpenMgmtLinkError> for OpenReceiverError {
    fn from(err: OpenMgmtLinkError) -> Self {
        match err {
            OpenMgmtLinkError::ScopeIsDisposed => OpenReceiverError::ScopeIsDisposed,
            OpenMgmtLinkError::Attach(err) => OpenReceiverError::ManagemetnLinkAttach(err),
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
        use fe2o3_amqp::link::{LinkStateError, SendError};
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
    #[error("Lock token cannot be converted from AMQP message")]
    InvalidLockTokenError,

    #[error(transparent)]
    Recv(#[from] RecvError),

    #[error(transparent)]
    LinkState(#[from] IllegalLinkStateError),

    #[error(transparent)]
    Elapsed(#[from] Elapsed),
}

impl From<InvalidLockTokenError> for AmqpRecvError {
    fn from(_: InvalidLockTokenError) -> Self {
        Self::InvalidLockTokenError
    }
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
    Elapsed(#[from] Elapsed),
}

impl ServiceBusRetryPolicyError for AmqpDispositionError {
    fn is_scope_disposed(&self) -> bool {
        matches!(
            self,
            Self::IllegalState(IllegalLinkStateError::IllegalSessionState)
        )
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

#[derive(Debug, thiserror::Error)]
pub enum CbsAuthError {
    #[error(transparent)]
    TokenCredential(#[from] azure_core::Error),

    #[error(transparent)]
    ExpirationDateTimeRange(#[from] time::error::ComponentRange),

    #[error(transparent)]
    Cbs(#[from] ManagementError),
}

#[derive(Debug, thiserror::Error)]
pub enum TryAddMessageError {
    #[error("Message is too large to fit in a batch")]
    BatchFull(ServiceBusMessage),

    #[error(transparent)]
    Codec(#[from] serde_amqp::Error),
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
