//! Azure App Configuration
//!
//! This crate is part of the unofficial Azure SDK effort in Rust. For more
//! information on the project, and an overview of other crates, please refer to
//! [our GitHub repository](https://github.com/azure/azure-sdk-for-rust).
extern crate azure_core;

pub(crate) mod auto_refresh;

#[cfg(feature = "configuration")]
pub mod configuration;
#[cfg(feature = "feature_manager")]
pub mod feature_manager;

pub mod prelude;
