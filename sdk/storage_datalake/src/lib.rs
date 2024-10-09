#![doc = include_str!("../README.md")]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;

pub mod clients;
pub mod file_system;
pub mod operations;
pub mod prelude;
mod properties;
pub mod request_options;
mod util;

pub use file_system::FileSystem;
pub use properties::Properties;
