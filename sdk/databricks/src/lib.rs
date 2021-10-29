#![recursion_limit = "256"]


#[macro_use]
extern crate azure_core;

#[cfg(feature = "clusters")]
pub mod clusters;

#[cfg(feature = "clusters")]
pub use clusters::*;
