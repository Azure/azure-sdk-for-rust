// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
//

#![cfg(test)]

use super::*;
use azure_core::error::ErrorKind as AzureErrorKind;
use std::error::Error;
use std::str::FromStr;

#[test]
fn test_amqp_error_condition_from_str() {
    // Test standard AMQP error conditions
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:decode-error").unwrap(),
        AmqpErrorCondition::DecodeError
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:frame-size-too-small").unwrap(),
        AmqpErrorCondition::FrameSizeTooSmall
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:illegal-state").unwrap(),
        AmqpErrorCondition::IllegalState
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:internal-error").unwrap(),
        AmqpErrorCondition::InternalError
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:invalid-field").unwrap(),
        AmqpErrorCondition::InvalidField
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:not-allowed").unwrap(),
        AmqpErrorCondition::NotAllowed
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:not-found").unwrap(),
        AmqpErrorCondition::NotFound
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:not-implemented").unwrap(),
        AmqpErrorCondition::NotImplemented
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:precondition-failed").unwrap(),
        AmqpErrorCondition::PreconditionFailed
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:resource-deleted").unwrap(),
        AmqpErrorCondition::ResourceDeleted
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:resource-limit-exceeded").unwrap(),
        AmqpErrorCondition::ResourceLimitExceeded
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:resource-locked").unwrap(),
        AmqpErrorCondition::ResourceLocked
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:unauthorized-access").unwrap(),
        AmqpErrorCondition::UnauthorizedAccess
    );

    // Test link error conditions
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:link:stolen").unwrap(),
        AmqpErrorCondition::LinkStolen
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:link:message-size-exceeded").unwrap(),
        AmqpErrorCondition::LinkPayloadSizeExceeded
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:link:detach-forced").unwrap(),
        AmqpErrorCondition::LinkDetachForced
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:link:redirect").unwrap(),
        AmqpErrorCondition::LinkRedirect
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:link:transfer-limit-exceeded").unwrap(),
        AmqpErrorCondition::TransferLimitExceeded
    );

    // Test connection error conditions
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:connection:forced").unwrap(),
        AmqpErrorCondition::ConnectionForced
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:connection:framing-error").unwrap(),
        AmqpErrorCondition::ConnectionFramingError
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:connection:redirect").unwrap(),
        AmqpErrorCondition::ConnectionRedirect
    );

    // Test session error conditions
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:session:window-violation").unwrap(),
        AmqpErrorCondition::SessionWindowViolation
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:session:errant-link").unwrap(),
        AmqpErrorCondition::SessionErrantLink
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:session:handle-in-use").unwrap(),
        AmqpErrorCondition::SessionHandleInUse
    );
    assert_eq!(
        AmqpErrorCondition::from_str("amqp:session:unattached-handle").unwrap(),
        AmqpErrorCondition::SessionUnattachedHandle
    );

    // Test Microsoft-specific error conditions
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:server-busy").unwrap(),
        AmqpErrorCondition::ServerBusyError
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:argument-error").unwrap(),
        AmqpErrorCondition::ArgumentError
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:argument-out-of-range").unwrap(),
        AmqpErrorCondition::ArgumentOutOfRangeError
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:entity-disabled").unwrap(),
        AmqpErrorCondition::EntityDisabledError
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:partition-not-owned").unwrap(),
        AmqpErrorCondition::PartitionNotOwnedError
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:store-lock-lost").unwrap(),
        AmqpErrorCondition::StoreLockLostError
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:publisher-revoked").unwrap(),
        AmqpErrorCondition::PublisherRevokedError
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:timeout").unwrap(),
        AmqpErrorCondition::TimeoutError
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:tracking-id").unwrap(),
        AmqpErrorCondition::TrackingIdProperty
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:operation-cancelled").unwrap(),
        AmqpErrorCondition::OperationCancelled
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:message-lock-lost").unwrap(),
        AmqpErrorCondition::MessageLockLost
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:session-lock-lost").unwrap(),
        AmqpErrorCondition::SessionLockLost
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:session-cannot-be-locked").unwrap(),
        AmqpErrorCondition::SessionCannotBeLocked
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:entity-updated").unwrap(),
        AmqpErrorCondition::EntityUpdated
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:message-not-found").unwrap(),
        AmqpErrorCondition::MessageNotFound
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:session-not-found").unwrap(),
        AmqpErrorCondition::SessionNotFound
    );
    assert_eq!(
        AmqpErrorCondition::from_str("com.microsoft:entity-already-exists").unwrap(),
        AmqpErrorCondition::EntityAlreadyExists
    );

    // Test Proton-specific error condition
    assert_eq!(
        AmqpErrorCondition::from_str("proton:io").unwrap(),
        AmqpErrorCondition::ProtonIo
    );

    // Test unknown value
    assert_eq!(
        AmqpErrorCondition::from_str("custom:unknown-error").unwrap(),
        AmqpErrorCondition::UnknownValue("custom:unknown-error".to_string())
    );
}

