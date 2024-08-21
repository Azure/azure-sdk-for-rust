// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
//cspell: words amqp

use super::messaging::{AmqpMessage, AmqpSource, AmqpTarget};
use super::session::AmqpSession;
use super::value::{AmqpOrderedMap, AmqpSymbol, AmqpValue};
use super::ReceiverSettleMode;
use azure_core::error::Result;

#[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
type ReceiverImplementation = super::fe2o3::receiver::Fe2o3AmqpReceiver;

#[cfg(any(not(feature = "fe2o3-amqp"), target_arch = "wasm32"))]
type ReceiverImplementation = super::noop::NoopAmqpReceiver;

/// Represents the mode of issuing credit to the sender in an AMQP receiver.
///
/// Credit can be issued automatically or manually, controlling the flow of messages from the sender to the receiver.
///
/// Variants:
/// - `Auto(u32)`: Automatically issue the specified number of credits to the sender. This allows the receiver to
///   control the flow of messages by specifying how many messages it is ready to receive.
/// - `Manual`: The receiver manually controls when to issue credit to the sender. This mode gives the receiver
///   complete control over the flow of messages, allowing it to request messages from the sender as needed.
#[derive(Debug, PartialEq, Clone)]
pub enum ReceiverCreditMode {
    Auto(u32),
    Manual,
}

impl Default for ReceiverCreditMode {
    fn default() -> Self {
        ReceiverCreditMode::Auto(100)
    }
}

#[derive(Debug, Default)]
pub struct AmqpReceiverOptions {
    receiver_settle_mode: Option<ReceiverSettleMode>,
    target: Option<AmqpTarget>,
    name: Option<String>,
    credit_mode: Option<ReceiverCreditMode>,
    auto_accept: bool,
    properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
}

impl AmqpReceiverOptions {
    pub fn builder() -> builders::AmqpReceiverOptionsBuilder {
        builders::AmqpReceiverOptionsBuilder::new()
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn credit_mode(&self) -> &Option<ReceiverCreditMode> {
        &self.credit_mode
    }

    pub fn auto_accept(&self) -> bool {
        self.auto_accept
    }

    pub fn properties(&self) -> &Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>> {
        &self.properties
    }

    pub fn receiver_settle_mode(&self) -> &Option<ReceiverSettleMode> {
        &self.receiver_settle_mode
    }

    pub fn target(&self) -> &Option<AmqpTarget> {
        &self.target
    }
}

#[allow(unused_variables)]
pub trait AmqpReceiverApis {
    fn attach(
        &self,
        session: &AmqpSession,
        source: impl Into<AmqpSource>,
        options: Option<AmqpReceiverOptions>,
    ) -> impl std::future::Future<Output = Result<()>>;
    fn max_message_size(&self) -> impl std::future::Future<Output = Option<u64>>;
    fn receive(&self) -> impl std::future::Future<Output = Result<AmqpMessage>>;
}

#[derive(Debug, Default)]
pub struct AmqpReceiver {
    implementation: ReceiverImplementation,
}

impl AmqpReceiverApis for AmqpReceiver {
    async fn attach(
        &self,
        session: &AmqpSession,
        source: impl Into<AmqpSource>,
        options: Option<AmqpReceiverOptions>,
    ) -> Result<()> {
        self.implementation.attach(session, source, options).await
    }
    async fn max_message_size(&self) -> Option<u64> {
        self.implementation.max_message_size().await
    }
    async fn receive(&self) -> Result<AmqpMessage> {
        self.implementation.receive().await
    }
}

impl AmqpReceiver {
    pub fn new() -> Self {
        Self {
            implementation: ReceiverImplementation::new(),
        }
    }
}

pub mod builders {
    use super::*;

    pub struct AmqpReceiverOptionsBuilder {
        options: AmqpReceiverOptions,
    }

