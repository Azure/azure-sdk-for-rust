// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use std::str::FromStr;

use azure_core::{create_enum, create_extensible_enum};

use crate::{AmqpOrderedMap, AmqpSymbol, AmqpValue};

/// Type of AMQP error.
pub enum AmqpErrorKind {
    /// Described error - An error described by the remote peer.
    AmqpDescribedError(AmqpDescribedError),

    /// Remote peer closed the link
    LinkClosedByRemote(Box<dyn std::error::Error + Send + Sync>),
    /// Remote peer closed the session
    SessionClosedByRemote(Box<dyn std::error::Error + Send + Sync>),
    /// Remote peer closed the connection
    ConnectionClosedByRemote(Box<dyn std::error::Error + Send + Sync>),

    /// Remote peer detached the link
    LinkDetachedByRemote(Box<dyn std::error::Error + Send + Sync>),
    /// Remote peer detached the session
    SessionDetachedByRemote(Box<dyn std::error::Error + Send + Sync>),

    /// Remote peer detached the connection
    ConnectionDetachedByRemote(Box<dyn std::error::Error + Send + Sync>),

    /// The send request was rejected by the remote peer.
    NonTerminalDeliveryState,

    /// The send request was rejected by the remote peer.
    IllegalDeliveryState,

    /// The connection was dropped.
    ConnectionDropped(Box<dyn std::error::Error + Send + Sync>),

    /// Link State error.
    LinkStateError(Box<dyn std::error::Error + Send + Sync>),

    /// Framing Error
    FramingError(Box<dyn std::error::Error + Send + Sync>),

    /// Idle Timeout Elapsed
    IdleTimeoutElapsed(Box<dyn std::error::Error + Send + Sync>),

    /// Transfer Limit Exceeded
    TransferLimitExceeded(Box<dyn std::error::Error + Send + Sync>),

    /// Management Status code
    ManagementStatusCode(azure_core::http::StatusCode, Option<String>),

    /// Detach Error
    DetachError(Box<dyn std::error::Error + Send + Sync>),
    /// Transport Implementation Error
    TransportImplementationError(Box<dyn std::error::Error + Send + Sync>),
}

