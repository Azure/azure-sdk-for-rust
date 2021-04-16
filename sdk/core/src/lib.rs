#![recursion_limit = "256"]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;

mod context;
pub mod errors;
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
mod sleep;
pub mod util;

use chrono::{DateTime, Utc};
use errors::AzureError;
use headers::*;
use oauth2::AccessToken;
use std::fmt::Debug;
use uuid::Uuid;

pub use context::Context;
pub use headers::AddAsHeader;
pub use http_client::{to_json, HttpClient};
pub use models::*;
pub use request::*;
pub use response::*;

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
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, AzureError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Consistency {
    Md5([u8; 16]),
    Crc64([u8; 8]),
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
