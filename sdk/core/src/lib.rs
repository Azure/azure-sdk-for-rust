//! Core crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).
#![recursion_limit = "256"]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;

mod bytes_response;
mod bytes_stream;
mod constants;
mod context;
mod errors;
pub mod headers;
mod http_client;
pub mod incompletevector;
#[cfg(feature = "mock_transport_framework")]
mod mock_transaction;
mod models;
mod options;
mod overridable_context;
pub mod parsing;
pub mod pipeline;
mod policies;
pub mod prelude;
mod request;
mod request_options;
mod response;
mod seekable_stream;
mod sleep;
mod type_map_context;
pub mod util;

use chrono::{DateTime, Utc};
use headers::*;
use oauth2::AccessToken;
use std::fmt::Debug;
use uuid::Uuid;

pub use bytes_stream::*;
pub use constants::*;
pub use context::Context;
pub use errors::*;
pub use headers::AddAsHeader;
pub use http_client::{new_http_client, to_json, HttpClient};
#[cfg(feature = "mock_transport_framework")]
pub use mock_transaction::constants::*;
pub use models::*;
pub use options::*;
pub use overridable_context::OverridableContext;
pub use policies::{Policy, PolicyResult};
pub use request::*;
pub use response::*;
pub use seekable_stream::*;
pub use type_map_context::TypeMapContext;

pub type RequestId = Uuid;
pub type SessionToken = String;
pub const EMPTY_BODY: &[u8] = &[];

/// Represents an Azure service bearer access token with expiry information.
#[derive(Debug, Clone)]
pub struct TokenResponse {
    /// Get the access token value.
    pub token: AccessToken,
    /// Gets the time when the provided token expires.
    pub expires_on: DateTime<Utc>,
}

impl TokenResponse {
    /// Create a new `TokenResponse`
    pub fn new(token: AccessToken, expires_on: DateTime<Utc>) -> Self {
        Self { token, expires_on }
    }
}

/// Represents a credential capable of providing an OAuth token.
#[async_trait::async_trait]
pub trait TokenCredential: Send + Sync {
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, Error>;
}

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
