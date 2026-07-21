// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::{AmqpDescribedError, AmqpError, AmqpErrorKind, Result},
    messaging::{AmqpMessage, AmqpTarget},
    sender::{
        AmqpSendOptions, AmqpSendOutcome, AmqpSenderApis, AmqpSenderOptions, SendModification,
    },
    session::AmqpSession,
    AmqpOrderedMap, AmqpSymbol, AmqpValue,
};
use std::borrow::BorrowMut;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tracing::{info, warn};

#[derive(Default)]
pub(crate) struct Fe2o3AmqpSender {
    sender: OnceLock<Mutex<fe2o3_amqp::Sender>>,
}

impl Fe2o3AmqpSender {
    fn could_not_set_message_sender() -> AmqpError {
        AmqpError::with_message("Could not set message sender")
    }
    fn could_not_get_message_sender() -> AmqpError {
        AmqpError::with_message("Could not get message sender")
    }
}

/// The fe2o3 link builder for a sender, after the name, source and target are set.
type Fe2o3SenderBuilder = fe2o3_amqp::link::builder::Builder<
    fe2o3_amqp::link::role::SenderMarker,
    fe2o3_amqp_types::messaging::Target,
    fe2o3_amqp::link::builder::WithName,
    fe2o3_amqp::link::builder::WithSource,
    fe2o3_amqp::link::builder::WithTarget,
>;

/// Makes the fe2o3 link builder for a sender.
///
/// The order of the builder calls is important. In fe2o3-amqp 0.14, the
/// `name`, `target`, `sender` and `receiver` methods change the type of the
/// builder. They rebuild it and set `properties` back to the default value.
/// Only `source` keeps the properties. So `name` and `target` must be called
/// before `properties`. If they are not, the link properties are dropped and
/// never reach the Attach frame.
fn build_sender_link(
    name: String,
    target: AmqpTarget,
    options: Option<AmqpSenderOptions>,
) -> Fe2o3SenderBuilder {
    let mut builder = fe2o3_amqp::Sender::builder().name(name).target(target);

    if let Some(options) = options {
        if let Some(sender_settle_mode) = options.sender_settle_mode {
            builder = builder.sender_settle_mode(sender_settle_mode.into());
        }
        if let Some(receiver_settle_mode) = options.receiver_settle_mode {
            builder = builder.receiver_settle_mode(receiver_settle_mode.into());
        }
        if let Some(max_message_size) = options.max_message_size {
            builder = builder.max_message_size(max_message_size);
        }
        if let Some(source) = options.source {
            builder = builder.source(source);
        }
        if let Some(offered_capabilities) = options.offered_capabilities {
            builder = builder.set_offered_capabilities(
                offered_capabilities.into_iter().map(Into::into).collect(),
            );
        }
        if let Some(desired_capabilities) = options.desired_capabilities {
            builder = builder.set_desired_capabilities(
                desired_capabilities.into_iter().map(Into::into).collect(),
            );
        }
        if let Some(properties) = options.properties {
            builder = builder.properties(properties.into());
        }
        if let Some(initial_delivery_count) = options.initial_delivery_count {
            builder = builder.initial_delivery_count(initial_delivery_count);
        }
    }
    builder
}

#[async_trait::async_trait]
impl AmqpSenderApis for Fe2o3AmqpSender {
    async fn attach(
        &self,
        session: &AmqpSession,
        name: String,
        target: impl Into<AmqpTarget> + Send,
        options: Option<AmqpSenderOptions>,
    ) -> Result<()> {
        let sender = build_sender_link(name, target.into(), options)
            .attach(session.implementation.get()?.lock().await.borrow_mut())
            .await
            .map_err(AmqpError::from)?;
        self.sender
            .set(Mutex::new(sender))
            .map_err(|_| Self::could_not_set_message_sender())?;
        Ok(())
    }

    async fn detach(mut self) -> Result<()> {
        let sender = self
            .sender
            .take()
            .ok_or_else(Self::could_not_get_message_sender)?;
        let res = sender
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
                    info!("Error detaching sender: {:?}", e);
                    Ok(())
                }
                _ => {
                    warn!("Error detaching sender: {:?}", e);
                    Err(e)
                }
            },
        }
    }

    async fn max_message_size(&self) -> Result<Option<u64>> {
        Ok(self
            .sender
            .get()
            .ok_or_else(Self::could_not_get_message_sender)?
            .lock()
            .await
            .max_message_size())
    }

    async fn send<M>(&self, message: M, options: Option<AmqpSendOptions>) -> Result<AmqpSendOutcome>
    where
        M: Into<AmqpMessage> + std::fmt::Debug + Send,
    {
        let message: AmqpMessage = message.into();
        self.send_ref(&message, options).await
    }

    async fn send_ref<M>(
        &self,
        message: M,
        options: Option<AmqpSendOptions>,
    ) -> Result<AmqpSendOutcome>
    where
        M: AsRef<AmqpMessage> + std::fmt::Debug + Send,
    {
        let message = message.as_ref();
        let message = fe2o3_amqp_types::messaging::Message::<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        >::from(message);

        let sendable = fe2o3_amqp::link::delivery::Sendable {
            message,
            message_format: options
                .as_ref()
                .and_then(|opt| opt.message_format)
                .unwrap_or(0),
            settled: options.as_ref().and_then(|opt| opt.settled),
        };

        let outcome = self
            .sender
            .get()
            .ok_or_else(Self::could_not_get_message_sender)?
            .lock()
            .await
            .borrow_mut()
            .send(sendable)
            .await
            .map_err(AmqpError::from)?;

        Ok(match outcome {
            fe2o3_amqp_types::messaging::Outcome::Accepted(_) => AmqpSendOutcome::Accepted,
            fe2o3_amqp_types::messaging::Outcome::Rejected(rejected) => {
                AmqpSendOutcome::Rejected(rejected.error.map(AmqpDescribedError::from))
            }
            fe2o3_amqp_types::messaging::Outcome::Released(_) => AmqpSendOutcome::Released,
            fe2o3_amqp_types::messaging::Outcome::Modified(ref m) => {
                AmqpSendOutcome::Modified(m.into())
            }
        })
    }
}

