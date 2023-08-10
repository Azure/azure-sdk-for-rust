//! Azure Security Key Vault
//!
//! This crate is part of the unofficial Azure SDK effort in Rust. For more
//! information on the project, and an overview of other crates, please refer to
//! [our GitHub repository](https://github.com/azure/azure-sdk-for-rust).

#![deny(clippy::unwrap_used, clippy::expect_used)]

#[macro_use]
extern crate azure_core;

mod account;
mod certificates;
mod clients;
mod keys;
pub mod prelude;
mod secrets;

pub use clients::*;
