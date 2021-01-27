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

pub mod ba512_range;
mod client_request_id;
mod content_disposition;
mod content_encoding;
mod content_language;
mod content_type;
mod delimiter;
pub mod errors;
pub mod headers;
mod http_client;
mod if_match_condition;
mod if_modified_since_condition;
mod if_source_match_condition;
mod if_source_modified_since_condition;
pub mod incompletevector;
pub mod lease;
mod lease_break_period;
mod lease_duration;
mod max_results;
mod metadata;
mod next_marker;
pub mod parsing;
mod prefix;
pub mod prelude;
mod proposed_lease_id;
pub mod range;
mod sequence_number;
mod sequence_number_condition;
mod source_content_md5;
mod source_lease_id;
mod stored_access_policy;
mod timeout;
pub mod util;

pub use self::stored_access_policy::{StoredAccessPolicy, StoredAccessPolicyList};
use crate::errors::AzureError;
use crate::lease::LeaseId;
use chrono::{DateTime, Utc};
pub use client_request_id::ClientRequestId;
pub use content_disposition::ContentDisposition;
pub use content_encoding::ContentEncoding;
pub use content_language::ContentLanguage;
pub use content_type::ContentType;
pub use delimiter::Delimiter;
use headers::*;
use http::request::Builder;
pub use http_client::*;
use hyper::header::{IF_MODIFIED_SINCE, USER_AGENT};
pub use if_match_condition::IfMatchCondition;
pub use if_modified_since_condition::IfModifiedSinceCondition;
pub use if_source_match_condition::IfSourceMatchCondition;
pub use if_source_modified_since_condition::IfSourceModifiedSinceCondition;
pub use lease_break_period::LeaseBreakPeriod;
pub use lease_duration::LeaseDuration;
pub use max_results::MaxResults;
pub use metadata::Metadata;
pub use next_marker::NextMarker;
use oauth2::AccessToken;
pub use prefix::Prefix;
pub use proposed_lease_id::ProposedLeaseId;
pub use sequence_number::SequenceNumber;
pub use sequence_number_condition::SequenceNumberCondition;
pub use source_content_md5::SourceContentMD5;
pub use source_lease_id::SourceLeaseId;
use std::fmt::Debug;
pub use timeout::Timeout;
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

#[macro_export]
macro_rules! response_from_headers {
    ($cn:ident, $($fh:path => $na:ident: $typ:ty),+) => {
        use http::HeaderMap;

        #[derive(Debug, Clone, PartialEq)]
        pub struct $cn {
             $(pub $na: $typ),+,
        }

        impl $cn {
            pub(crate) fn from_headers(headers: &HeaderMap) -> Result<$cn, $crate::errors::AzureError> {
               $(
                    let $na = $fh(headers)?;
                )+

                Ok($cn {
                    $($na,)+
                })
            }

        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Consistency {
    Md5([u8; 16]),
    Crc64([u8; 8]),
}

#[derive(Debug, Clone, Copy)]
pub struct ActivityId<'a>(&'a str);

impl<'a> ActivityId<'a> {
    pub fn new(id: &'a str) -> Self {
        Self(id)
    }
}

impl<'a> AddAsHeader for ActivityId<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(ACTIVITY_ID, self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Continuation<'a>(&'a str);

impl<'a> Continuation<'a> {
    pub fn new(c: &'a str) -> Self {
        Self(c)
    }
}

impl AddAsHeader for Continuation<'_> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(CONTINUATION, self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UserAgent<'a>(&'a str);

impl<'a> UserAgent<'a> {
    pub fn new(agent: &'a str) -> Self {
        Self(agent)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IfModifiedSince<'a>(&'a DateTime<Utc>);

impl<'a> IfModifiedSince<'a> {
    pub fn new(time: &'a DateTime<Utc>) -> Self {
        Self(time)
    }
}

impl AddAsHeader for IfModifiedSince<'_> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(IF_MODIFIED_SINCE, self.0.to_rfc2822())
    }
}

impl<'a> AddAsHeader for UserAgent<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(USER_AGENT, self.0)
    }
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