#[test]
fn test_amqp_error_condition_to_str() {
    assert_eq!(
        <&str>::from(&AmqpErrorCondition::DecodeError),
        "amqp:decode-error"
    );
    assert_eq!(
        <&str>::from(&AmqpErrorCondition::ServerBusyError),
        "com.microsoft:server-busy"
    );
    assert_eq!(<&str>::from(&AmqpErrorCondition::ProtonIo), "proton:io");
    assert_eq!(
        <&str>::from(&AmqpErrorCondition::UnknownValue("test".to_string())),
        "test"
    );
}

#[test]
fn test_amqp_error_condition_as_ref() {
    assert_eq!(
        AmqpErrorCondition::DecodeError.as_ref(),
        "amqp:decode-error"
    );
    assert_eq!(
        AmqpErrorCondition::ServerBusyError.as_ref(),
        "com.microsoft:server-busy"
    );
    assert_eq!(
        AmqpErrorCondition::UnknownValue("custom".to_string()).as_ref(),
        "custom"
    );
}

#[test]
fn test_amqp_error_condition_display() {
    assert_eq!(
        format!("{}", AmqpErrorCondition::DecodeError),
        "amqp:decode-error"
    );
    assert_eq!(
        format!("{}", AmqpErrorCondition::NotFound),
        "amqp:not-found"
    );
    assert_eq!(
        format!("{}", AmqpErrorCondition::UnknownValue("test".to_string())),
        "test"
    );
}

#[test]
fn test_amqp_error_condition_clone() {
    let condition = AmqpErrorCondition::DecodeError;
    let cloned = condition.clone();
    assert_eq!(condition, cloned);

    let unknown = AmqpErrorCondition::UnknownValue("test".to_string());
    let cloned_unknown = unknown.clone();
    assert_eq!(unknown, cloned_unknown);
}

#[test]
fn test_amqp_error_condition_from_symbol() {
    let symbol = AmqpSymbol("amqp:decode-error".to_string());
    let condition: AmqpErrorCondition = symbol.into();
    assert_eq!(condition, AmqpErrorCondition::DecodeError);

    let unknown_symbol = AmqpSymbol("unknown:condition".to_string());
    let unknown_condition: AmqpErrorCondition = unknown_symbol.into();
    assert_eq!(
        unknown_condition,
        AmqpErrorCondition::UnknownValue("unknown:condition".to_string())
    );
}

#[test]
fn test_amqp_described_error_new() {
    let mut info = AmqpOrderedMap::new();
    info.insert(
        AmqpSymbol("key".to_string()),
        AmqpValue::String("value".to_string()),
    );

    let error = AmqpDescribedError::new(
        AmqpErrorCondition::NotFound,
        Some("Resource not found".to_string()),
        info.clone(),
    );

    assert_eq!(error.condition, AmqpErrorCondition::NotFound);
    assert_eq!(error.description, Some("Resource not found".to_string()));
    assert_eq!(error.info, info);
}

#[test]
fn test_amqp_described_error_clone() {
    let mut info = AmqpOrderedMap::new();
    info.insert(
        AmqpSymbol("key".to_string()),
        AmqpValue::String("value".to_string()),
    );

    let error =
        AmqpDescribedError::new(AmqpErrorCondition::NotFound, Some("Test".to_string()), info);
    let cloned = error.clone();

    assert_eq!(error, cloned);
}

#[test]
fn test_amqp_error_with_message() {
    let error = AmqpError::with_message(Cow::Borrowed("Test message"));
    assert!(matches!(error.kind, AmqpErrorKind::SimpleMessage(_)));
    assert_eq!(format!("{}", error), "Test message");
}

#[test]
fn test_amqp_error_from_azure_core() {
    let azure_error = azure_core::Error::with_message(AzureErrorKind::Other, "Azure error");
    let amqp_error: AmqpError = azure_error.into();
    assert!(matches!(amqp_error.kind, AmqpErrorKind::AzureCore(_)));
    assert!(format!("{}", amqp_error).contains("Azure Core Error"));
}

#[test]
fn test_amqp_error_into_azure_core() {
    let azure_error = azure_core::Error::with_message(AzureErrorKind::DataConversion, "Test");
    let amqp_error: AmqpError = azure_error.into();
    let unwrapped: azure_core::Error = azure_core::Error::from(amqp_error);
    assert_eq!(*unwrapped.kind(), AzureErrorKind::DataConversion);

    let simple_error = AmqpError::with_message(Cow::Borrowed("Simple"));
    let unwrapped = azure_core::Error::from(simple_error);
    assert_eq!(*unwrapped.kind(), AzureErrorKind::Other);
}

