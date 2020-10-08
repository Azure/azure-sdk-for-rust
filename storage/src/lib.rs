#![recursion_limit = "128"]
#![allow(clippy::needless_lifetimes)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_sdk_core;
#[macro_use]
extern crate quick_error;

pub mod account;
pub mod blob;
pub mod core;
pub mod queue;
pub mod table;

pub use crate::core::*;
pub use account::*;
pub use blob::*;
pub use queue::*;
pub use table::*;
