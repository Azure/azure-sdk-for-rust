//! Azure storage crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).
#![recursion_limit = "256"]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::new_without_default)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;

pub use self::core::{Error, Result};

#[cfg(feature = "account")]
pub mod account;
// #[cfg(feature = "blob")]
// pub mod blob;
pub mod core;
#[cfg(feature = "data_lake")]
pub mod data_lake;
#[cfg(feature = "table")]
pub mod table;

pub use crate::core::*;
#[cfg(feature = "account")]
pub use account::*;
// #[cfg(feature = "blob")]
// pub use blob::*;
#[cfg(feature = "data_lake")]
pub use data_lake::*;
#[cfg(feature = "table")]
pub use table::*;
