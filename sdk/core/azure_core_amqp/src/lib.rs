// Copyright (c) Microsoft Corporation. All Rights reserved
// cspell: words amqp sasl
#[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
mod fe2o3;

#[cfg(any(not(feature = "fe2o3-amqp"), target_arch = "wasm32"))]
mod noop;

pub mod cbs;
pub mod connection;
pub mod error;
pub mod management;
pub mod messaging;
pub mod receiver;
pub mod sender;
pub mod session;
pub mod value;

pub use uuid::Uuid;

use std::fmt::Debug;

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
