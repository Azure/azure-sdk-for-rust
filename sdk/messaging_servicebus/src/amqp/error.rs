use fe2o3_amqp::{
    connection,
    link::{ReceiverAttachError, SenderAttachError},
    session,
};
use fe2o3_amqp_management::error::{AttachError, Error as MgmtError};
use fe2o3_amqp_types::messaging::{Modified, Rejected, Released};

use crate::ServiceBusMessage;

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

// #[inline]
// pub(crate) fn not_supported_error(field_type: &str, method: &str, alternative: &str) -> Error {
//     Error::NotSupported(
//         format!("{field_type} cannot be retrived using {method} method. Use {alternative} to access the underlying Amqp Message")
//     )
// }

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
pub enum CbsAuthError {
    #[error(transparent)]
    TokenCredential(#[from] azure_core::Error),

    #[error(transparent)]
    ExpirationDateTimeRange(#[from] time::error::ComponentRange),

    #[error(transparent)]
    Cbs(#[from] MgmtError),
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
