// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::error::{AmqpError, AmqpErrorKind};

macro_rules! impl_from_external_error {
    ($(($amqp_error:ident, $foreign_error:ty)),*) => {
        $(
            pub struct $amqp_error(pub $foreign_error);

            impl From<$foreign_error> for $amqp_error {
                fn from(e: $foreign_error) -> Self {
                    $amqp_error(e)
                }
            }

            impl From<$amqp_error> for Fe2o3AmqpError {
                fn from(e: $amqp_error) -> Self {
                    Fe2o3AmqpError {
                        kind: Fe2o3ErrorKind::$amqp_error { source: e },
                    }
                }
            }

            impl From<$foreign_error> for Fe2o3AmqpError {
                fn from(e: $foreign_error) -> Self {
                    Fe2o3AmqpError {
                        kind: Fe2o3ErrorKind::$amqp_error {
                            source: $amqp_error(e),
                        },
                    }
                }
            }

            impl From<$amqp_error> for azure_core::Error {
                fn from(e: $amqp_error) -> Self {
                    let fe = Fe2o3AmqpError::from(e);
                    let ak = AmqpErrorKind::from(fe);
                    Self::new(
                        azure_core::error::ErrorKind::Amqp,
                        AmqpError::new(ak),
                    )
                }
            }

            impl std::fmt::Debug for $amqp_error {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:?}", self.0)
                }
            }
        )*
    }
}

impl_from_external_error! {
    (AmqpSerialization, serde_amqp::error::Error),
    (TimeError, time::error::ComponentRange),
    (AmqpSession, fe2o3_amqp::session::Error),
    (AmqpLinkDetach, fe2o3_amqp::link::DetachError),
    (AmqpOpen, fe2o3_amqp::connection::OpenError),
    (AmqpConnection, fe2o3_amqp::connection::Error),
    (AmqpManagementAttach, fe2o3_amqp_management::error::AttachError),
    (AmqpManagement, fe2o3_amqp_management::error::Error),
    (AmqpBegin, fe2o3_amqp::session::BeginError),
    (AmqpSenderAttach, fe2o3_amqp::link::SenderAttachError),
    (AmqpReceiverAttach, fe2o3_amqp::link::ReceiverAttachError),
    (AmqpReceiver, fe2o3_amqp::link::RecvError),
    (AmqpIllegalLinkState, fe2o3_amqp::link::IllegalLinkStateError),
    (AmqpSenderSend, fe2o3_amqp::link::SendError),
    (AmqpDeliveryRejected, fe2o3_amqp::types::messaging::Rejected)
}

#[derive(Debug)]
pub enum AmqpNotAccepted {
    Rejected {
        source: fe2o3_amqp_types::messaging::Rejected,
    },
    Released {
        source: fe2o3_amqp_types::messaging::Released,
    },
    Modified {
        source: fe2o3_amqp_types::messaging::Modified,
    },
}

impl From<fe2o3_amqp_types::messaging::Outcome> for AmqpNotAccepted {
    fn from(outcome: fe2o3_amqp_types::messaging::Outcome) -> Self {
        match outcome {
            fe2o3_amqp_types::messaging::Outcome::Accepted(_) => {
                panic!("Accepted outcomes should not be converted to errors")
            }
            fe2o3_amqp_types::messaging::Outcome::Rejected(rejected) => {
                AmqpNotAccepted::Rejected { source: rejected }
            }
            fe2o3_amqp_types::messaging::Outcome::Released(released) => {
                AmqpNotAccepted::Released { source: released }
            }
            fe2o3_amqp_types::messaging::Outcome::Modified(modified) => {
                AmqpNotAccepted::Modified { source: modified }
            }
        }
    }
}

impl From<AmqpNotAccepted> for Fe2o3AmqpError {
    fn from(e: AmqpNotAccepted) -> Self {
        Fe2o3AmqpError {
            kind: Fe2o3ErrorKind::NotAccepted { source: e },
        }
    }
}

