// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::{AmqpDescribedError, AmqpErrorKind},
    AmqpError,
};

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

impl From<fe2o3_amqp::link::LinkStateError> for AmqpError {
    fn from(e: fe2o3_amqp::link::LinkStateError) -> Self {
        match e {
            fe2o3_amqp::link::LinkStateError::RemoteClosedWithError(e) => {
                AmqpErrorKind::ClosedByRemoteWithError(e.into()).into()
            }
            fe2o3_amqp::link::LinkStateError::RemoteDetachedWithError(e) => {
                AmqpErrorKind::DetachedByRemoteWithError(e.into()).into()
            }
            fe2o3_amqp::link::LinkStateError::RemoteClosed => AmqpErrorKind::ClosedByRemote.into(),
            fe2o3_amqp::link::LinkStateError::RemoteDetached => {
                AmqpErrorKind::DetachedByRemote.into()
            }
            _ => AmqpErrorKind::LinkStateError(e.into()).into(),
        }
    }
}
