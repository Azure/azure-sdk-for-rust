// Copyright (c) Microsoft Corporation. All Rights Reserved.
// Licensed under the MIT License.

use azure_core_amqp::{AmqpDescribedError, AmqpError};
use std::borrow::Cow;

/// A specialized `Result` type for Event Hubs operations.
pub type Result<T> = std::result::Result<T, EventHubsError>;

/// Represents the different kinds of errors that can occur in the Eventhubs module.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A simple error.
    SimpleMessage(Cow<'static, str>),

    /// The management response is invalid.
    InvalidManagementResponse,

    /// The message was rejected.
    SendRejected(Option<AmqpDescribedError>),

    /// Represents an Azure Core error
    AzureCore(azure_core::Error),

    /// Represents the source of the AMQP error.
    /// This is used to wrap an AMQP error in an Even Hubs error.
    ///
    AmqpError(AmqpError),
}

/// Represents an error that can occur in the Event Hubs module.
pub struct EventHubsError {
    /// The kind of error that occurred.
    pub kind: ErrorKind,
}

impl EventHubsError {
    pub(crate) fn with_message<C>(message: C) -> EventHubsError
    where
        C: Into<Cow<'static, str>>,
    {
        Self::from(ErrorKind::SimpleMessage(message.into()))
    }
}

impl std::error::Error for EventHubsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::AmqpError(source) => Some(source),
            ErrorKind::AzureCore(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for EventHubsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::SimpleMessage(msg) => write!(f, "{}", msg),
            ErrorKind::AzureCore(e) => write!(f, "Azure Core Error: {}", e),
            ErrorKind::SendRejected(e) => write!(f, "Send rejected: {:?}", e),
            ErrorKind::InvalidManagementResponse => f.write_str("Invalid management response"),
            ErrorKind::AmqpError(source) => write!(f, "AMQP Error: {:?}", source),
        }
    }
}

impl std::fmt::Debug for EventHubsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event Hubs Error: {}", self)
    }
}