pub enum Fe2o3ErrorKind {
    AmqpSerialization { source: AmqpSerialization },
    AmqpDeliveryRejected { source: AmqpDeliveryRejected },
    NotAccepted { source: AmqpNotAccepted },
    TimeError { source: TimeError },
    AmqpOpen { source: AmqpOpen },
    AmqpManagementAttach { source: AmqpManagementAttach },
    AmqpBegin { source: AmqpBegin },
    AmqpManagement { source: AmqpManagement },
    AmqpConnection { source: AmqpConnection },
    AmqpLinkDetach { source: AmqpLinkDetach },
    AmqpSession { source: AmqpSession },
    AmqpSenderAttach { source: AmqpSenderAttach },
    AmqpSenderSend { source: AmqpSenderSend },
    AmqpReceiverAttach { source: AmqpReceiverAttach },
    AmqpReceiver { source: AmqpReceiver },
    AmqpIllegalLinkState { source: AmqpIllegalLinkState },
}

pub struct Fe2o3AmqpError {
    kind: Fe2o3ErrorKind,
}

impl std::error::Error for Fe2o3AmqpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            Fe2o3ErrorKind::AmqpSerialization { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpDeliveryRejected { source: _ } => None,
            Fe2o3ErrorKind::NotAccepted { source: _ } => None,
            Fe2o3ErrorKind::TimeError { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpOpen { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpManagementAttach { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpBegin { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpManagement { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpConnection { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpLinkDetach { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpSession { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpSenderAttach { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpSenderSend { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpReceiverAttach { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpReceiver { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpIllegalLinkState { source } => source.0.source(),
        };
        None
    }
}

impl std::fmt::Display for Fe2o3AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            Fe2o3ErrorKind::TimeError { source } => {
                write!(f, "Time Component Range Error {:?}", source.0)
            }
            Fe2o3ErrorKind::AmqpIllegalLinkState { source } => {
                write!(f, "Illegal Link State Error {:?}", source.0)
            }
            Fe2o3ErrorKind::AmqpDeliveryRejected { source } => {
                write!(f, "Delivery Rejected Error: {:?}", source.0)
            }
            Fe2o3ErrorKind::AmqpOpen { source } => {
                write!(f, "Connection Open Error: {:?}", source.0)
            }
            Fe2o3ErrorKind::AmqpManagementAttach { source } => {
                write!(f, "Management Attach Error: {:?}", source.0)
            }
            Fe2o3ErrorKind::AmqpLinkDetach { source } => {
                write!(f, "Link Detach Error: {:?}", source.0)
            }
            Fe2o3ErrorKind::AmqpReceiverAttach { source } => {
                write!(f, "Receiver attach error {:?}", source.0)
            }
            Fe2o3ErrorKind::AmqpBegin { source } => write!(f, "BeginError: {:?}", source.0),
            Fe2o3ErrorKind::AmqpManagement { source } => {
                write!(f, "Management Error: {:?}", source.0)
            }
            Fe2o3ErrorKind::AmqpConnection { source } => {
                write!(f, "Connection : {:?}", source.0)
            }
            Fe2o3ErrorKind::AmqpSession { source } => write!(f, "Session error: {:?}", source.0),
            Fe2o3ErrorKind::AmqpSenderAttach { source } => {
                write!(f, "Sender attach error {:?}", source.0)
            }
            // ErrorKind::AmqpSerializationError { source } => {
            Fe2o3ErrorKind::NotAccepted { source } => {
                write!(f, "Not accepted error: {:?}", source)
            }
            Fe2o3ErrorKind::AmqpReceiver { source } => {
                write!(f, "Receiver error: {:?}", source.0)
            }
            Fe2o3ErrorKind::AmqpSenderSend { source } => {
                write!(f, "Sender send error {:?}", source.0)
            }
            Fe2o3ErrorKind::AmqpSerialization { source } => {
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

impl From<Fe2o3AmqpError> for AmqpErrorKind {
    fn from(e: Fe2o3AmqpError) -> Self {
        AmqpErrorKind::TransportImplementationError {
            source: Box::new(e),
        }
    }
}
