use fe2o3_amqp::link::ReceiverAttachError;
use fe2o3_amqp_management::error::AttachError;

use crate::amqp::error::{OpenReceiverError, CbsAuthError};

/// Error with accepting next session
#[derive(Debug, thiserror::Error)]
pub enum AcceptNextSessionError {
    #[error("The connection scope is disposed")]
    ScopeIsDisposed,

    #[error(transparent)]
    ManagemetnLinkAttach(#[from] AttachError),

    #[error(transparent)]
    ReceiverAttach(#[from] ReceiverAttachError),

    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),

    #[error("Session ID is not set")]
    SessionIdNotSet,
}

impl From<OpenReceiverError> for AcceptNextSessionError {
    fn from(error: OpenReceiverError) -> Self {
        match error {
            OpenReceiverError::ScopeIsDisposed => Self::ScopeIsDisposed,
            OpenReceiverError::ManagementLinkAttach(error) => Self::ManagemetnLinkAttach(error),
            OpenReceiverError::ReceiverAttach(error) => Self::ReceiverAttach(error),
            OpenReceiverError::CbsAuth(error) => Self::CbsAuth(error),
        }
    }
}
