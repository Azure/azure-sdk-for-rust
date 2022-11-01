use fe2o3_amqp::{connection, link::SenderAttachError, session};
use fe2o3_amqp_management::error::Error as MgmtError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("PartitionKey cannot be set to a different value than SessionId")]
    PartitionKeyAndSessionIdAreDifferent,

    #[error("The message is a raw AMQP message")]
    RawAmqpMessage,
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
pub enum OpenSenderError {
    #[error("The connection scope is disposed")]
    ScopeIsDisposed,

    #[error(transparent)]
    Attach(#[from] SenderAttachError),
}

#[derive(Debug, thiserror::Error)]
pub enum CbsAuthError {
    #[error(transparent)]
    TokenCredential(#[from] azure_core::Error),

    #[error(transparent)]
    Cbs(#[from] MgmtError),
}
