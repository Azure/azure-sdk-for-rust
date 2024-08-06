// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
//cspell: words amqp

use super::messaging::{AmqpMessage, AmqpSource, AmqpTarget};
use super::session::AmqpSession;
use super::value::{AmqpOrderedMap, AmqpSymbol, AmqpValue};
use super::{ReceiverSettleMode, SenderSettleMode};
use azure_core::error::Result;

#[derive(Debug, Default)]
pub struct AmqpSenderOptions {
    pub(super) sender_settle_mode: Option<SenderSettleMode>,
    pub(super) receiver_settle_mode: Option<ReceiverSettleMode>,
    pub(super) source: Option<AmqpSource>,
    pub(super) offered_capabilities: Option<Vec<AmqpSymbol>>,
    pub(super) desired_capabilities: Option<Vec<AmqpSymbol>>,
    pub(super) properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    pub(super) initial_delivery_count: Option<u32>,
    pub(super) max_message_size: Option<u64>,
}
impl AmqpSenderOptions {
    pub fn builder() -> builders::AmqpSenderOptionsBuilder {
        builders::AmqpSenderOptionsBuilder::new()
    }
}

#[allow(unused_variables)]
pub trait AmqpSenderTrait {
    fn attach(
        &self,
        session: &AmqpSession,
        name: impl Into<String>,
        target: impl Into<AmqpTarget>,
        options: Option<AmqpSenderOptions>,
    ) -> impl std::future::Future<Output = Result<()>>;
    fn max_message_size(&self) -> impl std::future::Future<Output = Option<u64>>;
    fn send(
        &self,
        message: AmqpMessage,
        options: Option<AmqpSendOptions>,
    ) -> impl std::future::Future<Output = Result<()>>;
}

#[derive(Debug, Default)]
struct AmqpSenderImpl<T>(T);

impl<T> AmqpSenderImpl<T>
where
    T: AmqpSenderTrait,
{
    pub fn new(session: T) -> Self {
        Self(session)
    }
}

#[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
type SenderImplementation = super::fe2o3::sender::Fe2o3AmqpSender;

#[cfg(any(not(feature = "fe2o3-amqp"), target_arch = "wasm32"))]
type SenderImplementation = super::noop::NoopAmqpSender;

#[derive(Debug, Default)]
pub struct AmqpSender(AmqpSenderImpl<SenderImplementation>);

impl AmqpSenderTrait for AmqpSender {
    async fn attach(
        &self,
        session: &AmqpSession,
        name: impl Into<String>,
        target: impl Into<AmqpTarget>,
        options: Option<AmqpSenderOptions>,
    ) -> Result<()> {
        self.0 .0.attach(session, name, target, options).await
    }
    async fn max_message_size(&self) -> Option<u64> {
        self.0 .0.max_message_size().await
    }
    async fn send(&self, message: AmqpMessage, options: Option<AmqpSendOptions>) -> Result<()> {
        self.0 .0.send(message, options).await
    }
}

impl AmqpSender {
    pub fn new() -> Self {
        Self(AmqpSenderImpl::new(SenderImplementation::new()))
    }
}

/// Options for sending an AMQP message.
#[derive(Debug, Default)]
pub struct AmqpSendOptions {
    /// The message format.
    pub(crate) message_format: Option<u32>,

    /// The message priority.
    pub(crate) settled: Option<bool>,
}

impl AmqpSendOptions {
    pub fn builder() -> builders::AmqpSendOptionsBuilder {
        builders::AmqpSendOptionsBuilder::new()
    }
}

pub mod builders {
    use super::*;

    pub struct AmqpSendOptionsBuilder {
        options: AmqpSendOptions,
    }

    impl AmqpSendOptionsBuilder {
        pub(super) fn new() -> Self {
            AmqpSendOptionsBuilder {
                options: Default::default(),
            }
        }
        #[allow(dead_code)]
        pub fn with_message_format(mut self, message_format: u32) -> Self {
            self.options.message_format = Some(message_format);
            self
        }
        #[allow(dead_code)]
        pub fn with_settled(mut self, settled: bool) -> Self {
            self.options.settled = Some(settled);
            self
        }
        pub fn build(self) -> Option<AmqpSendOptions> {
            Some(self.options)
        }
    }

    pub struct AmqpSenderOptionsBuilder {
        options: AmqpSenderOptions,
    }

    impl AmqpSenderOptionsBuilder {
        pub(super) fn new() -> Self {
            AmqpSenderOptionsBuilder {
                options: Default::default(),
            }
        }
        #[allow(dead_code)]
        pub fn with_sender_settle_mode(mut self, sender_settle_mode: SenderSettleMode) -> Self {
            self.options.sender_settle_mode = Some(sender_settle_mode);
            self
        }
        #[allow(dead_code)]
        pub fn with_receiver_settle_mode(
            mut self,
            receiver_settle_mode: ReceiverSettleMode,
        ) -> Self {
            self.options.receiver_settle_mode = Some(receiver_settle_mode);
            self
        }
        #[allow(dead_code)]
        pub fn with_source(mut self, source: impl Into<AmqpSource>) -> Self {
            self.options.source = Some(source.into());
            self
        }
        #[allow(dead_code)]
        pub fn with_offered_capabilities(mut self, offered_capabilities: Vec<AmqpSymbol>) -> Self {
            self.options.offered_capabilities = Some(offered_capabilities);
            self
        }
        #[allow(dead_code)]
        pub fn with_desired_capabilities(mut self, desired_capabilities: Vec<AmqpSymbol>) -> Self {
            self.options.desired_capabilities = Some(desired_capabilities);
            self
        }
        #[allow(dead_code)]
        pub fn with_properties(
            mut self,
            properties: impl Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
        ) -> Self {
            let properties_map: AmqpOrderedMap<AmqpSymbol, AmqpValue> =
                properties.into().iter().collect();

            self.options.properties = Some(properties_map);
            self
        }
        #[allow(dead_code)]
        pub fn with_initial_delivery_count(mut self, initial_delivery_count: u32) -> Self {
            self.options.initial_delivery_count = Some(initial_delivery_count);
            self
        }
        pub fn with_max_message_size(mut self, max_message_size: u64) -> Self {
            self.options.max_message_size = Some(max_message_size);
            self
        }

        pub fn build(self) -> AmqpSenderOptions {
            self.options
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amqp_sender_options_builder() {
        let mut properties: AmqpOrderedMap<AmqpSymbol, AmqpValue> = AmqpOrderedMap::new();
        properties.insert(AmqpSymbol::from("key"), AmqpValue::from("value"));

        let sender_options = AmqpSenderOptions::builder()
            .with_sender_settle_mode(SenderSettleMode::Unsettled)
            .with_sender_settle_mode(SenderSettleMode::Settled)
            .with_sender_settle_mode(SenderSettleMode::Mixed)
            .with_receiver_settle_mode(ReceiverSettleMode::Second)
            .with_receiver_settle_mode(ReceiverSettleMode::First)
            .with_source(AmqpSource::builder().with_address("address").build())
            .with_offered_capabilities(vec!["capability".into()])
            .with_desired_capabilities(vec!["capability".into()])
            .with_properties(properties)
            .with_initial_delivery_count(27)
            .with_max_message_size(1024)
            .build();

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
