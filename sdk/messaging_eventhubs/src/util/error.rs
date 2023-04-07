//! Implements conversion to azure_core::Error for external error types.
//!
//! TODO: should all AMQP related errors be categorized as `ErrorKind::Io`?

use fe2o3_amqp::{session::BeginError, connection::OpenError, link::{SenderAttachError, ReceiverAttachError}};

use super::IntoAzureCoreError;

impl IntoAzureCoreError for BeginError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            BeginError::LocalChannelMaxReached => azure_core::Error::new(ErrorKind::Other, self),
            BeginError::IllegalState
            | BeginError::IllegalConnectionState
            | BeginError::RemoteEnded
            | BeginError::RemoteEndedWithError(_) => azure_core::Error::new(ErrorKind::Io, self),
        }
    }
}

impl IntoAzureCoreError for OpenError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            OpenError::Io(err) => azure_core::Error::new(ErrorKind::Io, err),
            OpenError::TransportError(err) => err.into_azure_core_error(),
            OpenError::ProtocolHeaderMismatch(_)
            | OpenError::IllegalState
            | OpenError::RemoteClosed
            | OpenError::RemoteClosedWithError(_) => azure_core::Error::new(ErrorKind::Io, self),
            OpenError::SaslError { .. } => azure_core::Error::new(ErrorKind::Credential, self),
            OpenError::DecodeError(_) => azure_core::Error::new(ErrorKind::DataConversion, self),
            OpenError::UrlError(err) => err.into(),
            OpenError::InvalidDomain
            | OpenError::InvalidScheme
            | OpenError::TlsConnectorNotFound
            | OpenError::NotImplemented(_) => azure_core::Error::new(ErrorKind::Other, self),
        }
    }
}

impl IntoAzureCoreError for fe2o3_amqp::transport::Error {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            fe2o3_amqp::transport::Error::Io(err) => azure_core::Error::new(ErrorKind::Io, err),
            fe2o3_amqp::transport::Error::DecodeError(_) => azure_core::Error::new(ErrorKind::DataConversion, self),
            fe2o3_amqp::transport::Error::NotImplemented(_) => azure_core::Error::new(ErrorKind::Other, self),
            fe2o3_amqp::transport::Error::IdleTimeoutElapsed
            | fe2o3_amqp::transport::Error::FramingError => azure_core::Error::new(ErrorKind::Io, self),
        }
    }
}

impl IntoAzureCoreError for fe2o3_amqp_ws::Error {
    fn into_azure_core_error(self) -> azure_core::Error {
        use fe2o3_amqp_ws::Error;
        use azure_core::error::ErrorKind;

        match self {
            Error::Io(err) => azure_core::Error::new(ErrorKind::Io, err),
            _ => azure_core::Error::new(ErrorKind::Io, self),
        }
    }
}

impl IntoAzureCoreError for SenderAttachError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            SenderAttachError::IllegalSessionState
            | SenderAttachError::IllegalState
            | SenderAttachError::RemoteClosedWithError(_) => azure_core::Error::new(ErrorKind::Io, self),
            _ => azure_core::Error::new(ErrorKind::Other, self),
        }
    }
}

impl IntoAzureCoreError for ReceiverAttachError {
    fn into_azure_core_error(self) -> azure_core::Error {
        match self {
            ReceiverAttachError::IllegalSessionState
            | ReceiverAttachError::IllegalState
            | ReceiverAttachError::RemoteClosedWithError(_) => azure_core::Error::new(azure_core::error::ErrorKind::Io, self),
            _ => azure_core::Error::new(azure_core::error::ErrorKind::Other, self),
        }
    }
}
