//cspell: words amqp

use super::messaging::{AmqpMessage, AmqpSource, AmqpTarget};
use super::session::AmqpSession;
use super::value::{AmqpOrderedMap, AmqpSymbol, AmqpValue};
use crate::amqp_client::{ReceiverSettleMode, SenderSettleMode};
use azure_core::error::Result;

#[derive(Debug)]
pub(crate) struct AmqpSenderOptions {
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
pub(crate) trait AmqpSenderTrait {
    async fn attach(
        &self,
        session: &AmqpSession,
        name: impl Into<String>,
        target: impl Into<AmqpTarget>,
        options: Option<AmqpSenderOptions>,
    ) -> Result<()> {
        unimplemented!()
    }
    async fn max_message_size(&self) -> Option<u64> {
        unimplemented!()
    }
    async fn send(&self, message: AmqpMessage) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub(crate) struct AmqpSenderImpl<T>(pub(crate) T);

impl<T> AmqpSenderImpl<T>
where
    T: AmqpSenderTrait,
{
    pub(crate) fn new(session: T) -> Self {
        Self(session)
    }
}

#[cfg(any(feature = "enable-fe2o3-amqp"))]
type SenderImplementation = super::fe2o3::sender::Fe2o3AmqpSender;

#[cfg(not(any(feature = "enable-fe2o3-amqp")))]
type SenderImplementation = super::noop::NoopAmqpSender;

#[derive(Debug)]
pub(crate) struct AmqpSender(pub(crate) AmqpSenderImpl<SenderImplementation>);

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
    async fn send(&self, message: AmqpMessage) -> Result<()> {
        self.0 .0.send(message).await
    }
}

impl AmqpSender {
    pub(crate) fn new() -> Self {
        Self(AmqpSenderImpl::new(SenderImplementation::new()))
    }
}

pub mod builders {
    use super::*;

    pub(crate) struct AmqpSenderOptionsBuilder {
        options: AmqpSenderOptions,
    }

    impl AmqpSenderOptionsBuilder {
        pub(super) fn new() -> Self {
            AmqpSenderOptionsBuilder {
                options: AmqpSenderOptions {
                    sender_settle_mode: None,
                    receiver_settle_mode: None,
                    source: None,
                    offered_capabilities: None,
                    desired_capabilities: None,
                    properties: None,
                    initial_delivery_count: None,
                    max_message_size: None,
                },
            }
        }
        pub fn with_sender_settle_mode(mut self, sender_settle_mode: SenderSettleMode) -> Self {
            self.options.sender_settle_mode = Some(sender_settle_mode);
            self
        }
        pub fn with_receiver_settle_mode(
            mut self,
            receiver_settle_mode: ReceiverSettleMode,
        ) -> Self {
            self.options.receiver_settle_mode = Some(receiver_settle_mode);
            self
        }
        pub fn with_source(mut self, source: impl Into<AmqpSource>) -> Self {
            self.options.source = Some(source.into());
            self
        }
        pub fn with_offered_capabilities(mut self, offered_capabilities: Vec<AmqpSymbol>) -> Self {
            self.options.offered_capabilities = Some(offered_capabilities);
            self
        }
        pub fn with_desired_capabilities(mut self, desired_capabilities: Vec<AmqpSymbol>) -> Self {
            self.options.desired_capabilities = Some(desired_capabilities);
            self
        }
        pub fn with_properties(
            mut self,
            properties: impl Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
        ) -> Self {
            let properties_map: AmqpOrderedMap<AmqpSymbol, AmqpValue> = properties
                .into()
                .into_iter()
                .map(|(k, v)| (AmqpSymbol::from(k), AmqpValue::from(v)))
                .collect();

            self.options.properties = Some(properties_map);
            self
        }
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
