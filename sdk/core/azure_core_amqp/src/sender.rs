// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::error::AmqpDescribedError;

use super::messaging::{AmqpMessage, AmqpSource, AmqpTarget};
use super::session::AmqpSession;
use super::value::{AmqpOrderedMap, AmqpSymbol, AmqpValue};
use super::{ReceiverSettleMode, SenderSettleMode};
use async_trait::async_trait;
use azure_core::error::Result;

#[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
type SenderImplementation = super::fe2o3::sender::Fe2o3AmqpSender;

#[cfg(any(not(feature = "fe2o3_amqp"), target_arch = "wasm32"))]
type SenderImplementation = super::noop::NoopAmqpSender;

#[derive(Debug, Default, Clone)]
pub struct AmqpSenderOptions {
    pub sender_settle_mode: Option<SenderSettleMode>,
    pub receiver_settle_mode: Option<ReceiverSettleMode>,
    pub source: Option<AmqpSource>,
    pub offered_capabilities: Option<Vec<AmqpSymbol>>,
    pub desired_capabilities: Option<Vec<AmqpSymbol>>,
    pub properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    pub initial_delivery_count: Option<u32>,
    pub max_message_size: Option<u64>,
}
impl AmqpSenderOptions {}

#[async_trait]
pub trait AmqpSenderApis {
    async fn attach(
        &self,
        session: &AmqpSession,
        name: String,
        target: impl Into<AmqpTarget> + Send,
        options: Option<AmqpSenderOptions>,
    ) -> Result<()>;
    async fn detach(self) -> Result<()>;
    async fn max_message_size(&self) -> Result<Option<u64>>;
    async fn send(
        &self,
        message: impl Into<AmqpMessage> + std::fmt::Debug + Send,
        options: Option<AmqpSendOptions>,
    ) -> Result<AmqpSendOutcome>;
}

/// Possible outcomes from a Send operation.
pub enum AmqpSendOutcome {
    /// The message was accepted by the receiver.
    ///
    /// At the source the accepted state means that the message has been retired
    /// from the node, and transfer of payload data will not be able to be resumed
    ///  if the link becomes suspended. A delivery can become accepted at the source
    ///  even before all transfer frames have been sent, this does not imply that
    /// the remaining transfers for the delivery will not be sent - only the
    /// aborted flag on the transfer performative can be used to indicate a
    /// premature termination of the transfer.
    ///
    /// At the target, the accepted outcome is used to indicate that an incoming
    /// message has been successfully processed, and that the receiver of the
    /// message is expecting the sender to transition the delivery to the accepted
    /// state at the source.
    ///
    /// The accepted outcome does not increment the delivery-count in the header of the accepted message.
    Accepted,
    /// The message was rejected by the receiver.
    ///
    /// At the target, the rejected outcome is used to indicate that an incoming message is invalid and therefore unprocessable. The rejected outcome when applied to a message will cause the delivery-count to be incremented in the header of the rejected message.
    /// At the source, the rejected outcome means that the target has informed the source that the message was rejected, and the source has taken the necessary action. The delivery SHOULD NOT ever spontaneously attain the rejected state at the source.
    Rejected(Option<AmqpDescribedError>),

    /// The message was released by the receiver.
    ///
    /// At the source the released outcome means that the message is no longer acquired
    ///  by the receiver, and has been made available for (re-)delivery to the same or
    /// other targets receiving from the node. The message is unchanged at the node (i.e., the
    /// delivery-count of the header of the released message MUST NOT be incremented). As released
    /// is a terminal outcome, transfer of payload data will not be able to be resumed if the
    /// link becomes suspended. A delivery can become released at the source even before all
    /// transfer frames have been sent. This does not imply that the remaining transfers for
    /// the delivery will not be sent. The source MAY spontaneously attain the released outcome
    /// for a message (for example the source might implement some sort of time-bound
    /// acquisition lock, after which the acquisition of a message at a node is revoked
    /// to allow for delivery to an alternative consumer).
    ///
    /// At the target, the released outcome is used to indicate that a given transfer was
    /// not and will not be acted upon.
    Released,
    /// The message was modified by the receiver.
    ///
    /// At the source the modified outcome means that the message is no longer acquired
    /// by the receiver, and has been made available for (re-)delivery to the same or
    /// other targets receiving from the node. The message has been changed at the node
    /// in the ways indicated by the fields of the outcome. As modified is a terminal
    /// outcome, transfer of payload data will not be able to be resumed if the link
    /// becomes suspended. A delivery can become modified at the source even before all
    /// transfer frames have been sent. This does not imply that the remaining
    /// transfers for the delivery will not be sent. The source MAY spontaneously
    /// attain the modified outcome for a message (for example the source might
    /// implement some sort of time-bound acquisition lock, after which the
    /// acquisition of a message at a node is revoked to allow for delivery
    /// to an alternative consumer with the message modified in some way to
    /// denote the previous failed, e.g., with delivery-failed set to true).
    ///
    /// At the target, the modified outcome is used to indicate that a given transfer was
    /// not and will not be acted upon, and that the message SHOULD be modified in the
    /// specified ways at the node.
    Modified(SendModification),
}

