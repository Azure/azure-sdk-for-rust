// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
mod fe2o3;

#[cfg(any(not(feature = "fe2o3_amqp"), target_arch = "wasm32"))]
mod noop;

mod cbs;
mod connection;
pub mod error;
mod management;
mod messaging;
mod receiver;
mod sender;
mod session;
mod simple_value;
mod value;

pub use cbs::{AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityApis};
pub use connection::{AmqpConnection, AmqpConnectionApis, AmqpConnectionOptions};
pub use error::{AmqpDescribedError, AmqpError};
pub use management::{AmqpManagement, AmqpManagementApis};
pub use messaging::{AmqpDelivery, AmqpDeliveryApis, AmqpMessage, AmqpSource, AmqpTarget};
pub use receiver::{AmqpReceiver, AmqpReceiverApis, AmqpReceiverOptions, ReceiverCreditMode};
pub use sender::{AmqpSendOptions, AmqpSendOutcome, AmqpSender, AmqpSenderApis, AmqpSenderOptions};
pub use session::{AmqpSession, AmqpSessionApis, AmqpSessionOptions};
pub use simple_value::AmqpSimpleValue;
use std::fmt::Debug;
pub use value::{AmqpDescribed, AmqpList, AmqpOrderedMap, AmqpSymbol, AmqpTimestamp, AmqpValue};

pub mod builder {
    pub use crate::messaging::builders::{
        AmqpMessageBuilder, AmqpSourceBuilder, AmqpTargetBuilder,
    };
}

pub mod message {
    pub use crate::messaging::{
        AmqpAnnotationKey, AmqpAnnotations, AmqpApplicationProperties, AmqpMessageBody,
        AmqpMessageHeader, AmqpMessageId, AmqpMessageProperties, AmqpOutcome, AmqpSourceFilter,
        DistributionMode, TerminusDurability, TerminusExpiryPolicy,
    };
}

// AMQP Settle mode:
// https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-sender-settle-mode
const AMQP_SENDER_SETTLE_MODE_UNSETTLED: isize = 0;
const AMQP_SENDER_SETTLE_MODE_SETTLED: isize = 1;
const AMQP_SENDER_SETTLE_MODE_MIXED: isize = 2;

#[derive(Debug, Clone, PartialEq)]
pub enum SenderSettleMode {
    Unsettled = AMQP_SENDER_SETTLE_MODE_UNSETTLED,
    Settled = AMQP_SENDER_SETTLE_MODE_SETTLED,
    Mixed = AMQP_SENDER_SETTLE_MODE_MIXED,
}

// AMQP Receiver settle mode:
// https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-receiver-settle-mode

const AMQP_RECEIVER_SETTLE_MODE_FIRST: isize = 0;
const AMQP_RECEIVER_SETTLE_MODE_SECOND: isize = 1;

#[derive(Debug, Clone, PartialEq)]
pub enum ReceiverSettleMode {
    First = AMQP_RECEIVER_SETTLE_MODE_FIRST,
    Second = AMQP_RECEIVER_SETTLE_MODE_SECOND,
}

#[cfg(feature = "cplusplus")]
pub trait Serializable {
    fn serialize(&self, buffer: &mut [u8]) -> azure_core::Result<()>;

    fn encoded_size(&self) -> azure_core::Result<usize>;
}

#[cfg(feature = "cplusplus")]
pub trait Deserializable<T> {
    fn decode(data: &[u8]) -> azure_core::Result<T>;
}

#[cfg(feature = "cplusplus")]
pub use value::{AmqpComposite, AmqpDescriptor};
