pub mod authorization_token;
pub mod client;
pub mod database;

pub mod request_response;

pub mod collection;
pub mod document;
pub mod get_document;
pub mod list_documents;
pub mod partition_key;
pub mod query;
pub mod query_document;

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
