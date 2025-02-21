// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::messaging::{AmqpDelivery, AmqpSource, AmqpTarget};
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
    /// If set, then the receiver will automatically accept messages as they are received.
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

    fn detach(self) -> impl std::future::Future<Output = Result<()>>;
    fn set_credit_mode(
        &self,
        credit_mode: ReceiverCreditMode,
    ) -> impl std::future::Future<Output = Result<()>>;
    fn credit_mode(&self) -> impl std::future::Future<Output = Result<ReceiverCreditMode>>;
    fn receive_delivery(&self) -> impl std::future::Future<Output = Result<AmqpDelivery>>;
    fn accept_delivery(
        &self,
        delivery: &AmqpDelivery,
    ) -> impl std::future::Future<Output = Result<()>>;
    fn reject_delivery(
        &self,
        delivery: &AmqpDelivery,
    ) -> impl std::future::Future<Output = Result<()>>;
    fn release_delivery(
        &self,
        delivery: &AmqpDelivery,
    ) -> impl std::future::Future<Output = Result<()>>;
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
    pub fn new() -> Self {
        Self {
            implementation: ReceiverImplementation::new(),
        }
    }
}

pub(crate) mod error {
    use crate::{
        error::{AmqpDescribedError, AmqpDetachError, AmqpErrorKind, AmqpLinkStateError},
        AmqpError,
    };

    /// Receiver Errors
    #[derive(Debug)]
    pub enum AmqpReceiverError {
        ReceiverAlreadyAttached,
        CouldNotSetMessageReceiver,
        ReceiverNotSet,

        // Errors that should end the session
        /// The associated session has dropped
        IllegalSessionState,

        /// Link name is already in use
        DuplicatedLinkName,

        /// Illegal link state
        IllegalState,

        /// The local terminus is expecting an Attach from the remote peer
        NonAttachFrameReceived,

        /// The link is expected to be detached immediately but didn't receive
        /// an incoming Detach frame
        ExpectImmediateDetach,

        // Errors that should reject Attach
        /// Incoming Attach frame's Source field is None
        IncomingSourceIsNone,

        /// The remote Attach contains a [`Coordinator`] in the Target
        CoordinatorIsNotImplemented,

        /// This MUST NOT be null if role is sender
        InitialDeliveryCountIsNone,

        // /// When set at the sender this indicates the actual settlement mode in use.
        // ///
        // /// The sender SHOULD respect the receiver’s desired settlement mode ***if
        // /// the receiver initiates*** the attach exchange and the sender supports the desired mode
        // #[error("When set at the sender this indicates the actual settlement mode in use")]
        // SndSettleModeNotSupported,
        /// "When set at the receiver this indicates the actual settlement mode in use"
        ///
        /// The receiver SHOULD respect the sender’s desired settlement mode ***if
        /// the sender initiates*** the attach exchange and the receiver supports the desired mode
        RcvSettleModeNotSupported,

        /// When dynamic is set to true by the sending link endpoint, this field constitutes a request
        /// for the receiving peer to dynamically create a node at the target. In this case the address
        /// field MUST NOT be set.
        TargetAddressIsSomeWhenDynamicIsTrue,

        /// When set to true by the sending link endpoint this field indicates creation of a dynamically created
        /// node. In this case the address field will contain the address of the created node
        SourceAddressIsNoneWhenDynamicIsTrue,

        /// If the dynamic field is not set to true this field MUST be left unset.
        DynamicNodePropertiesIsSomeWhenDynamicIsFalse,

        /// Remote peer closed the link with an error
        RemoteClosedWithError(AmqpDescribedError),

        /// The desired filter(s) on the receiver is not supported by the remote peer
        DesiredFilterNotSupported,

        /// Errors found in link state
        LinkStateError(AmqpLinkStateError),

        /// An error has occurred with Detaching the link.
        DetachError(AmqpDetachError),

        /// The peer sent more message transfers than currently allowed on the link.
        TransferLimitExceeded,

        /// The delivery-id is not found in Transfer
        DeliveryIdIsNone,

        /// The delivery-tag is not found in Transfer
        DeliveryTagIsNone,

        /// Decoding Message failed
        MessageDecode,

        /// If the negotiated link value is first, then it is illegal to set this
        /// field to second.
        IllegalRcvSettleModeInTransfer,

