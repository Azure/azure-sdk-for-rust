// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::{AmqpDescribedError, AmqpErrorCondition, AmqpErrorKind},
    value::AmqpSymbol,
    AmqpError,
};
use std::str::FromStr;

// newtype implementations for fe2o3_amqp errors. These should only be used if the transform of the
// fe2o3_amqp error directly to an AmqpError is not possible. This is the case for errors which end up
// being mapped to azure_core::Error types directly (I/O errors, etc).
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

pub(crate) struct Fe2o3ReceiverAttachError(pub fe2o3_amqp::link::ReceiverAttachError);
impl From<fe2o3_amqp::link::ReceiverAttachError> for Fe2o3ReceiverAttachError {
    fn from(e: fe2o3_amqp::link::ReceiverAttachError) -> Self {
        Fe2o3ReceiverAttachError(e)
    }
}

pub(crate) struct Fe2o3TransportError(pub fe2o3_amqp::transport::Error);
impl From<fe2o3_amqp::transport::Error> for Fe2o3TransportError {
    fn from(e: fe2o3_amqp::transport::Error) -> Self {
        Fe2o3TransportError(e)
    }
}

// Specializations of From for common AMQP types.
impl From<&fe2o3_amqp_types::definitions::ErrorCondition> for AmqpErrorCondition {
    fn from(e: &fe2o3_amqp_types::definitions::ErrorCondition) -> Self {
        match e {
            fe2o3_amqp_types::definitions::ErrorCondition::AmqpError(amqp_error) => {
                AmqpErrorCondition::from(amqp_error)
            }
            fe2o3_amqp_types::definitions::ErrorCondition::ConnectionError(connection_error) => {
                AmqpErrorCondition::from(connection_error)
            }
            fe2o3_amqp_types::definitions::ErrorCondition::SessionError(session_error) => {
                AmqpErrorCondition::from(session_error)
            }
            fe2o3_amqp_types::definitions::ErrorCondition::LinkError(link_error) => {
                AmqpErrorCondition::from(link_error)
            }
            fe2o3_amqp_types::definitions::ErrorCondition::Custom(symbol) => {
                AmqpErrorCondition::from(AmqpSymbol::from(symbol))
            }
        }
    }
}

// Implement specific From traits for the error types we need instead of using a generic implementation
impl From<&fe2o3_amqp_types::definitions::AmqpError> for AmqpErrorCondition {
    fn from(e: &fe2o3_amqp_types::definitions::AmqpError) -> Self {
        // Note that the `from_str` implementation for `AmqpErrorCondition` will
        // never return an error. So the `unwrap` is there to silence the compiler.
        AmqpErrorCondition::from_str(fe2o3_amqp_types::primitives::Symbol::from(e).as_str())
            .unwrap()
    }
}

impl From<&fe2o3_amqp_types::definitions::ConnectionError> for AmqpErrorCondition {
    fn from(e: &fe2o3_amqp_types::definitions::ConnectionError) -> Self {
        AmqpErrorCondition::from_str(fe2o3_amqp_types::primitives::Symbol::from(e).as_str())
            .unwrap()
    }
}

impl From<&fe2o3_amqp_types::definitions::SessionError> for AmqpErrorCondition {
    fn from(e: &fe2o3_amqp_types::definitions::SessionError) -> Self {
        AmqpErrorCondition::from_str(fe2o3_amqp_types::primitives::Symbol::from(e).as_str())
            .unwrap()
    }
}

impl From<&fe2o3_amqp_types::definitions::LinkError> for AmqpErrorCondition {
    fn from(e: &fe2o3_amqp_types::definitions::LinkError) -> Self {
        AmqpErrorCondition::from_str(fe2o3_amqp_types::primitives::Symbol::from(e).as_str())
            .unwrap()
    }
}
impl From<fe2o3_amqp_types::definitions::Error> for AmqpDescribedError {
    fn from(e: fe2o3_amqp_types::definitions::Error) -> Self {
        AmqpDescribedError::new(
            (&e.condition).into(),
            e.description,
            e.info
                .map(|boxed| boxed.as_ref().into())
                .unwrap_or_default(),
        )
    }
}

impl From<fe2o3_amqp::link::DetachError> for AmqpError {
    fn from(e: fe2o3_amqp::link::DetachError) -> Self {
        match e {
            fe2o3_amqp::link::DetachError::DetachedByRemote => {
                Self::from(AmqpErrorKind::LinkDetachedByRemote(Box::new(e)))
            }
            fe2o3_amqp::link::DetachError::RemoteDetachedWithError(error) => {
                Self::from(AmqpErrorKind::AmqpDescribedError(error.into()))
            }
            fe2o3_amqp::link::DetachError::ClosedByRemote => {
                Self::from(AmqpErrorKind::LinkClosedByRemote(Box::new(e)))
            }
            fe2o3_amqp::link::DetachError::RemoteClosedWithError(error) => {
                Self::from(AmqpErrorKind::AmqpDescribedError(error.into()))
            }
            _ => Self::from(AmqpErrorKind::DetachError(Box::new(e))),
        }
    }
}

