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

pub mod errors;
pub mod headers;
mod http_client;
pub mod incompletevector;
pub mod lease;
pub mod parsing;
pub mod prelude;
mod request_options;
mod stored_access_policy;
pub mod util;

pub use self::stored_access_policy::{StoredAccessPolicy, StoredAccessPolicyList};
use crate::errors::AzureError;
use chrono::{DateTime, Utc};
use headers::*;
use http::request::Builder;
pub use http_client::*;
use oauth2::AccessToken;
use std::fmt::Debug;
use uuid::Uuid;

pub type RequestId = Uuid;
pub type SessionToken = String;

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
pub trait AddAsHeader {
    fn add_as_header(&self, builder: Builder) -> Builder;
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
