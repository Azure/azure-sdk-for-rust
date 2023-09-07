//! Implements `IntoAzureCoreError` for external error types

//! Implements conversion to azure_core::Error for external error types.
//!
//! TODO: should all AMQP related errors be categorized as `ErrorKind::Io`?

use super::IntoAzureCoreError;
use fe2o3_amqp::{
    connection::OpenError,
    link::{
        DetachError, IllegalLinkStateError, ReceiverAttachError, ReceiverResumeErrorKind,
        RecvError, SendError, SenderAttachError, SenderResumeErrorKind,
    },
    session::BeginError,
};
use fe2o3_amqp_management::error::{
    AttachError as ManagementAttachError, Error as ManagementError,
};

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
            fe2o3_amqp::transport::Error::DecodeError(_) => {
                azure_core::Error::new(ErrorKind::DataConversion, self)
            }
            fe2o3_amqp::transport::Error::NotImplemented(_) => {
                azure_core::Error::new(ErrorKind::Other, self)
            }
            fe2o3_amqp::transport::Error::IdleTimeoutElapsed
            | fe2o3_amqp::transport::Error::FramingError => {
                azure_core::Error::new(ErrorKind::Io, self)
            }
        }
    }
}

impl IntoAzureCoreError for fe2o3_amqp_ws::Error {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;
        use fe2o3_amqp_ws::Error;

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
            | SenderAttachError::RemoteClosedWithError(_) => {
                azure_core::Error::new(ErrorKind::Io, self)
            }
            _ => azure_core::Error::new(ErrorKind::Other, self),
        }
    }
}

impl IntoAzureCoreError for ReceiverAttachError {
    fn into_azure_core_error(self) -> azure_core::Error {
        match self {
            ReceiverAttachError::IllegalSessionState
            | ReceiverAttachError::IllegalState
            | ReceiverAttachError::RemoteClosedWithError(_) => {
                azure_core::Error::new(azure_core::error::ErrorKind::Io, self)
            }
            _ => azure_core::Error::new(azure_core::error::ErrorKind::Other, self),
        }
    }
}

impl IntoAzureCoreError for fe2o3_amqp::connection::Error {
    fn into_azure_core_error(self) -> azure_core::Error {
        match self {
            fe2o3_amqp::connection::Error::TransportError(err) => err.into_azure_core_error(),
            fe2o3_amqp::connection::Error::IllegalState
            | fe2o3_amqp::connection::Error::RemoteClosed
            | fe2o3_amqp::connection::Error::RemoteClosedWithError(_) => {
                azure_core::Error::new(azure_core::error::ErrorKind::Io, self)
            }
            fe2o3_amqp::connection::Error::NotImplemented(_)
            | fe2o3_amqp::connection::Error::NotFound(_)
            | fe2o3_amqp::connection::Error::NotAllowed(_)
            | fe2o3_amqp::connection::Error::JoinError(_) => {
                azure_core::Error::new(azure_core::error::ErrorKind::Other, self)
            }
        }
    }
}

impl IntoAzureCoreError for fe2o3_amqp::session::Error {
    fn into_azure_core_error(self) -> azure_core::Error {
        match self {
            fe2o3_amqp::session::Error::IllegalState
            | fe2o3_amqp::session::Error::IllegalConnectionState
            | fe2o3_amqp::session::Error::RemoteEndedWithError(_)
            | fe2o3_amqp::session::Error::RemoteEnded => {
                azure_core::Error::new(azure_core::error::ErrorKind::Io, self)
            }
            _ => azure_core::Error::new(azure_core::error::ErrorKind::Other, self),
        }
    }
}

impl IntoAzureCoreError for DetachError {
    fn into_azure_core_error(self) -> azure_core::Error {
        azure_core::Error::new(azure_core::error::ErrorKind::Io, self)
    }
}

impl IntoAzureCoreError for SendError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            SendError::LinkStateError(_)
            | SendError::Detached(_)
            | SendError::NonTerminalDeliveryState
            | SendError::IllegalDeliveryState => azure_core::Error::new(ErrorKind::Io, self),
            SendError::MessageEncodeError => {
                azure_core::Error::new(ErrorKind::DataConversion, self)
            }
        }
    }
}

impl IntoAzureCoreError for SenderResumeErrorKind {
    fn into_azure_core_error(self) -> azure_core::Error {
        match self {
            SenderResumeErrorKind::AttachError(err) => err.into_azure_core_error(),
            SenderResumeErrorKind::SendError(err) => err.into_azure_core_error(),
            SenderResumeErrorKind::DetachError(err) => err.into_azure_core_error(),
            SenderResumeErrorKind::Timeout => {
                azure_core::Error::new(azure_core::error::ErrorKind::Other, self)
            }
        }
    }
}

impl IntoAzureCoreError for IllegalLinkStateError {
    fn into_azure_core_error(self) -> azure_core::Error {
        azure_core::Error::new(azure_core::error::ErrorKind::Io, self)
    }
}

impl IntoAzureCoreError for ReceiverResumeErrorKind {
    fn into_azure_core_error(self) -> azure_core::Error {
        match self {
            ReceiverResumeErrorKind::AttachError(err) => err.into_azure_core_error(),
            ReceiverResumeErrorKind::FlowError(err) => err.into_azure_core_error(),
            ReceiverResumeErrorKind::DetachError(err) => err.into_azure_core_error(),
            ReceiverResumeErrorKind::Timeout => {
                azure_core::Error::new(azure_core::error::ErrorKind::Other, self)
            }
        }
    }
}

impl IntoAzureCoreError for RecvError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            RecvError::LinkStateError(_) => azure_core::Error::new(ErrorKind::Io, self),
            RecvError::MessageDecodeError => {
                azure_core::Error::new(ErrorKind::DataConversion, self)
            }
            _ => azure_core::Error::new(ErrorKind::Other, self),
        }
    }
}

impl IntoAzureCoreError for serde_amqp::Error {
    fn into_azure_core_error(self) -> azure_core::Error {
        azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, self)
    }
}

impl IntoAzureCoreError for ManagementError {
    fn into_azure_core_error(self) -> azure_core::Error {
        use azure_core::error::ErrorKind;

        match self {
            ManagementError::Send(_) | ManagementError::Recv(_) => {
                azure_core::Error::new(ErrorKind::Io, self)
            }
            _ => azure_core::Error::new(ErrorKind::Other, self),
        }
    }
}

impl IntoAzureCoreError for ManagementAttachError {
    fn into_azure_core_error(self) -> azure_core::Error {
        match self {
            ManagementAttachError::Sender(error) => error.into_azure_core_error(),
            ManagementAttachError::Receiver(error) => error.into_azure_core_error(),
        }
    }
}
