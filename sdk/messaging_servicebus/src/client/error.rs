//! Error types for the client

use fe2o3_amqp::link::ReceiverAttachError;
use fe2o3_amqp_management::error::AttachError;

use crate::amqp::error::{CbsAuthError, OpenReceiverError};

/// Error with accepting next session
#[derive(Debug, thiserror::Error)]
pub enum AcceptNextSessionError {
    /// The connection scope is disposed
    #[error("The connection scope is disposed")]
    ScopeIsDisposed,

    /// Error with management link attach
    #[error(transparent)]
    ManagementLinkAttach(#[from] AttachError),

    /// Error with receiver attach
    #[error(transparent)]
    ReceiverAttach(#[from] ReceiverAttachError),

    /// Error with CBS authentication
    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),

    /// Session ID from the service is not set
    #[error("Session ID from the serivce is not set")]
    SessionIdNotSet,
}

impl From<OpenReceiverError> for AcceptNextSessionError {
    fn from(error: OpenReceiverError) -> Self {
        match error {
            OpenReceiverError::ScopeIsDisposed => Self::ScopeIsDisposed,
            OpenReceiverError::ManagementLinkAttach(error) => Self::ManagementLinkAttach(error),
            OpenReceiverError::ReceiverAttach(error) => Self::ReceiverAttach(error),
            OpenReceiverError::CbsAuth(error) => Self::CbsAuth(error),
        }
    }
}
