// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use azure_core::{error::ErrorKind as CoreErrorKind, fmt::SafeDebug};
use std::fmt;

/// The kind of Service Bus error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// An error occurred in the underlying AMQP transport.
    Amqp,
    /// The operation was cancelled.
    Cancelled,
    /// The entity (queue, topic, or subscription) was not found.
    EntityNotFound,
    /// The request was invalid or malformed.
    InvalidRequest,
    /// A message lock was lost.
    MessageLockLost,
    /// A message was not found.
    MessageNotFound,
    /// The message size exceeds the maximum allowed size.
    MessageSizeExceeded,
    /// A quota was exceeded.
    QuotaExceeded,
    /// The request timed out.
    RequestTimeout,
    /// The sender or receiver has been closed.
    ServiceBusClosed,
    /// A session lock was lost.
    SessionLockLost,
    /// An unknown error occurred.
    Unknown,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Amqp => write!(f, "AMQP error"),
            ErrorKind::Cancelled => write!(f, "Operation was cancelled"),
            ErrorKind::EntityNotFound => write!(f, "Entity not found"),
            ErrorKind::InvalidRequest => write!(f, "Invalid request"),
            ErrorKind::MessageLockLost => write!(f, "Message lock lost"),
            ErrorKind::MessageNotFound => write!(f, "Message not found"),
            ErrorKind::MessageSizeExceeded => write!(f, "Message size exceeded"),
            ErrorKind::QuotaExceeded => write!(f, "Quota exceeded"),
            ErrorKind::RequestTimeout => write!(f, "Request timeout"),
            ErrorKind::ServiceBusClosed => write!(f, "Service Bus client closed"),
            ErrorKind::SessionLockLost => write!(f, "Session lock lost"),
            ErrorKind::Unknown => write!(f, "Unknown error"),
        }
    }
}

/// A Service Bus specific error.
#[derive(SafeDebug)]
pub struct ServiceBusError {
    kind: ErrorKind,
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl ServiceBusError {
    /// Creates a new Service Bus error.
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new Service Bus error with a source error.
    pub fn with_source(
        kind: ErrorKind,
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Returns the error kind.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the source error, if any.
    pub fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl fmt::Display for ServiceBusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::error::Error for ServiceBusError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl From<azure_core::error::Error> for ServiceBusError {
    fn from(error: azure_core::error::Error) -> Self {
        let kind = match error.kind() {
            CoreErrorKind::Io => ErrorKind::Amqp,
            CoreErrorKind::HttpResponse {
                status: _,
                error_code: _,
            } => ErrorKind::InvalidRequest,
            CoreErrorKind::Other => ErrorKind::Unknown,
            _ => ErrorKind::Unknown,
        };

        ServiceBusError::with_source(kind, error.to_string(), error)
    }
}

impl From<azure_core_amqp::AmqpError> for ServiceBusError {
    fn from(error: azure_core_amqp::AmqpError) -> Self {
        ServiceBusError::with_source(ErrorKind::Amqp, error.to_string(), error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_servicebus_error_can_store_any_std_error() {
        // Test that we can store any std::error::Error as a source
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "test error");
        let service_bus_error =
            ServiceBusError::with_source(ErrorKind::Unknown, "wrapper error", io_error);

        assert_eq!(service_bus_error.kind(), &ErrorKind::Unknown);
        assert_eq!(service_bus_error.message(), "wrapper error");
        assert!(service_bus_error.source().is_some());

        // Verify the source can be downcast to the original error type
        let source = service_bus_error.source().unwrap();
        assert!(source.downcast_ref::<std::io::Error>().is_some());
    }

    #[test]
    fn test_servicebus_error_implements_std_error() {
        let error = ServiceBusError::new(ErrorKind::InvalidRequest, "test message");

        // Should implement std::error::Error
        let _: &dyn std::error::Error = &error;

        // Should return None for source when no source is set
        assert!(error.source().is_none());
    }

    #[test]
    fn test_servicebus_error_with_chain() {
        let inner_error = std::io::Error::new(std::io::ErrorKind::Other, "inner error");
        let middle_error =
            ServiceBusError::with_source(ErrorKind::Amqp, "middle error", inner_error);
        let outer_error =
            ServiceBusError::with_source(ErrorKind::Unknown, "outer error", middle_error);

        // Check that we can traverse the error chain
        assert_eq!(outer_error.kind(), &ErrorKind::Unknown);
        assert_eq!(outer_error.message(), "outer error");

        let source = outer_error.source().unwrap();
        let middle_as_servicebus = source.downcast_ref::<ServiceBusError>().unwrap();
        assert_eq!(middle_as_servicebus.kind(), &ErrorKind::Amqp);
        assert_eq!(middle_as_servicebus.message(), "middle error");

        let inner_source = middle_as_servicebus.source().unwrap();
        assert!(inner_source.downcast_ref::<std::io::Error>().is_some());
    }
}
