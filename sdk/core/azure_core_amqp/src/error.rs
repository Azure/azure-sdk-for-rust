// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

pub use crate::sender::error::AmqpSenderError;
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

    FramingError(Box<dyn std::error::Error + Send + Sync>),
    IdleTimeoutElapsed(Box<dyn std::error::Error + Send + Sync>),

    /// Transfer Limit Exceeded
    TransferLimitExceeded(Box<dyn std::error::Error + Send + Sync>),

    /// Management Status code
    ManagementStatusCode(azure_core::StatusCode, Option<String>),

    DetachError(Box<dyn std::error::Error + Send + Sync>),
    SenderError(AmqpSenderError),
    TransportImplementationError(Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, Clone, PartialEq)]
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
            AmqpErrorKind::SenderError(e) => e.source(),
            AmqpErrorKind::ManagementStatusCode(_, _) => None,
            AmqpErrorKind::ClosedByRemote(_) | AmqpErrorKind::DetachedByRemote(_) => None,
            AmqpErrorKind::TransferLimitExceeded(e) => Some(e.as_ref()),
            AmqpErrorKind::FramingError(e) => Some(e.as_ref()),
            AmqpErrorKind::IdleTimeoutElapsed(e) => Some(e.as_ref()),
        }
    }
}

impl std::fmt::Display for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AmqpErrorKind::ManagementStatusCode(status_code, d) => {
                if let Some(d) = d {
                    write!(
                        f,
                        "Management API returned status code: {} ({})",
                        status_code, d
                    )
                } else {
                    write!(f, "Management API returned status code: {}", status_code,)
                }
            }
            AmqpErrorKind::DetachedByRemote(err) => {
                write!(f, "Remote detached with error: {:?}", err)
            }
            AmqpErrorKind::ClosedByRemote(err) => {
                write!(f, "Remote closed with error: {:?}", err)
            }
            AmqpErrorKind::DetachError(err) => {
                write!(f, "AMQP Detach Error: {} ", err)
            }
            AmqpErrorKind::TransportImplementationError(s) => {
                write!(f, "Transport Implementation Error: {}", s)
            }
            AmqpErrorKind::ConnectionDropped(s) => {
                write!(f, "Connection dropped: {}", s)
            }
            AmqpErrorKind::FramingError(s) => {
                write!(f, "Connection Framing error: {}", s)
            }
            AmqpErrorKind::IdleTimeoutElapsed(s) => {
                write!(f, "Connection Idle Timeout elapsed: {}", s)
            }
            AmqpErrorKind::SenderError(err) => {
                write!(f, "AMQP Sender Error: {} ", err)
            }
            AmqpErrorKind::LinkStateError(err) => {
                write!(f, "AMQP Link State Error: {} ", err)
            }
            AmqpErrorKind::TransferLimitExceeded(e) => {
                write!(f, "AMQP Transfer Limit Exceeded: {e}")
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
