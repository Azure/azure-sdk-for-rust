// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::{AmqpDetachError, AmqpErrorKind, AmqpLinkStateError, AmqpReceiverError},
    messaging::{AmqpDelivery, AmqpSource},
    receiver::{AmqpReceiverApis, AmqpReceiverOptions, ReceiverCreditMode},
    session::AmqpSession,
    AmqpError,
};
use azure_core::error::Result;
use std::borrow::BorrowMut;
use std::sync::OnceLock;
use tokio::sync::Mutex;
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
            return Err(AmqpError::new(AmqpErrorKind::ReceiverError(
                AmqpReceiverError::ReceiverAlreadyAttached,
            ))
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
            .map_err(AmqpReceiverError::from)?;
        self.receiver
            .set(Mutex::new(receiver))
            .map_err(|_| AmqpReceiverError::CouldNotSetMessageReceiver)?;
        Ok(())
    }

    async fn detach(mut self) -> Result<()> {
        let receiver = self
            .receiver
            .take()
            .ok_or(AmqpReceiverError::ReceiverNotSet)?;
        let res = receiver
            .into_inner()
            .detach()
            .await
            .map_err(|e| AmqpReceiverError::DetachError(e.1.into()));
        match res {
            Ok(_) => Ok(()),
            Err(e) => match e {
                AmqpReceiverError::DetachError(AmqpDetachError::ClosedByRemote) => {
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

    async fn set_credit_mode(&self, credit_mode: ReceiverCreditMode) -> Result<()> {
        let receiver = self
            .receiver
            .get()
            .ok_or(AmqpReceiverError::ReceiverNotSet)?;
        receiver.lock().await.set_credit_mode(credit_mode.into());
        Ok(())
    }

    async fn credit_mode(&self) -> Result<ReceiverCreditMode> {
        let receiver = self
            .receiver
            .get()
            .ok_or(AmqpReceiverError::ReceiverNotSet)?;
        Ok(receiver.lock().await.credit_mode().into())
    }

    async fn receive_delivery(&self) -> Result<AmqpDelivery> {
        let mut receiver = self
            .receiver
            .get()
            .ok_or(AmqpReceiverError::ReceiverNotSet)?
            .lock()
            .await;

        let delivery: fe2o3_amqp::link::delivery::Delivery<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        > = receiver.recv().await.map_err(AmqpReceiverError::from)?;

        trace!("Received delivery: {:?}", delivery);

        Ok(delivery.into())
    }

    async fn accept_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        let receiver = self
            .receiver
            .get()
            .ok_or(AmqpReceiverError::ReceiverNotSet)?
            .lock()
            .await;

        trace!("Accepting delivery.");
        receiver
            .accept(&delivery.0.delivery)
            .await
            .map_err(AmqpReceiverError::from)?;
        trace!("Accepted delivery.");

        Ok(())
    }

    async fn reject_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        let receiver = self
            .receiver
            .get()
            .ok_or(AmqpReceiverError::ReceiverNotSet)?
            .lock()
            .await;

        trace!("Rejecting delivery.");
        receiver
            .reject(&delivery.0.delivery, None)
            .await
            .map_err(AmqpReceiverError::from)?;
        trace!("Rejected delivery.");

        Ok(())
    }

    async fn release_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        let receiver = self
            .receiver
            .get()
            .ok_or(AmqpReceiverError::ReceiverNotSet)?
            .lock()
            .await;

        trace!("Releasing delivery.");
        receiver
            .release(&delivery.0.delivery)
            .await
            .map_err(AmqpReceiverError::from)?;
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

impl From<fe2o3_amqp::link::ReceiverAttachError> for AmqpReceiverError {
    fn from(e: fe2o3_amqp::link::ReceiverAttachError) -> Self {
        match e {
            fe2o3_amqp::link::ReceiverAttachError::IllegalSessionState => AmqpReceiverError::IllegalSessionState,
            fe2o3_amqp::link::ReceiverAttachError::DuplicatedLinkName => AmqpReceiverError::DuplicatedLinkName,
            fe2o3_amqp::link::ReceiverAttachError::IllegalState => AmqpReceiverError::IllegalState,
            fe2o3_amqp::link::ReceiverAttachError::NonAttachFrameReceived => AmqpReceiverError::NonAttachFrameReceived,
            fe2o3_amqp::link::ReceiverAttachError::ExpectImmediateDetach => AmqpReceiverError::ExpectImmediateDetach,
            fe2o3_amqp::link::ReceiverAttachError::IncomingSourceIsNone => AmqpReceiverError::IncomingSourceIsNone,
            fe2o3_amqp::link::ReceiverAttachError::CoordinatorIsNotImplemented => AmqpReceiverError::CoordinatorIsNotImplemented,
            fe2o3_amqp::link::ReceiverAttachError::InitialDeliveryCountIsNone => AmqpReceiverError::InitialDeliveryCountIsNone,
            fe2o3_amqp::link::ReceiverAttachError::RcvSettleModeNotSupported => AmqpReceiverError::RcvSettleModeNotSupported,
            fe2o3_amqp::link::ReceiverAttachError::TargetAddressIsSomeWhenDynamicIsTrue => AmqpReceiverError::TargetAddressIsSomeWhenDynamicIsTrue,
            fe2o3_amqp::link::ReceiverAttachError::SourceAddressIsNoneWhenDynamicIsTrue => AmqpReceiverError::SourceAddressIsNoneWhenDynamicIsTrue,
            fe2o3_amqp::link::ReceiverAttachError::DynamicNodePropertiesIsSomeWhenDynamicIsFalse => AmqpReceiverError::DynamicNodePropertiesIsSomeWhenDynamicIsFalse,
            fe2o3_amqp::link::ReceiverAttachError::RemoteClosedWithError(error) => AmqpReceiverError::RemoteClosedWithError(error.into()),
            fe2o3_amqp::link::ReceiverAttachError::DesiredFilterNotSupported(_desired_filter_not_supported) =>
                AmqpReceiverError::DesiredFilterNotSupported,
        }
    }
}

impl From<fe2o3_amqp::link::IllegalLinkStateError> for AmqpReceiverError {
    fn from(e: fe2o3_amqp::link::IllegalLinkStateError) -> Self {
        match e {
            fe2o3_amqp::link::IllegalLinkStateError::IllegalState => {
                AmqpReceiverError::IllegalState
            }
            fe2o3_amqp::link::IllegalLinkStateError::IllegalSessionState => {
                AmqpReceiverError::IllegalSessionState
            }
        }
    }
}

impl From<fe2o3_amqp::link::RecvError> for AmqpReceiverError {
    fn from(e: fe2o3_amqp::link::RecvError) -> Self {
        match e {
            fe2o3_amqp::link::RecvError::LinkStateError(link_state_error) => {
                link_state_error.into()
            }
            fe2o3_amqp::link::RecvError::TransferLimitExceeded => {
                AmqpReceiverError::TransferLimitExceeded
            }
            fe2o3_amqp::link::RecvError::DeliveryIdIsNone => AmqpReceiverError::DeliveryIdIsNone,
            fe2o3_amqp::link::RecvError::DeliveryTagIsNone => AmqpReceiverError::DeliveryTagIsNone,
            fe2o3_amqp::link::RecvError::MessageDecode(_message_decode_error) => {
                AmqpReceiverError::MessageDecode
            }
            fe2o3_amqp::link::RecvError::IllegalRcvSettleModeInTransfer => {
                AmqpReceiverError::IllegalRcvSettleModeInTransfer
            }
            fe2o3_amqp::link::RecvError::InconsistentFieldInMultiFrameDelivery => {
                AmqpReceiverError::InconsistentFieldInMultiFrameDelivery
            }
            // cspell: ignore: Imeplemented
            fe2o3_amqp::link::RecvError::TransactionalAcquisitionIsNotImeplemented => {
                AmqpReceiverError::TransactionalAcquisitionIsNotImplemented
            }
        }
    }
}

impl From<fe2o3_amqp::link::LinkStateError> for AmqpReceiverError {
    fn from(e: fe2o3_amqp::link::LinkStateError) -> Self {
        match e {
            fe2o3_amqp::link::LinkStateError::IllegalState => {
                AmqpReceiverError::LinkStateError(AmqpLinkStateError::IllegalState)
            }
            fe2o3_amqp::link::LinkStateError::IllegalSessionState => {
                AmqpReceiverError::LinkStateError(AmqpLinkStateError::IllegalSessionState)
            }
            fe2o3_amqp::link::LinkStateError::ExpectImmediateDetach => {
                AmqpReceiverError::LinkStateError(AmqpLinkStateError::ExpectImmediateDetach)
            }
            fe2o3_amqp::link::LinkStateError::RemoteDetachedWithError(error) => {
                AmqpReceiverError::DetachError(AmqpDetachError::RemoteDetachedWithError(
                    error.into(),
                ))
            }

            fe2o3_amqp::link::LinkStateError::RemoteClosedWithError(error) => {
                AmqpReceiverError::RemoteClosedWithError(error.into())
            }

            fe2o3_amqp::link::LinkStateError::RemoteDetached => {
                AmqpReceiverError::DetachError(AmqpDetachError::DetachedByRemote)
            }
            fe2o3_amqp::link::LinkStateError::RemoteClosed => {
                AmqpReceiverError::DetachError(AmqpDetachError::ClosedByRemote)
            }
        }
    }
}