        /// Field is inconsistent in multi-frame delivery
        InconsistentFieldInMultiFrameDelivery,

        /// Transactional acquisition is not supported yet
        TransactionalAcquisitionIsNotImplemented,
    }

    impl std::fmt::Display for AmqpReceiverError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AmqpReceiverError::ReceiverAlreadyAttached => {
                    f.write_str("AMQP Receiver is already attached")
                }
                AmqpReceiverError::CouldNotSetMessageReceiver => {
                    f.write_str("Could not set message receiver.")
                }
                AmqpReceiverError::ReceiverNotSet => f.write_str("Receiver is not set"),
                AmqpReceiverError::IllegalSessionState => {
                    f.write_str("Illegal session state. Session might have stopped.")
                }
                AmqpReceiverError::DuplicatedLinkName => f.write_str("Link name is not unique."),
                AmqpReceiverError::IllegalState => f.write_str("Illegal session state"),
                AmqpReceiverError::NonAttachFrameReceived => {
                    f.write_str("Expecting an Attach frame but received a non-Attach frame")
                }
                AmqpReceiverError::ExpectImmediateDetach => {
                    f.write_str("Expecting the remote peer to immediately detach")
                }
                AmqpReceiverError::IncomingSourceIsNone => f.write_str("Source field is None"),
                AmqpReceiverError::CoordinatorIsNotImplemented => {
                    f.write_str("Control link is not implemented without enabling the `transaction` feature")
                }
                AmqpReceiverError::InitialDeliveryCountIsNone => {
                    f.write_str("Initial delivery count is None")
                }
                AmqpReceiverError::RcvSettleModeNotSupported => {
                    f.write_str("The desired ReceiverSettleMode is not supported by the remote peer")
                }
                AmqpReceiverError::TargetAddressIsSomeWhenDynamicIsTrue => {
                    f.write_str("Target address MUST not be set when dynamic is set to by a sending link endpoint")
                }
                AmqpReceiverError::SourceAddressIsNoneWhenDynamicIsTrue => {
                    f.write_str("When set to true by the sending link endpoint this field indicates creation of a dynamically created node")
                }
                AmqpReceiverError::DynamicNodePropertiesIsSomeWhenDynamicIsFalse => {
                    f.write_str("If the dynamic field is not set to true this field MUST be left unset")
                }
                AmqpReceiverError::RemoteClosedWithError(e) => {
                    f.write_fmt(format_args!("Remote peer closed the link with an error: {:?}", e))
                }
                AmqpReceiverError::DesiredFilterNotSupported => {
                    f.write_fmt(format_args!("The desired filter(s) on the receiver is not supported by the remote peer."))
                }
                AmqpReceiverError::LinkStateError(e) =>
                    f.write_fmt(format_args!("Link state error: {:?}", e)),

                AmqpReceiverError::DetachError(e) => f.write_fmt(format_args!("Detach error: {:?}", e)),
                AmqpReceiverError::TransferLimitExceeded => {
                    f.write_str("The peer sent more message transfers than currently allowed on the link")
                }
                AmqpReceiverError::DeliveryIdIsNone => {
                    f.write_str("The delivery-id is not found in Transfer")
                }
                AmqpReceiverError::DeliveryTagIsNone => {
                    f.write_str("The delivery-tag is not found in Transfer")
                }
                AmqpReceiverError::MessageDecode => {
                    f.write_str("Decoding Message failed")
                }
                AmqpReceiverError::IllegalRcvSettleModeInTransfer => {
                    f.write_str("If the negotiated link value is first, then it is illegal to set this field to second")
                }
                AmqpReceiverError::InconsistentFieldInMultiFrameDelivery => {
                    f.write_str("Field is inconsistent in multi-frame delivery")
                }
                AmqpReceiverError::TransactionalAcquisitionIsNotImplemented => {
                    f.write_str("Transactional acquisition is not implemented")
                }
            }
        }
    }

    impl std::error::Error for AmqpReceiverError {}

    impl From<AmqpReceiverError> for AmqpErrorKind {
        fn from(e: AmqpReceiverError) -> Self {
            AmqpErrorKind::ReceiverError(e)
        }
    }
    impl From<AmqpReceiverError> for azure_core::Error {
        fn from(e: AmqpReceiverError) -> Self {
            AmqpError::from(AmqpErrorKind::ReceiverError(e)).into()
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