    impl AmqpReceiverOptionsBuilder {
        pub(super) fn new() -> Self {
            AmqpReceiverOptionsBuilder {
                options: Default::default(),
            }
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
        pub fn with_target(mut self, target: impl Into<AmqpTarget>) -> Self {
            self.options.target = Some(target.into());
            self
        }
        #[allow(dead_code)]
        pub fn with_name(mut self, name: impl Into<String>) -> Self {
            self.options.name = Some(name.into());
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
        pub fn add_property(mut self, key: impl Into<String>, value: impl Into<AmqpValue>) -> Self {
            let key = AmqpSymbol::from(key.into());
            let value = value.into();
            if let Some(properties) = self.options.properties.as_mut() {
                properties.insert(key, value);
            } else {
                let mut properties = AmqpOrderedMap::new();
                properties.insert(key, value);
                self.options.properties = Some(properties);
            }
            self
        }
        pub fn with_credit_mode(mut self, credit_mode: ReceiverCreditMode) -> Self {
            self.options.credit_mode = Some(credit_mode);
            self
        }
        pub fn with_auto_accept(mut self, auto_accept: bool) -> Self {
            self.options.auto_accept = auto_accept;
            self
        }

        pub fn build(self) -> AmqpReceiverOptions {
            self.options
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amqp_receiver_options_builder() {
        let mut properties: AmqpOrderedMap<AmqpSymbol, AmqpValue> = AmqpOrderedMap::new();
        properties.insert(AmqpSymbol::from("key"), AmqpValue::from("value"));

        let receiver_options = AmqpReceiverOptions::builder()
            .with_receiver_settle_mode(ReceiverSettleMode::Second)
            .with_receiver_settle_mode(ReceiverSettleMode::First)
            .with_target(AmqpTarget::builder().with_address("address").build())
            .with_properties(properties)
            .build();

        assert_eq!(
            receiver_options.receiver_settle_mode,
            Some(ReceiverSettleMode::First)
        );
        assert!(receiver_options.properties.is_some());
        let properties = receiver_options.properties.clone().unwrap();
        assert!(properties.contains_key("key"));
        assert_eq!(
            *properties.get("key").unwrap(),
            AmqpValue::String("value".to_string())
        );
    }

    #[test]
    fn test_amqp_receiver_options_builder_with_target() {
        let receiver_options = AmqpReceiverOptions::builder()
            .with_target(AmqpTarget::builder().with_address("test_address").build())
            .build();

        assert!(receiver_options.target.is_some());
        assert_eq!(
            receiver_options.target.unwrap().address.unwrap(),
            "test_address".to_string()
        );
    }

    #[test]
    fn test_amqp_receiver_options_builder_with_name() {
        let receiver_options = AmqpReceiverOptions::builder()
            .with_name("test_receiver")
            .build();

        assert!(receiver_options.name.is_some());
        assert_eq!(receiver_options.name.unwrap(), "test_receiver".to_string());
    }

    #[test]
    fn test_amqp_receiver_options_builder_with_credit_mode() {
        let receiver_options = AmqpReceiverOptions::builder()
            .with_credit_mode(ReceiverCreditMode::Auto(200))
            .build();

        assert!(receiver_options.credit_mode.is_some());
        assert_eq!(
            receiver_options.credit_mode.unwrap(),
            ReceiverCreditMode::Auto(200)
        );
    }

    #[test]
    fn test_amqp_receiver_options_builder_with_auto_accept() {
        let receiver_options = AmqpReceiverOptions::builder()
            .with_auto_accept(true)
            .build();

        assert!(receiver_options.auto_accept);
    }

    #[test]
    fn test_amqp_receiver_options_builder_combination() {
        let mut properties: AmqpOrderedMap<AmqpSymbol, AmqpValue> = AmqpOrderedMap::new();
        properties.insert(
            AmqpSymbol::from("combo_key"),
            AmqpValue::from("combo_value"),
        );

        let receiver_options = AmqpReceiverOptions::builder()
            .with_receiver_settle_mode(ReceiverSettleMode::First)
            .with_target(AmqpTarget::builder().with_address("combo_address").build())
            .with_name("combo_name")
            .with_properties(properties.clone())
            .with_credit_mode(ReceiverCreditMode::Manual)
            .with_auto_accept(false)
            .build();

        assert_eq!(
            receiver_options.receiver_settle_mode,
            Some(ReceiverSettleMode::First)
        );
        assert!(receiver_options.target.is_some());
        assert_eq!(
            receiver_options.target.unwrap().address.unwrap(),
            "combo_address".to_string()
        );
        assert_eq!(receiver_options.name.unwrap(), "combo_name".to_string());
        assert!(receiver_options.properties.is_some());
        let properties_option = receiver_options.properties.unwrap();
        assert_eq!(
            *properties_option.get("combo_key").unwrap(),
            AmqpValue::String("combo_value".to_string())
        );
        assert_eq!(
            receiver_options.credit_mode.unwrap(),
            ReceiverCreditMode::Manual
        );
        assert!(!receiver_options.auto_accept);
    }
}
