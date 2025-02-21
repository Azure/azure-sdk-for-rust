// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::error::{AmqpDeliveryRejected, AmqpLinkDetach, AmqpNotAccepted, Fe2o3AmqpError};
use crate::{
    error::{AmqpDetachError, AmqpError, AmqpErrorKind, AmqpSenderError},
    messaging::{AmqpMessage, AmqpTarget},
    sender::{AmqpSendOptions, AmqpSenderApis, AmqpSenderOptions},
    session::AmqpSession,
};
use azure_core::Result;
use std::borrow::BorrowMut;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tracing::{info, warn};

#[derive(Default)]
pub(crate) struct Fe2o3AmqpSender {
    sender: OnceLock<Mutex<fe2o3_amqp::Sender>>,
}

impl AmqpSenderApis for Fe2o3AmqpSender {
    async fn attach(
        &self,
        session: &AmqpSession,
        name: String,
        target: impl Into<AmqpTarget>,
        options: Option<AmqpSenderOptions>,
    ) -> Result<()> {
        let mut session_builder = fe2o3_amqp::Sender::builder();

        if let Some(options) = options {
            // if let Some(link_credit) = options.link_credit {
            //     session_builder = session_builder.link_credit(link_credit);
            // }
            if let Some(sender_settle_mode) = options.sender_settle_mode {
                session_builder = session_builder.sender_settle_mode(sender_settle_mode.into());
            }
            if let Some(receiver_settle_mode) = options.receiver_settle_mode {
                session_builder = session_builder.receiver_settle_mode(receiver_settle_mode.into());
            }
            if let Some(max_message_size) = options.max_message_size {
                session_builder = session_builder.max_message_size(max_message_size);
            }

            if let Some(source) = options.source {
                session_builder = session_builder.source(source);
            }
            if let Some(offered_capabilities) = options.offered_capabilities {
                let capabilities = offered_capabilities.into_iter().map(|c| c.into()).collect();
                session_builder = session_builder.set_offered_capabilities(capabilities);
            }
            if let Some(desired_capabilities) = options.desired_capabilities {
                let capabilities = desired_capabilities.into_iter().map(|c| c.into()).collect();
                session_builder = session_builder.set_desired_capabilities(capabilities);
            }
            if let Some(properties) = options.properties {
                session_builder = session_builder.properties(properties.into());
            }
            if let Some(initial_delivery_count) = options.initial_delivery_count {
                session_builder = session_builder.initial_delivery_count(initial_delivery_count);
            }
        }
        let sender = session_builder
            .name(name)
            .target(target.into())
            .attach(session.implementation.get()?.lock().await.borrow_mut())
            .await
            .map_err(|e| AmqpError::new(AmqpErrorKind::SenderError(e.into())))?;
        self.sender.set(Mutex::new(sender)).map_err(|_| {
            AmqpError::new(AmqpErrorKind::SenderError(
                AmqpSenderError::CouldNotSetMessageSender,
            ))
        })?;
        Ok(())
    }

    async fn detach(mut self) -> Result<()> {
        let sender = self.sender.take().ok_or_else(|| {
            AmqpError::new(AmqpErrorKind::SenderError(
                AmqpSenderError::CouldNotGetMessageSender,
            ))
        })?;
        let res = sender
            .into_inner()
            .detach()
            .await
            .map_err(|e| AmqpLinkDetach::from(e.1));
        match res {
            Ok(_) => Ok(()),
            Err(e) => match e.0 {
                fe2o3_amqp::link::DetachError::ClosedByRemote => {
                    info!("Error detaching sender: {:?}", e);
                    Ok(())
                }
                _ => {
                    warn!("Error detaching sender: {:?}", e);
                    Err(e.into())
                }
            },
        }
    }

    async fn max_message_size(&self) -> azure_core::Result<Option<u64>> {
        Ok(self
            .sender
            .get()
            .ok_or_else(|| {
                AmqpError::new(AmqpErrorKind::SenderError(
                    AmqpSenderError::CouldNotGetMessageSender,
                ))
            })?
            .lock()
            .await
            .max_message_size())
    }

    async fn send(
        &self,
        message: impl Into<AmqpMessage> + std::fmt::Debug,
        options: Option<AmqpSendOptions>,
    ) -> Result<()> {
        let message: AmqpMessage = message.into();
        let message: fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        > = message.into();
        let mut sendable = fe2o3_amqp::link::delivery::Sendable {
            message,
            message_format: 0,
            settled: Default::default(),
        };
        if let Some(options) = options {
            if let Some(message_format) = options.message_format {
                sendable.message_format = message_format;
            }
            sendable.settled = options.settled;
        }

        let outcome = self
            .sender
            .get()
            .ok_or_else(|| {
                AmqpError::new(AmqpErrorKind::SenderError(
                    AmqpSenderError::CouldNotGetMessageSender,
                ))
            })?
            .lock()
            .await
            .borrow_mut()
            .send(sendable)
            .await
            .map_err(AmqpError::from)?;

        match outcome {
            fe2o3_amqp_types::messaging::Outcome::Accepted(_) => Ok(()),
            fe2o3_amqp_types::messaging::Outcome::Rejected(rejected) => {
                Err(AmqpDeliveryRejected(rejected).into())
            }
            _ => Err(azure_core::Error::from(AmqpError::new(
                AmqpErrorKind::TransportImplementationError {
                    source: Box::new(Fe2o3AmqpError::from(AmqpNotAccepted::from(outcome))),
                },
            ))),
        }
    }
}

