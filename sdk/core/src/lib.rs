#![recursion_limit = "256"]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;

mod bytes_stream;
mod context;
mod errors;
pub mod headers;
mod http_client;
pub mod incompletevector;
mod models;
pub mod parsing;
pub mod pipeline;
pub mod policies;
pub mod prelude;
mod request;
mod request_options;
mod response;
mod seekable_stream;
mod sleep;
pub mod util;

use chrono::{DateTime, Utc};
use headers::*;
use oauth2::AccessToken;
use std::fmt::Debug;
use uuid::Uuid;

pub use bytes_stream::*;
pub use context::Context;
pub use errors::*;
pub use headers::AddAsHeader;
pub use http_client::{to_json, HttpClient};
pub use models::*;
pub use request::*;
pub use response::*;
pub use seekable_stream::*;

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
    type Error;
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, Self::Error>;
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