#[test]
fn test_amqp_error_kind() {
    let error = AmqpError::with_message(Cow::Borrowed("Test"));
    assert!(matches!(error.kind(), AmqpErrorKind::SimpleMessage(_)));
}

#[test]
fn test_amqp_error_display_simple_message() {
    let error = AmqpError::with_message(Cow::Borrowed("Simple error"));
    assert_eq!(format!("{}", error), "Simple error");
}

#[test]
fn test_amqp_error_display_management_status_code() {
    let error = AmqpError::from(AmqpErrorKind::ManagementStatusCode(
        azure_core::http::StatusCode::NotFound,
        Some("Entity not found".to_string()),
    ));
    let display = format!("{}", error);
    assert!(display.contains("Management API returned status code"));
    assert!(display.contains("404"));
    assert!(display.contains("Entity not found"));

    let error_no_desc = AmqpError::from(AmqpErrorKind::ManagementStatusCode(
        azure_core::http::StatusCode::InternalServerError,
        None,
    ));
    let display_no_desc = format!("{}", error_no_desc);
    assert!(display_no_desc.contains("Management API returned status code"));
    assert!(display_no_desc.contains("500"));
}

#[test]
fn test_amqp_error_display_described_error() {
    let mut info = AmqpOrderedMap::new();
    info.insert(
        AmqpSymbol("detail".to_string()),
        AmqpValue::String("info".to_string()),
    );

    let described = AmqpDescribedError::new(
        AmqpErrorCondition::NotFound,
        Some("Not found".to_string()),
        info,
    );
    let error = AmqpError::from(AmqpErrorKind::AmqpDescribedError(described));
    let display = format!("{}", error);
    assert!(display.contains("AMQP Described Error"));
    assert!(display.contains("condition"));
    assert!(display.contains("description"));
}

#[test]
fn test_amqp_error_display_link_detached() {
    let inner_error = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "Reset");
    let error = AmqpError::from(AmqpErrorKind::LinkDetachedByRemote(Box::new(inner_error)));
    let display = format!("{}", error);
    assert!(display.contains("Remote link detached"));
}

#[test]
fn test_amqp_error_display_session_closed() {
    let inner_error = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Pipe");
    let error = AmqpError::from(AmqpErrorKind::SessionClosedByRemote(Box::new(inner_error)));
    let display = format!("{}", error);
    assert!(display.contains("Remote session closed"));
}

#[test]
fn test_amqp_error_display_connection_dropped() {
    let inner_error = std::io::Error::other("Dropped");
    let error = AmqpError::from(AmqpErrorKind::ConnectionDropped(Box::new(inner_error)));
    let display = format!("{}", error);
    assert!(display.contains("Connection dropped"));
}

#[test]
fn test_amqp_error_display_send_rejected() {
    let error = AmqpError::from(AmqpErrorKind::SendRejected);
    assert_eq!(
        format!("{}", error),
        "Send Rejected with no error information"
    );
}

#[test]
fn test_amqp_error_display_non_terminal_delivery_state() {
    let error = AmqpError::from(AmqpErrorKind::NonTerminalDeliveryState);
    assert_eq!(format!("{}", error), "Non Terminal Delivery State");
}

#[test]
fn test_amqp_error_display_illegal_delivery_state() {
    let error = AmqpError::from(AmqpErrorKind::IllegalDeliveryState);
    assert_eq!(format!("{}", error), "Illegal Delivery State");
}

#[test]
fn test_amqp_error_source() {
    // Test Azure Core error has source
    let azure_error = azure_core::Error::with_message(AzureErrorKind::Other, "Test");
    let amqp_error: AmqpError = azure_error.into();
    assert!(amqp_error.source().is_some());

    // Test boxed errors have source
    let inner_error = std::io::Error::other("Inner");
    let error = AmqpError::from(AmqpErrorKind::LinkStateError(Box::new(inner_error)));
    assert!(error.source().is_some());

    // Test simple message has no source
    let simple_error = AmqpError::with_message(Cow::Borrowed("Simple"));
    assert!(simple_error.source().is_none());

    // Test described error has no source
    let described =
        AmqpDescribedError::new(AmqpErrorCondition::NotFound, None, AmqpOrderedMap::new());
    let described_error = AmqpError::from(AmqpErrorKind::AmqpDescribedError(described));
    assert!(described_error.source().is_none());
}

