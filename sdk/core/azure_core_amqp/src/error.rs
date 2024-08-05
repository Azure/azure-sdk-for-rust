// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp

pub enum ErrorKind {
    AmqpReceiverAlreadyAttached,
    TransportImplementationError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::TransportImplementationError { source } => source.source(),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::AmqpReceiverAlreadyAttached => {
                write!(f, "AMQP Receiver is already attached")
            }
            ErrorKind::TransportImplementationError { source } => {
                write!(f, "Transport Implementation Error: {:?}", source)
            }
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AMQP Transport Error: {}", self)
    }
}

impl From<Error> for azure_core::Error {
    fn from(e: Error) -> Self {
        Self::new(azure_core::error::ErrorKind::Other, Box::new(e))
    }
}

impl From<ErrorKind> for azure_core::Error {
    fn from(e: ErrorKind) -> Self {
        Self::new(
            azure_core::error::ErrorKind::Other,
            Box::new(Error { kind: e }),
        )
    }
}
