// Copyright (c) Microsoft Corp. All Rights Reserved.
// cspell: words amqp sasl
#[cfg(all(feature = "enable-fe2o3-amqp", not(target_arch = "wasm32")))]
mod fe2o3;

#[cfg(any(not(feature = "enable-fe2o3-amqp"), target_arch = "wasm32"))]
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

use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum SenderSettleMode {
    Unsettled = 0,
    Settled = 1,
    Mixed = 2,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReceiverSettleMode {
    First = 0,
    Second = 1,
}
