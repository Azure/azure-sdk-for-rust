//! Azure iot_hub crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).
#![deny(missing_docs)]
//! The IoT Hub crate contains a client that can be used to manage the IoT Hub.

mod authorization_policy;
/// The service module contains the IoT Hub Service Client that can be used to manage the IoT Hub.
pub mod service;
