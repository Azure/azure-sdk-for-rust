// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
//cspell: words amqp

use crate::messaging::{AmqpMessage, AmqpTarget};
use crate::sender::{AmqpSendOptions, AmqpSenderApis, AmqpSenderOptions};
use crate::session::AmqpSession;
use async_std::sync::Mutex;
use azure_core::Result;
use std::borrow::BorrowMut;
use std::sync::OnceLock;

use super::error::{
    AmqpDeliveryRejected, AmqpLinkDetach, AmqpNotAccepted, AmqpSenderAttach, AmqpSenderSend,
    Fe2o3AmqpError,
};

#[derive(Debug, Default)]
pub(crate) struct Fe2o3AmqpSender {
//    phantom_mutex: Mutex<()>,
    sender: OnceLock<Mutex<fe2o3_amqp::Sender>>,
//sender: OnceLock<fe2o3_amqp::Sender>,
}

impl AmqpSenderApis for Fe2o3AmqpSender {
    async fn attach(
        &self,
        session: &AmqpSession,
        name: impl Into<String>,
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
            .name(name.into())
            .target(target.into())
            .attach(session.implementation.get()?.lock().await.borrow_mut())
            .await
            .map_err(AmqpSenderAttach::from)?;
//        self.sender.set(Mutex::new(sender)).map_err(|_| {
    self.sender.set(Mutex::new(sender)).map_err(|_| {
                    azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                "Could not set message sender.",
            )
        })?;
        Ok(())
    }

    async fn detach(mut self) -> Result<()> {
        let sender = self.sender.take().ok_or_else(|| {
            azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                "Message Sender not set.",
            )
        })?;
        sender.into_inner()
            .detach()
            .await
            .map_err(|e| AmqpLinkDetach::from(e.1))?;
        Ok(())
    }

    fn max_message_size(&self) -> azure_core::Result<Option<u64>> {
        Ok(self
            .sender
            .get()
            .ok_or_else(|| {
                azure_core::Error::message(
                    azure_core::error::ErrorKind::Other,
                    "Message Sender not set.",
                )
            })?.lock_blocking()
            .max_message_size())
    }

    #[tracing::instrument]
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
                azure_core::Error::message(
                    azure_core::error::ErrorKind::Other,
                    "Message Sender not set.",
                )
            })?.lock().await.borrow_mut()
            .send(sendable)
            .await
            .map_err(AmqpSenderSend::from)?;

        match outcome {
            fe2o3_amqp_types::messaging::Outcome::Accepted(_) => Ok(()),
            fe2o3_amqp_types::messaging::Outcome::Rejected(rejected) => {
                Err(AmqpDeliveryRejected(rejected).into())
            }
            _ => Err(Fe2o3AmqpError::from(AmqpNotAccepted::from(outcome)).into()),
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
