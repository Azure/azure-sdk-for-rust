// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use azure_core::{error::ErrorKind as CoreErrorKind, fmt::SafeDebug};
use std::fmt;

/// The kind of Service Bus error.
#[derive(Debug, PartialEq, Eq)]
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
    source: Option<Box<dyn std::error::Error + 'static>>,
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
        source: Box<dyn std::error::Error + 'static>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            source: Some(source),
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
}

impl fmt::Display for ServiceBusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::error::Error for ServiceBusError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
    }
}

impl From<azure_core::error::Error> for ServiceBusError {
    fn from(error: azure_core::error::Error) -> Self {
        let kind = match error.kind() {
            CoreErrorKind::Io => ErrorKind::Amqp,
            CoreErrorKind::HttpResponse { .. } => ErrorKind::InvalidRequest,
            CoreErrorKind::Other => ErrorKind::Unknown,
            _ => ErrorKind::Unknown,
        };

        ServiceBusError::new(kind, error.to_string())
    }
}

impl From<azure_core_amqp::AmqpError> for ServiceBusError {
    fn from(error: azure_core_amqp::AmqpError) -> Self {
        ServiceBusError::with_source(ErrorKind::Amqp, format!("{}", error), Box::new(error))
    }
}