impl From<ErrorKind> for EventHubsError {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl From<AmqpError> for EventHubsError {
    fn from(e: AmqpError) -> Self {
        Self {
            kind: ErrorKind::AmqpError(e),
        }
    }
}

impl From<azure_core::Error> for EventHubsError {
    fn from(e: azure_core::Error) -> Self {
        Self {
            kind: ErrorKind::AzureCore(e),
        }
    }
}

impl From<EventHubsError> for azure_core::Error {
    fn from(value: EventHubsError) -> Self {
        match value.kind {
            ErrorKind::AzureCore(e) => e,
            _ => azure_core::Error::with_error(
                azure_core::error::ErrorKind::Other,
                value,
                "EventHubs Error",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core_amqp::{AmqpErrorCondition, AmqpOrderedMap, AmqpSymbol};
    use std::error::Error;

    #[test]
    fn test_eventhubs_error_with_message_borrowed() {
        let error = EventHubsError::with_message("Test error");
        assert!(matches!(error.kind, ErrorKind::SimpleMessage(_)));
        assert_eq!(format!("{}", error), "Test error");
    }

    #[test]
    fn test_eventhubs_error_with_message_owned() {
        let error = EventHubsError::with_message("Owned error".to_string());
        assert!(matches!(error.kind, ErrorKind::SimpleMessage(_)));
        assert_eq!(format!("{}", error), "Owned error");
    }

    #[test]
    fn test_eventhubs_error_from_error_kind_simple_message() {
        let kind = ErrorKind::SimpleMessage(Cow::Borrowed("Simple message"));
        let error: EventHubsError = kind.into();
        assert!(matches!(error.kind, ErrorKind::SimpleMessage(_)));
        assert_eq!(format!("{}", error), "Simple message");
    }

    #[test]
    fn test_eventhubs_error_from_error_kind_invalid_management_response() {
        let kind = ErrorKind::InvalidManagementResponse;
        let error: EventHubsError = kind.into();
        assert!(matches!(error.kind, ErrorKind::InvalidManagementResponse));
        assert_eq!(format!("{}", error), "Invalid management response");
    }

    #[test]
    fn test_eventhubs_error_from_error_kind_send_rejected_with_details() {
        let mut info = AmqpOrderedMap::new();
        info.insert(
            AmqpSymbol("error-detail".to_string()),
            azure_core_amqp::AmqpValue::String("Quota exceeded".to_string()),
        );
        let described_error = AmqpDescribedError::new(
            AmqpErrorCondition::ResourceLimitExceeded,
            Some("Send quota exceeded".to_string()),
            info,
        );
        let kind = ErrorKind::SendRejected(Some(described_error));
        let error: EventHubsError = kind.into();
        assert!(matches!(error.kind, ErrorKind::SendRejected(_)));
        let display = format!("{}", error);
        assert!(display.contains("Send rejected"));
    }

    #[test]
    fn test_eventhubs_error_from_error_kind_send_rejected_without_details() {
        let kind = ErrorKind::SendRejected(None);
        let error: EventHubsError = kind.into();
        assert!(matches!(error.kind, ErrorKind::SendRejected(None)));
        let display = format!("{}", error);
        assert!(display.contains("Send rejected"));
    }

    #[test]
    fn test_eventhubs_error_from_amqp_error() {
        let amqp_error = AmqpError::from(azure_core_amqp::AmqpErrorKind::SimpleMessage(
            Cow::Borrowed("AMQP error"),
        ));
        let error: EventHubsError = amqp_error.into();
        assert!(matches!(error.kind, ErrorKind::AmqpError(_)));
        let display = format!("{}", error);
        assert!(display.contains("AMQP Error"));
    }

    #[test]
    fn test_eventhubs_error_from_azure_core_error() {
        let azure_error =
            azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "Azure error");
        let error: EventHubsError = azure_error.into();
        assert!(matches!(error.kind, ErrorKind::AzureCore(_)));
        let display = format!("{}", error);
        assert!(display.contains("Azure Core Error"));
    }
    #[test]
    fn test_eventhubs_error_to_azure_core_error_from_simple_message() {
        let eventhubs_error = EventHubsError::with_message("Simple error");
        let converted: azure_core::Error = eventhubs_error.into();
        let display = format!("{}", converted);
        assert!(display.contains("EventHubs Error"));
    }

    #[test]
    fn test_eventhubs_error_to_azure_core_error_from_azure_core() {
        let azure_error = azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "Original Azure error",
        );
        let eventhubs_error: EventHubsError = azure_error.into();
        let converted: azure_core::Error = eventhubs_error.into();
        assert!(format!("{}", converted).contains("Original Azure error"));
    }

    #[test]
    fn test_eventhubs_error_source_amqp() {
        let amqp_error = AmqpError::from(azure_core_amqp::AmqpErrorKind::SimpleMessage(
            Cow::Borrowed("AMQP source"),
        ));
        let error: EventHubsError = amqp_error.into();
        assert!(error.source().is_some());
    }

    #[test]
    fn test_eventhubs_error_source_azure_core() {
        let azure_error =
            azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "Azure source");
        let error: EventHubsError = azure_error.into();
        assert!(error.source().is_some());
    }

    #[test]
    fn test_eventhubs_error_source_simple_message() {
        let error = EventHubsError::with_message("No source");
        assert!(error.source().is_none());
    }

    #[test]
    fn test_eventhubs_error_source_invalid_management_response() {
        let error = EventHubsError::from(ErrorKind::InvalidManagementResponse);
        assert!(error.source().is_none());
    }

    #[test]
    fn test_eventhubs_error_source_send_rejected() {
        let error = EventHubsError::from(ErrorKind::SendRejected(None));
        assert!(error.source().is_none());
    }

    #[test]
    fn test_eventhubs_error_display_simple_message() {
        let error = EventHubsError::with_message("Display test");
        assert_eq!(format!("{}", error), "Display test");
    }

