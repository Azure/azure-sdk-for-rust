// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::AmqpSymbol;
use std::str::FromStr;

/// AMQP protocol defined error conditions
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum AmqpErrorCondition {
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    DecodeError,
    ///  See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    FrameSizeTooSmall,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    IllegalState,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    InternalError,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    InvalidField,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    NotAllowed,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    NotFound,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    NotImplemented,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    PreconditionFailed,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    ResourceDeleted,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    ResourceLimitExceeded,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    ResourceLocked,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    UnauthorizedAccess,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    LinkStolen,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    LinkPayloadSizeExceeded,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    LinkDetachForced,
    /// See [AMQP Error](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-amqp-error) for more information.
    ConnectionForced,
    /// Microsoft specific error conditions: server busy.
    ServerBusyError,
    /// Microsoft specific error conditions: argument error.
    ArgumentError,
    /// Microsoft specific error conditions: argument out of range.
    ArgumentOutOfRangeError,
    /// Microsoft specific error conditions: entity disabled.
    EntityDisabledError,
    /// Microsoft specific error conditions: partition not owned.
    PartitionNotOwnedError,
    /// Microsoft specific error conditions: store lock lost.
    StoreLockLostError,
    /// Microsoft specific error conditions: publisher revoked.
    PublisherRevokedError,
    /// Microsoft specific error conditions: timeout.
    TimeoutError,
    /// Microsoft specific error conditions: tracking id property.
    TrackingIdProperty,
    /// Proton specific error conditions: io error.
    ProtonIo,
    /// AMQP specific error conditions: connection framing error.
    ConnectionFramingError,
    /// Microsoft specific error conditions: operation cancelled.
    OperationCancelled,
    /// Microsoft specific error conditions: message lock lost.
    MessageLockLost,
    /// Microsoft specific error conditions: session lock lost.
    SessionLockLost,
    /// Microsoft specific error conditions: session cannot be locked.
    SessionCannotBeLocked,
    /// Microsoft specific error conditions: entity updated.
    EntityUpdated,
    /// Microsoft specific error conditions: message not found.
    MessageNotFound,
    /// Microsoft specific error conditions: session not found.
    SessionNotFound,
    /// Microsoft specific error conditions: entity already exists.
    EntityAlreadyExists,
    /// AMQP specific error conditions: connection redirect.
    ConnectionRedirect,
    /// AMQP specific error conditions: link redirect.
    LinkRedirect,
    /// AMQP specific error conditions: transfer limit exceeded.
    TransferLimitExceeded,
    /// AMQP specific error conditions: session window violation.
    SessionWindowViolation,
    /// AMQP specific error conditions: session errant link.
    SessionErrantLink,
    /// AMQP specific error conditions: session handle in use.
    SessionHandleInUse,
    /// AMQP specific error conditions: session unattached handle.
    SessionUnattachedHandle,
    /// Any other value not defined in `AmqpErrorCondition`.
    UnknownValue(String),
}
impl<'a> ::std::convert::From<&'a AmqpErrorCondition> for &'a str {
    fn from(e: &'a AmqpErrorCondition) -> Self {
        match e {
            AmqpErrorCondition::DecodeError => "amqp:decode-error",
            AmqpErrorCondition::FrameSizeTooSmall => "amqp:frame-size-too-small",
            AmqpErrorCondition::IllegalState => "amqp:illegal-state",
            AmqpErrorCondition::InternalError => "amqp:internal-error",
            AmqpErrorCondition::InvalidField => "amqp:invalid-field",
            AmqpErrorCondition::NotAllowed => "amqp:not-allowed",
            AmqpErrorCondition::NotFound => "amqp:not-found",
            AmqpErrorCondition::NotImplemented => "amqp:not-implemented",
            AmqpErrorCondition::PreconditionFailed => "amqp:precondition-failed",
            AmqpErrorCondition::ResourceDeleted => "amqp:resource-deleted",
            AmqpErrorCondition::ResourceLimitExceeded => "amqp:resource-limit-exceeded",
            AmqpErrorCondition::ResourceLocked => "amqp:resource-locked",
            AmqpErrorCondition::UnauthorizedAccess => "amqp:unauthorized-access",
            AmqpErrorCondition::LinkStolen => "amqp:link:stolen",
            AmqpErrorCondition::LinkPayloadSizeExceeded => "amqp:link:message-size-exceeded",
            AmqpErrorCondition::LinkDetachForced => "amqp:link:detach-forced",
            AmqpErrorCondition::ConnectionForced => "amqp:connection:forced",
            AmqpErrorCondition::ServerBusyError => "com.microsoft:server-busy",
            AmqpErrorCondition::ArgumentError => "com.microsoft:argument-error",
            AmqpErrorCondition::ArgumentOutOfRangeError => "com.microsoft:argument-out-of-range",
            AmqpErrorCondition::EntityDisabledError => "com.microsoft:entity-disabled",
            AmqpErrorCondition::PartitionNotOwnedError => "com.microsoft:partition-not-owned",
            AmqpErrorCondition::StoreLockLostError => "com.microsoft:store-lock-lost",
            AmqpErrorCondition::PublisherRevokedError => "com.microsoft:publisher-revoked",
            AmqpErrorCondition::TimeoutError => "com.microsoft:timeout",
            AmqpErrorCondition::TrackingIdProperty => "com.microsoft:tracking-id",
            AmqpErrorCondition::ProtonIo => "proton:io",
            AmqpErrorCondition::ConnectionFramingError => "amqp:connection:framing-error",
            AmqpErrorCondition::OperationCancelled => "com.microsoft:operation-cancelled",
            AmqpErrorCondition::MessageLockLost => "com.microsoft:message-lock-lost",
            AmqpErrorCondition::SessionLockLost => "com.microsoft:session-lock-lost",
            AmqpErrorCondition::SessionCannotBeLocked => "com.microsoft:session-cannot-be-locked",
            AmqpErrorCondition::EntityUpdated => "com.microsoft:entity-updated",
            AmqpErrorCondition::MessageNotFound => "com.microsoft:message-not-found",
            AmqpErrorCondition::SessionNotFound => "com.microsoft:session-not-found",
            AmqpErrorCondition::EntityAlreadyExists => "com.microsoft:entity-already-exists",
            AmqpErrorCondition::ConnectionRedirect => "amqp:connection:redirect",
            AmqpErrorCondition::LinkRedirect => "amqp:link:redirect",
            AmqpErrorCondition::TransferLimitExceeded => "amqp:link:transfer-limit-exceeded",
            AmqpErrorCondition::SessionWindowViolation => "amqp:session:window-violation",
            AmqpErrorCondition::SessionErrantLink => "amqp:session:errant-link",
            AmqpErrorCondition::SessionHandleInUse => "amqp:session:handle-in-use",
            AmqpErrorCondition::SessionUnattachedHandle => "amqp:session:unattached-handle",
            AmqpErrorCondition::UnknownValue(s) => s.as_ref(),
        }
    }
}
impl ::std::str::FromStr for AmqpErrorCondition {
    type Err = ::std::convert::Infallible;
    fn from_str(s: &str) -> ::core::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        Ok(match s {
            "amqp:decode-error" => AmqpErrorCondition::DecodeError,
            "amqp:frame-size-too-small" => AmqpErrorCondition::FrameSizeTooSmall,
            "amqp:illegal-state" => AmqpErrorCondition::IllegalState,
            "amqp:internal-error" => AmqpErrorCondition::InternalError,
            "amqp:invalid-field" => AmqpErrorCondition::InvalidField,
            "amqp:not-allowed" => AmqpErrorCondition::NotAllowed,
            "amqp:not-found" => AmqpErrorCondition::NotFound,
            "amqp:not-implemented" => AmqpErrorCondition::NotImplemented,
            "amqp:precondition-failed" => AmqpErrorCondition::PreconditionFailed,
            "amqp:resource-deleted" => AmqpErrorCondition::ResourceDeleted,
            "amqp:resource-limit-exceeded" => AmqpErrorCondition::ResourceLimitExceeded,
            "amqp:resource-locked" => AmqpErrorCondition::ResourceLocked,
            "amqp:unauthorized-access" => AmqpErrorCondition::UnauthorizedAccess,
            "amqp:link:stolen" => AmqpErrorCondition::LinkStolen,
            "amqp:link:message-size-exceeded" => AmqpErrorCondition::LinkPayloadSizeExceeded,
            "amqp:link:detach-forced" => AmqpErrorCondition::LinkDetachForced,
            "amqp:connection:forced" => AmqpErrorCondition::ConnectionForced,
            "com.microsoft:server-busy" => AmqpErrorCondition::ServerBusyError,
            "com.microsoft:argument-error" => AmqpErrorCondition::ArgumentError,
            "com.microsoft:argument-out-of-range" => AmqpErrorCondition::ArgumentOutOfRangeError,
            "com.microsoft:entity-disabled" => AmqpErrorCondition::EntityDisabledError,
            "com.microsoft:partition-not-owned" => AmqpErrorCondition::PartitionNotOwnedError,
            "com.microsoft:store-lock-lost" => AmqpErrorCondition::StoreLockLostError,
            "com.microsoft:publisher-revoked" => AmqpErrorCondition::PublisherRevokedError,
            "com.microsoft:timeout" => AmqpErrorCondition::TimeoutError,
            "com.microsoft:tracking-id" => AmqpErrorCondition::TrackingIdProperty,
            "proton:io" => AmqpErrorCondition::ProtonIo,
            "amqp:connection:framing-error" => AmqpErrorCondition::ConnectionFramingError,
            "com.microsoft:operation-cancelled" => AmqpErrorCondition::OperationCancelled,
            "com.microsoft:message-lock-lost" => AmqpErrorCondition::MessageLockLost,
            "com.microsoft:session-lock-lost" => AmqpErrorCondition::SessionLockLost,
            "com.microsoft:session-cannot-be-locked" => AmqpErrorCondition::SessionCannotBeLocked,
            "com.microsoft:entity-updated" => AmqpErrorCondition::EntityUpdated,
            "com.microsoft:message-not-found" => AmqpErrorCondition::MessageNotFound,
            "com.microsoft:session-not-found" => AmqpErrorCondition::SessionNotFound,
            "com.microsoft:entity-already-exists" => AmqpErrorCondition::EntityAlreadyExists,
            "amqp:connection:redirect" => AmqpErrorCondition::ConnectionRedirect,
            "amqp:link:redirect" => AmqpErrorCondition::LinkRedirect,
            "amqp:link:transfer-limit-exceeded" => AmqpErrorCondition::TransferLimitExceeded,
            "amqp:session:window-violation" => AmqpErrorCondition::SessionWindowViolation,
            "amqp:session:errant-link" => AmqpErrorCondition::SessionErrantLink,
            "amqp:session:handle-in-use" => AmqpErrorCondition::SessionHandleInUse,
            "amqp:session:unattached-handle" => AmqpErrorCondition::SessionUnattachedHandle,
            _ => AmqpErrorCondition::UnknownValue(s.to_string()),
        })
    }
}
impl ::std::convert::AsRef<str> for AmqpErrorCondition {
    fn as_ref(&self) -> &str {
        match self {
            AmqpErrorCondition::DecodeError => "amqp:decode-error",
            AmqpErrorCondition::FrameSizeTooSmall => "amqp:frame-size-too-small",
            AmqpErrorCondition::IllegalState => "amqp:illegal-state",
            AmqpErrorCondition::InternalError => "amqp:internal-error",
            AmqpErrorCondition::InvalidField => "amqp:invalid-field",
            AmqpErrorCondition::NotAllowed => "amqp:not-allowed",
            AmqpErrorCondition::NotFound => "amqp:not-found",
            AmqpErrorCondition::NotImplemented => "amqp:not-implemented",
            AmqpErrorCondition::PreconditionFailed => "amqp:precondition-failed",
            AmqpErrorCondition::ResourceDeleted => "amqp:resource-deleted",
            AmqpErrorCondition::ResourceLimitExceeded => "amqp:resource-limit-exceeded",
            AmqpErrorCondition::ResourceLocked => "amqp:resource-locked",
            AmqpErrorCondition::UnauthorizedAccess => "amqp:unauthorized-access",
            AmqpErrorCondition::LinkStolen => "amqp:link:stolen",
            AmqpErrorCondition::LinkPayloadSizeExceeded => "amqp:link:message-size-exceeded",
            AmqpErrorCondition::LinkDetachForced => "amqp:link:detach-forced",
            AmqpErrorCondition::ConnectionForced => "amqp:connection:forced",
            AmqpErrorCondition::ServerBusyError => "com.microsoft:server-busy",
            AmqpErrorCondition::ArgumentError => "com.microsoft:argument-error",
            AmqpErrorCondition::ArgumentOutOfRangeError => "com.microsoft:argument-out-of-range",
            AmqpErrorCondition::EntityDisabledError => "com.microsoft:entity-disabled",
            AmqpErrorCondition::PartitionNotOwnedError => "com.microsoft:partition-not-owned",
            AmqpErrorCondition::StoreLockLostError => "com.microsoft:store-lock-lost",
            AmqpErrorCondition::PublisherRevokedError => "com.microsoft:publisher-revoked",
            AmqpErrorCondition::TimeoutError => "com.microsoft:timeout",
            AmqpErrorCondition::TrackingIdProperty => "com.microsoft:tracking-id",
            AmqpErrorCondition::ProtonIo => "proton:io",
            AmqpErrorCondition::ConnectionFramingError => "amqp:connection:framing-error",
            AmqpErrorCondition::OperationCancelled => "com.microsoft:operation-cancelled",
            AmqpErrorCondition::MessageLockLost => "com.microsoft:message-lock-lost",
            AmqpErrorCondition::SessionLockLost => "com.microsoft:session-lock-lost",
            AmqpErrorCondition::SessionCannotBeLocked => "com.microsoft:session-cannot-be-locked",
            AmqpErrorCondition::EntityUpdated => "com.microsoft:entity-updated",
            AmqpErrorCondition::MessageNotFound => "com.microsoft:message-not-found",
            AmqpErrorCondition::SessionNotFound => "com.microsoft:session-not-found",
            AmqpErrorCondition::EntityAlreadyExists => "com.microsoft:entity-already-exists",
            AmqpErrorCondition::ConnectionRedirect => "amqp:connection:redirect",
            AmqpErrorCondition::LinkRedirect => "amqp:link:redirect",
            AmqpErrorCondition::TransferLimitExceeded => "amqp:link:transfer-limit-exceeded",
            AmqpErrorCondition::SessionWindowViolation => "amqp:session:window-violation",
            AmqpErrorCondition::SessionErrantLink => "amqp:session:errant-link",
            AmqpErrorCondition::SessionHandleInUse => "amqp:session:handle-in-use",
            AmqpErrorCondition::SessionUnattachedHandle => "amqp:session:unattached-handle",
            AmqpErrorCondition::UnknownValue(s) => s.as_str(),
        }
    }
}
impl ::std::fmt::Display for AmqpErrorCondition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            AmqpErrorCondition::DecodeError => f.write_str("amqp:decode-error"),
            AmqpErrorCondition::FrameSizeTooSmall => f.write_str("amqp:frame-size-too-small"),
            AmqpErrorCondition::IllegalState => f.write_str("amqp:illegal-state"),
            AmqpErrorCondition::InternalError => f.write_str("amqp:internal-error"),
            AmqpErrorCondition::InvalidField => f.write_str("amqp:invalid-field"),
            AmqpErrorCondition::NotAllowed => f.write_str("amqp:not-allowed"),
            AmqpErrorCondition::NotFound => f.write_str("amqp:not-found"),
            AmqpErrorCondition::NotImplemented => f.write_str("amqp:not-implemented"),
            AmqpErrorCondition::PreconditionFailed => f.write_str("amqp:precondition-failed"),
            AmqpErrorCondition::ResourceDeleted => f.write_str("amqp:resource-deleted"),
            AmqpErrorCondition::ResourceLimitExceeded => {
                f.write_str("amqp:resource-limit-exceeded")
            }
            AmqpErrorCondition::ResourceLocked => f.write_str("amqp:resource-locked"),
            AmqpErrorCondition::UnauthorizedAccess => f.write_str("amqp:unauthorized-access"),
            AmqpErrorCondition::LinkStolen => f.write_str("amqp:link:stolen"),
            AmqpErrorCondition::LinkPayloadSizeExceeded => {
                f.write_str("amqp:link:message-size-exceeded")
            }
            AmqpErrorCondition::LinkDetachForced => f.write_str("amqp:link:detach-forced"),
            AmqpErrorCondition::ConnectionForced => f.write_str("amqp:connection:forced"),
            AmqpErrorCondition::ServerBusyError => f.write_str("com.microsoft:server-busy"),
            AmqpErrorCondition::ArgumentError => f.write_str("com.microsoft:argument-error"),
            AmqpErrorCondition::ArgumentOutOfRangeError => {
                f.write_str("com.microsoft:argument-out-of-range")
            }
            AmqpErrorCondition::EntityDisabledError => f.write_str("com.microsoft:entity-disabled"),
            AmqpErrorCondition::PartitionNotOwnedError => {
                f.write_str("com.microsoft:partition-not-owned")
            }
            AmqpErrorCondition::StoreLockLostError => f.write_str("com.microsoft:store-lock-lost"),
            AmqpErrorCondition::PublisherRevokedError => {
                f.write_str("com.microsoft:publisher-revoked")
            }
            AmqpErrorCondition::TimeoutError => f.write_str("com.microsoft:timeout"),
            AmqpErrorCondition::TrackingIdProperty => f.write_str("com.microsoft:tracking-id"),
            AmqpErrorCondition::ProtonIo => f.write_str("proton:io"),
            AmqpErrorCondition::ConnectionFramingError => {
                f.write_str("amqp:connection:framing-error")
            }
            AmqpErrorCondition::OperationCancelled => {
                f.write_str("com.microsoft:operation-cancelled")
            }
            AmqpErrorCondition::MessageLockLost => f.write_str("com.microsoft:message-lock-lost"),
            AmqpErrorCondition::SessionLockLost => f.write_str("com.microsoft:session-lock-lost"),
            AmqpErrorCondition::SessionCannotBeLocked => {
                f.write_str("com.microsoft:session-cannot-be-locked")
            }
            AmqpErrorCondition::EntityUpdated => f.write_str("com.microsoft:entity-updated"),
            AmqpErrorCondition::MessageNotFound => f.write_str("com.microsoft:message-not-found"),
            AmqpErrorCondition::SessionNotFound => f.write_str("com.microsoft:session-not-found"),
            AmqpErrorCondition::EntityAlreadyExists => {
                f.write_str("com.microsoft:entity-already-exists")
            }
            AmqpErrorCondition::ConnectionRedirect => f.write_str("amqp:connection:redirect"),
            AmqpErrorCondition::LinkRedirect => f.write_str("amqp:link:redirect"),
            AmqpErrorCondition::TransferLimitExceeded => {
                f.write_str("amqp:link:transfer-limit-exceeded")
            }
            AmqpErrorCondition::SessionWindowViolation => {
                f.write_str("amqp:session:window-violation")
            }
            AmqpErrorCondition::SessionErrantLink => f.write_str("amqp:session:errant-link"),
            AmqpErrorCondition::SessionHandleInUse => f.write_str("amqp:session:handle-in-use"),
            AmqpErrorCondition::SessionUnattachedHandle => {
                f.write_str("amqp:session:unattached-handle")
            }
            AmqpErrorCondition::UnknownValue(s) => f.write_str(s.as_str()),
        }
    }
}
impl<'de> serde::Deserialize<'de> for AmqpErrorCondition {
    fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
impl serde::Serialize for AmqpErrorCondition {
    fn serialize<S>(&self, s: S) -> ::core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(self.as_ref())
    }
}

impl From<AmqpSymbol> for AmqpErrorCondition {
    fn from(condition: AmqpSymbol) -> Self {
        // Note that the `from_str` implementation for `AmqpErrorCondition` will
        // never return an error. So the `unwrap` is there to silence the compiler.
        AmqpErrorCondition::from_str(condition.0.as_str()).unwrap()
    }
}
