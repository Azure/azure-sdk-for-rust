#![doc = include_str!("../README.md")]
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
pub mod hmac;
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
pub mod lro;
pub mod parsing;
pub mod prelude;
pub mod request_options;
pub mod sleep;
pub mod util;

use uuid::Uuid;

#[cfg(feature = "xml")]
pub mod xml;

pub mod tokio;

pub mod base64;
pub use bytes_stream::*;
pub use constants::*;
pub use context::Context;
pub use error::{Error, Result};
#[doc(inline)]
pub use headers::Header;
pub use http_client::{from_json, new_http_client, to_json, HttpClient};
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
    fn append_to_url_query(&self, url: &mut crate::Url);
}

impl<T> AppendToUrlQuery for &T
where
    T: AppendToUrlQuery,
{
    fn append_to_url_query(&self, url: &mut crate::Url) {
        (*self).append_to_url_query(url);
    }
}

impl<T> AppendToUrlQuery for Option<T>
where
    T: AppendToUrlQuery,
{
    fn append_to_url_query(&self, url: &mut crate::Url) {
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
