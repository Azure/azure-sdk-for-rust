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

#[derive(Debug, Default, Clone)]
pub struct AmqpReceiverOptions {
    pub receiver_settle_mode: Option<ReceiverSettleMode>,
    pub target: Option<AmqpTarget>,
    pub name: Option<String>,
    pub credit_mode: Option<ReceiverCreditMode>,
    pub auto_accept: bool,
    pub properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
}

impl AmqpReceiverOptions {}

#[allow(unused_variables)]
pub trait AmqpReceiverApis {
    fn attach(
        &self,
        session: &AmqpSession,
        source: impl Into<AmqpSource>,
        options: Option<AmqpReceiverOptions>,
    ) -> impl std::future::Future<Output = Result<()>>;
    fn max_message_size(&self) -> impl std::future::Future<Output = Result<Option<u64>>>;
    fn receive(&self) -> impl std::future::Future<Output = Result<AmqpMessage>>;
}

#[derive(Default)]
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
    async fn max_message_size(&self) -> Result<Option<u64>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amqp_receiver_options_builder() {
        let mut properties: AmqpOrderedMap<AmqpSymbol, AmqpValue> = AmqpOrderedMap::new();
        properties.insert(AmqpSymbol::from("key"), AmqpValue::from("value"));

        let receiver_options = AmqpReceiverOptions {
            receiver_settle_mode: Some(ReceiverSettleMode::First),
            target: Some(
                AmqpTarget::builder()
                    .with_address("address".to_string())
                    .build(),
            ),
            properties: Some(properties),
            ..Default::default()
        };

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
        let receiver_options = AmqpReceiverOptions {
            target: Some(
                AmqpTarget::builder()
                    .with_address("test_address".to_string())
                    .build(),
            ),
            ..Default::default()
        };

        assert!(receiver_options.target.is_some());
        assert_eq!(
            receiver_options.target.unwrap().address().unwrap().clone(),
            "test_address".to_string()
        );
    }

    #[test]
    fn test_amqp_receiver_options_builder_with_name() {
        let receiver_options = AmqpReceiverOptions {
            name: Some("test_receiver".into()),
            ..Default::default()
        };

        assert!(receiver_options.name.is_some());
        assert_eq!(receiver_options.name.unwrap(), "test_receiver".to_string());
    }

    #[test]
    fn test_amqp_receiver_options_builder_with_credit_mode() {
        let receiver_options = AmqpReceiverOptions {
            credit_mode: Some(ReceiverCreditMode::Auto(200)),
            ..Default::default()
        };

        assert!(receiver_options.credit_mode.is_some());
        assert_eq!(
            receiver_options.credit_mode.unwrap(),
            ReceiverCreditMode::Auto(200)
        );
    }

    #[test]
    fn test_amqp_receiver_options_builder_with_auto_accept() {
        let receiver_options = AmqpReceiverOptions {
            auto_accept: true,
            ..Default::default()
        };

        assert!(receiver_options.auto_accept);
    }

    #[test]
    fn test_amqp_receiver_options_builder_combination() {
        let mut properties: AmqpOrderedMap<AmqpSymbol, AmqpValue> = AmqpOrderedMap::new();
        properties.insert(
            AmqpSymbol::from("combo_key"),
            AmqpValue::from("combo_value"),
        );

        let receiver_options = AmqpReceiverOptions {
            receiver_settle_mode: Some(ReceiverSettleMode::First),
            target: Some(
                AmqpTarget::builder()
                    .with_address("combo_address".to_string())
                    .build(),
            ),
            name: Some("combo_name".into()),
            properties: Some(properties.clone()),
            credit_mode: Some(ReceiverCreditMode::Manual),
            auto_accept: false,
        };

        assert_eq!(
            receiver_options.receiver_settle_mode,
            Some(ReceiverSettleMode::First)
        );
        assert!(receiver_options.target.is_some());
        assert_eq!(
            receiver_options.target.unwrap().address().unwrap().clone(),
            "combo_address".to_string()
        );
        assert_eq!(receiver_options.name.unwrap(), "combo_name".to_string());
        assert!(receiver_options.properties.is_some());
        let properties_option = receiver_options.properties.unwrap();
        assert_eq!(
            *properties_option
                .get(&AmqpSymbol::from("combo_key"))
                .unwrap(),
            AmqpValue::String("combo_value".to_string())
        );
        assert_eq!(
            receiver_options.credit_mode.unwrap(),
            ReceiverCreditMode::Manual
        );
        assert!(!receiver_options.auto_accept);
    }
}