#[test]
fn test_amqp_error_debug() {
    let error = AmqpError::with_message(Cow::Borrowed("Debug test"));
    let debug_output = format!("{:?}", error);
    assert!(debug_output.contains("AMQP Error:"));
    assert!(debug_output.contains("Debug test"));
}

#[test]
#[cfg(feature = "test")]
fn test_amqp_error_new_management_error() {
    let error = AmqpError::new_management_error(
        azure_core::http::StatusCode::BadRequest,
        Some("Bad request".to_string()),
    );
    assert!(matches!(
        error.kind,
        AmqpErrorKind::ManagementStatusCode(_, _)
    ));
}

#[test]
#[cfg(feature = "test")]
fn test_amqp_error_new_described_error() {
    let mut info = AmqpOrderedMap::new();
    info.insert(AmqpSymbol("test".to_string()), AmqpValue::Int(42));

    let error = AmqpError::new_described_error(
        AmqpErrorCondition::InternalError,
        Some("Internal error".to_string()),
        info.clone(),
    );

    match error.kind {
        AmqpErrorKind::AmqpDescribedError(ref desc) => {
            assert_eq!(desc.condition, AmqpErrorCondition::InternalError);
            assert_eq!(desc.description, Some("Internal error".to_string()));
            assert_eq!(desc.info, info);
        }
        _ => panic!("Expected AmqpDescribedError"),
    }
}

#[test]
fn test_amqp_error_from_kind() {
    let kind = AmqpErrorKind::SendRejected;
    let error: AmqpError = kind.into();
    assert!(matches!(error.kind, AmqpErrorKind::SendRejected));
}

#[test]
fn test_amqp_error_condition_serde_roundtrip() {
    let condition = AmqpErrorCondition::NotFound;
    let serialized = serde_json::to_string(&condition).unwrap();
    assert_eq!(serialized, "\"amqp:not-found\"");

    let deserialized: AmqpErrorCondition = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, condition);

    let unknown = AmqpErrorCondition::UnknownValue("custom:error".to_string());
    let serialized_unknown = serde_json::to_string(&unknown).unwrap();
    let deserialized_unknown: AmqpErrorCondition =
        serde_json::from_str(&serialized_unknown).unwrap();
    assert_eq!(deserialized_unknown, unknown);
}

#[test]
fn test_amqp_error_all_display_variants() {
    // Test all display implementations to ensure coverage
    let variants = vec![
        AmqpError::from(AmqpErrorKind::SimpleMessage(Cow::Borrowed("test"))),
        AmqpError::from(AmqpErrorKind::AzureCore(azure_core::Error::with_message(
            AzureErrorKind::Other,
            "azure",
        ))),
        AmqpError::from(AmqpErrorKind::LinkClosedByRemote(Box::new(
            std::io::Error::other("link"),
        ))),
        AmqpError::from(AmqpErrorKind::SessionClosedByRemote(Box::new(
            std::io::Error::other("session"),
        ))),
        AmqpError::from(AmqpErrorKind::ConnectionClosedByRemote(Box::new(
            std::io::Error::other("connection"),
        ))),
        AmqpError::from(AmqpErrorKind::LinkDetachedByRemote(Box::new(
            std::io::Error::other("link"),
        ))),
        AmqpError::from(AmqpErrorKind::SessionDetachedByRemote(Box::new(
            std::io::Error::other("session"),
        ))),
        AmqpError::from(AmqpErrorKind::ConnectionDetachedByRemote(Box::new(
            std::io::Error::other("connection"),
        ))),
        AmqpError::from(AmqpErrorKind::DetachError(Box::new(std::io::Error::other(
            "detach",
        )))),
        AmqpError::from(AmqpErrorKind::SendRejected),
        AmqpError::from(AmqpErrorKind::TransportImplementationError(Box::new(
            std::io::Error::other("transport"),
        ))),
        AmqpError::from(AmqpErrorKind::ConnectionDropped(Box::new(
            std::io::Error::other("dropped"),
        ))),
        AmqpError::from(AmqpErrorKind::FramingError(Box::new(
            std::io::Error::other("framing"),
        ))),
        AmqpError::from(AmqpErrorKind::IdleTimeoutElapsed(Box::new(
            std::io::Error::other("timeout"),
        ))),
        AmqpError::from(AmqpErrorKind::LinkStateError(Box::new(
            std::io::Error::other("state"),
        ))),
        AmqpError::from(AmqpErrorKind::TransferLimitExceeded(Box::new(
            std::io::Error::other("limit"),
        ))),
        AmqpError::from(AmqpErrorKind::NonTerminalDeliveryState),
        AmqpError::from(AmqpErrorKind::IllegalDeliveryState),
    ];

    for error in variants {
        let _ = format!("{}", error);
        let _ = format!("{:?}", error);
    }
}
