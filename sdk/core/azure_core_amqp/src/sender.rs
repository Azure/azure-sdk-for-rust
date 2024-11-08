// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
//cspell: words amqp

use super::messaging::{AmqpMessage, AmqpSource, AmqpTarget};
use super::session::AmqpSession;
use super::value::{AmqpOrderedMap, AmqpSymbol, AmqpValue};
use super::{ReceiverSettleMode, SenderSettleMode};
use azure_core::error::Result;

#[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
type SenderImplementation = super::fe2o3::sender::Fe2o3AmqpSender;

#[cfg(any(not(feature = "fe2o3-amqp"), target_arch = "wasm32"))]
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

#[allow(unused_variables)]
pub trait AmqpSenderApis {
    fn attach(
        &self,
        session: &AmqpSession,
        name: String,
        target: impl Into<AmqpTarget>,
        options: Option<AmqpSenderOptions>,
    ) -> impl std::future::Future<Output = Result<()>>;
    fn detach(self) -> impl std::future::Future<Output = Result<()>>;
    fn max_message_size(&self) -> Result<Option<u64>>;
    fn send(
        &self,
        message: impl Into<AmqpMessage> + std::fmt::Debug,
        options: Option<AmqpSendOptions>,
    ) -> impl std::future::Future<Output = Result<()>>;
}

#[derive(Default)]
pub struct AmqpSender {
    implementation: SenderImplementation,
}

impl AmqpSenderApis for AmqpSender {
    async fn attach(
        &self,
        session: &AmqpSession,
        name: String,
        target: impl Into<AmqpTarget>,
        options: Option<AmqpSenderOptions>,
    ) -> Result<()> {
        self.implementation
            .attach(session, name, target, options)
            .await
    }
    async fn detach(self) -> Result<()> {
        self.implementation.detach().await
    }

    fn max_message_size(&self) -> Result<Option<u64>> {
        self.implementation.max_message_size()
    }
    async fn send(
        &self,
        message: impl Into<AmqpMessage> + std::fmt::Debug,
        options: Option<AmqpSendOptions>,
    ) -> Result<()> {
        self.implementation.send(message, options).await
    }
}

impl AmqpSender {
    pub fn new() -> Self {
        Self {
            implementation: SenderImplementation::new(),
        }
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
