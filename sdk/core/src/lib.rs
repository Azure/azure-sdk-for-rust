//! Core types and traits for the Rust Azure SDK.
//!
//! This crate is part of the unofficial Azure SDK effort in Rust. For more
//! information on the project, and an overview of other crates, please refer to
//! [our GitHub repository](https://github.com/azure/azure-sdk-for-rust).

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
// #![warn(missing_docs, future_incompatible, unreachable_pub)]

#[macro_use]
mod macros;

mod bytes_stream;
mod constants;
mod context;
pub mod date;
pub mod error;
mod http_client;
mod models;
mod options;
mod pageable;
mod pipeline;
mod policies;
mod request;
mod response;
mod seekable_stream;

pub mod auth;
pub mod headers;
pub mod parsing;
pub mod prelude;
pub mod request_options;
pub mod sleep;
pub mod util;

use uuid::Uuid;

pub use bytes_stream::*;
pub use constants::*;
pub use context::Context;
pub use error::Result;
#[doc(inline)]
pub use headers::Header;
pub use http_client::{new_http_client, to_json, HttpClient};
pub use models::*;
pub use options::*;
pub use pageable::*;
pub use pipeline::Pipeline;
pub use policies::*;
pub use request::*;
pub use response::*;
pub use seekable_stream::*;
pub use sleep::sleep;

// re-export important types at crate level
pub use http_types::Method;
pub use http_types::StatusCode;
pub use url::Url;

/// A unique identifier for a request.
// NOTE: only used for Storage?
pub type RequestId = Uuid;

/// A unique session token.
// NOTE: only used for Cosmos?
pub type SessionToken = String;

/// An empty HTTP body.
#[allow(clippy::declare_interior_mutable_const)]
pub const EMPTY_BODY: bytes::Bytes = bytes::Bytes::new();

/// Add a new query pair into the target URL's query string.
pub trait AppendToUrlQuery {
    fn append_to_url_query(&self, url: &mut url::Url);
}

impl<T> AppendToUrlQuery for &T
where
    T: AppendToUrlQuery,
{
    fn append_to_url_query(&self, url: &mut url::Url) {
        (*self).append_to_url_query(url)
    }
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

#[doc(hidden)]
/// Used by macros as an implementation detail
pub mod __private {
    pub use paste::paste;
}
