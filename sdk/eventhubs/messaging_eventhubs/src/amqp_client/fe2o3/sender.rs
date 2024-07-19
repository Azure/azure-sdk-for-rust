//cspell: words amqp

use crate::amqp_client::messaging::{AmqpMessage, AmqpTarget};
use crate::amqp_client::sender::{AmqpSenderOptions, AmqpSenderTrait};
use crate::amqp_client::session::AmqpSession;
use azure_core::error::Result;
use std::borrow::BorrowMut;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

use super::error::AmqpSenderAttachError;

#[derive(Debug)]
pub(crate) struct Fe2o3AmqpSender {
    sender: OnceLock<Arc<Mutex<fe2o3_amqp::Sender>>>,
}

impl AmqpSenderTrait for Fe2o3AmqpSender {
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
            .attach(session.0 .0.get().lock().await.borrow_mut())
            .await
            .map_err(AmqpSenderAttachError::from)?;
        self.sender.set(Arc::new(Mutex::new(sender))).unwrap();
        Ok(())
    }

    async fn max_message_size(&self) -> Option<u64> {
        self.sender.get().unwrap().lock().await.max_message_size()
    }

    async fn send(&self, _message: AmqpMessage) -> Result<()> {
        todo!()
    }
}

impl Fe2o3AmqpSender {
    pub(crate) fn new() -> Self {
        Self {
            sender: OnceLock::new(),
        }
    }

    /// Returns a reference to the sender
    pub(crate) fn get(&self) -> Arc<Mutex<fe2o3_amqp::Sender>> {
        self.sender.get().unwrap().clone()
    }
}
