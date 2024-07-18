// Copyright (c) Microsoft Corp. All Rights Reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.

// cspell: words thiserror eventhubs amqp

macro_rules! impl_from_external_error {
    ($amqp_error:ident, $foreign_error:ty) => {
    pub struct $amqp_error(pub $foreign_error);

    impl From<$foreign_error> for $amqp_error {
        fn from(e: $foreign_error) -> Self {
            $amqp_error(e)
        }
    }

    impl From<$amqp_error> for Fe2o3AmqpError {
        fn from(e: $amqp_error) -> Self {
            Fe2o3AmqpError {
                kind: ErrorKind::$amqp_error { source: e },
            }
        }
    }

    impl From<$foreign_error> for Fe2o3AmqpError {
        fn from(e: $foreign_error) -> Self {
            Fe2o3AmqpError {
                kind: ErrorKind::$amqp_error {
                    source: $amqp_error(e),
                },
            }
        }
    }

    impl From<$amqp_error> for azure_core::Error {
        fn from(e: $amqp_error) -> Self {
            Self::new(
                azure_core::error::ErrorKind::Other,
                Box::new(Fe2o3AmqpError::from(e)),
            )
        }
    }

    impl std::fmt::Debug for $amqp_error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
};

    ($($amqp_error:ident, $foreign_error:ty),*) => {
        $(impl_from_external_error!($amqp_error, $foreign_error);)*
    }
}
impl_from_external_error! {
    AmqpSerializationError, serde_amqp::error::Error,
    TimeError, time::error::ComponentRange,
    AmqpSessionError, fe2o3_amqp::session::Error,
    AmqpLinkDetachError, fe2o3_amqp::link::DetachError,
    AmqpOpenError, fe2o3_amqp::connection::OpenError,
    AmqpConnectionError, fe2o3_amqp::connection::Error,
    AmqpManagementAttachError, fe2o3_amqp_management::error::AttachError,
    AmqpManagementError, fe2o3_amqp_management::error::Error,
    AmqpBeginError, fe2o3_amqp::session::BeginError,
    AmqpSenderAttachError, fe2o3_amqp::link::SenderAttachError,
    AmqpReceiverAttachError, fe2o3_amqp::link::ReceiverAttachError,
    AmqpReceiverError, fe2o3_amqp::link::RecvError,
    AmqpIllegalLinkStateError, fe2o3_amqp::link::IllegalLinkStateError,
    AmqpSenderSendError, fe2o3_amqp::link::SendError,
    AmqpDeliveryRejectedError, fe2o3_amqp::types::messaging::Rejected
}

#[derive(Debug)]
pub enum AmqpNotAcceptedError {
    AmqpNotAcceptedError {
        source: fe2o3_amqp_types::messaging::Rejected,
    },
    AmqpReleasedError {
        source: fe2o3_amqp_types::messaging::Released,
    },
    AmqpModifiedError {
        source: fe2o3_amqp_types::messaging::Modified,
    },
}

pub enum ErrorKind {
    AmqpSerializationError { source: AmqpSerializationError },
    AmqpDeliveryRejectedError { source: AmqpDeliveryRejectedError },
    NotAcceptedError { source: AmqpNotAcceptedError },
    TimeError { source: TimeError },
    AmqpOpenError { source: AmqpOpenError },
    AmqpManagementAttachError { source: AmqpManagementAttachError },
    AmqpBeginError { source: AmqpBeginError },
    AmqpManagementError { source: AmqpManagementError },
    AmqpConnectionError { source: AmqpConnectionError },
    AmqpLinkDetachError { source: AmqpLinkDetachError },
    AmqpSessionError { source: AmqpSessionError },
    AmqpSenderAttachError { source: AmqpSenderAttachError },
    AmqpSenderSendError { source: AmqpSenderSendError },
    AmqpReceiverAttachError { source: AmqpReceiverAttachError },
    AmqpReceiverError { source: AmqpReceiverError },
    AmqpIllegalLinkStateError { source: AmqpIllegalLinkStateError },
}

pub struct Fe2o3AmqpError {
    kind: ErrorKind,
}

impl std::error::Error for Fe2o3AmqpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for Fe2o3AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::TimeError { source } => {
                write!(f, "Time Component Range Error {:?}", source.0)
            }
            ErrorKind::AmqpIllegalLinkStateError { source } => {
                write!(f, "Illegal Link State Error {:?}", source.0)
            }
            ErrorKind::AmqpDeliveryRejectedError { source } => {
                write!(f, "Delivery Rejected Error: {:?}", source.0)
            }
            ErrorKind::AmqpOpenError { source } => {
                write!(f, "Connection Open Error: {:?}", source.0)
            }
            ErrorKind::AmqpManagementAttachError { source } => {
                write!(f, "AttachError: {:?}", source.0)
            }
            ErrorKind::AmqpLinkDetachError { source } => {
                write!(f, "Link Detach Error: {:?}", source.0)
            }
            ErrorKind::AmqpReceiverAttachError { source } => {
                write!(f, "Receiver attach error {:?}", source.0)
            }
            ErrorKind::AmqpBeginError { source } => write!(f, "BeginError: {:?}", source.0),
            ErrorKind::AmqpManagementError { source } => {
                write!(f, "ManagementError: {:?}", source.0)
            }
            ErrorKind::AmqpConnectionError { source } => {
                write!(f, "ConnectionError: {:?}", source.0)
            }
            ErrorKind::AmqpSessionError { source } => write!(f, "Session error: {:?}", source.0),
            ErrorKind::AmqpSenderAttachError { source } => {
                write!(f, "Sender attach error {:?}", source.0)
            }
            // ErrorKind::AmqpSerializationError { source } => {
            ErrorKind::NotAcceptedError { source } => {
                write!(f, "Not accepted error: {:?}", source)
            }
            ErrorKind::AmqpReceiverError { source } => {
                write!(f, "Receiver error: {:?}", source.0)
            }
            ErrorKind::AmqpSenderSendError { source } => {
                write!(f, "Sender send error {:?}", source.0)
            }
            ErrorKind::AmqpSerializationError { source } => {
                write!(f, "Serialization error: {:?}", source.0)
            }
        }
    }
}

impl std::fmt::Debug for Fe2o3AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fe2o3 Error: {}", self)
    }
}

impl From<Fe2o3AmqpError> for azure_core::Error {
    fn from(e: Fe2o3AmqpError) -> Self {
        Self::new(azure_core::error::ErrorKind::Other, Box::new(e))
    }
}

impl From<ErrorKind> for azure_core::Error {
    fn from(e: ErrorKind) -> Self {
        Self::new(
            azure_core::error::ErrorKind::Other,
            Box::new(Fe2o3AmqpError { kind: e }),
        )
    }
}
