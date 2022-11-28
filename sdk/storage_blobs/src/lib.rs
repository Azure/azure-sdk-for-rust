#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;

pub mod blob;
pub mod container;
pub mod prelude;
pub mod service;

mod clients;
mod options;