create_extensible_enum!(
    #[doc = "AMQP protocol defined error conditions"]
    AmqpErrorCondition,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (DecodeError, "amqp:decode-error"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (FrameSizeTooSmall, "amqp:frame-size-too-small"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (IllegalState, "amqp:illegal-state"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (InternalError, "amqp:internal-error"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (InvalidField, "amqp:invalid-field"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (NotAllowed, "amqp:not-allowed"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (NotFound, "amqp:not-found"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (NotImplemented, "amqp:not-implemented"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (PreconditionFailed, "amqp:precondition-failed"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (ResourceDeleted, "amqp:resource-deleted"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (ResourceLimitExceeded, "amqp:resource-limit-exceeded"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (ResourceLocked, "amqp:resource-locked"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (UnauthorizedAccess, "amqp:unauthorized-access"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (LinkStolen, "amqp:link:stolen"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (LinkPayloadSizeExceeded, "amqp:link:message-size-exceeded"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (LinkDetachForced, "amqp:link:detach-forced"),
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    (ConnectionForced, "amqp:connection:forced"),
    #[allow(missing_docs)]
    (ServerBusyError, "com.microsoft:server-busy"),
    #[allow(missing_docs)]
    (ArgumentError, "com.microsoft:argument-error"),
    #[allow(missing_docs)]
    (
        ArgumentOutOfRangeError,
        "com.microsoft:argument-out-of-range"
    ),
    #[allow(missing_docs)]
    (EntityDisabledError, "com.microsoft:entity-disabled"),
    #[allow(missing_docs)]
    (PartitionNotOwnedError, "com.microsoft:partition-not-owned"),
    #[allow(missing_docs)]
    (StoreLockLostError, "com.microsoft:store-lock-lost"),
    #[allow(missing_docs)]
    (PublisherRevokedError, "com.microsoft:publisher-revoked"),
    #[allow(missing_docs)]
    (TimeoutError, "com.microsoft:timeout"),
    #[allow(missing_docs)]
    (TrackingIdProperty, "com.microsoft:tracking-id"),
    #[allow(missing_docs)]
    (ProtonIo, "proton:io"),
    #[allow(missing_docs)]
    (ConnectionFramingError, "amqp:connection:framing-error"),
    #[allow(missing_docs)]
    (OperationCancelled, "com.microsoft:operation-cancelled"),
    #[allow(missing_docs)]
    (MessageLockLost, "com.microsoft:message-lock-lost"),
    #[allow(missing_docs)]
    (SessionLockLost, "com.microsoft:session-lock-lost"),
    #[allow(missing_docs)]
    (
        SessionCannotBeLocked,
        "com.microsoft:session-cannot-be-locked"
    ),
    #[allow(missing_docs)]
    (EntityUpdated, "com.microsoft:entity-updated"),
    #[allow(missing_docs)]
    (MessageNotFound, "com.microsoft:message-not-found"),
    #[allow(missing_docs)]
    (SessionNotFound, "com.microsoft:session-not-found"),
    #[allow(missing_docs)]
    (EntityAlreadyExists, "com.microsoft:entity-already-exists"),
    #[allow(missing_docs)]
    (ConnectionRedirect, "amqp:connection:redirect"),
    #[allow(missing_docs)]
    (LinkRedirect, "amqp:link:redirect"),
    #[allow(missing_docs)]
    (TransferLimitExceeded, "amqp:link:transfer-limit-exceeded"),
    #[allow(missing_docs)]
    (SessionWindowViolation, "amqp:session:window-violation"),
    #[allow(missing_docs)]
    (SessionErrantLink, "amqp:session:errant-link"),
    #[allow(missing_docs)]
    (SessionHandleInUse, "amqp:session:handle-in-use"),
    #[allow(missing_docs)]
    (SessionUnattachedHandle, "amqp:session:unattached-handle"),
);

impl From<AmqpSymbol> for AmqpErrorCondition {
    fn from(condition: AmqpSymbol) -> Self {
        // Note that the `from_str` implementation from `create_extensible_enum` will
        // never return an error. So the `unwrap` is there to silence the compiler.
        AmqpErrorCondition::from_str(condition.0.as_str()).unwrap()
    }
}

/// An AMQP described error.
#[derive(Debug, Clone, PartialEq)]
pub struct AmqpDescribedError {
    /// The error condition.
    pub condition: AmqpErrorCondition,
    /// An optional description of the error.
    pub description: Option<String>,
    /// Optional additional information about the error.
    pub info: AmqpOrderedMap<AmqpSymbol, AmqpValue>,
}

impl AmqpDescribedError {
    /// Creates a new instance of `AmqpDescribedError`.
    ///
    /// # Arguments
    /// - `condition`: The error condition as an `AmqpErrorCondition`.
    /// - `description`: An optional description of the error.
    /// - `info`: Optional additional information as an `AmqpOrderedMap`.
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
}

/// An AMQP error from the AMQP stack.
pub struct AmqpError {
    /// Type of error.
    kind: AmqpErrorKind,
}

impl AmqpError {
    /// Returns a reference to the kind of AMQP error.
    pub fn kind(&self) -> &AmqpErrorKind {
        &self.kind
    }

    /// Creates a new management error. For test purposes only.
    #[cfg(feature = "test")]
    pub fn new_management_error(
        status_code: azure_core::http::StatusCode,
        description: Option<String>,
    ) -> Self {
        Self {
            kind: AmqpErrorKind::ManagementStatusCode(status_code, description),
        }
    }

    /// Creates a new described error. For test purposes only.
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
            | AmqpErrorKind::LinkClosedByRemote(s)
            | AmqpErrorKind::LinkDetachedByRemote(s)
            | AmqpErrorKind::SessionClosedByRemote(s)
            | AmqpErrorKind::SessionDetachedByRemote(s)
            | AmqpErrorKind::ConnectionClosedByRemote(s)
            | AmqpErrorKind::ConnectionDetachedByRemote(s)
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
            AmqpErrorKind::ConnectionDetachedByRemote(err) => {
                write!(f, "Remote connection detached with error: {}", err)
            }
            AmqpErrorKind::LinkDetachedByRemote(err) => {
                write!(f, "Remote link detached with error: {}", err)
            }
            AmqpErrorKind::SessionDetachedByRemote(err) => {
                write!(f, "Remote session detached with error: {}", err)
            }
            AmqpErrorKind::LinkClosedByRemote(err) => {
                write!(f, "Remote link closed with error: {}", err)
            }
            AmqpErrorKind::SessionClosedByRemote(err) => {
                write!(f, "Remote session closed with error: {}", err)
            }
            AmqpErrorKind::ConnectionClosedByRemote(err) => {
                write!(f, "Remote connection closed with error: {}", err)
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
        Self::new(azure_core::error::ErrorKind::Amqp, e)
    }
}

impl From<AmqpErrorKind> for azure_core::Error {
    fn from(e: AmqpErrorKind) -> Self {
        AmqpError::from(e).into()
    }
}
