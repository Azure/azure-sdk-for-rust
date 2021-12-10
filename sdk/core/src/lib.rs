//! Core types and traits for the Rust Azure SDK.
//!
//! This crate is part of the unofficial Azure SDK effort in Rust. For more
//! information on the project, and an overview of other crates, please refer to
//! [our GitHub repository](https://github.com/azure/azure-sdk-for-rust).

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
// #![warn(missing_docs, future_incompatible, unreachable_pub)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;

#[cfg(feature = "mock_transport_framework")]
mod bytes_response;
mod bytes_stream;
mod constants;
mod context;
mod errors;
mod http_client;
mod models;
mod options;
mod pipeline;
mod policies;
mod request;
mod request_options;
mod response;
mod seekable_stream;
mod sleep;

pub mod auth;
pub mod headers;
#[cfg(feature = "mock_transport_framework")]
mod mock_transaction;
pub mod parsing;
pub mod prelude;
pub mod util;

use headers::*;
use uuid::Uuid;

pub use bytes_stream::*;
pub use constants::*;
pub use context::Context;
pub use errors::*;
#[doc(inline)]
pub use headers::AddAsHeader;
pub use http_client::{new_http_client, to_json, HttpClient};
#[cfg(feature = "mock_transport_framework")]
pub use mock_transaction::constants::*;
pub use models::*;
pub use options::*;
pub use pipeline::Pipeline;
pub use policies::{Policy, PolicyResult};
pub use request::*;
pub use response::*;
pub use seekable_stream::*;

/// A unique identifier for a request.
// NOTE: only used for Storage?
pub type RequestId = Uuid;

/// A unique session token.
// NOTE: only used for Cosmos?
pub type SessionToken = String;

/// An empty HTTP body.
#[allow(clippy::declare_interior_mutable_const)]
pub const EMPTY_BODY: bytes::Bytes = bytes::Bytes::from_static(&[]);

/// Add a new query pair into the target URL's query string.
pub trait AppendToUrlQuery {
    fn append_to_url_query(&self, url: &mut url::Url);
}

impl<T> AppendToUrlQuery for Option<T>
where
    T: AppendToUrlQuery,
{
    fn append_to_url_query(&self, url: &mut url::Url) {
        if let Some(i) = self {
            i.append_to_url_query(url);
        }
    }
}
