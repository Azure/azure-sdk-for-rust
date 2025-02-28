// Copyright (c) Microsoft Corporation. All Rights Reserved.
// Licensed under the MIT License.

use azure_core_amqp::AmqpDescribedError;

/// Represents the different kinds of errors that can occur in the Eventhubs module.
#[derive(Debug)]
pub enum ErrorKind {
    /// The session was missing for the partition.
    MissingSession,
    //
    /// The host is missing in the endpoint.
    MissingHostInEndpoint,

    /// Missing Message Sender
    MissingMessageSender,

    /// The connection is not yet open.
    MissingConnection,

    /// The management client is not yet open.
    MissingManagementClient,

    /// The management response is invalid.
    InvalidManagementResponse,

    /// Unable to add authentication token.
    UnableToAddAuthenticationToken,

    /// Unable to add a connection.
    UnableToAddConnection,

    /// The message was rejected.
    SendRejected(Option<AmqpDescribedError>),

    /// Represents the source of the AMQP error.
    /// This is used to wrap an AMQP error in an Even Hubs error.
    ///
    AmqpError(Box<dyn std::error::Error + Send + Sync>),
}

/// Represents an error that can occur in the Event Hubs module.
pub struct EventHubsError {
    /// The kind of error that occurred.
    pub kind: ErrorKind,
}

impl std::error::Error for EventHubsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::AmqpError(source) => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl std::fmt::Display for EventHubsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::UnableToAddConnection => f.write_str("Unable to add connection."),
            ErrorKind::MissingMessageSender => f.write_str("Missing message sender."),
            ErrorKind::SendRejected(e) => write!(f, "Send rejected: {:?}", e),
            ErrorKind::InvalidManagementResponse => f.write_str("Invalid management response"),
            ErrorKind::UnableToAddAuthenticationToken => {
                f.write_str("Unable to add authentication token")
            }
            ErrorKind::MissingSession => {
                f.write_str("The session for the specified partition is missing.")
            }
            ErrorKind::AmqpError(source) => write!(f, "AmqpError: {:?}", source),
            ErrorKind::MissingConnection => f.write_str("Connection is not yet open."),
            ErrorKind::MissingManagementClient => f.write_str("Missing management client."),
            ErrorKind::MissingHostInEndpoint => f.write_str("Missing host in endpoint"),
        }
    }
}

impl std::fmt::Debug for EventHubsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event Hubs Error: {}", self)
    }
}

impl From<EventHubsError> for azure_core::Error {
    fn from(e: EventHubsError) -> Self {
        Self::new(azure_core::error::ErrorKind::Other, Box::new(e))
    }
}

impl From<ErrorKind> for EventHubsError {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}
