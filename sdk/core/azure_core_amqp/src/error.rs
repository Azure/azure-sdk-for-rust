// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

pub use crate::management::error::AmqpManagementError;
pub use crate::sender::error::AmqpSenderError;
use crate::{AmqpOrderedMap, AmqpSymbol, AmqpValue};

pub enum AmqpErrorKind {
    ReceiverAlreadyAttached,
    CouldNotSetMessageReceiver,
    CbsAlreadyAttached,
    CbsNotSet,
    CbsNotAttached,
    LinkStateError(AmqpLinkStateError),
    DetachError(AmqpDetachError),
    ManagementError(AmqpManagementError),
    SenderError(AmqpSenderError),
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

impl std::error::Error for AmqpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            AmqpErrorKind::TransportImplementationError { source } => Some(source.as_ref()),
            AmqpErrorKind::ManagementError(e) => e.source(),
            AmqpErrorKind::SenderError(e) => e.source(),
            AmqpErrorKind::DetachError(e) => e.source(),
            AmqpErrorKind::LinkStateError(e) => e.source(),
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
            AmqpErrorKind::DetachError(err) => {
                write!(f, "AMQP Detach Error: {} ", err)
            }
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
            AmqpErrorKind::SenderError(err) => {
                write!(f, "AMQP Sender Error: {} ", err)
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
