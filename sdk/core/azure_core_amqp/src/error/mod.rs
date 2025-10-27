// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

mod error_condition;

use crate::{AmqpOrderedMap, AmqpSymbol, AmqpValue};
use std::borrow::Cow;

pub use error_condition::*;

/// A convenience alias for `Result` where the error type is hard coded to [`AmqpError`].
pub type Result<T> = std::result::Result<T, AmqpError>;

/// Type of AMQP error.
pub enum AmqpErrorKind {
    /// A simple message
    SimpleMessage(Cow<'static, str>),

    /// Azure Core error.
    AzureCore(azure_core::Error),

    /// Described error - An error described by the remote peer.
    AmqpDescribedError(AmqpDescribedError),

    /// Remote peer closed the link
    LinkClosedByRemote(Box<dyn std::error::Error + Send + Sync>),
    /// Remote peer closed the session
    SessionClosedByRemote(Box<dyn std::error::Error + Send + Sync>),
    /// Remote peer closed the connection
    ConnectionClosedByRemote(Box<dyn std::error::Error + Send + Sync>),

    /// Remote peer detached the link
    LinkDetachedByRemote(Box<dyn std::error::Error + Send + Sync>),
    /// Remote peer detached the session
    SessionDetachedByRemote(Box<dyn std::error::Error + Send + Sync>),

    /// Remote peer detached the connection
    ConnectionDetachedByRemote(Box<dyn std::error::Error + Send + Sync>),

    /// The send request was rejected by the remote peer.
    NonTerminalDeliveryState,

    /// The send request was rejected by the remote peer.
    IllegalDeliveryState,

    /// The connection was dropped.
    ConnectionDropped(Box<dyn std::error::Error + Send + Sync>),

    /// Link State error.
    LinkStateError(Box<dyn std::error::Error + Send + Sync>),

    /// Framing Error
    FramingError(Box<dyn std::error::Error + Send + Sync>),

    /// Idle Timeout Elapsed
    IdleTimeoutElapsed(Box<dyn std::error::Error + Send + Sync>),

    /// Transfer Limit Exceeded
    TransferLimitExceeded(Box<dyn std::error::Error + Send + Sync>),

    /// Management Status code
    ManagementStatusCode(azure_core::http::StatusCode, Option<String>),

    /// Detach Error
    DetachError(Box<dyn std::error::Error + Send + Sync>),
    /// Transport Implementation Error
    TransportImplementationError(Box<dyn std::error::Error + Send + Sync>),

    /// A send was rejected.
    SendRejected,
}

impl From<azure_core::Error> for AmqpError {
    fn from(error: azure_core::Error) -> Self {
        AmqpErrorKind::AzureCore(error).into()
    }
}

impl From<AmqpError> for azure_core::Error {
    fn from(value: AmqpError) -> Self {
        match value.kind {
            AmqpErrorKind::AzureCore(e) => e,
            _ => azure_core::Error::with_error(
                azure_core::error::ErrorKind::Other,
                value,
                "AMQP error",
            ),
        }
    }
}

/// An AMQP described error.
#[derive(Debug, Clone, PartialEq)]
pub struct AmqpDescribedError {
    /// The error condition.
    pub condition: AmqpErrorCondition,
    /// An optional description of the error.
    pub description: Option<String>,
    /// Optional additional information about the error.
    pub info: AmqpOrderedMap<AmqpSymbol, AmqpValue>,
}

impl AmqpDescribedError {
    /// Creates a new instance of `AmqpDescribedError`.
    ///
    /// # Arguments
    /// - `condition`: The error condition as an `AmqpErrorCondition`.
    /// - `description`: An optional description of the error.
    /// - `info`: Optional additional information as an `AmqpOrderedMap`.
    pub fn new(
        condition: AmqpErrorCondition,
        description: Option<String>,
        info: AmqpOrderedMap<AmqpSymbol, AmqpValue>,
    ) -> Self {
        Self {
            condition,
            description,
            info,
        }
    }
}

/// An AMQP error from the AMQP stack.
pub struct AmqpError {
    /// Type of error.
    kind: AmqpErrorKind,
}

impl AmqpError {
    /// Returns a reference to the kind of AMQP error.
    pub fn kind(&self) -> &AmqpErrorKind {
        &self.kind
    }

