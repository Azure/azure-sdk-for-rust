#![recursion_limit = "128"]

extern crate base64;
extern crate chrono;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_rustls;
extern crate md5;
extern crate ring;
extern crate time;
extern crate url;
extern crate uuid;
extern crate xml;
#[macro_use]
extern crate log;
extern crate quick_error;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bytes;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate smallvec;

#[macro_use]
extern crate azure_sdk_core;

mod authorization_token;
mod client;
mod create_collection_builder;
pub mod database;
mod requests;

pub mod request_response;

pub mod collection;
pub mod document;
pub mod offer;
mod partition_key;
pub mod prelude;
pub mod query;

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
