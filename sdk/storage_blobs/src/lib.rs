#![cfg_attr(feature = "into_future", feature(into_future))]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;

pub use azure_core::error::{Error, ErrorKind, ResultExt};

pub mod blob;
pub mod container;
pub mod service;
pub mod prelude;

mod clients;
mod options;
