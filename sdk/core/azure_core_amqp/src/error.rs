// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use azure_core::{create_enum, create_extensible_enum};

//pub use crate::sender::error::AmqpSenderError;
use crate::{AmqpOrderedMap, AmqpSymbol, AmqpValue};

/// Type of AMQP error.
pub enum AmqpErrorKind {
    AmqpDescribedError(AmqpDescribedError),
    /// Remote peer closed the link
    ClosedByRemote(Box<dyn std::error::Error + Send + Sync>),

    /// Remote peer detached
    DetachedByRemote(Box<dyn std::error::Error + Send + Sync>),

    /// The send request was rejected by the remote peer.
    NonTerminalDeliveryState,

    /// The send request was rejected by the remote peer.
    IllegalDeliveryState,

    /// The connection was dropped.
    ConnectionDropped(Box<dyn std::error::Error + Send + Sync>),

    /// Link State error.
    LinkStateError(Box<dyn std::error::Error + Send + Sync>),

    FramingError(Box<dyn std::error::Error + Send + Sync>),
    IdleTimeoutElapsed(Box<dyn std::error::Error + Send + Sync>),

    /// Transfer Limit Exceeded
    TransferLimitExceeded(Box<dyn std::error::Error + Send + Sync>),

    /// Management Status code
    ManagementStatusCode(azure_core::http::StatusCode, Option<String>),

    DetachError(Box<dyn std::error::Error + Send + Sync>),
    //    SenderError(AmqpSenderError),
    TransportImplementationError(Box<dyn std::error::Error + Send + Sync>),
}

create_extensible_enum!(
    #[doc = "AMQP protocol defined error conditions"]
    AmqpErrorCondition,
    (DecodeError, "amqp:decode-error"),
    (FrameSizeTooSmall, "amqp:frame-size-too-small"),
    (IllegalState, "amqp:illegal-state"),
    (InternalError, "amqp:internal-error"),
    (InvalidField, "amqp:invalid-field"),
    (NotAllowed, "amqp:not-allowed"),
    (NotFound, "amqp:not-found"),
    (NotImplemented, "amqp:not-implemented"),
    (PreconditionFailed, "amqp:precondition-failed"),
    (ResourceDeleted, "amqp:resource-deleted"),
    (ResourceLimitExceeded, "amqp:resource-limit-exceeded"),
    (ResourceLocked, "amqp:resource-locked"),
    (UnauthorizedAccess, "amqp:unauthorized-access"),
    (LinkStolen, "amqp:link:stolen"),
    (LinkPayloadSizeExceeded, "amqp:link:message-size-exceeded"),
    (LinkDetachForced, "amqp:link:detach-forced"),
    (ConnectionForced, "amqp:connection:forced"),
    (ServerBusyError, "com.microsoft:server-busy"),
    (ArgumentError, "com.microsoft:argument-error"),
    (
        ArgumentOutOfRangeError,
        "com.microsoft:argument-out-of-range"
    ),
    (EntityDisabledError, "com.microsoft:entity-disabled"),
    (PartitionNotOwnedError, "com.microsoft:partition-not-owned"),
    (StoreLockLostError, "com.microsoft:store-lock-lost"),
    (PublisherRevokedError, "com.microsoft:publisher-revoked"),
    (TimeoutError, "com.microsoft:timeout"),
    (TrackingIdProperty, "com.microsoft:tracking-id"),
    (ProtonIo, "proton:io"),
    (ConnectionFramingError, "amqp:connection:framing-error"),
    (OperationCancelled, "com.microsoft:operation-cancelled"),
    (MessageLockLost, "com.microsoft:message-lock-lost"),
    (SessionLockLost, "com.microsoft:session-lock-lost"),
    (
        SessionCannotBeLocked,
        "com.microsoft:session-cannot-be-locked"
    ),
    (MessageNotFound, "com.microsoft:message-not-found"),
    (SessionNotFound, "com.microsoft:session-not-found"),
    (EntityAlreadyExists, "com.microsoft:entity-already-exists"),
    (ConnectionRedirect, "amqp:connection:redirect"),
    (LinkRedirect, "amqp:link:redirect"),
    (TransferLimitExceeded, "amqp:link:transfer-limit-exceeded"),
    (SessionWindowViolation, "amqp:session:window-violation"),
    (SessionErrantLink, "amqp:session:errant-link"),
    (SessionHandleInUse, "amqp:session:handle-in-use"),
    (SessionUnattachedHandle, "amqp:session:unattached-handle"),
);

