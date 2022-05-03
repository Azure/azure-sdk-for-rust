//! Azure HTTP headers.
mod utilities;

pub use utilities::*;

use http::request::Builder;
use std::collections::HashMap;

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
#[derive(Clone, Debug)]
pub struct Headers(std::collections::HashMap<HeaderName, HeaderValue>);

impl Headers {
    pub(crate) fn new() -> Self {
        Self(Default::default())
    }

    /// Get a header value given a specific header name
    pub fn get(&self, key: &HeaderName) -> Option<&HeaderValue> {
        self.0.get(key)
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

impl From<http::HeaderMap> for Headers {
    fn from(map: http::HeaderMap) -> Self {
        let map = map
            .into_iter()
            .filter_map(|(k, v)| {
                let key = k?.as_str().to_owned();
                let value = std::str::from_utf8(v.as_bytes())
                    .expect("non-UTF8 header value")
                    .to_owned();
                Some((key.into(), value.into()))
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

#[must_use]
pub fn add_optional_header_ref<T: Header>(item: &Option<&T>, mut builder: Builder) -> Builder {
    if let Some(item) = item {
        builder = builder.header(item.name().as_str(), item.value().as_str())
    }
    builder
}

#[must_use]
pub fn add_optional_header<T: Header>(item: &Option<T>, mut builder: Builder) -> Builder {
    if let Some(item) = item {
        builder = builder.header(item.name().as_str(), item.value().as_str())
    }
    builder
}

#[must_use]
pub fn add_mandatory_header<T: Header>(item: &T, builder: Builder) -> Builder {
    builder.header(item.name().as_str(), item.value().as_str())
}

pub const ACL: &str = "x-ms-acl";
pub const ACCOUNT_KIND: &str = "x-ms-account-kind";
pub const ACTIVITY_ID: &str = "x-ms-activity-id";
pub const APP: &str = "x-ms-app";
pub const APPEND_POSITION: &str = "x-ms-blob-condition-appendpos";
pub const BLOB_ACCESS_TIER: &str = "x-ms-access-tier";
pub const BLOB_CONTENT_LENGTH: &str = "x-ms-blob-content-length";
pub const BLOB_PUBLIC_ACCESS: &str = "x-ms-blob-public-access";
pub const BLOB_SEQUENCE_NUMBER: &str = "x-ms-blob-sequence-number";
pub const BLOB_TYPE: &str = "x-ms-blob-type";
pub const CACHE_CONTROL: &str = "x-ms-blob-cache-control";
pub const CLIENT_REQUEST_ID: &str = "x-ms-client-request-id";
pub const CLIENT_VERSION: &str = "x-ms-client-version";
pub const CONTENT_DISPOSITION: &str = "x-ms-blob-content-disposition";
pub const CONTINUATION: &str = "x-ms-continuation";
pub const COPY_COMPLETION_TIME: &str = "x-ms-copy-completion-time";
pub const COPY_PROGRESS: &str = "x-ms-copy-progress";
pub const COPY_SOURCE: &str = "x-ms-copy-source";
pub const COPY_STATUS: &str = "x-ms-copy-status";
pub const COPY_STATUS_DESCRIPTION: &str = "x-ms-copy-status-description";
pub const CREATION_TIME: &str = "x-ms-creation-time";
pub const DELETE_SNAPSHOTS: &str = "x-ms-delete-snapshots";
pub const DELETE_TYPE_PERMANENT: &str = "x-ms-delete-type-permanent";
pub const HAS_IMMUTABILITY_POLICY: &str = "x-ms-has-immutability-policy";
pub const HAS_LEGAL_HOLD: &str = "x-ms-has-legal-hold";
pub const IF_SEQUENCE_NUMBER_EQ: &str = "x-ms-if-sequence-number-eq";
pub const IF_SEQUENCE_NUMBER_LE: &str = "x-ms-if-sequence-number-le";
pub const IF_SEQUENCE_NUMBER_LT: &str = "x-ms-if-sequence-number-lt";
pub const ITEM_COUNT: &str = "x-ms-item-count";
pub const ITEM_TYPE: &str = "x-ms-item-type";
pub const LEASE_ACTION: &str = "x-ms-lease-action";
pub const LEASE_BREAK_PERIOD: &str = "x-ms-lease-break-period";
pub const LEASE_DURATION: &str = "x-ms-lease-duration";
pub const LEASE_ID: &str = "x-ms-lease-id";
pub const LEASE_STATE: &str = "x-ms-lease-state";
pub const LEASE_STATUS: &str = "x-ms-lease-status";
pub const LEASE_TIME: &str = "x-ms-lease-time";
pub const MAX_ITEM_COUNT: &str = "x-ms-max-item-count";
pub const META_PREFIX: &str = "x-ms-meta-";
pub const MS_DATE: &str = "x-ms-date";
pub const NAMESPACE_ENABLED: &str = "x-ms-namespace-enabled";
pub const PAGE_WRITE: &str = "x-ms-page-write";
pub const PROPERTIES: &str = "x-ms-properties";
pub const PROPOSED_LEASE_ID: &str = "x-ms-proposed-lease-id";
pub const RANGE_GET_CONTENT_MD5: &str = "x-ms-range-get-content-md5";
pub const REQUEST_ID: &str = "x-ms-request-id";
pub const REQUEST_SERVER_ENCRYPTED: &str = "x-ms-request-server-encrypted";
pub const REQUIRES_SYNC: &str = "x-ms-requires-sync";
pub const SERVER_ENCRYPTED: &str = "x-ms-server-encrypted";
pub const SESSION_TOKEN: &str = "x-ms-session-token";
pub const SKU_NAME: &str = "x-ms-sku-name";
pub const SOURCE_IF_MATCH: &str = "x-ms-source-if-match";
pub const SOURCE_IF_MODIFIED_SINCE: &str = "x-ms-source-if-modified-since";
pub const SOURCE_IF_NONE_MATCH: &str = "x-ms-source-if-none-match";
pub const SOURCE_IF_UNMODIFIED_SINCE: &str = "x-ms-source-if-unmodified-since";
pub const SOURCE_LEASE_ID: &str = "x-ms-source-lease-id";
pub const USER: &str = "x-ms-user";
pub const VERSION: &str = "x-ms-version";
