// Copyright (c) Microsoft Corporation. All Rights Reserved.
// Licensed under the MIT License.

// cspell: words amqp
use azure_core_amqp::error::Error;

/// Represents the different kinds of errors that can occur in the Eventhubs module.
pub enum ErrorKind {
    /// An arithmetic overflow has occurred.
    ArithmeticError,
    /// An invalid parameter was passed to a function.
    InvalidParameter(String),

    /// The connection string is missing.
    MissingConnectionString,

    /// The shared access key name is missing.
    MissingSharedAccessKeyName,

    /// The endpoint is missing.
    MissingEndpoint,

    /// The session was missing for the partition.
    MissingSession,

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

    /// Represents the source of the AMQP error.
    /// This is used to wrap an AMQP error in an Eventhubs error.
    ///
    AmqpError(Error),
}

/// Represents an error that can occur in the Eventhubs module.
pub struct EventhubsError {
    kind: ErrorKind,
}

impl std::error::Error for EventhubsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::AmqpError(source) => Some(source),
            _ => None,
        }
    }
}

impl std::fmt::Display for EventhubsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::MissingMessageSender => write!(f, "Missing message sender."),
            ErrorKind::ArithmeticError => write!(f, "Arithmetic overflow has occurred."),
            ErrorKind::InvalidManagementResponse => write!(f, "Invalid management response"),
            ErrorKind::UnableToAddAuthenticationToken => {
                write!(f, "Unable to add authentication token")
            }
            ErrorKind::MissingSession => {
                write!(f, "The session for the specified partition is missing.")
            }
            ErrorKind::AmqpError(source) => write!(f, "AmqpError: {:?}", source),
            ErrorKind::MissingConnection => write!(f, "Connection is not yet open."),
            ErrorKind::MissingManagementClient => write!(f, "Missing management client."),
            ErrorKind::InvalidParameter(s) => write!(f, "Invalid parameter: {}", s),
            ErrorKind::MissingConnectionString => write!(f, "Missing connection string"),
            ErrorKind::MissingSharedAccessKeyName => {
                write!(f, "Missing shared access key name")
            }
            ErrorKind::MissingEndpoint => write!(f, "Missing endpoint"),
            ErrorKind::MissingHostInEndpoint => write!(f, "Missing host in endpoint"),
        }
    }
}

impl std::fmt::Debug for EventhubsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventhubsError: {}", self)
    }
}

impl From<EventhubsError> for azure_core::Error {
    fn from(e: EventhubsError) -> Self {
        Self::new(azure_core::error::ErrorKind::Other, Box::new(e))
    }
}

impl From<ErrorKind> for azure_core::Error {
    fn from(e: ErrorKind) -> Self {
        Self::new(
            azure_core::error::ErrorKind::Other,
            Box::new(EventhubsError { kind: e }),
        )
    }
}
