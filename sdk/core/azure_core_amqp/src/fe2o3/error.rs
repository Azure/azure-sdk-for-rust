// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::error::{AmqpDescribedError, AmqpDetachError, AmqpError, AmqpErrorKind};

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
    (AmqpOpen, fe2o3_amqp::connection::OpenError),
    (AmqpConnection, fe2o3_amqp::connection::Error),
    (AmqpBegin, fe2o3_amqp::session::BeginError)
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
    NotAccepted { source: AmqpNotAccepted },
    TimeError { source: TimeError },
    AmqpOpen { source: AmqpOpen },
    AmqpBegin { source: AmqpBegin },
    AmqpConnection { source: AmqpConnection },
    AmqpSession { source: AmqpSession },
}

pub struct Fe2o3AmqpError {
    kind: Fe2o3ErrorKind,
}

impl std::error::Error for Fe2o3AmqpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            Fe2o3ErrorKind::AmqpSerialization { source } => source.0.source(),
            Fe2o3ErrorKind::NotAccepted { source: _ } => None,
            Fe2o3ErrorKind::TimeError { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpOpen { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpBegin { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpConnection { source } => source.0.source(),
            Fe2o3ErrorKind::AmqpSession { source } => source.0.source(),
        };
        None
    }
}

impl std::fmt::Display for Fe2o3AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            Fe2o3ErrorKind::TimeError { source } => {
                write!(f, "Time Component Range Error {}", source.0)
            }
            Fe2o3ErrorKind::AmqpOpen { source } => {
                write!(f, "Connection Open Error: {}", source.0)
            }
            Fe2o3ErrorKind::AmqpBegin { source } => write!(f, "BeginError: {:?}", source.0),
            Fe2o3ErrorKind::AmqpConnection { source } => {
                write!(f, "Connection : {}", source.0)
            }
            Fe2o3ErrorKind::AmqpSession { source } => write!(f, "Session error: {:?}", source.0),
            Fe2o3ErrorKind::NotAccepted { source } => {
                write!(f, "Not accepted error: {:?}", source)
            }
            Fe2o3ErrorKind::AmqpSerialization { source } => {
                write!(f, "Serialization error: {}", source.0)
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

impl From<fe2o3_amqp::link::DetachError> for AmqpDetachError {
    fn from(e: fe2o3_amqp::link::DetachError) -> Self {
        match e {
            fe2o3_amqp::link::DetachError::IllegalState => Self::IllegalState,
            fe2o3_amqp::link::DetachError::IllegalSessionState => Self::IllegalSessionState,
            fe2o3_amqp::link::DetachError::RemoteDetachedWithError(error) => {
                Self::RemoteDetachedWithError(error.into())
            }
            fe2o3_amqp::link::DetachError::ClosedByRemote => Self::ClosedByRemote,
            fe2o3_amqp::link::DetachError::DetachedByRemote => Self::DetachedByRemote,
            fe2o3_amqp::link::DetachError::RemoteClosedWithError(error) => {
                Self::RemoteClosedWithError(error.into())
            }
        }
    }
}
