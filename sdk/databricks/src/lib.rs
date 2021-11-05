#![recursion_limit = "256"]

pub mod prelude;

#[macro_use]
extern crate azure_core;
pub mod client;

#[cfg(feature = "clusters")]
pub mod clusters;

#[cfg(feature = "clusters")]
pub use clusters::*;
