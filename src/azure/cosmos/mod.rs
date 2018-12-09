mod authorization_token;
mod client;
pub mod database;
mod requests;

pub mod request_response;

pub mod collection;
pub mod document;
mod partition_key;
pub mod query;

pub use self::authorization_token::*;
pub use self::client::*;
pub use self::partition_key::*;
pub use self::requests::*;

use crate::azure::core::enumerations;
use crate::azure::core::errors::TraversingError;
use crate::azure::core::parsing::FromStringOptional;
use std::fmt;
use std::str::FromStr;

create_enum!(
    ConsistencyLevel,
    (Strong, "Strong"),
    (Bounded, "Bounded"),
    (Session, "Session"),
    (Eventual, "Eventual")
);
