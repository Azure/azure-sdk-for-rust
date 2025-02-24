// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

pub use crate::connection::error::AmqpConnectionError;
pub use crate::management::error::AmqpManagementError;
pub use crate::receiver::error::AmqpReceiverError;
pub use crate::sender::error::AmqpSenderError;
pub use crate::session::error::AmqpSessionError;
use crate::{AmqpOrderedMap, AmqpSymbol, AmqpValue};

/// Type of AMQP error.
pub enum AmqpErrorKind {
    CbsAlreadyAttached,
    CbsNotSet,
    CbsNotAttached,
    ClosedByRemote,
    ClosedByRemoteWithError(AmqpDescribedError),
    /// Remote peer detached
    DetachedByRemote,

    /// Remote peer detached with error
    DetachedByRemoteWithError(AmqpDescribedError),

    /// Link State error.
    LinkStateError(Box<dyn std::error::Error + Send + Sync>),
    DetachError(Box<dyn std::error::Error + Send + Sync>),
    ConnectionError(AmqpConnectionError),
    SessionError(AmqpSessionError),
    ManagementError(AmqpManagementError),
    SenderError(AmqpSenderError),
    ReceiverError(AmqpReceiverError),
    TransportImplementationError(Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, Clone)]
pub struct AmqpDescribedError {
    condition: AmqpSymbol,
    description: Option<String>,
    info: AmqpOrderedMap<AmqpSymbol, AmqpValue>,
}

impl AmqpDescribedError {
    pub fn new(
        condition: AmqpSymbol,
        description: Option<String>,
        info: AmqpOrderedMap<AmqpSymbol, AmqpValue>,
    ) -> Self {
        Self {
            condition,
            description,
            info,
        }
    }

    pub fn condition(&self) -> &AmqpSymbol {
        &self.condition
    }
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn info(&self) -> &AmqpOrderedMap<AmqpSymbol, AmqpValue> {
        &self.info
    }
}

/// An AMQP error from the AMQP stack.
pub struct AmqpError {
    /// Type of error.
    kind: AmqpErrorKind,
}

impl AmqpError {
    pub fn kind(&self) -> &AmqpErrorKind {
        &self.kind
    }
}

impl From<AmqpErrorKind> for AmqpError {
    fn from(kind: AmqpErrorKind) -> Self {
        Self { kind }
    }
}

impl std::error::Error for AmqpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            AmqpErrorKind::TransportImplementationError(s) => Some(s.as_ref()),
            AmqpErrorKind::ManagementError(e) => e.source(),
            AmqpErrorKind::SenderError(e) => e.source(),
            AmqpErrorKind::ReceiverError(e) => e.source(),
            AmqpErrorKind::SessionError(e) => e.source(),
            AmqpErrorKind::ConnectionError(e) => e.source(),
            AmqpErrorKind::LinkStateError(e) => Some(e.as_ref()),
            AmqpErrorKind::DetachError(e) => Some(e.as_ref()),
            AmqpErrorKind::ClosedByRemoteWithError(_)
            | AmqpErrorKind::DetachedByRemoteWithError(_) => None,
            AmqpErrorKind::CbsAlreadyAttached
            | AmqpErrorKind::CbsNotSet
            | AmqpErrorKind::CbsNotAttached
            | AmqpErrorKind::DetachedByRemote
            | AmqpErrorKind::ClosedByRemote => None,
        }
    }
}

impl std::fmt::Display for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AmqpErrorKind::ClosedByRemote => f.write_str("Remote closed"),
            AmqpErrorKind::DetachedByRemote => f.write_str("Remote detached"),
            AmqpErrorKind::DetachedByRemoteWithError(err) => {
                write!(f, "Remote detached with error: {:?}", err)
            }
            AmqpErrorKind::ClosedByRemoteWithError(err) => {
                write!(f, "Remote closed with error: {:?}", err)
            }
            AmqpErrorKind::ConnectionError(err) => {
                write!(f, "AMQP Connection Error: {} ", err)
            }
            AmqpErrorKind::DetachError(err) => {
                write!(f, "AMQP Detach Error: {} ", err)
            }
            AmqpErrorKind::SessionError(err) => {
                write!(f, "AMQP Session Error: {} ", err)
            }
            AmqpErrorKind::ManagementError(err) => {
                write!(f, "AMQP Management Error: {} ", err)
            }
            AmqpErrorKind::CbsAlreadyAttached => {
                f.write_str("Claims Based Security is already attached")
            }
            AmqpErrorKind::CbsNotSet => f.write_str("Claims Based Security is not set"),
            AmqpErrorKind::CbsNotAttached => f.write_str("Claims Based Security is not attached"),
            AmqpErrorKind::TransportImplementationError(s) => {
                write!(f, "Transport Implementation Error: {:?}", s)
            }
            AmqpErrorKind::SenderError(err) => {
                write!(f, "AMQP Sender Error: {} ", err)
            }
            AmqpErrorKind::ReceiverError(err) => {
                write!(f, "AMQP Receiver Error: {} ", err)
            }
            AmqpErrorKind::LinkStateError(err) => {
                write!(f, "AMQP Link State Error: {} ", err)
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

impl From<AmqpErrorKind> for azure_core::Error {
    fn from(e: AmqpErrorKind) -> Self {
        AmqpError::from(e).into()
    }
}