impl From<AmqpSymbol> for AmqpErrorCondition {
    fn from(condition: AmqpSymbol) -> Self {
        match condition.0.as_str() {
            "amqp:decode-error" => Self::DecodeError,
            "amqp:frame-size-too-small" => Self::FrameSizeTooSmall,
            "amqp:illegal-state" => Self::IllegalState,
            "amqp:internal-error" => Self::InternalError,
            "amqp:invalid-field" => Self::InvalidField,
            "amqp:not-allowed" => Self::NotAllowed,
            "amqp:not-found" => Self::NotFound,
            "amqp:not-implemented" => Self::NotImplemented,
            "amqp:precondition-failed" => Self::PreconditionFailed,
            "amqp:resource-deleted" => Self::ResourceDeleted,
            "amqp:resource-limit-exceeded" => Self::ResourceLimitExceeded,
            "amqp:resource-locked" => Self::ResourceLocked,
            "amqp:unauthorized-access" => Self::UnauthorizedAccess,
            "amqp:link:stolen" => Self::LinkStolen,
            "amqp:link:message-size-exceeded" => Self::LinkPayloadSizeExceeded,
            "amqp:link:detach-forced" => Self::LinkDetachForced,
            "amqp:connection:forced" => Self::ConnectionForced,
            "com.microsoft:server-busy" => Self::ServerBusyError,
            "com.microsoft:argument-error" => Self::ArgumentError,
            "com.microsoft:argument-out-of-range" => Self::ArgumentOutOfRangeError,
            "com.microsoft:entity-disabled" => Self::EntityDisabledError,
            "com.microsoft:partition-not-owned" => Self::PartitionNotOwnedError,
            "com.microsoft:store-lock-lost" => Self::StoreLockLostError,
            "com.microsoft:publisher-revoked" => Self::PublisherRevokedError,
            "com.microsoft:timeout" => Self::TimeoutError,
            "com.microsoft:tracking-id" => Self::TrackingIdProperty,
            "proton:io" => Self::ProtonIo,
            "amqp:connection:framing-error" => Self::ConnectionFramingError,
            "com.microsoft:operation-cancelled" => Self::OperationCancelled,
            "com.microsoft:message-lock-lost" => Self::MessageLockLost,
            "com.microsoft:session-lock-lost" => Self::SessionLockLost,
            "com.microsoft:session-cannot-be-locked" => Self::SessionCannotBeLocked,
            "com.microsoft:message-not-found" => Self::MessageNotFound,
            "com.microsoft:session-not-found" => Self::SessionNotFound,
            "com.microsoft:entity-already-exists" => Self::EntityAlreadyExists,
            "amqp:connection:redirect" => Self::ConnectionRedirect,
            "amqp:link:redirect" => Self::LinkRedirect,
            "amqp:link:transfer-limit-exceeded" => Self::TransferLimitExceeded,
            "amqp:session:window-violation" => Self::SessionWindowViolation,
            "amqp:session:errant-link" => Self::SessionErrantLink,
            "amqp:session:handle-in-use" => Self::SessionHandleInUse,
            "amqp:session:unattached-handle" => Self::SessionUnattachedHandle,
            _ => Self::UnknownValue(condition.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AmqpDescribedError {
    condition: AmqpErrorCondition,
    description: Option<String>,
    info: AmqpOrderedMap<AmqpSymbol, AmqpValue>,
}

impl AmqpDescribedError {
    pub fn new(
        condition: AmqpErrorCondition,
        description: Option<String>,
        info: AmqpOrderedMap<AmqpSymbol, AmqpValue>,
    ) -> Self {
        Self {
            condition,
            description,
            info,
        }
    }

    pub fn condition(&self) -> &AmqpErrorCondition {
        &self.condition
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn info(&self) -> &AmqpOrderedMap<AmqpSymbol, AmqpValue> {
        &self.info
    }
}

/// An AMQP error from the AMQP stack.
pub struct AmqpError {
    /// Type of error.
    kind: AmqpErrorKind,
}

impl AmqpError {
    pub fn kind(&self) -> &AmqpErrorKind {
        &self.kind
    }

    #[cfg(feature = "test")]
    pub fn new_management_error(
        status_code: azure_core::http::StatusCode,
        description: Option<String>,
    ) -> Self {
        Self {
            kind: AmqpErrorKind::ManagementStatusCode(status_code, description),
        }
    }

    #[cfg(feature = "test")]
    pub fn new_described_error(
        condition: AmqpErrorCondition,
        description: Option<String>,
        info: AmqpOrderedMap<AmqpSymbol, AmqpValue>,
    ) -> Self {
        Self {
            kind: AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
                condition,
                description,
                info,
            )),
        }
    }
}

impl From<AmqpErrorKind> for AmqpError {
    fn from(kind: AmqpErrorKind) -> Self {
        Self { kind }
    }
}

impl std::error::Error for AmqpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            AmqpErrorKind::TransportImplementationError(s)
            | AmqpErrorKind::DetachError(s)
            | AmqpErrorKind::ClosedByRemote(s)
            | AmqpErrorKind::DetachedByRemote(s)
            | AmqpErrorKind::LinkStateError(s)
            | AmqpErrorKind::ConnectionDropped(s) => Some(s.as_ref()),
            AmqpErrorKind::ManagementStatusCode(_, _) => None,
            AmqpErrorKind::TransferLimitExceeded(e) => Some(e.as_ref()),
            AmqpErrorKind::FramingError(e) => Some(e.as_ref()),
            AmqpErrorKind::IdleTimeoutElapsed(e) => Some(e.as_ref()),
            AmqpErrorKind::NonTerminalDeliveryState => None,
            AmqpErrorKind::IllegalDeliveryState => None,
            AmqpErrorKind::AmqpDescribedError(_) => None,
        }
    }
}

