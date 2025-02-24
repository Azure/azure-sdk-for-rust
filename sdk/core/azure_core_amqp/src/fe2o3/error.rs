// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::{AmqpDescribedError, AmqpErrorKind},
    AmqpError,
};

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
//    (AmqpOpen, fe2o3_amqp::connection::OpenError),
//    (AmqpConnection, fe2o3_amqp::connection::Error)
}

pub struct Fe2o3SerializationError(pub serde_amqp::error::Error);
impl From<serde_amqp::error::Error> for Fe2o3SerializationError {
    fn from(e: serde_amqp::error::Error) -> Self {
        Fe2o3SerializationError(e)
    }
}

pub struct Fe2o3ConnectionOpenError(pub fe2o3_amqp::connection::OpenError);
impl From<fe2o3_amqp::connection::OpenError> for Fe2o3ConnectionOpenError {
    fn from(e: fe2o3_amqp::connection::OpenError) -> Self {
        Fe2o3ConnectionOpenError(e)
    }
}

pub struct Fe2o3ConnectionError(pub fe2o3_amqp::connection::Error);
impl From<fe2o3_amqp::connection::Error> for Fe2o3ConnectionError {
    fn from(e: fe2o3_amqp::connection::Error) -> Self {
        Fe2o3ConnectionError(e)
    }
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
    NotAccepted { source: AmqpNotAccepted },
}

pub struct Fe2o3AmqpError {
    kind: Fe2o3ErrorKind,
}

impl std::error::Error for Fe2o3AmqpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for Fe2o3AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            Fe2o3ErrorKind::NotAccepted { source } => {
                write!(f, "Not accepted error: {:?}", source)
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

impl From<fe2o3_amqp_types::definitions::Error> for AmqpDescribedError {
    fn from(e: fe2o3_amqp_types::definitions::Error) -> Self {
        AmqpDescribedError::new(
            match e.condition {
                fe2o3_amqp_types::definitions::ErrorCondition::AmqpError(amqp_error) => {
                    fe2o3_amqp_types::primitives::Symbol::from(&amqp_error).into()
                }
                fe2o3_amqp_types::definitions::ErrorCondition::ConnectionError(
                    connection_error,
                ) => fe2o3_amqp_types::primitives::Symbol::from(&connection_error).into(),
                fe2o3_amqp_types::definitions::ErrorCondition::SessionError(session_error) => {
                    fe2o3_amqp_types::primitives::Symbol::from(&session_error).into()
                }
                fe2o3_amqp_types::definitions::ErrorCondition::LinkError(link_error) => {
                    fe2o3_amqp_types::primitives::Symbol::from(&link_error).into()
                }
                fe2o3_amqp_types::definitions::ErrorCondition::Custom(symbol) => symbol.into(),
            },
            e.description,
            e.info.unwrap_or_default().into(),
        )
    }
}

impl From<fe2o3_amqp::link::DetachError> for AmqpError {
    fn from(e: fe2o3_amqp::link::DetachError) -> Self {
        match e {
            fe2o3_amqp::link::DetachError::DetachedByRemote => {
                Self::from(AmqpErrorKind::DetachedByRemote)
            }
            fe2o3_amqp::link::DetachError::RemoteDetachedWithError(error) => {
                Self::from(AmqpErrorKind::DetachedByRemoteWithError(error.into()))
            }
            fe2o3_amqp::link::DetachError::ClosedByRemote => {
                Self::from(AmqpErrorKind::ClosedByRemote)
            }
            fe2o3_amqp::link::DetachError::RemoteClosedWithError(error) => {
                Self::from(AmqpErrorKind::ClosedByRemoteWithError(error.into()))
            }
            _ => Self::from(AmqpErrorKind::DetachError(Box::new(e))),
        }
    }
}
