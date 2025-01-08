// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
//cspell: words amqp

use super::error::{AmqpIllegalLinkState, AmqpLinkDetach, AmqpReceiver, AmqpReceiverAttach};
use crate::{
    messaging::{AmqpDelivery, AmqpSource},
    receiver::{AmqpReceiverApis, AmqpReceiverOptions, ReceiverCreditMode},
    session::AmqpSession,
};
use async_std::sync::Mutex;
use azure_core::error::Result;
use std::borrow::BorrowMut;
use std::sync::OnceLock;
use tracing::{info, trace, warn};

#[derive(Default)]
pub(crate) struct Fe2o3AmqpReceiver {
    receiver: OnceLock<Mutex<fe2o3_amqp::Receiver>>,
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

impl From<&fe2o3_amqp::link::receiver::CreditMode> for ReceiverCreditMode {
    fn from(credit_mode: &fe2o3_amqp::link::receiver::CreditMode) -> Self {
        match credit_mode {
            fe2o3_amqp::link::receiver::CreditMode::Auto(prefetch) => {
                ReceiverCreditMode::Auto(*prefetch)
            }
            fe2o3_amqp::link::receiver::CreditMode::Manual => ReceiverCreditMode::Manual,
        }
    }
}

impl AmqpReceiverApis for Fe2o3AmqpReceiver {
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
        let name = options.name.unwrap_or_default();
        let credit_mode = options.credit_mode.clone().unwrap_or_default();
        let auto_accept = options.auto_accept;
        let properties = options.properties.clone().unwrap_or_default();
        let source = source.into();

        let receiver = fe2o3_amqp::Receiver::builder()
            .receiver_settle_mode(fe2o3_amqp_types::definitions::ReceiverSettleMode::First)
            .source(source)
            .credit_mode(credit_mode.into())
            .auto_accept(auto_accept)
            .properties(properties.into())
            .name(name)
            .attach(session.implementation.get()?.lock().await.borrow_mut())
            .await
            .map_err(AmqpReceiverAttach::from)?;
        self.receiver.set(Mutex::new(receiver)).map_err(|_| {
            azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                "Could not set message receiver.",
            )
        })?;
        Ok(())
    }

    async fn detach(mut self) -> Result<()> {
        let receiver = self.receiver.take().ok_or_else(|| {
            azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                "Message Sender not set.",
            )
        })?;
        let res = receiver
            .into_inner()
            .detach()
            .await
            .map_err(|e| AmqpLinkDetach::from(e.1));
        match res {
            Ok(_) => Ok(()),
            Err(e) => match e.0 {
                fe2o3_amqp::link::DetachError::ClosedByRemote => {
                    info!("Error detaching receiver: {:?} - ignored", e);
                    Ok(())
                }
                _ => {
                    warn!("Error detaching receiver: {:?}", e);
                    Err(e.into())
                }
            },
        }
    }

    fn set_credit_mode(&self, credit_mode: ReceiverCreditMode) -> Result<()> {
        let receiver = self.receiver.get().ok_or_else(|| {
            azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                "Message receiver is not set.",
            )
        })?;
        receiver.lock_blocking().set_credit_mode(credit_mode.into());
        Ok(())
    }

    fn credit_mode(&self) -> Result<ReceiverCreditMode> {
        let receiver = self.receiver.get().ok_or_else(|| {
            azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                "Message receiver is not set.",
            )
        })?;
        Ok(receiver.lock_blocking().credit_mode().into())
    }

    async fn receive_delivery(&self) -> Result<AmqpDelivery> {
        let mut receiver = self
            .receiver
            .get()
            .ok_or_else(|| {
                azure_core::Error::message(
                    azure_core::error::ErrorKind::Other,
                    "Message receiver is not set.",
                )
            })?
            .lock()
            .await;

        let delivery: fe2o3_amqp::link::delivery::Delivery<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        > = receiver.recv().await.map_err(AmqpReceiver::from)?;

        trace!("Received delivery: {:?}", delivery);

        Ok(delivery.into())
    }

    async fn accept_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        let receiver = self
            .receiver
            .get()
            .ok_or_else(|| {
                azure_core::Error::message(
                    azure_core::error::ErrorKind::Other,
                    "Message receiver is not set.",
                )
            })?
            .lock()
            .await;

        trace!("Accepting delivery.");
        receiver
            .accept(&delivery.0.delivery)
            .await
            .map_err(AmqpIllegalLinkState::from)?;
        trace!("Accepted delivery.");

        Ok(())
    }

    async fn reject_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        let receiver = self
            .receiver
            .get()
            .ok_or_else(|| {
                azure_core::Error::message(
                    azure_core::error::ErrorKind::Other,
                    "Message receiver is not set.",
                )
            })?
            .lock()
            .await;

        trace!("Rejecting delivery.");
        receiver
            .reject(&delivery.0.delivery, None)
            .await
            .map_err(AmqpIllegalLinkState::from)?;
        trace!("Rejected delivery.");

        Ok(())
    }

    async fn release_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        let receiver = self
            .receiver
            .get()
            .ok_or_else(|| {
                azure_core::Error::message(
                    azure_core::error::ErrorKind::Other,
                    "Message receiver is not set.",
                )
            })?
            .lock()
            .await;

        trace!("Releasing delivery.");
        receiver
            .release(&delivery.0.delivery)
            .await
            .map_err(AmqpIllegalLinkState::from)?;
        trace!("Released delivery.");

        Ok(())
    }
}

impl Fe2o3AmqpReceiver {
    pub fn new() -> Self {
        Self {
            receiver: OnceLock::new(),
        }
    }
}