    #[test]
    fn test_eventhubs_error_display_azure_core() {
        let azure_error =
            azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "Azure display");
        let error: EventHubsError = azure_error.into();
        let display = format!("{}", error);
        assert!(display.contains("Azure Core Error"));
        assert!(display.contains("Azure display"));
    }

    #[test]
    fn test_eventhubs_error_display_send_rejected() {
        let error = EventHubsError::from(ErrorKind::SendRejected(None));
        let display = format!("{}", error);
        assert!(display.contains("Send rejected"));
    }

    #[test]
    fn test_eventhubs_error_display_invalid_management_response() {
        let error = EventHubsError::from(ErrorKind::InvalidManagementResponse);
        assert_eq!(format!("{}", error), "Invalid management response");
    }

    #[test]
    fn test_eventhubs_error_display_amqp_error() {
        let amqp_error = AmqpError::from(azure_core_amqp::AmqpErrorKind::SimpleMessage(
            Cow::Borrowed("AMQP display"),
        ));
        let error: EventHubsError = amqp_error.into();
        let display = format!("{}", error);
        assert!(display.contains("AMQP Error"));
    }

    #[test]
    fn test_eventhubs_error_debug() {
        let error = EventHubsError::with_message("Debug test");
        let debug_output = format!("{:?}", error);
        assert!(debug_output.contains("Event Hubs Error"));
        assert!(debug_output.contains("Debug test"));
    }

    #[test]
    fn test_eventhubs_error_debug_complex() {
        let azure_error =
            azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "Complex");
        let error: EventHubsError = azure_error.into();
        let debug_output = format!("{:?}", error);
        assert!(debug_output.contains("Event Hubs Error"));
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_result() -> Result<String> {
            Ok("Success".to_string())
        }

        let result = returns_result();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
    }

    #[test]
    fn test_result_type_alias_error() {
        fn returns_error() -> Result<String> {
            Err(EventHubsError::with_message("Error"))
        }

        let result = returns_error();
        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "Error");
    }

    #[test]
    fn test_eventhubs_error_chain_conversions() {
        // Create an Azure Core error
        let azure_error =
            azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "Original error");

        // Convert to EventHubsError
        let eventhubs_error: EventHubsError = azure_error.into();

        // Verify it's stored as AzureCore variant
        assert!(matches!(eventhubs_error.kind, ErrorKind::AzureCore(_)));

        // Convert back to Azure Core error
        let converted_azure: azure_core::Error = eventhubs_error.into();

        assert!(format!("{}", converted_azure).contains("Original error"));
    }

    #[test]
    fn test_eventhubs_error_amqp_described_error_integration() {
        let mut info = AmqpOrderedMap::new();
        info.insert(
            AmqpSymbol("tracking-id".to_string()),
            azure_core_amqp::AmqpValue::String("12345".to_string()),
        );

        let described_error = AmqpDescribedError::new(
            AmqpErrorCondition::UnauthorizedAccess,
            Some("Unauthorized access to partition".to_string()),
            info,
        );

        let error = EventHubsError::from(ErrorKind::SendRejected(Some(described_error)));

        assert!(matches!(error.kind, ErrorKind::SendRejected(Some(_))));
        let display = format!("{}", error);
        assert!(display.contains("Send rejected"));
    }

    #[test]
    fn test_eventhubs_error_all_variants_can_be_displayed() {
        let errors = vec![
            EventHubsError::with_message("Simple"),
            EventHubsError::from(ErrorKind::InvalidManagementResponse),
            EventHubsError::from(ErrorKind::SendRejected(None)),
            EventHubsError::from(ErrorKind::AzureCore(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "Azure",
            ))),
            EventHubsError::from(ErrorKind::AmqpError(AmqpError::from(
                azure_core_amqp::AmqpErrorKind::SimpleMessage(Cow::Borrowed("AMQP")),
            ))),
        ];

        for error in errors {
            let _ = format!("{}", error);
            let _ = format!("{:?}", error);
        }
    }
}