impl Fe2o3AmqpSender {
    pub fn new() -> Self {
        Self {
            sender: OnceLock::new(),
        }
    }
}

impl From<fe2o3_amqp::link::DetachError> for AmqpError {
    fn from(e: fe2o3_amqp::link::DetachError) -> Self {
        AmqpError::new(AmqpErrorKind::DetachError(AmqpDetachError::from(e)))
    }
}

impl From<fe2o3_amqp::link::SendError> for AmqpError {
    fn from(e: fe2o3_amqp::link::SendError) -> Self {
        match e {
            fe2o3_amqp::link::SendError::LinkStateError(link_state_error) => {
                link_state_error.into()
            }
            fe2o3_amqp::link::SendError::Detached(detach_error) => detach_error.into(),
            fe2o3_amqp::link::SendError::NonTerminalDeliveryState => AmqpError::new(
                AmqpErrorKind::SenderError(AmqpSenderError::NonTerminalDeliveryState),
            ),
            fe2o3_amqp::link::SendError::IllegalDeliveryState => AmqpError::new(
                AmqpErrorKind::SenderError(AmqpSenderError::IllegalDeliveryState),
            ),
            fe2o3_amqp::link::SendError::MessageEncodeError => AmqpError::new(
                AmqpErrorKind::SenderError(AmqpSenderError::MessageEncodeError),
            ),
        }
    }
}

impl From<fe2o3_amqp::link::SenderAttachError> for AmqpSenderError {
    fn from(e: fe2o3_amqp::link::SenderAttachError) -> Self {
        match e {
            fe2o3_amqp::link::SenderAttachError::IllegalSessionState => {
                AmqpSenderError::IllegalSessionState
            }
            fe2o3_amqp::link::SenderAttachError::DuplicatedLinkName => {
                AmqpSenderError::DuplicatedLinkName
            }
            fe2o3_amqp::link::SenderAttachError::IllegalState => AmqpSenderError::IllegalState,
            fe2o3_amqp::link::SenderAttachError::NonAttachFrameReceived => {
                AmqpSenderError::NonAttachFrameReceived
            }
            fe2o3_amqp::link::SenderAttachError::ExpectImmediateDetach => {
                AmqpSenderError::ExpectImmediateDetach
            }
            fe2o3_amqp::link::SenderAttachError::IncomingTargetIsNone => {
                AmqpSenderError::IncomingTargetIsNone
            }
            fe2o3_amqp::link::SenderAttachError::CoordinatorIsNotImplemented => {
                AmqpSenderError::CoordinatorIsNotImplemented
            }
            fe2o3_amqp::link::SenderAttachError::SndSettleModeNotSupported => {
                AmqpSenderError::SndSettleModeNotSupported
            }
            fe2o3_amqp::link::SenderAttachError::RcvSettleModeNotSupported => {
                AmqpSenderError::RcvSettleModeNotSupported
            }
            fe2o3_amqp::link::SenderAttachError::TargetAddressIsNoneWhenDynamicIsTrue => {
                AmqpSenderError::TargetAddressIsNoneWhenDynamicIsTrue
            }
            fe2o3_amqp::link::SenderAttachError::SourceAddressIsSomeWhenDynamicIsTrue => {
                AmqpSenderError::SourceAddressIsSomeWhenDynamicIsTrue
            }
            fe2o3_amqp::link::SenderAttachError::DynamicNodePropertiesIsSomeWhenDynamicIsFalse => {
                AmqpSenderError::DynamicNodePropertiesIsSomeWhenDynamicIsFalse
            }
            fe2o3_amqp::link::SenderAttachError::RemoteClosedWithError(error) => {
                AmqpSenderError::RemoteClosedWithError(error.into())
            }
        }
    }
}

impl From<fe2o3_amqp::link::LinkStateError> for AmqpError {
    fn from(e: fe2o3_amqp::link::LinkStateError) -> Self {
        match e {
            fe2o3_amqp::link::LinkStateError::IllegalState => {
                AmqpError::new(AmqpErrorKind::SenderError(AmqpSenderError::IllegalState))
            }
            fe2o3_amqp::link::LinkStateError::IllegalSessionState => AmqpError::new(
                AmqpErrorKind::SenderError(AmqpSenderError::IllegalSessionState),
            ),
            fe2o3_amqp::link::LinkStateError::ExpectImmediateDetach => AmqpError::new(
                AmqpErrorKind::SenderError(AmqpSenderError::ExpectImmediateDetach),
            ),
            fe2o3_amqp::link::LinkStateError::RemoteDetachedWithError(error) => AmqpError::new(
                AmqpErrorKind::DetachError(AmqpDetachError::RemoteDetachedWithError(error.into())),
            ),
            fe2o3_amqp::link::LinkStateError::RemoteClosedWithError(error) => AmqpError::new(
                AmqpErrorKind::SenderError(AmqpSenderError::RemoteClosedWithError(error.into())),
            ),
            fe2o3_amqp::link::LinkStateError::RemoteDetached => AmqpError::new(
                AmqpErrorKind::DetachError(AmqpDetachError::DetachedByRemote),
            ),
            fe2o3_amqp::link::LinkStateError::RemoteClosed => {
                AmqpError::new(AmqpErrorKind::DetachError(AmqpDetachError::ClosedByRemote))
            }
        }
    }
}
