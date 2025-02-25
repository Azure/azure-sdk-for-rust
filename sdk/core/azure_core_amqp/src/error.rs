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
    /// Remote peer closed the link
    ClosedByRemote(Option<AmqpDescribedError>),

    /// Remote peer detached
    DetachedByRemote(Option<AmqpDescribedError>),

    /// The connection was dropped.
    ConnectionDropped(Box<dyn std::error::Error + Send + Sync>),

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
            AmqpErrorKind::TransportImplementationError(s)
            | AmqpErrorKind::DetachError(s)
            | AmqpErrorKind::LinkStateError(s)
            | AmqpErrorKind::ConnectionDropped(s) => Some(s.as_ref()),
            AmqpErrorKind::ManagementError(e) => e.source(),
            AmqpErrorKind::SenderError(e) => e.source(),
            AmqpErrorKind::ReceiverError(e) => e.source(),
            AmqpErrorKind::SessionError(_) => None,
            AmqpErrorKind::ConnectionError(e) => e.source(),
            AmqpErrorKind::ClosedByRemote(_) | AmqpErrorKind::DetachedByRemote(_) => None,
        }
    }
}

impl std::fmt::Display for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AmqpErrorKind::DetachedByRemote(err) => {
                write!(f, "Remote detached with error: {:?}", err)
            }
            AmqpErrorKind::ClosedByRemote(err) => {
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
            AmqpErrorKind::TransportImplementationError(s) => {
                write!(f, "Transport Implementation Error: {}", s)
            }
            AmqpErrorKind::ConnectionDropped(s) => {
                write!(f, "Connection dropped: {}", s)
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