impl From<fe2o3_amqp::link::LinkStateError> for AmqpError {
    fn from(e: fe2o3_amqp::link::LinkStateError) -> Self {
        match e {
            fe2o3_amqp::link::LinkStateError::RemoteClosedWithError(e) => {
                AmqpErrorKind::AmqpDescribedError(e.into()).into()
            }
            fe2o3_amqp::link::LinkStateError::RemoteDetachedWithError(e) => {
                AmqpErrorKind::AmqpDescribedError(e.into()).into()
            }
            fe2o3_amqp::link::LinkStateError::RemoteClosed => {
                AmqpErrorKind::LinkClosedByRemote(Box::new(e)).into()
            }
            fe2o3_amqp::link::LinkStateError::RemoteDetached => {
                AmqpErrorKind::LinkDetachedByRemote(Box::new(e)).into()
            }
            _ => AmqpErrorKind::LinkStateError(e.into()).into(),
        }
    }
}

impl From<fe2o3_amqp::link::IllegalLinkStateError> for AmqpError {
    fn from(e: fe2o3_amqp::link::IllegalLinkStateError) -> Self {
        AmqpError::from(AmqpErrorKind::ConnectionDropped(Box::new(e)))
    }
}

impl From<Fe2o3TransportError> for AmqpError {
    fn from(e: Fe2o3TransportError) -> Self {
        match e.0 {
            fe2o3_amqp::transport::Error::Io(e) => azure_core::Error::from(e).into(),
            fe2o3_amqp::transport::Error::IdleTimeoutElapsed => {
                AmqpError::from(AmqpErrorKind::IdleTimeoutElapsed(Box::new(e.0)))
            }
            fe2o3_amqp::transport::Error::FramingError => {
                AmqpError::from(AmqpErrorKind::FramingError(Box::new(e.0)))
            }
            fe2o3_amqp::transport::Error::NotImplemented(_)
            | fe2o3_amqp::transport::Error::DecodeError(_) => {
                AmqpError::from(AmqpErrorKind::TransportImplementationError(Box::new(e.0)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::AmqpErrorCondition;

    // Tests to ensure the fidelity of conversion from the fe2o3 AMQP error
    // conditions to the AmqpErrorCondition type.
    macro_rules! test_amqp_error {
        ($test_name:ident, $fe2o3_type:ident, $variant:ident, $amqp_variant:ident) => {
            #[test]
            fn $test_name() {
                let error = fe2o3_amqp_types::definitions::ErrorCondition::$fe2o3_type(
                    fe2o3_amqp_types::definitions::$fe2o3_type::$variant,
                );
                let amqp_error = AmqpErrorCondition::from(&error);
                assert_eq!(amqp_error, AmqpErrorCondition::$amqp_variant);
            }
        };
    }
    test_amqp_error!(test_internal_error, AmqpError, InternalError, InternalError);
    test_amqp_error!(test_not_found, AmqpError, NotFound, NotFound);
    test_amqp_error!(
        test_unauthorized_access,
        AmqpError,
        UnauthorizedAccess,
        UnauthorizedAccess
    );
    test_amqp_error!(test_decode_error, AmqpError, DecodeError, DecodeError);
    test_amqp_error!(
        test_resource_limit_exceeded,
        AmqpError,
        ResourceLimitExceeded,
        ResourceLimitExceeded
    );
    test_amqp_error!(test_not_allowed, AmqpError, NotAllowed, NotAllowed);
    test_amqp_error!(test_invalid_field, AmqpError, InvalidField, InvalidField);
    test_amqp_error!(
        test_not_implemented,
        AmqpError,
        NotImplemented,
        NotImplemented
    );
    test_amqp_error!(
        test_resource_locked,
        AmqpError,
        ResourceLocked,
        ResourceLocked
    );
    test_amqp_error!(
        test_precondition_failed,
        AmqpError,
        PreconditionFailed,
        PreconditionFailed
    );
    test_amqp_error!(
        test_resource_deleted,
        AmqpError,
        ResourceDeleted,
        ResourceDeleted
    );
    test_amqp_error!(test_illegal_state, AmqpError, IllegalState, IllegalState);
    test_amqp_error!(
        test_frame_size_too_small,
        AmqpError,
        FrameSizeTooSmall,
        FrameSizeTooSmall
    );
    test_amqp_error!(
        test_connection_forced,
        ConnectionError,
        ConnectionForced,
        ConnectionForced
    );
    test_amqp_error!(
        test_framing_error,
        ConnectionError,
        FramingError,
        ConnectionFramingError
    );
    test_amqp_error!(test_redirect, ConnectionError, Redirect, ConnectionRedirect);
    test_amqp_error!(
        test_window_violation,
        SessionError,
        WindowViolation,
        SessionWindowViolation
    );
    test_amqp_error!(
        test_errant_link,
        SessionError,
        ErrantLink,
        SessionErrantLink
    );
    test_amqp_error!(
        test_handle_in_use,
        SessionError,
        HandleInUse,
        SessionHandleInUse
    );
    test_amqp_error!(
        test_unattached_handle,
        SessionError,
        UnattachedHandle,
        SessionUnattachedHandle
    );

    test_amqp_error!(
        test_detach_forced,
        LinkError,
        DetachForced,
        LinkDetachForced
    );
    test_amqp_error!(
        test_transfer_limit_exceeded,
        LinkError,
        TransferLimitExceeded,
        TransferLimitExceeded
    );
    test_amqp_error!(
        test_message_size_exceeded,
        LinkError,
        MessageSizeExceeded,
        LinkPayloadSizeExceeded
    );
    test_amqp_error!(test_link_redirect, LinkError, Redirect, LinkRedirect);
    test_amqp_error!(test_stolen, LinkError, Stolen, LinkStolen);
}
