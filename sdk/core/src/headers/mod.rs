//! Azure HTTP headers.
mod utilities;

use crate::error::{Error, ErrorKind, ResultExt};
use std::{collections::HashMap, fmt::Debug, str::FromStr};
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
#[derive(Clone, Debug, PartialEq)]
pub struct Headers(std::collections::HashMap<HeaderName, HeaderValue>);

impl Headers {
    pub(crate) fn new() -> Self {
        Self(Default::default())
    }

    /// Get a header value given a specific header name
    pub fn get<T: Into<HeaderName>>(&self, key: T) -> Option<&HeaderValue> {
        self.0.get(&key.into())
    }

    /// Get a header value or error if it is not found
    pub fn get_or_err<T: Into<HeaderName>>(&self, key: T) -> crate::Result<&HeaderValue> {
        let key: &HeaderName = &key.into();
        let value = self.0.get(key);
        value.ok_or_else(|| {
            Error::with_message(ErrorKind::Other, || {
                format!("header not found {}", key.as_str())
            })
        })
    }

    /// Get a header value as a str
    pub fn get_as_str<T: Into<HeaderName>>(&self, key: T) -> Option<&str> {
        self.get(key).map(|v| v.as_str())
    }

    /// Get a header value as a str or error if it is not found
    pub fn get_as_str_or_err<T: Into<HeaderName>>(&self, key: T) -> crate::Result<&str> {
        self.get_or_err(key).map(|v| v.as_str())
    }

    /// Get a header value as a String
    pub fn get_as_string<T: Into<HeaderName>>(&self, key: T) -> Option<String> {
        self.get(key).map(|v| v.as_str().to_string())
    }

    /// Get a header value as a String or error if it is not found
    pub fn get_as_string_or_err<T: Into<HeaderName>>(&self, key: T) -> crate::Result<String> {
        self.get_or_err(key).map(|v| v.as_str().to_string())
    }

    /// Get a header value as a u64
    pub fn get_as_u64<T: Into<HeaderName>>(&self, key: T) -> crate::Result<Option<u64>> {
        let key = key.into();
        self.get(key.clone())
            .map(|v: &HeaderValue| {
                let v = v.as_str();
                v.parse::<u64>()
                    .with_context(ErrorKind::DataConversion, || {
                        format!("unable to parse header into u64 {key:?}: {v}")
                    })
            })
            .transpose()
    }

    /// Get a header value as a u64 or error if it is not found
    pub fn get_as_u64_or_err<T: Into<HeaderName>>(&self, key: T) -> crate::Result<u64> {
        let key = key.into();
        let v = self.get_or_err(key.clone())?;
        let v = v.as_str();
        v.parse::<u64>()
            .with_context(ErrorKind::DataConversion, || {
                format!("unable to parse header into u64 {key:?}: {v}")
            })
    }

