//! Error types for the client

use fe2o3_amqp::link::ReceiverAttachError;
use fe2o3_amqp_management::error::AttachError;

use crate::{
    amqp::error::{CbsAuthError, OpenReceiverError},
    primitives::error::ClientDisposedError,
    util::IntoAzureCoreError,
};

/// Session ID from the serivce is not set
#[derive(Debug)]
pub struct SessionIdNotSet;

impl std::fmt::Display for SessionIdNotSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session ID from the serivce is not set")
    }
}

impl std::error::Error for SessionIdNotSet {}

impl From<SessionIdNotSet> for azure_core::Error {
    fn from(value: SessionIdNotSet) -> Self {
        azure_core::Error::new(azure_core::error::ErrorKind::Other, value)
    }
}

/// Error with accepting next session
#[derive(Debug, thiserror::Error)]
pub(crate) enum AcceptNextSessionError {
    /// The connection scope is disposed
    #[error("The connection scope is disposed")]
    ConnectionScopeDisposed,

    /// Error with management link attach
    #[error(transparent)]
    ManagementLinkAttach(#[from] AttachError),

    /// Error with receiver attach
    #[error(transparent)]
    ReceiverAttach(#[from] ReceiverAttachError),

    /// Error with CBS auth
    #[error(transparent)]
    CbsAuth(#[from] CbsAuthError),

    /// Session ID from the service is not set
    #[error("Session ID from the serivce is not set")]
    SessionIdNotSet,
}

impl From<AcceptNextSessionError> for azure_core::Error {
    fn from(value: AcceptNextSessionError) -> Self {
        match value {
            AcceptNextSessionError::ConnectionScopeDisposed => ClientDisposedError.into(),
            AcceptNextSessionError::ManagementLinkAttach(error) => error.into_azure_core_error(),
            AcceptNextSessionError::ReceiverAttach(error) => error.into_azure_core_error(),
            AcceptNextSessionError::CbsAuth(error) => error.into(),
            AcceptNextSessionError::SessionIdNotSet => SessionIdNotSet.into(),
        }
    }
}

impl From<OpenReceiverError> for AcceptNextSessionError {
    fn from(error: OpenReceiverError) -> Self {
        match error {
            OpenReceiverError::ConnectionScopeDisposed => Self::ConnectionScopeDisposed,
            OpenReceiverError::ManagementLinkAttach(error) => Self::ManagementLinkAttach(error),
            OpenReceiverError::ReceiverAttach(error) => Self::ReceiverAttach(error),
            OpenReceiverError::CbsAuth(error) => Self::CbsAuth(error),
        }
    }
}
