//! Azure HTTP headers.
mod utilities;

use crate::error::{Error, ErrorKind, ResultExt};
use std::{fmt::Debug, str::FromStr};
pub use utilities::*;

/// A trait for converting a type into request headers
pub trait AsHeaders {
    type Iter: Iterator<Item = (HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Self::Iter;
}

impl<T> AsHeaders for T
where
    T: Header,
{
    type Iter = std::option::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Self::Iter {
        Some((self.name(), self.value())).into_iter()
    }
}

impl<T> AsHeaders for Option<T>
where
    T: Header,
{
    type Iter = std::option::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Self::Iter {
        match self {
            Some(h) => h.as_headers(),
            None => None.into_iter(),
        }
    }
}

/// View a type as an HTTP header.
///
/// Ad interim there are two default functions: `add_to_builder` and `add_to_request`.
///
/// While not restricted by the type system, please add HTTP headers only. In particular, do not
/// interact with the body of the request.
///
/// As soon as the migration to the pipeline architecture will be complete we will phase out
/// `add_to_builder`.
pub trait Header {
    fn name(&self) -> HeaderName;
    fn value(&self) -> HeaderValue;
}

/// A collection of headers
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Headers(std::collections::HashMap<HeaderName, HeaderValue>);

impl Headers {
    pub fn new() -> Self {
        Self::default()
    }

    /// Optionally get a header value as a String
    pub fn get_optional_string(&self, key: &HeaderName) -> Option<String> {
        self.get_as(key).ok()
    }

    /// Get a header value as a str or error if it is not found
    pub fn get_str(&self, key: &HeaderName) -> crate::Result<&str> {
        self.get_with(key, |s| crate::Result::Ok(s.as_str()))
    }

    /// Optionally get a header value as a str
    pub fn get_optional_str(&self, key: &HeaderName) -> Option<&str> {
        self.get_str(key).ok()
    }

    /// Get a header value parsing it as the type or error if it's not found or it fails to parse
    pub fn get_as<V, E>(&self, key: &HeaderName) -> crate::Result<V>
    where
        V: FromStr<Err = E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.get_with(key, |s| s.as_str().parse())
    }

    /// Optionally get a header value parsing it as the type or error if it fails to parse
    pub fn get_optional_as<V, E>(&self, key: &HeaderName) -> crate::Result<Option<V>>
    where
        V: FromStr<Err = E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.get_optional_with(key, |s| s.as_str().parse())
    }

    /// Get a header value using the parser or error if it is not found or fails to parse
    pub fn get_with<'a, V, F, E>(&'a self, key: &HeaderName, parser: F) -> crate::Result<V>
    where
        F: FnOnce(&'a HeaderValue) -> Result<V, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.get_optional_with(key, parser)?.ok_or_else(|| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("header not found {}", key.as_str())
            })
        })
    }

    /// Optionally get a header value using the parser or error if it fails to parse
    pub fn get_optional_with<'a, V, F, E>(
        &'a self,
        key: &HeaderName,
        parser: F,
    ) -> crate::Result<Option<V>>
    where
        F: FnOnce(&'a HeaderValue) -> Result<V, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.0
            .get(key)
            .map(|v: &HeaderValue| {
                parser(v).with_context(ErrorKind::DataConversion, || {
                    let ty = std::any::type_name::<V>();
                    format!("unable to parse header '{key:?}: {v:?}' into {ty}",)
                })
            })
            .transpose()
    }

    /// Insert a header name/value pair
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        self.0.insert(key.into(), value.into());
    }

    /// Add headers to the headers collection
    pub fn add<H>(&mut self, header: H)
    where
        H: AsHeaders,
    {
        for (key, value) in header.as_headers() {
            self.insert(key, value);
        }
    }

    /// Iterate over all the header name/value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&HeaderName, &HeaderValue)> {
        self.0.iter()
    }
}

impl IntoIterator for Headers {
    type Item = (HeaderName, HeaderValue);

    type IntoIter = std::collections::hash_map::IntoIter<HeaderName, HeaderValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<std::collections::HashMap<HeaderName, HeaderValue>> for Headers {
    fn from(c: std::collections::HashMap<HeaderName, HeaderValue>) -> Self {
        Self(c)
    }
}

/// A header name
#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HeaderName(std::borrow::Cow<'static, str>);

impl HeaderName {
    pub const fn from_static(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }

    fn from_cow<C>(c: C) -> Self
    where
        C: Into<std::borrow::Cow<'static, str>>,
    {
        let c = c.into();
        assert!(
            c.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()),
            "header names must be lowercase: {c}"
        );
        Self(c)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&'static str> for HeaderName {
    fn from(s: &'static str) -> Self {
        Self::from_cow(s)
    }
}

impl From<String> for HeaderName {
    fn from(s: String) -> Self {
        Self::from_cow(s.to_lowercase())
    }
}

/// A header value
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeaderValue(std::borrow::Cow<'static, str>);

impl HeaderValue {
    pub const fn from_static(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }

    pub fn from_cow<C>(c: C) -> Self
    where
        C: Into<std::borrow::Cow<'static, str>>,
    {
        Self(c.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&'static str> for HeaderValue {
    fn from(s: &'static str) -> Self {
        Self::from_cow(s)
    }
}

impl From<String> for HeaderValue {
    fn from(s: String) -> Self {
        Self::from_cow(s)
    }
}

impl From<&String> for HeaderValue {
    fn from(s: &String) -> Self {
        s.clone().into()
    }
}

// headers are case insensitive
// we are using all lowercase values
// same as https://github.com/hyperium/http/blob/master/util/src/main.rs

pub const ACCEPT: HeaderName = HeaderName::from_static("accept");
pub const ACCEPT_ENCODING: HeaderName = HeaderName::from_static("accept-encoding");
pub const ACL: HeaderName = HeaderName::from_static("x-ms-acl");
pub const ACCOUNT_KIND: HeaderName = HeaderName::from_static("x-ms-account-kind");
pub const ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
pub const APP: HeaderName = HeaderName::from_static("x-ms-app");
pub const AUTHORIZATION: HeaderName = HeaderName::from_static("authorization");
pub const APPEND_POSITION: HeaderName = HeaderName::from_static("x-ms-blob-condition-appendpos");
pub const BLOB_ACCESS_TIER: HeaderName = HeaderName::from_static("x-ms-access-tier");
pub const BLOB_CONTENT_LENGTH: HeaderName = HeaderName::from_static("x-ms-blob-content-length");
pub const BLOB_PUBLIC_ACCESS: HeaderName = HeaderName::from_static("x-ms-blob-public-access");
pub const BLOB_SEQUENCE_NUMBER: HeaderName = HeaderName::from_static("x-ms-blob-sequence-number");
pub const BLOB_TYPE: HeaderName = HeaderName::from_static("x-ms-blob-type");
pub const BLOB_CACHE_CONTROL: HeaderName = HeaderName::from_static("x-ms-blob-cache-control");
pub const CACHE_CONTROL: HeaderName = HeaderName::from_static("cache-control");
pub const CLIENT_REQUEST_ID: HeaderName = HeaderName::from_static("x-ms-client-request-id");
pub const CLIENT_VERSION: HeaderName = HeaderName::from_static("x-ms-client-version");
pub const CONTENT_DISPOSITION: HeaderName =
    HeaderName::from_static("x-ms-blob-content-disposition");
pub const CONTENT_ENCODING: HeaderName = HeaderName::from_static("content-encoding");
pub const CONTENT_LANGUAGE: HeaderName = HeaderName::from_static("content-language");
pub const CONTENT_LENGTH: HeaderName = HeaderName::from_static("content-length");
pub const CONTENT_LOCATION: HeaderName = HeaderName::from_static("content-location");
pub const CONTENT_MD5: HeaderName = HeaderName::from_static("content-md5");
pub const CONTENT_RANGE: HeaderName = HeaderName::from_static("content-range");
pub const CONTENT_SECURITY_POLICY: HeaderName = HeaderName::from_static("content-security-policy");
pub const CONTENT_TYPE: HeaderName = HeaderName::from_static("content-type");
pub const CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
pub const COPY_COMPLETION_TIME: HeaderName = HeaderName::from_static("x-ms-copy-completion-time");
pub const COPY_PROGRESS: HeaderName = HeaderName::from_static("x-ms-copy-progress");
pub const COPY_SOURCE: HeaderName = HeaderName::from_static("x-ms-copy-source");
pub const COPY_STATUS: HeaderName = HeaderName::from_static("x-ms-copy-status");
pub const COPY_STATUS_DESCRIPTION: HeaderName =
    HeaderName::from_static("x-ms-copy-status-description");
pub const CREATION_TIME: HeaderName = HeaderName::from_static("x-ms-creation-time");
pub const DATE: HeaderName = HeaderName::from_static("date");
pub const DELETE_SNAPSHOTS: HeaderName = HeaderName::from_static("x-ms-delete-snapshots");
pub const DELETE_TYPE_PERMANENT: HeaderName = HeaderName::from_static("x-ms-delete-type-permanent");
pub const ETAG: HeaderName = HeaderName::from_static("etag");
pub const ERROR_CODE: HeaderName = HeaderName::from_static("x-ms-error-code");
pub const HAS_IMMUTABILITY_POLICY: HeaderName =
    HeaderName::from_static("x-ms-has-immutability-policy");
pub const HAS_LEGAL_HOLD: HeaderName = HeaderName::from_static("x-ms-has-legal-hold");
pub const IF_MATCH: HeaderName = HeaderName::from_static("if-match");
pub const IF_MODIFIED_SINCE: HeaderName = HeaderName::from_static("if-modified-since");
pub const IF_NONE_MATCH: HeaderName = HeaderName::from_static("if-none-match");
pub const IF_RANGE: HeaderName = HeaderName::from_static("if-range");
pub const IF_UNMODIFIED_SINCE: HeaderName = HeaderName::from_static("if-unmodified-since");
pub const IF_SEQUENCE_NUMBER_EQ: HeaderName = HeaderName::from_static("x-ms-if-sequence-number-eq");
pub const IF_SEQUENCE_NUMBER_LE: HeaderName = HeaderName::from_static("x-ms-if-sequence-number-le");
pub const IF_SEQUENCE_NUMBER_LT: HeaderName = HeaderName::from_static("x-ms-if-sequence-number-lt");
pub const IF_TAGS: HeaderName = HeaderName::from_static("x-ms-if-tags");
pub const ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-item-count");
pub const ITEM_TYPE: HeaderName = HeaderName::from_static("x-ms-item-type");
pub const KEEP_ALIVE: HeaderName = HeaderName::from_static("keep-alive");
pub const LAST_MODIFIED: HeaderName = HeaderName::from_static("last-modified");
pub const LEASE_ACTION: HeaderName = HeaderName::from_static("x-ms-lease-action");
pub const LEASE_BREAK_PERIOD: HeaderName = HeaderName::from_static("x-ms-lease-break-period");
pub const LEASE_DURATION: HeaderName = HeaderName::from_static("x-ms-lease-duration");
pub const LEASE_ID: HeaderName = HeaderName::from_static("x-ms-lease-id");
pub const LEASE_STATE: HeaderName = HeaderName::from_static("x-ms-lease-state");
pub const LEASE_STATUS: HeaderName = HeaderName::from_static("x-ms-lease-status");
pub const LEASE_TIME: HeaderName = HeaderName::from_static("x-ms-lease-time");
pub const LINK: HeaderName = HeaderName::from_static("link");
pub const LOCATION: HeaderName = HeaderName::from_static("location");
pub const MAX_ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-max-item-count");
pub const META_PREFIX: HeaderName = HeaderName::from_static("x-ms-meta-");
pub const MS_DATE: HeaderName = HeaderName::from_static("x-ms-date");
pub const MS_RANGE: HeaderName = HeaderName::from_static("x-ms-range");
pub const NAMESPACE_ENABLED: HeaderName = HeaderName::from_static("x-ms-namespace-enabled");
pub const PAGE_WRITE: HeaderName = HeaderName::from_static("x-ms-page-write");
pub const PROPERTIES: HeaderName = HeaderName::from_static("x-ms-properties");
pub const PREFER: HeaderName = HeaderName::from_static("prefer");
pub const PROPOSED_LEASE_ID: HeaderName = HeaderName::from_static("x-ms-proposed-lease-id");
pub const RANGE: HeaderName = HeaderName::from_static("range");
pub const RANGE_GET_CONTENT_CRC64: HeaderName =
    HeaderName::from_static("x-ms-range-get-content-crc64");
pub const RANGE_GET_CONTENT_MD5: HeaderName = HeaderName::from_static("x-ms-range-get-content-md5");
pub const REQUEST_ID: HeaderName = HeaderName::from_static("x-ms-request-id");
pub const REQUEST_SERVER_ENCRYPTED: HeaderName =
    HeaderName::from_static("x-ms-request-server-encrypted");
pub const REQUIRES_SYNC: HeaderName = HeaderName::from_static("x-ms-requires-sync");
pub const RETRY_AFTER: HeaderName = HeaderName::from_static("retry-after");
pub const SERVER: HeaderName = HeaderName::from_static("server");
pub const SERVER_ENCRYPTED: HeaderName = HeaderName::from_static("x-ms-server-encrypted");
pub const SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
pub const SKU_NAME: HeaderName = HeaderName::from_static("x-ms-sku-name");
pub const SOURCE_IF_MATCH: HeaderName = HeaderName::from_static("x-ms-source-if-match");
pub const SOURCE_IF_MODIFIED_SINCE: HeaderName =
    HeaderName::from_static("x-ms-source-if-modified-since");
pub const SOURCE_IF_NONE_MATCH: HeaderName = HeaderName::from_static("x-ms-source-if-none-match");
pub const SOURCE_IF_UNMODIFIED_SINCE: HeaderName =
    HeaderName::from_static("x-ms-source-if-unmodified-since");
pub const SOURCE_LEASE_ID: HeaderName = HeaderName::from_static("x-ms-source-lease-id");
pub const TAGS: HeaderName = HeaderName::from_static("x-ms-tags");
pub const USER: HeaderName = HeaderName::from_static("x-ms-user");
pub const USER_AGENT: HeaderName = HeaderName::from_static("user-agent");
pub const VERSION: HeaderName = HeaderName::from_static("x-ms-version");
pub const WWW_AUTHENTICATE: HeaderName = HeaderName::from_static("www-authenticate");
