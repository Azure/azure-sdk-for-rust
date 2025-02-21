// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

pub use crate::management::error::AmqpManagementError;

pub enum AmqpErrorKind {
    ReceiverAlreadyAttached,
    CouldNotSetMessageReceiver,
    CbsAlreadyAttached,
    CbsNotSet,
    CbsNotAttached,
    ManagementError(AmqpManagementError),
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
            AmqpErrorKind::TransportImplementationError { source } => Some(source.as_ref()),
            AmqpErrorKind::ManagementError(e) => e.source(),
            AmqpErrorKind::ReceiverAlreadyAttached
            | AmqpErrorKind::CouldNotSetMessageReceiver
            | AmqpErrorKind::CbsAlreadyAttached
            | AmqpErrorKind::CbsNotSet
            | AmqpErrorKind::CbsNotAttached => None,
        }
    }
}

impl std::fmt::Display for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AmqpErrorKind::ManagementError(err) => {
                write!(f, "AMQP Management Error: {} ", err)
            }
            AmqpErrorKind::ReceiverAlreadyAttached => {
                f.write_str("AMQP Receiver is already attached")
            }
            AmqpErrorKind::CouldNotSetMessageReceiver => {
                f.write_str("Could not set message receiver.")
            }
            AmqpErrorKind::CbsAlreadyAttached => {
                f.write_str("Claims Based Security is already attached")
            }
            AmqpErrorKind::CbsNotSet => f.write_str("Claims Based Security is not set"),
            AmqpErrorKind::CbsNotAttached => f.write_str("Claims Based Security is not attached"),
            AmqpErrorKind::TransportImplementationError { source } => {
                write!(f, "Transport Implementation Error: {:?}", source)
            }
        }
    }
}

impl std::fmt::Debug for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AmqpError: {}", self)?;
        Ok(())
    }
}

impl From<AmqpError> for azure_core::Error {
    fn from(e: AmqpError) -> Self {
        Self::new(azure_core::error::ErrorKind::Amqp, Box::new(e))
    }
}
