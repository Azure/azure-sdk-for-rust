//! Azure service bus crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).
#![recursion_limit = "128"]

pub mod administration;
pub mod amqp;
pub mod client;
pub mod core;
pub mod prelude;
pub mod primitives;
pub mod receiver;
pub mod service_bus;
pub mod utils;
