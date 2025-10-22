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

    /// The host is missing in the endpoint.
    MissingHostInEndpoint,

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
            ErrorKind::AmqpError(source) => write!(f, "AmqpError: {:?}", source),
            ErrorKind::MissingHostInEndpoint => f.write_str("Missing host in endpoint"),
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

impl TryFrom<EventHubsError> for azure_core::Error {
    type Error = EventHubsError;
    fn try_from(value: EventHubsError) -> std::result::Result<Self, Self::Error> {
        match value.kind {
            ErrorKind::AzureCore(e) => Ok(e),
            _ => Err(EventHubsError::with_message(
                "EventHubs error is not an Azure Error",
            )),
        }
    }
}
