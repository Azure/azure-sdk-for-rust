// cspell: words amqp eventhubs
use crate::amqp_client::error::AmqpError;

pub enum ErrorKind {
    InvalidParameter(String),
    MissingConnectionString,
    MissingSharedAccessKeyName,
    MissingEndpoint,
    MissingHostInEndpoint,
    MissingConnection,
    InvalidManagementResponse,
    AmqpError { source: AmqpError },
}

pub struct EventhubsError {
    kind: ErrorKind,
}

impl std::error::Error for EventhubsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for EventhubsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::InvalidManagementResponse => write!(f, "Invalid management response"),
            ErrorKind::AmqpError { source } => write!(f, "AmqpError: {:?}", source),
            ErrorKind::MissingConnection => write!(f, "Connection is not yet open."),
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