    pub fn get_as_enum<T: Into<HeaderName>, V: FromStr<Err = E>, E>(
        &self,
        key: T,
    ) -> crate::Result<Option<V>>
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        let key = key.into();
        self.get(key.clone())
            .map(|v: &HeaderValue| {
                let v = v.as_str();
                v.parse::<V>().with_context(ErrorKind::DataConversion, || {
                    format!("unable to parse header into enum {key:?}: {v}")
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

impl From<&http::HeaderMap> for Headers {
    fn from(map: &http::HeaderMap) -> Self {
        let map = map
            .into_iter()
            .map(|(k, v)| {
                let key = k.as_str().to_owned();
                let value = std::str::from_utf8(v.as_bytes())
                    .expect("non-UTF8 header value")
                    .to_owned();
                (key.into(), value.into())
            })
            .collect::<HashMap<HeaderName, HeaderValue>>();
        Self(map)
    }
}

/// A header name
#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HeaderName(std::borrow::Cow<'static, str>);

impl HeaderName {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<http::header::HeaderName> for HeaderName {
    fn from(n: http::header::HeaderName) -> Self {
        Self(std::borrow::Cow::Owned(n.as_str().into()))
    }
}

impl From<&'static str> for HeaderName {
    fn from(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }
}

impl From<String> for HeaderName {
    fn from(s: String) -> Self {
        Self(std::borrow::Cow::Owned(s))
    }
}

impl From<&HeaderName> for http::header::HeaderName {
    fn from(n: &HeaderName) -> Self {
        http::header::HeaderName::from_bytes(n.as_str().as_bytes()).unwrap()
    }
}

/// A header value
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeaderValue(std::borrow::Cow<'static, str>);

impl HeaderValue {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<http::header::HeaderValue> for HeaderValue {
    fn from(n: http::header::HeaderValue) -> Self {
        Self(std::borrow::Cow::Owned(
            n.to_str().expect("non-UTF8 header value").to_owned(),
        ))
    }
}

impl From<&'static str> for HeaderValue {
    fn from(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }
}

impl From<String> for HeaderValue {
    fn from(s: String) -> Self {
        Self(std::borrow::Cow::Owned(s))
    }
}

impl From<&String> for HeaderValue {
    fn from(s: &String) -> Self {
        Self(std::borrow::Cow::Owned(s.clone()))
    }
}

impl From<&HeaderValue> for http::header::HeaderValue {
    fn from(n: &HeaderValue) -> Self {
        http::header::HeaderValue::from_bytes(n.as_str().as_bytes()).unwrap()
    }
}

// headers are case insensitive
// we are using all lowercase values
// same as https://github.com/hyperium/http/blob/master/util/src/main.rs

pub const ACL: &str = "x-ms-acl";
pub const ACCOUNT_KIND: &str = "x-ms-account-kind";
pub const ACTIVITY_ID: &str = "x-ms-activity-id";
pub const APP: &str = "x-ms-app";
pub const AUTHORIZATION: &str = "authorization";
pub const APPEND_POSITION: &str = "x-ms-blob-condition-appendpos";
pub const BLOB_ACCESS_TIER: &str = "x-ms-access-tier";
pub const BLOB_CONTENT_LENGTH: &str = "x-ms-blob-content-length";
pub const BLOB_PUBLIC_ACCESS: &str = "x-ms-blob-public-access";
pub const BLOB_SEQUENCE_NUMBER: &str = "x-ms-blob-sequence-number";
pub const BLOB_TYPE: &str = "x-ms-blob-type";
pub const BLOB_CACHE_CONTROL: &str = "x-ms-blob-cache-control";
pub const CACHE_CONTROL: &str = "cache-control";
pub const CLIENT_REQUEST_ID: &str = "x-ms-client-request-id";
pub const CLIENT_VERSION: &str = "x-ms-client-version";
pub const CONTENT_DISPOSITION: &str = "x-ms-blob-content-disposition";
pub const CONTENT_ENCODING: &str = "content-encoding";
pub const CONTENT_LANGUAGE: &str = "content-language";
pub const CONTENT_LENGTH: &str = "content-length";
pub const CONTENT_LOCATION: &str = "content-location";
pub const CONTENT_MD5: &str = "content-md5";
pub const CONTENT_RANGE: &str = "content-range";
pub const CONTENT_SECURITY_POLICY: &str = "content-security-policy";
pub const CONTENT_TYPE: &str = "content-type";
pub const CONTINUATION: &str = "x-ms-continuation";
pub const COPY_COMPLETION_TIME: &str = "x-ms-copy-completion-time";
pub const COPY_PROGRESS: &str = "x-ms-copy-progress";
pub const COPY_SOURCE: &str = "x-ms-copy-source";
pub const COPY_STATUS: &str = "x-ms-copy-status";
pub const COPY_STATUS_DESCRIPTION: &str = "x-ms-copy-status-description";
pub const CREATION_TIME: &str = "x-ms-creation-time";
pub const DATE: &str = "date";
pub const DELETE_SNAPSHOTS: &str = "x-ms-delete-snapshots";
pub const DELETE_TYPE_PERMANENT: &str = "x-ms-delete-type-permanent";
pub const ETAG: &str = "etag";
pub const HAS_IMMUTABILITY_POLICY: &str = "x-ms-has-immutability-policy";
pub const HAS_LEGAL_HOLD: &str = "x-ms-has-legal-hold";
pub const IF_MATCH: &str = "if-match";
pub const IF_MODIFIED_SINCE: &str = "if-modified-since";
pub const IF_NONE_MATCH: &str = "if-none-match";
pub const IF_RANGE: &str = "if-range";
pub const IF_UNMODIFIED_SINCE: &str = "if-unmodified-since";
pub const IF_SEQUENCE_NUMBER_EQ: &str = "x-ms-if-sequence-number-eq";
pub const IF_SEQUENCE_NUMBER_LE: &str = "x-ms-if-sequence-number-le";
pub const IF_SEQUENCE_NUMBER_LT: &str = "x-ms-if-sequence-number-lt";
pub const ITEM_COUNT: &str = "x-ms-item-count";
pub const ITEM_TYPE: &str = "x-ms-item-type";
pub const KEEP_ALIVE: &str = "keep-alive";
pub const LAST_MODIFIED: &str = "last-modified";
pub const LEASE_ACTION: &str = "x-ms-lease-action";
pub const LEASE_BREAK_PERIOD: &str = "x-ms-lease-break-period";
pub const LEASE_DURATION: &str = "x-ms-lease-duration";
pub const LEASE_ID: &str = "x-ms-lease-id";
pub const LEASE_STATE: &str = "x-ms-lease-state";
pub const LEASE_STATUS: &str = "x-ms-lease-status";
pub const LEASE_TIME: &str = "x-ms-lease-time";
pub const LINK: &str = "link";
pub const LOCATION: &str = "location";
pub const MAX_ITEM_COUNT: &str = "x-ms-max-item-count";
pub const META_PREFIX: &str = "x-ms-meta-";
pub const MS_DATE: &str = "x-ms-date";
pub const NAMESPACE_ENABLED: &str = "x-ms-namespace-enabled";
pub const PAGE_WRITE: &str = "x-ms-page-write";
pub const PROPERTIES: &str = "x-ms-properties";
pub const PROPOSED_LEASE_ID: &str = "x-ms-proposed-lease-id";
pub const RANGE: &str = "range";
pub const RANGE_GET_CONTENT_MD5: &str = "x-ms-range-get-content-md5";
pub const REQUEST_ID: &str = "x-ms-request-id";
pub const REQUEST_SERVER_ENCRYPTED: &str = "x-ms-request-server-encrypted";
pub const REQUIRES_SYNC: &str = "x-ms-requires-sync";
pub const RETRY_AFTER: &str = "retry-after";
pub const SERVER_ENCRYPTED: &str = "x-ms-server-encrypted";
pub const SESSION_TOKEN: &str = "x-ms-session-token";
pub const SKU_NAME: &str = "x-ms-sku-name";
pub const SOURCE_IF_MATCH: &str = "x-ms-source-if-match";
pub const SOURCE_IF_MODIFIED_SINCE: &str = "x-ms-source-if-modified-since";
pub const SOURCE_IF_NONE_MATCH: &str = "x-ms-source-if-none-match";
pub const SOURCE_IF_UNMODIFIED_SINCE: &str = "x-ms-source-if-unmodified-since";
pub const SOURCE_LEASE_ID: &str = "x-ms-source-lease-id";
pub const USER: &str = "x-ms-user";
pub const USER_AGENT: &str = "user-agent";
pub const VERSION: &str = "x-ms-version";
pub const WWW_AUTHENTICATE: &str = "www-authenticate";