    /// Returns a simple message AMQP error.
    pub fn with_message<C>(message: C) -> AmqpError
    where
        C: Into<Cow<'static, str>>,
    {
        Self::from(AmqpErrorKind::SimpleMessage(message.into()))
    }

    /// Creates a new management error. For test purposes only.
    #[cfg(feature = "test")]
    pub fn new_management_error(
        status_code: azure_core::http::StatusCode,
        description: Option<String>,
    ) -> Self {
        Self {
            kind: AmqpErrorKind::ManagementStatusCode(status_code, description),
        }
    }

    /// Creates a new described error. For test purposes only.
    #[cfg(feature = "test")]
    pub fn new_described_error(
        condition: AmqpErrorCondition,
        description: Option<String>,
        info: AmqpOrderedMap<AmqpSymbol, AmqpValue>,
    ) -> Self {
        Self {
            kind: AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
                condition,
                description,
                info,
            )),
        }
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
            AmqpErrorKind::AzureCore(e) => Some(e),

            AmqpErrorKind::TransportImplementationError(e)
            | AmqpErrorKind::DetachError(e)
            | AmqpErrorKind::LinkClosedByRemote(e)
            | AmqpErrorKind::LinkDetachedByRemote(e)
            | AmqpErrorKind::SessionClosedByRemote(e)
            | AmqpErrorKind::SessionDetachedByRemote(e)
            | AmqpErrorKind::ConnectionClosedByRemote(e)
            | AmqpErrorKind::ConnectionDetachedByRemote(e)
            | AmqpErrorKind::LinkStateError(e)
            | AmqpErrorKind::ConnectionDropped(e)
            | AmqpErrorKind::TransferLimitExceeded(e)
            | AmqpErrorKind::FramingError(e)
            | AmqpErrorKind::IdleTimeoutElapsed(e) => Some(e.as_ref()),

            AmqpErrorKind::ManagementStatusCode(_, _)
            | AmqpErrorKind::NonTerminalDeliveryState
            | AmqpErrorKind::SimpleMessage(_)
            | AmqpErrorKind::IllegalDeliveryState
            | AmqpErrorKind::SendRejected
            | AmqpErrorKind::AmqpDescribedError(_) => None,
        }
    }
}

impl std::fmt::Display for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AmqpErrorKind::SimpleMessage(msg) => write!(f, "{}", msg),
            AmqpErrorKind::AzureCore(e) => {
                write!(f, "Azure Core Error: {}", e)
            }
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
            AmqpErrorKind::ConnectionDetachedByRemote(err) => {
                write!(f, "Remote connection detached with error: {}", err)
            }
            AmqpErrorKind::LinkDetachedByRemote(err) => {
                write!(f, "Remote link detached with error: {}", err)
            }
            AmqpErrorKind::SessionDetachedByRemote(err) => {
                write!(f, "Remote session detached with error: {}", err)
            }
            AmqpErrorKind::LinkClosedByRemote(err) => {
                write!(f, "Remote link closed with error: {}", err)
            }
            AmqpErrorKind::SessionClosedByRemote(err) => {
                write!(f, "Remote session closed with error: {}", err)
            }
            AmqpErrorKind::ConnectionClosedByRemote(err) => {
                write!(f, "Remote connection closed with error: {}", err)
            }
            AmqpErrorKind::DetachError(err) => {
                write!(f, "Detach Error: {} ", err)
            }
            AmqpErrorKind::SendRejected => {
                write!(f, "Send Rejected with no error information")
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
            AmqpErrorKind::LinkStateError(err) => {
                write!(f, "Link State Error: {} ", err)
            }
            AmqpErrorKind::TransferLimitExceeded(e) => {
                write!(f, "Transfer Limit Exceeded: {e}")
            }
            AmqpErrorKind::NonTerminalDeliveryState => {
                write!(f, "Non Terminal Delivery State")
            }
            AmqpErrorKind::IllegalDeliveryState => {
                write!(f, "Illegal Delivery State")
            }
            AmqpErrorKind::AmqpDescribedError(e) => {
                write!(
                    f,
                    "AMQP Described Error: condition: {:?}, description: {:?}, info: {:?}",
                    e.condition, e.description, e.info
                )
            }
        }
    }
}

impl std::fmt::Debug for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AMQP Error: {}", self)?;
        Ok(())
    }
}

mod tests;
