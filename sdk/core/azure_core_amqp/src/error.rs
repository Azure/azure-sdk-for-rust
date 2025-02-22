// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

pub use crate::connection::error::AmqpConnectionError;
pub use crate::management::error::AmqpManagementError;
pub use crate::receiver::error::AmqpReceiverError;
pub use crate::sender::error::AmqpSenderError;
pub use crate::session::error::AmqpSessionError;
use crate::{AmqpOrderedMap, AmqpSymbol, AmqpValue};

pub enum AmqpErrorKind {
    CbsAlreadyAttached,
    CbsNotSet,
    CbsNotAttached,
    ConnectionError(AmqpConnectionError),
    SessionError(AmqpSessionError),
    ManagementError(AmqpManagementError),
    SenderError(AmqpSenderError),
    ReceiverError(AmqpReceiverError),
    TransportImplementationError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
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

pub struct AmqpError {
    kind: AmqpErrorKind,
}

impl AmqpError {
    pub fn new(kind: AmqpErrorKind) -> Self {
        Self { kind }
    }
}

impl From<AmqpErrorKind> for AmqpError {
    fn from(kind: AmqpErrorKind) -> Self {
        Self::new(kind)
    }
}

impl std::error::Error for AmqpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            AmqpErrorKind::TransportImplementationError { source } => Some(source.as_ref()),
            AmqpErrorKind::ManagementError(e) => e.source(),
            AmqpErrorKind::SenderError(e) => e.source(),
            AmqpErrorKind::ReceiverError(e) => e.source(),
            AmqpErrorKind::SessionError(e) => e.source(),
            AmqpErrorKind::ConnectionError(e) => e.source(),
            AmqpErrorKind::CbsAlreadyAttached
            | AmqpErrorKind::CbsNotSet
            | AmqpErrorKind::CbsNotAttached => None,
        }
    }
}

impl std::fmt::Display for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AmqpErrorKind::ConnectionError(err) => {
                write!(f, "AMQP Connection Error: {} ", err)
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
            AmqpErrorKind::TransportImplementationError { source } => {
                write!(f, "Transport Implementation Error: {:?}", source)
            }
            AmqpErrorKind::SenderError(err) => {
                write!(f, "AMQP Sender Error: {} ", err)
            }
            AmqpErrorKind::ReceiverError(err) => {
                write!(f, "AMQP Receiver Error: {} ", err)
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
        AmqpError::new(e).into()
    }
}

/// Errors from detaching a link. Common to both sender and receiver.
pub enum AmqpDetachError {
    /// ILlegal link state
    IllegalState,

    /// Session has dropped
    IllegalSessionState,

    // /// Expecting a detach but found other frame
    // #[error("Expecting a Detach")]
    // NonDetachFrameReceived,
    /// Remote peer detached with error
    RemoteDetachedWithError(AmqpDescribedError),

    /// Remote peer sent a closing detach when the local terminus sent a non-closing detach
    ClosedByRemote,

    /// Remote peer sent a non-closing detach when the local terminus is sending a closing detach
    DetachedByRemote,

    /// Remote peer closed the link with an error
    RemoteClosedWithError(AmqpDescribedError),
}

impl std::fmt::Display for AmqpDetachError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmqpDetachError::IllegalState => f.write_str("Illegal local state"),
            AmqpDetachError::IllegalSessionState => f.write_str("Illegal session state"),
            AmqpDetachError::RemoteDetachedWithError(e) => {
                write!(f, "Remote detached with an error: {:?}", e)
            }
            AmqpDetachError::ClosedByRemote => f.write_str("Link closed by remote"),
            AmqpDetachError::DetachedByRemote => {
                f.write_str("Link will be closed by local terminus")
            }
            AmqpDetachError::RemoteClosedWithError(e) => {
                write!(f, "Remote peer closed the link with an error: {:?}", e)
            }
        }
    }
}
impl std::fmt::Debug for AmqpDetachError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AmqpDetachError: {}", self)?;
        Ok(())
    }
}
impl std::error::Error for AmqpDetachError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// State management errors from the AMQP link - common for both sender and receiver
pub enum AmqpLinkStateError {
    /// Illegal link state
    IllegalState,

    /// Session has dropped
    IllegalSessionState,

    /// Remote peer detached
    RemoteDetached,

    /// Remote peer detached with error
    RemoteDetachedWithError(AmqpDescribedError),

    /// Remote peer closed
    RemoteClosed,

    /// Remote peer closed the link with an error
    RemoteClosedWithError(AmqpDescribedError),

    /// The link is expected to be detached immediately but didn't receive
    /// an incoming Detach frame
    ExpectImmediateDetach,
}

impl std::fmt::Display for AmqpLinkStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmqpLinkStateError::IllegalState => f.write_str("Illegal local state"),
            AmqpLinkStateError::IllegalSessionState => f.write_str("Illegal session state"),
            AmqpLinkStateError::RemoteDetached => f.write_str("Remote detached"),
            AmqpLinkStateError::RemoteDetachedWithError(e) => {
                write!(f, "Remote detached with an error: {:?}", e)
            }
            AmqpLinkStateError::RemoteClosed => f.write_str("Remote closed"),
            AmqpLinkStateError::RemoteClosedWithError(e) => {
                write!(f, "Remote peer closed the link with an error: {:?}", e)
            }
            AmqpLinkStateError::ExpectImmediateDetach => {
                f.write_str("Expecting an immediate detach")
            }
        }
    }
}
impl std::fmt::Debug for AmqpLinkStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AmqpLinkStateError: {}", self)?;
        Ok(())
    }
}
impl std::error::Error for AmqpLinkStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
