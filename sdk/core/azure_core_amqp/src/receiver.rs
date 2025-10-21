// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::Result,
    messaging::{AmqpDelivery, AmqpSource, AmqpTarget},
    session::AmqpSession,
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
    ReceiverSettleMode,
};

#[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
type ReceiverImplementation = super::fe2o3::receiver::Fe2o3AmqpReceiver;

#[cfg(any(not(feature = "fe2o3_amqp"), target_arch = "wasm32"))]
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
    /// Automatically issue the specified number of credits to the sender. This allows the receiver to
    /// control the flow of messages by specifying how many messages it is ready to receive.
    Auto(u32),

    /// The receiver manually controls when to issue credit to the sender. This mode gives the receiver
    /// complete control over the flow of messages, allowing it to request messages from the sender as needed.
    Manual,
}

impl Default for ReceiverCreditMode {
    fn default() -> Self {
        ReceiverCreditMode::Auto(100)
    }
}

/// Options for configuring an AMQP receiver.
#[derive(Debug, Default, Clone)]
pub struct AmqpReceiverOptions {
    /// The receiver settle mode for the AMQP receiver.
    pub receiver_settle_mode: Option<ReceiverSettleMode>,

    /// The target for the AMQP receiver.
    pub target: Option<AmqpTarget>,

    /// The name of the AMQP receiver.
    pub name: Option<String>,

    /// The credit mode for the AMQP receiver.
    pub credit_mode: Option<ReceiverCreditMode>,
    /// If set, then the receiver will automatically accept messages as they are received.
    pub auto_accept: bool,

    /// Additional properties for the AMQP receiver.
    pub properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
}

impl AmqpReceiverOptions {}

/// Trait defining the asynchronous APIs for AMQP receiver operations.
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait AmqpReceiverApis {
    /// Attaches the AMQP receiver to the specified session and source.
    ///
    /// # Arguments
    /// * `session` - The AMQP session to attach the receiver to.
    /// * `source` - The source from which the receiver will receive messages.
    /// * `options` - Optional configuration options for the receiver.
    ///
    async fn attach(
        &self,
        session: &AmqpSession,
        source: impl Into<AmqpSource> + Send,
        options: Option<AmqpReceiverOptions>,
    ) -> Result<()>;

    /// Detaches the AMQP receiver from the session.
    ///
    /// This method cleans up any resources associated with the receiver and ensures that it is properly
    /// disconnected from the AMQP session.
    ///
    async fn detach(self) -> Result<()>;

    /// Sets the credit mode for the AMQP receiver.
    ///
    /// # Arguments
    /// * `credit_mode` - The credit mode to set for the receiver.
    async fn set_credit_mode(&self, credit_mode: ReceiverCreditMode) -> Result<()>;

    /// Gets the current credit mode of the AMQP receiver.
    async fn credit_mode(&self) -> Result<ReceiverCreditMode>;

    /// Receives a delivery from the AMQP receiver.
    async fn receive_delivery(&self) -> Result<AmqpDelivery>;

    /// Accepts a delivery from the AMQP receiver.
    async fn accept_delivery(&self, delivery: &AmqpDelivery) -> Result<()>;

    /// Rejects a delivery from the AMQP receiver.
    async fn reject_delivery(&self, delivery: &AmqpDelivery) -> Result<()>;

    /// Releases a delivery from the AMQP receiver.
    async fn release_delivery(&self, delivery: &AmqpDelivery) -> Result<()>;
}

/// Struct representing the AMQP receiver functionality.
#[derive(Default)]
pub struct AmqpReceiver {
    implementation: ReceiverImplementation,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpReceiverApis for AmqpReceiver {
    async fn attach(
        &self,
        session: &AmqpSession,
        source: impl Into<AmqpSource> + Send,
        options: Option<AmqpReceiverOptions>,
    ) -> Result<()> {
        self.implementation.attach(session, source, options).await
    }
    async fn detach(self) -> Result<()> {
        self.implementation.detach().await
    }

    async fn set_credit_mode(&self, credit_mode: ReceiverCreditMode) -> Result<()> {
        self.implementation.set_credit_mode(credit_mode).await
    }

    async fn credit_mode(&self) -> Result<ReceiverCreditMode> {
        self.implementation.credit_mode().await
    }

    /// Receives a delivery from the AMQP receiver.
    ///
    /// This method returns a single [`AmqpDelivery`] that can be used to receive a message from the AMQP receiver.
    ///
    /// If the receiver options are set to [`AmqpReceiverOptions::auto_accept`], the delivery will have already been accepted, and no
    /// further actions are required to accept the message.
    ///
    async fn receive_delivery(&self) -> Result<AmqpDelivery> {
        self.implementation.receive_delivery().await
    }

    async fn accept_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        self.implementation.accept_delivery(delivery).await
    }

    async fn reject_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        self.implementation.reject_delivery(delivery).await
    }

    async fn release_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        self.implementation.release_delivery(delivery).await
    }
}

impl AmqpReceiver {
    /// Creates a new instance of `AmqpReceiver`.
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
            receiver_options.target.unwrap().address.unwrap(),
            "test_address"
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
            receiver_options.target.unwrap().address.unwrap(),
            "combo_address"
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

    // #[test]
    // async fn test_amqp_receiver_set_credit_mode() {
    //     let receiver = AmqpReceiver::new();

    //     receiver.attach(session, source, options)
    //     receiver.set_credit_mode(ReceiverCreditMode::Manual);

    //     // Assuming the implementation has a method to get the current credit mode for testing purposes
    //     assert_eq!(
    //         receiver.implementation.get_credit_mode(),
    //         ReceiverCreditMode::Manual
    //     );

    //     receiver.set_credit_mode(ReceiverCreditMode::Auto(100));

    //     assert_eq!(
    //         receiver.implementation.get_credit_mode(),
    //         ReceiverCreditMode::Auto(100)
    //     );
    // }
}