impl std::fmt::Display for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AmqpErrorKind::ManagementStatusCode(status_code, d) => {
                if let Some(d) = d {
                    write!(
                        f,
                        "Management API returned status code: {} ({})",
                        status_code, d
                    )
                } else {
                    write!(f, "Management API returned status code: {}", status_code,)
                }
            }
            AmqpErrorKind::DetachedByRemote(err) => {
                write!(f, "Remote detached with error: {}", err)
            }
            AmqpErrorKind::ClosedByRemote(err) => {
                write!(f, "Remote closed with error: {}", err)
            }
            AmqpErrorKind::DetachError(err) => {
                write!(f, "AMQP Detach Error: {} ", err)
            }
            AmqpErrorKind::TransportImplementationError(s) => {
                write!(f, "Transport Implementation Error: {}", s)
            }
            AmqpErrorKind::ConnectionDropped(s) => {
                write!(f, "Connection dropped: {}", s)
            }
            AmqpErrorKind::FramingError(s) => {
                write!(f, "Connection Framing error: {}", s)
            }
            AmqpErrorKind::IdleTimeoutElapsed(s) => {
                write!(f, "Connection Idle Timeout elapsed: {}", s)
            }
            // AmqpErrorKind::SenderError(err) => {
            //     write!(f, "AMQP Sender Error: {} ", err)
            // }
            AmqpErrorKind::LinkStateError(err) => {
                write!(f, "AMQP Link State Error: {} ", err)
            }
            AmqpErrorKind::TransferLimitExceeded(e) => {
                write!(f, "AMQP Transfer Limit Exceeded: {e}")
            }
            AmqpErrorKind::NonTerminalDeliveryState => {
                write!(f, "AMQP Non Terminal Delivery State")
            }
            AmqpErrorKind::IllegalDeliveryState => {
                write!(f, "AMQP Illegal Delivery State")
            }
            AmqpErrorKind::AmqpDescribedError(e) => {
                write!(
                    f,
                    "AMQP Described Error: condition: {:?}, description: {:?}, info: {:?}",
                    e.condition, e.description, e.info
                )
            }
        }
    }
}

impl std::fmt::Debug for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AmqpError: {}", self)?;
        Ok(())
    }
}

impl From<AmqpError> for azure_core::Error {
    fn from(e: AmqpError) -> Self {
        Self::new(azure_core::error::ErrorKind::Amqp, Box::new(e))
    }
}

impl From<AmqpErrorKind> for azure_core::Error {
    fn from(e: AmqpErrorKind) -> Self {
        AmqpError::from(e).into()
    }
}
