// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::{AmqpConnectionError, AmqpDescribedError, AmqpErrorKind},
    AmqpError,
};

pub(crate) struct Fe2o3SerializationError(pub serde_amqp::error::Error);
impl From<serde_amqp::error::Error> for Fe2o3SerializationError {
    fn from(e: serde_amqp::error::Error) -> Self {
        Fe2o3SerializationError(e)
    }
}

pub(crate) struct Fe2o3ConnectionOpenError(pub fe2o3_amqp::connection::OpenError);
impl From<fe2o3_amqp::connection::OpenError> for Fe2o3ConnectionOpenError {
    fn from(e: fe2o3_amqp::connection::OpenError) -> Self {
        Fe2o3ConnectionOpenError(e)
    }
}

pub(crate) struct Fe2o3ConnectionError(pub fe2o3_amqp::connection::Error);
impl From<fe2o3_amqp::connection::Error> for Fe2o3ConnectionError {
    fn from(e: fe2o3_amqp::connection::Error) -> Self {
        Fe2o3ConnectionError(e)
    }
}

pub(crate) struct Fe2o3ReceiverError(pub fe2o3_amqp::link::RecvError);
impl From<fe2o3_amqp::link::RecvError> for Fe2o3ReceiverError {
    fn from(e: fe2o3_amqp::link::RecvError) -> Self {
        Fe2o3ReceiverError(e)
    }
}

pub(crate) struct Fe2o3ReceiverAttachError(pub fe2o3_amqp::link::ReceiverAttachError);
impl From<fe2o3_amqp::link::ReceiverAttachError> for Fe2o3ReceiverAttachError {
    fn from(e: fe2o3_amqp::link::ReceiverAttachError) -> Self {
        Fe2o3ReceiverAttachError(e)
    }
}

pub(crate) struct Fe2o3LinkStateError(pub fe2o3_amqp::link::LinkStateError);
impl From<fe2o3_amqp::link::LinkStateError> for Fe2o3LinkStateError {
    fn from(e: fe2o3_amqp::link::LinkStateError) -> Self {
        Fe2o3LinkStateError(e)
    }
}

pub(crate) struct Fe2o3IllegalLinkStateError(pub fe2o3_amqp::link::IllegalLinkStateError);
impl From<fe2o3_amqp::link::IllegalLinkStateError> for Fe2o3IllegalLinkStateError {
    fn from(e: fe2o3_amqp::link::IllegalLinkStateError) -> Self {
        Fe2o3IllegalLinkStateError(e)
    }
}

pub(crate) struct Fe2o3ManagementError(pub fe2o3_amqp_management::error::Error);
impl From<fe2o3_amqp_management::error::Error> for Fe2o3ManagementError {
    fn from(e: fe2o3_amqp_management::error::Error) -> Self {
        Fe2o3ManagementError(e)
    }
}

pub(crate) struct Fe2o3TransportError(pub fe2o3_amqp::transport::Error);
impl From<fe2o3_amqp::transport::Error> for Fe2o3TransportError {
    fn from(e: fe2o3_amqp::transport::Error) -> Self {
        Fe2o3TransportError(e)
    }
}

// Specializations of From for common AMQP types.
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
                Self::from(AmqpErrorKind::DetachedByRemote(None))
            }
            fe2o3_amqp::link::DetachError::RemoteDetachedWithError(error) => {
                Self::from(AmqpErrorKind::DetachedByRemote(Some(error.into())))
            }
            fe2o3_amqp::link::DetachError::ClosedByRemote => {
                Self::from(AmqpErrorKind::ClosedByRemote(None))
            }
            fe2o3_amqp::link::DetachError::RemoteClosedWithError(error) => {
                Self::from(AmqpErrorKind::ClosedByRemote(Some(error.into())))
            }
            _ => Self::from(AmqpErrorKind::DetachError(Box::new(e))),
        }
    }
}

impl From<Fe2o3LinkStateError> for azure_core::Error {
    fn from(e: Fe2o3LinkStateError) -> Self {
        AmqpErrorKind::LinkStateError(e.0.into()).into()
    }
}

impl From<fe2o3_amqp::link::LinkStateError> for AmqpError {
    fn from(e: fe2o3_amqp::link::LinkStateError) -> Self {
        match e {
            fe2o3_amqp::link::LinkStateError::RemoteClosedWithError(e) => {
                AmqpErrorKind::ClosedByRemote(Some(e.into())).into()
            }
            fe2o3_amqp::link::LinkStateError::RemoteDetachedWithError(e) => {
                AmqpErrorKind::DetachedByRemote(Some(e.into())).into()
            }
            fe2o3_amqp::link::LinkStateError::RemoteClosed => {
                AmqpErrorKind::ClosedByRemote(None).into()
            }
            fe2o3_amqp::link::LinkStateError::RemoteDetached => {
                AmqpErrorKind::DetachedByRemote(None).into()
            }
            _ => AmqpErrorKind::LinkStateError(e.into()).into(),
        }
    }
}

impl From<Fe2o3IllegalLinkStateError> for azure_core::Error {
    fn from(e: Fe2o3IllegalLinkStateError) -> Self {
        AmqpErrorKind::LinkStateError(e.0.into()).into()
    }
}

impl From<fe2o3_amqp::link::IllegalLinkStateError> for AmqpError {
    fn from(e: fe2o3_amqp::link::IllegalLinkStateError) -> Self {
        AmqpError::from(AmqpErrorKind::ConnectionDropped(Box::new(e)))
    }
}

impl From<Fe2o3TransportError> for azure_core::Error {
    fn from(e: Fe2o3TransportError) -> Self {
        match e.0 {
            fe2o3_amqp::transport::Error::Io(e) => azure_core::Error::from(e),
            fe2o3_amqp::transport::Error::IdleTimeoutElapsed => {
                AmqpError::from(AmqpErrorKind::from(AmqpConnectionError::IdleTimeoutElapsed)).into()
            }
            fe2o3_amqp::transport::Error::FramingError => {
                AmqpError::from(AmqpErrorKind::from(AmqpConnectionError::FramingError)).into()
            }
            fe2o3_amqp::transport::Error::NotImplemented(_)
            | fe2o3_amqp::transport::Error::DecodeError(_) => {
                AmqpError::from(AmqpErrorKind::TransportImplementationError(Box::new(e.0))).into()
            }
        }
    }
}
