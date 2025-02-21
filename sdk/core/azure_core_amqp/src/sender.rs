// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

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
    fn max_message_size(&self) -> impl std::future::Future<Output = Result<Option<u64>>>;
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

    async fn max_message_size(&self) -> Result<Option<u64>> {
        self.implementation.max_message_size().await
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

pub(crate) mod error {
    use crate::{
        error::{AmqpDescribedError, AmqpErrorKind},
        AmqpError,
    };

    pub enum AmqpSenderError {
        // Sender Errors
        NonTerminalDeliveryState,
        IllegalDeliveryState,
        MessageEncodeError,

        /// Could not set message sender.
        CouldNotSetMessageSender,

        ///  Could not get message sender.
        CouldNotGetMessageSender,
        // Illegal session state
        /// Session stopped
        IllegalSessionState,

        /// Link name duplicated
        DuplicatedLinkName,

        /// Illegal link state
        IllegalState,

        /// The local terminus is expecting an Attach from the remote peer
        NonAttachFrameReceived,

        /// The link is expected to be detached immediately but didn't receive
        /// an incoming Detach frame
        ExpectImmediateDetach,

        /// Incoming Attach frame's Target field is None
        IncomingTargetIsNone,

        /// The remote Attach contains a [`Coordinator`] in the Target
        CoordinatorIsNotImplemented,

        /// When set at the sender this indicates the actual settlement mode in use.
        ///
        /// The sender SHOULD respect the receiver’s desired settlement mode ***if
        /// the receiver initiates*** the attach exchange and the sender supports the desired mode
        SndSettleModeNotSupported,

        /// "When set at the receiver this indicates the actual settlement mode in use"
        ///
        /// The receiver SHOULD respect the sender’s desired settlement mode ***if
        /// the sender initiates*** the attach exchange and the receiver supports the desired mode
        RcvSettleModeNotSupported,

        /// When set to true by the receiving link endpoint this field indicates creation of a
        /// dynamically created node. In this case the address field will contain the address of the
        /// created node.
        TargetAddressIsNoneWhenDynamicIsTrue,

        /// When set to true by the receiving link endpoint, this field constitutes a request for the sending
        /// peer to dynamically create a node at the source. In this case the address field MUST NOT be set
        SourceAddressIsSomeWhenDynamicIsTrue,

        /// If the dynamic field is not set to true this field MUST be left unset.
        DynamicNodePropertiesIsSomeWhenDynamicIsFalse,

        /// Remote peer closed the link with an error
        RemoteClosedWithError(AmqpDescribedError),
    }

    impl std::error::Error for AmqpSenderError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            None
        }
    }

    impl std::fmt::Debug for AmqpSenderError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AmqpSenderError: {}", self)?;
            Ok(())
        }
    }

    impl std::fmt::Display for AmqpSenderError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AmqpSenderError::IllegalSessionState => {
                    write!(f, "Illegal session state. Session might have stopped.")
                }
                AmqpSenderError::DuplicatedLinkName => write!(f, "Link name is not unique."),
                AmqpSenderError::IllegalState => write!(f, "Illegal session state"),
                AmqpSenderError::NonAttachFrameReceived => {
                    write!(f, "Expecting an Attach frame but received a non-Attach frame")
                }
                AmqpSenderError::ExpectImmediateDetach => {
                    write!(f, "Expecting the remote peer to immediately detach")
                }
                AmqpSenderError::IncomingTargetIsNone => write!(f, "Target field is None"),
                AmqpSenderError::CoordinatorIsNotImplemented => write!(
                    f,
                    "Control link is not implemented without enabling the `transaction` feature"
                ),
                AmqpSenderError::SndSettleModeNotSupported => write!(
                    f,
                    "When set at the sender this indicates the actual settlement mode in use"
                ),
                AmqpSenderError::RcvSettleModeNotSupported => write!(
                    f,
                    "The desired ReceiverSettleMode is not supported by the remote peer"
                ),
                AmqpSenderError::TargetAddressIsNoneWhenDynamicIsTrue => write!(
                    f,
                    "The address field contains the address of the created node when dynamic is set by the receiving endpoint"
                ),
                AmqpSenderError::SourceAddressIsSomeWhenDynamicIsTrue => write!(
                    f,
                    "Source address must not be set when dynamic is set by the receiving endpoint"
                ),
                AmqpSenderError::DynamicNodePropertiesIsSomeWhenDynamicIsFalse => write!(
                    f,
                    "If the dynamic field is not set to true this field MUST be left unset."
                ),
                AmqpSenderError::RemoteClosedWithError(e) => {
                    write!(f, "Remote peer closed with error {:?}", e)
                }
                AmqpSenderError::CouldNotSetMessageSender => {
                    write!(f, "Could not set message sender")
                }
                AmqpSenderError::CouldNotGetMessageSender => {
                    write!(f, "Could not get message sender")
                }
                AmqpSenderError::NonTerminalDeliveryState => {
                    write!(f, "Non-terminal delivery state")
                }
                AmqpSenderError::IllegalDeliveryState => write!(f, "Illegal delivery state"),
                AmqpSenderError::MessageEncodeError => write!(f, "Message encode error"),
            }
        }
    }

    impl From<AmqpSenderError> for AmqpError {
        fn from(e: AmqpSenderError) -> Self {
            AmqpError::new(AmqpErrorKind::SenderError(e))
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
