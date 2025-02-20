// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

pub struct AmqpManagementError {
    pub source: Box<dyn std::error::Error + Send + Sync>,
    pub management_operation: String,
    pub management_status: i16,
    pub management_description: String,
}
pub enum AmqpErrorKind {
    AmqpManagementAlreadyAttached,
    AmqpManagementNotAttached,
    AmqpReceiverAlreadyAttached,
    AmqpManagementError(AmqpManagementError),
    TransportImplementationError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub struct AmqpError {
    kind: AmqpErrorKind,
}

impl AmqpError {
    pub fn new(kind: AmqpErrorKind) -> Self {
        Self { kind }
    }
}

impl std::error::Error for AmqpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            AmqpErrorKind::TransportImplementationError { source } => source.source(),
            AmqpErrorKind::AmqpManagementError(err) => err.source.source(),
            _ => None,
        }
    }
}

impl std::fmt::Display for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AmqpErrorKind::AmqpManagementAlreadyAttached => {
                f.write_str("AMQP Management is already attached")
            }
            AmqpErrorKind::AmqpManagementNotAttached => {
                f.write_str("AMQP Management is not attached")
            }
            AmqpErrorKind::AmqpManagementError(err) => {
                write!(
                    f,
                    "AMQP Management Error: {}: {}: {}",
                    err.management_operation, err.management_status, err.management_description
                )
            }
            AmqpErrorKind::AmqpReceiverAlreadyAttached => {
                f.write_str("AMQP Receiver is already attached")
            }
            AmqpErrorKind::TransportImplementationError { source } => {
                write!(f, "Transport Implementation Error: {:?}", source)
            }
        }
    }
}

impl std::fmt::Debug for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AmqpError: {}", self);
        Ok(())
    }
}

impl From<AmqpError> for azure_core::Error {
    fn from(e: AmqpError) -> Self {
        Self::new(azure_core::error::ErrorKind::Amqp, Box::new(e))
    }
}
