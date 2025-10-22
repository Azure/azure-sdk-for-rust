// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::{AmqpErrorKind, Result},
    messaging::{AmqpDelivery, AmqpSource},
    receiver::{AmqpReceiverApis, AmqpReceiverOptions, ReceiverCreditMode},
    session::AmqpSession,
    AmqpError,
};
use std::borrow::BorrowMut;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tracing::{info, trace, warn};

use super::error::Fe2o3ReceiverAttachError;

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

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpReceiverApis for Fe2o3AmqpReceiver {
    async fn attach(
        &self,
        session: &AmqpSession,
        source: impl Into<AmqpSource> + Send,
        options: Option<AmqpReceiverOptions>,
    ) -> Result<()> {
        if self.receiver.get().is_some() {
            return Err(Self::receiver_already_attached());
        }
        let options = options.unwrap_or_default();
        let name = options.name.unwrap_or_default();
        let credit_mode = options.credit_mode.unwrap_or_default();
        let auto_accept = options.auto_accept;
        let properties = options.properties.unwrap_or_default();
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
            .map_err(|e| AmqpError::from(Fe2o3ReceiverAttachError(e)))?;
        self.receiver
            .set(Mutex::new(receiver))
            .map_err(|_| Self::could_not_set_message_receiver())?;
        Ok(())
    }

    async fn detach(mut self) -> Result<()> {
        let receiver = self.receiver.take().ok_or_else(Self::receiver_not_set)?;
        let res = receiver
            .into_inner()
            .detach()
            .await
            .map_err(|e| AmqpError::from(e.1));
        match res {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                AmqpErrorKind::LinkClosedByRemote(_)
                | AmqpErrorKind::SessionClosedByRemote(_)
                | AmqpErrorKind::ConnectionClosedByRemote(_) => {
                    info!("Error detaching receiver: {:?} - ignored", e);
                    Ok(())
                }
                _ => {
                    warn!("Error detaching receiver: {:?}", e);
                    Err(e)
                }
            },
        }
    }

    async fn set_credit_mode(&self, credit_mode: ReceiverCreditMode) -> Result<()> {
        let receiver = self.receiver.get().ok_or_else(Self::receiver_not_set)?;
        receiver.lock().await.set_credit_mode(credit_mode.into());
        Ok(())
    }

    async fn credit_mode(&self) -> Result<ReceiverCreditMode> {
        let receiver = self.receiver.get().ok_or_else(Self::receiver_not_set)?;
        Ok(receiver.lock().await.credit_mode().into())
    }

    async fn receive_delivery(&self) -> Result<AmqpDelivery> {
        let mut receiver = self
            .receiver
            .get()
            .ok_or_else(Self::receiver_not_set)?
            .lock()
            .await;

        let delivery: fe2o3_amqp::link::delivery::Delivery<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        > = receiver.recv().await.map_err(AmqpError::from)?;
        trace!("Received delivery: {:?}", delivery);
        Ok(delivery.into())
    }

    async fn accept_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        let receiver = self
            .receiver
            .get()
            .ok_or_else(Self::receiver_not_set)?
            .lock()
            .await;

        trace!("Accepting delivery.");
        receiver
            .accept(&delivery.0.delivery)
            .await
            .map_err(AmqpError::from)?;
        trace!("Accepted delivery.");

        Ok(())
    }

    async fn reject_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        let receiver = self
            .receiver
            .get()
            .ok_or_else(Self::receiver_not_set)?
            .lock()
            .await;

        trace!("Rejecting delivery.");
        receiver
            .reject(&delivery.0.delivery, None)
            .await
            .map_err(AmqpError::from)?;
        trace!("Rejected delivery.");

        Ok(())
    }

    async fn release_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        let receiver = self
            .receiver
            .get()
            .ok_or_else(Self::receiver_not_set)?
            .lock()
            .await;

        trace!("Releasing delivery.");
        receiver
            .release(&delivery.0.delivery)
            .await
            .map_err(AmqpError::from)?;
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

    fn receiver_already_attached() -> AmqpError {
        AmqpError::with_message("AMQP Receiver is already attached")
    }

    fn could_not_set_message_receiver() -> AmqpError {
        AmqpError::with_message("Could not set message receiver")
    }

    fn receiver_not_set() -> AmqpError {
        AmqpError::with_message("AMQP Receiver is not set")
    }
}

impl From<Fe2o3ReceiverAttachError> for AmqpError {
    fn from(e: Fe2o3ReceiverAttachError) -> Self {
        AmqpError::from(e.0)
    }
}

impl From<fe2o3_amqp::link::ReceiverAttachError> for AmqpError {
    fn from(e: fe2o3_amqp::link::ReceiverAttachError) -> Self {
        match e {
            fe2o3_amqp::link::ReceiverAttachError::RemoteClosedWithError(e) => {
                AmqpErrorKind::AmqpDescribedError(e.into()).into()
            }
            fe2o3_amqp::link::ReceiverAttachError::IllegalSessionState
            | fe2o3_amqp::link::ReceiverAttachError::IllegalState => {
                AmqpErrorKind::ConnectionDropped(Box::new(e)).into()
            }
            _ => AmqpErrorKind::TransportImplementationError(Box::new(e)).into(),
        }
    }
}

impl From<fe2o3_amqp::link::RecvError> for AmqpError {
    fn from(e: fe2o3_amqp::link::RecvError) -> Self {
        match e {
            fe2o3_amqp::link::RecvError::LinkStateError(_) => {
                AmqpErrorKind::LinkStateError(Box::new(e)).into()
            }
            fe2o3_amqp::link::RecvError::TransferLimitExceeded => {
                AmqpErrorKind::TransferLimitExceeded(Box::new(e)).into()
            }
            // cspell: ignore Imeplemented
            fe2o3_amqp::link::RecvError::DeliveryIdIsNone
            | fe2o3_amqp::link::RecvError::DeliveryTagIsNone
            | fe2o3_amqp::link::RecvError::MessageDecode(_)
            | fe2o3_amqp::link::RecvError::IllegalRcvSettleModeInTransfer
            | fe2o3_amqp::link::RecvError::InconsistentFieldInMultiFrameDelivery
            | fe2o3_amqp::link::RecvError::TransactionalAcquisitionIsNotImeplemented => {
                AmqpErrorKind::TransportImplementationError(Box::new(e)).into()
            }
        }
    }
}
