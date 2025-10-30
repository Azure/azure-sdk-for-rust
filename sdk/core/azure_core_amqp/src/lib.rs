// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

#[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
mod fe2o3;

#[cfg(any(not(feature = "fe2o3_amqp"), target_arch = "wasm32"))]
mod noop;

mod cbs;
mod connection;
/// AMQP error types.
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
pub use error::*;
pub use management::{AmqpManagement, AmqpManagementApis};
pub use messaging::{AmqpDelivery, AmqpDeliveryApis, AmqpMessage, AmqpSource, AmqpTarget};
pub use receiver::{AmqpReceiver, AmqpReceiverApis, AmqpReceiverOptions, ReceiverCreditMode};
pub use sender::{AmqpSendOptions, AmqpSendOutcome, AmqpSender, AmqpSenderApis, AmqpSenderOptions};
pub use session::{AmqpSession, AmqpSessionApis, AmqpSessionOptions};
pub use simple_value::AmqpSimpleValue;
use std::fmt::Debug;
pub use value::{AmqpDescribed, AmqpList, AmqpOrderedMap, AmqpSymbol, AmqpTimestamp, AmqpValue};

/// Builders for AMQP types.
pub mod builder {
    pub use crate::messaging::builders::{
        AmqpMessageBuilder, AmqpSourceBuilder, AmqpTargetBuilder,
    };
}

/// AMQP message related types.
pub mod message {
    pub use crate::messaging::{
        AmqpAnnotationKey, AmqpAnnotations, AmqpApplicationProperties, AmqpMessageBody,
        AmqpMessageHeader, AmqpMessageId, AmqpMessageProperties, AmqpOutcome, AmqpSourceFilter,
        DistributionMode, TerminusDurability, TerminusExpiryPolicy,
    };
}

// AMQP Settle mode:
// See also: [AMQP Sender Settle Mode](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-sender-settle-mode)
const AMQP_SENDER_SETTLE_MODE_UNSETTLED: isize = 0;
const AMQP_SENDER_SETTLE_MODE_SETTLED: isize = 1;
const AMQP_SENDER_SETTLE_MODE_MIXED: isize = 2;

/// AMQP Sender settle mode.
///
/// See also: [AMQP Sender Settle Mode](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-sender-settle-mode)
#[derive(Debug, Clone, PartialEq)]
pub enum SenderSettleMode {
    /// Unsettled mode.
    Unsettled = AMQP_SENDER_SETTLE_MODE_UNSETTLED,
    /// Settled mode.
    Settled = AMQP_SENDER_SETTLE_MODE_SETTLED,
    /// Mixed mode.
    Mixed = AMQP_SENDER_SETTLE_MODE_MIXED,
}

// AMQP Receiver settle mode:
// See also: [AMQP Receiver Settle Mode](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-receiver-settle-mode)

const AMQP_RECEIVER_SETTLE_MODE_FIRST: isize = 0;
const AMQP_RECEIVER_SETTLE_MODE_SECOND: isize = 1;

/// AMQP Receiver settle mode.
///
/// See also: [AMQP Receiver Settle Mode](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-transport-v1.0-os.html#type-receiver-settle-mode)
#[derive(Debug, Clone, PartialEq)]
pub enum ReceiverSettleMode {
    /// First mode.
    First = AMQP_RECEIVER_SETTLE_MODE_FIRST,
    /// Second mode.
    Second = AMQP_RECEIVER_SETTLE_MODE_SECOND,
}

/// Trait for types that can be serialized to and deserialized from a byte buffer.
#[cfg(feature = "ffi")]
pub trait Serializable {
    /// Serializes the type into the provided byte buffer.
    fn serialize(&self, buffer: &mut [u8]) -> crate::error::Result<()>;

    /// Returns the size in bytes that the type will occupy when serialized.
    fn encoded_size(&self) -> crate::error::Result<usize>;
}

/// Trait for types that can be deserialized from a byte buffer.
#[cfg(feature = "ffi")]
pub trait Deserializable<T> {
    /// Deserializes the type from the provided byte buffer.
    fn decode(data: &[u8]) -> crate::error::Result<T>;
}

#[cfg(feature = "ffi")]
pub use value::{AmqpComposite, AmqpDescriptor};
