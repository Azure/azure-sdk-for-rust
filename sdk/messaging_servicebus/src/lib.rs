//! Azure service bus crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).
#![recursion_limit = "128"]

#[macro_use]
extern crate log;

pub mod errors;
pub mod event_hub;
pub mod prelude;

pub use self::errors::{Error, Result};
