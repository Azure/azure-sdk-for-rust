#![recursion_limit = "128"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_sdk_core;

mod authorization_token;
mod client;
pub mod collection;
mod create_collection_builder;
pub mod database;
pub mod document;
pub mod offer;
mod partition_key;
pub mod prelude;
pub mod query;
pub mod request_response;
mod requests;

pub use self::authorization_token::*;
pub use self::client::*;
pub use self::offer::Offer;
pub use self::partition_key::*;
pub use self::requests::*;

use azure_sdk_core::enumerations;
use azure_sdk_core::errors::TraversingError;
use azure_sdk_core::parsing::FromStringOptional;
use std::fmt;
use std::str::FromStr;

create_enum!(
    ConsistencyLevel,
    (Strong, "Strong"),
    (Bounded, "Bounded"),
    (Session, "Session"),
    (Eventual, "Eventual")
);
