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

/// The fe2o3 link builder for a receiver, after the name, source and target are set.
type Fe2o3ReceiverBuilder = fe2o3_amqp::link::builder::Builder<
    fe2o3_amqp::link::role::ReceiverMarker,
    fe2o3_amqp_types::messaging::Target,
    fe2o3_amqp::link::builder::WithName,
    fe2o3_amqp::link::builder::WithSource,
    fe2o3_amqp::link::builder::WithTarget,
>;

/// Makes the fe2o3 link builder for a receiver.
///
/// The order of the builder calls is important. In fe2o3-amqp 0.14, the
/// `name`, `target`, `sender` and `receiver` methods change the type of the
/// builder. They rebuild it and set `properties` back to the default value.
/// Only `source` keeps the properties. So `name` must be called before
/// `properties`. If it is not, the link properties are dropped and never reach
/// the Attach frame. Event Hubs sends `com.microsoft:epoch` (the owner level)
/// as a link property, so the wrong order makes the owner level a silent no-op.
fn build_receiver_link(source: AmqpSource, options: AmqpReceiverOptions) -> Fe2o3ReceiverBuilder {
    let name = options.name.unwrap_or_default();
    let credit_mode = options.credit_mode.unwrap_or_default();
    let properties = options.properties.unwrap_or_default();

    fe2o3_amqp::Receiver::builder()
        .name(name)
        .source(source)
        .receiver_settle_mode(fe2o3_amqp_types::definitions::ReceiverSettleMode::First)
        .credit_mode(credit_mode.into())
        .auto_accept(options.auto_accept)
        .properties(properties.into())
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

#[async_trait::async_trait]
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
        let source = source.into();

        let receiver = build_receiver_link(source, options)
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
            // Delegate to the `LinkStateError` conversion so a remote close or
            // detach that carries an AMQP error (for example `amqp:link:stolen`)
            // becomes an `AmqpDescribedError`. Keeping the condition reachable
            // lets callers classify the failure. The sender path already does
            // this for `SendError::LinkStateError`.
            fe2o3_amqp::link::RecvError::LinkStateError(link_state_error) => {
                AmqpError::from(link_state_error)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messaging::AmqpSource;
    use crate::value::{AmqpOrderedMap, AmqpSymbol, AmqpValue};

    // Makes sure the link properties survive the fe2o3 builder chain. The
    // `name` method clears the properties, so it must run before
    // `properties`. Event Hubs sends the owner level as the
    // `com.microsoft:epoch` link property, and a lost property makes the owner
    // level a silent no-op.
    #[test]
    fn receiver_link_keeps_properties() {
        let mut properties: AmqpOrderedMap<AmqpSymbol, AmqpValue> = AmqpOrderedMap::new();
        properties.insert(
            "com.microsoft.com:receiver-name".into(),
            AmqpValue::from("test-receiver"),
        );
        properties.insert("com.microsoft:epoch".into(), AmqpValue::from(7i64));

        let options = AmqpReceiverOptions {
            name: Some("test-receiver".into()),
            properties: Some(properties),
            credit_mode: Some(ReceiverCreditMode::Auto(300)),
            auto_accept: true,
            ..Default::default()
        };
        let source = AmqpSource::builder()
            .with_address("amqps://example.servicebus.windows.net/eh/Partitions/0".to_string())
            .build();

        let builder = build_receiver_link(source, options);

        assert_eq!(builder.name, "test-receiver");
        let fields = builder
            .properties
            .expect("link properties must survive the builder chain");
        assert_eq!(
            fields.get(&fe2o3_amqp_types::primitives::Symbol::from(
                "com.microsoft:epoch"
            )),
            Some(&fe2o3_amqp_types::primitives::Value::Long(7))
        );
        assert_eq!(
            fields.get(&fe2o3_amqp_types::primitives::Symbol::from(
                "com.microsoft.com:receiver-name"
            )),
            Some(&fe2o3_amqp_types::primitives::Value::String(
                "test-receiver".into()
            ))
        );
    }

    use crate::error::AmqpErrorCondition;

    fn stolen_error() -> fe2o3_amqp_types::definitions::Error {
        fe2o3_amqp_types::definitions::Error::new(
            fe2o3_amqp_types::definitions::LinkError::Stolen,
            Some("New receiver 'x' with higher epoch of '1' is created".to_string()),
            None,
        )
    }

    // A receive that fails because the broker closed the link with
    // `amqp:link:stolen` must keep the described error. If it collapses to
    // `LinkStateError`, the condition is only reachable through a fe2o3 type
    // and no caller can classify the displacement.
    #[test]
    fn recv_link_state_remote_closed_with_error_keeps_described_error() {
        let recv_error = fe2o3_amqp::link::RecvError::LinkStateError(
            fe2o3_amqp::link::LinkStateError::RemoteClosedWithError(stolen_error()),
        );
        let amqp_error = AmqpError::from(recv_error);
        match amqp_error.kind() {
            AmqpErrorKind::AmqpDescribedError(described) => {
                assert_eq!(described.condition, AmqpErrorCondition::LinkStolen);
            }
            _ => panic!("expected AmqpDescribedError, got {amqp_error:?}"),
        }
    }

    #[test]
    fn recv_link_state_remote_detached_with_error_keeps_described_error() {
        let recv_error = fe2o3_amqp::link::RecvError::LinkStateError(
            fe2o3_amqp::link::LinkStateError::RemoteDetachedWithError(stolen_error()),
        );
        let amqp_error = AmqpError::from(recv_error);
        match amqp_error.kind() {
            AmqpErrorKind::AmqpDescribedError(described) => {
                assert_eq!(described.condition, AmqpErrorCondition::LinkStolen);
            }
            _ => panic!("expected AmqpDescribedError, got {amqp_error:?}"),
        }
    }

    // A plain remote close with no error keeps its transport-level kind so the
    // retry layer still classifies it as a link reattach.
    #[test]
    fn recv_link_state_remote_closed_maps_to_link_closed_by_remote() {
        let recv_error = fe2o3_amqp::link::RecvError::LinkStateError(
            fe2o3_amqp::link::LinkStateError::RemoteClosed,
        );
        let amqp_error = AmqpError::from(recv_error);
        assert!(matches!(
            amqp_error.kind(),
            AmqpErrorKind::LinkClosedByRemote(_)
        ));
    }

    // A plain remote detach with no error keeps its own kind, separate from a
    // remote close. The two arms are distinct, so both need a test.
    #[test]
    fn recv_link_state_remote_detached_maps_to_link_detached_by_remote() {
        let recv_error = fe2o3_amqp::link::RecvError::LinkStateError(
            fe2o3_amqp::link::LinkStateError::RemoteDetached,
        );
        let amqp_error = AmqpError::from(recv_error);
        assert!(matches!(
            amqp_error.kind(),
            AmqpErrorKind::LinkDetachedByRemote(_)
        ));
    }

    // A link-state failure that is neither a close nor a detach still reports
    // `LinkStateError`, and it now carries the `LinkStateError` itself. It
    // carried the enclosing `RecvError` before the delegation.
    #[test]
    fn recv_link_state_other_carries_the_link_state_error() {
        let recv_error = fe2o3_amqp::link::RecvError::LinkStateError(
            fe2o3_amqp::link::LinkStateError::IllegalSessionState,
        );
        let amqp_error = AmqpError::from(recv_error);
        let AmqpErrorKind::LinkStateError(source) = amqp_error.kind() else {
            panic!("a link-state failure must report LinkStateError");
        };
        assert!(
            source
                .downcast_ref::<fe2o3_amqp::link::LinkStateError>()
                .is_some(),
            "the reported error must carry the LinkStateError, not the enclosing RecvError"
        );
    }
}