impl From<&fe2o3_amqp_types::messaging::Modified> for SendModification {
    fn from(m: &fe2o3_amqp_types::messaging::Modified) -> Self {
        Self {
            delivery_failed: m.delivery_failed,
            undeliverable_here: m.undeliverable_here,
            message_annotations: m.message_annotations.as_ref().map(|m| {
                m.iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect::<AmqpOrderedMap<AmqpSymbol, AmqpValue>>()
            }),
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

impl From<fe2o3_amqp::link::SendError> for AmqpError {
    fn from(e: fe2o3_amqp::link::SendError) -> Self {
        match e {
            fe2o3_amqp::link::SendError::LinkStateError(link_state_error) => {
                AmqpError::from(link_state_error)
            }
            fe2o3_amqp::link::SendError::Detached(detach_error) => detach_error.into(),
            fe2o3_amqp::link::SendError::NonTerminalDeliveryState => {
                AmqpErrorKind::NonTerminalDeliveryState.into()
            }
            fe2o3_amqp::link::SendError::IllegalDeliveryState => {
                AmqpErrorKind::IllegalDeliveryState.into()
            }
            fe2o3_amqp::link::SendError::MessageEncodeError => {
                AmqpError::from(AmqpErrorKind::TransportImplementationError(Box::new(e)))
            }
        }
    }
}

impl From<fe2o3_amqp::link::SenderAttachError> for AmqpError {
    fn from(e: fe2o3_amqp::link::SenderAttachError) -> Self {
        match e {
            fe2o3_amqp::link::SenderAttachError::RemoteClosedWithError(e) => {
                AmqpErrorKind::AmqpDescribedError(e.into()).into()
            }
            fe2o3_amqp::link::SenderAttachError::IllegalSessionState
            | fe2o3_amqp::link::SenderAttachError::IllegalState => {
                AmqpErrorKind::ConnectionDropped(Box::new(e)).into()
            }
            fe2o3_amqp::link::SenderAttachError::CoordinatorIsNotImplemented
            | fe2o3_amqp::link::SenderAttachError::DuplicatedLinkName
            | fe2o3_amqp::link::SenderAttachError::NonAttachFrameReceived
            | fe2o3_amqp::link::SenderAttachError::ExpectImmediateDetach
            | fe2o3_amqp::link::SenderAttachError::IncomingTargetIsNone
            | fe2o3_amqp::link::SenderAttachError::SndSettleModeNotSupported
            | fe2o3_amqp::link::SenderAttachError::RcvSettleModeNotSupported
            | fe2o3_amqp::link::SenderAttachError::TargetAddressIsNoneWhenDynamicIsTrue
            | fe2o3_amqp::link::SenderAttachError::SourceAddressIsSomeWhenDynamicIsTrue
            | fe2o3_amqp::link::SenderAttachError::DynamicNodePropertiesIsSomeWhenDynamicIsFalse => {
                AmqpErrorKind::TransportImplementationError(Box::new(e)).into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Makes sure the link properties survive the fe2o3 builder chain. Both
    // `name` and `target` clear the properties, so they must run before
    // `properties`.
    #[test]
    fn sender_link_keeps_properties() {
        let mut properties: AmqpOrderedMap<AmqpSymbol, AmqpValue> = AmqpOrderedMap::new();
        properties.insert("com.microsoft:epoch".into(), AmqpValue::from(3i64));

        let options = AmqpSenderOptions {
            properties: Some(properties),
            ..Default::default()
        };
        let target = AmqpTarget::builder()
            .with_address("amqps://example.servicebus.windows.net/eh".to_string())
            .build();

        let builder = build_sender_link("test-sender".into(), target, Some(options));

        assert_eq!(builder.name, "test-sender");
        let fields = builder
            .properties
            .expect("link properties must survive the builder chain");
        assert_eq!(
            fields.get(&fe2o3_amqp_types::primitives::Symbol::from(
                "com.microsoft:epoch"
            )),
            Some(&fe2o3_amqp_types::primitives::Value::Long(3))
        );
    }
}