/// If the message was modified in transit, this struct contains the details of the modification.
#[derive(Debug, Default)]
pub struct SendModification {
    /// The message was not delivered to the receiver.
    pub delivery_failed: Option<bool>,
    /// The message was not delivered to the receiver because it was undeliverable at the receiver.
    pub undeliverable_here: Option<bool>,
    /// The message was not delivered to the receiver because it was not accepted by the receiver.
    pub message_annotations: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
}

pub struct AmqpSender {
    implementation: SenderImplementation,
}

#[async_trait]
impl AmqpSenderApis for AmqpSender {
    async fn attach(
        &self,
        session: &AmqpSession,
        name: String,
        target: impl Into<AmqpTarget> + Send,
        options: Option<AmqpSenderOptions>,
    ) -> Result<()> {
        self.implementation
            .attach(session, name, target, options)
            .await
    }
    async fn detach(self) -> Result<()> {
        self.implementation.detach().await
    }

    async fn max_message_size(&self) -> Result<Option<u64>> {
        self.implementation.max_message_size().await
    }
    async fn send(
        &self,
        message: impl Into<AmqpMessage> + std::fmt::Debug + Send,
        options: Option<AmqpSendOptions>,
    ) -> Result<AmqpSendOutcome> {
        self.implementation.send(message, options).await
    }
}

impl AmqpSender {
    /// Construct a new AMQP message sender.
    pub fn new() -> Self {
        Self {
            implementation: SenderImplementation::new(),
        }
    }
}

impl Default for AmqpSender {
    fn default() -> Self {
        Self::new()
    }
}

/// Options for sending an AMQP message.
#[derive(Debug, Default, Clone)]
pub struct AmqpSendOptions {
    /// The message format.
    pub message_format: Option<u32>,

    /// The message priority.
    pub settled: Option<bool>,
}

impl AmqpSendOptions {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amqp_sender_options_builder() {
        let mut properties: AmqpOrderedMap<AmqpSymbol, AmqpValue> = AmqpOrderedMap::new();
        properties.insert(AmqpSymbol::from("key"), AmqpValue::from("value"));

        let sender_options = AmqpSenderOptions {
            sender_settle_mode: Some(SenderSettleMode::Mixed),
            receiver_settle_mode: Some(ReceiverSettleMode::First),
            source: Some(
                AmqpSource::builder()
                    .with_address("address".to_string())
                    .build(),
            ),
            offered_capabilities: Some(vec!["capability".into()]),
            desired_capabilities: Some(vec!["capability".into()]),
            properties: Some(properties),
            initial_delivery_count: Some(27),
            max_message_size: Some(1024),
        };

        assert_eq!(
            sender_options.sender_settle_mode,
            Some(SenderSettleMode::Mixed)
        );
        assert_eq!(
            sender_options.receiver_settle_mode,
            Some(ReceiverSettleMode::First)
        );
        assert_eq!(
            sender_options.offered_capabilities,
            Some(vec!["capability".into()])
        );
        assert_eq!(
            sender_options.desired_capabilities,
            Some(vec!["capability".into()])
        );
        assert!(sender_options.properties.is_some());
        let properties = sender_options.properties.clone().unwrap();
        assert!(properties.contains_key("key"));
        assert_eq!(
            *properties.get("key").unwrap(),
            AmqpValue::String("value".to_string())
        );

        assert_eq!(sender_options.initial_delivery_count, Some(27));
        assert_eq!(sender_options.max_message_size, Some(1024));
    }
}
