#![allow(write_literal)]

mod authorization_token;
mod client;
pub mod database;

pub mod request_response;

pub mod collection;
pub mod document;
mod document_options;
mod partition_key;
pub mod query;
pub mod query_document;

pub use self::authorization_token::*;
pub use self::client::*;
pub use self::document_options::*;
pub use self::partition_key::*;

use azure::core::enumerations;
use azure::core::errors::TraversingError;
use azure::core::parsing::FromStringOptional;
use std::fmt;
use std::str::FromStr;

create_enum!(
    ConsistencyLevel,
    (Strong, "Strong"),
    (Bounded, "Bounded"),
    (Session, "Session"),
    (Eventual, "Eventual")
);
