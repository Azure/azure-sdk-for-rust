// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
//cspell: words amqp

use super::error::{AmqpIllegalLinkState, AmqpReceiver, AmqpReceiverAttach};
use crate::messaging::{AmqpMessage, AmqpSource};
use crate::receiver::{AmqpReceiverOptions, AmqpReceiverTrait, ReceiverCreditMode};
use crate::session::AmqpSession;
use async_std::sync::Mutex;
use azure_core::error::Result;
use std::borrow::BorrowMut;
use std::sync::{Arc, OnceLock};
use tracing::trace;

#[derive(Debug, Default)]
pub(crate) struct Fe2o3AmqpReceiver {
    receiver: OnceLock<Arc<Mutex<fe2o3_amqp::Receiver>>>,
}

impl From<ReceiverCreditMode> for fe2o3_amqp::link::receiver::CreditMode {
    fn from(credit_mode: ReceiverCreditMode) -> Self {
        match credit_mode {
            ReceiverCreditMode::Auto(prefetch) => {
                fe2o3_amqp::link::receiver::CreditMode::Auto(prefetch)
            }
            ReceiverCreditMode::Manual => fe2o3_amqp::link::receiver::CreditMode::Manual,
        }
    }
}

impl AmqpReceiverTrait for Fe2o3AmqpReceiver {
    #[allow(unused_variables)]
    async fn attach(
        &self,
        session: &AmqpSession,
        source: impl Into<AmqpSource>,
        options: Option<AmqpReceiverOptions>,
    ) -> Result<()> {
        if self.receiver.get().is_some() {
            return Err(crate::error::Error::new(
                crate::error::ErrorKind::AmqpReceiverAlreadyAttached,
            )
            .into());
        }
        let options = options.unwrap_or_default();
        let name = options.name().clone().unwrap_or_default();
        let credit_mode = options.credit_mode().clone().unwrap_or_default();
        let auto_accept = options.auto_accept();
        let properties = options.properties().clone().unwrap_or_default();
        let source = source.into();

        let receiver = fe2o3_amqp::Receiver::builder()
            .receiver_settle_mode(fe2o3_amqp_types::definitions::ReceiverSettleMode::First)
            .source(source)
            .credit_mode(credit_mode.into())
            .auto_accept(auto_accept)
            .properties(properties.into())
            .name(name)
            .attach(session.0 .0.get().lock().await.borrow_mut())
            .await
            .map_err(AmqpReceiverAttach::from)?;
        self.receiver.set(Arc::new(Mutex::new(receiver))).unwrap();
        Ok(())
    }

    async fn max_message_size(&self) -> Option<u64> {
        self.receiver.get().unwrap().lock().await.max_message_size()
    }

    #[tracing::instrument]
    async fn receive(&self) -> Result<AmqpMessage> {
        let mut receiver = self.receiver.get().unwrap().lock().await;

        let delivery: fe2o3_amqp::link::delivery::Delivery<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        > = receiver.recv().await.map_err(AmqpReceiver::from)?;

        trace!("Received delivery: {:?}", delivery);

        trace!("Accepting delivery.");
        receiver
            .accept(&delivery)
            .await
            .map_err(AmqpIllegalLinkState::from)?;
        trace!("Accepted delivery");

        let message = AmqpMessage::from(delivery.into_message());
        Ok(message)
    }
}

impl Fe2o3AmqpReceiver {
    pub(crate) fn new() -> Self {
        Self {
            receiver: OnceLock::new(),
        }
    }
}
