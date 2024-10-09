#![doc = include_str!("../README.md")]
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
