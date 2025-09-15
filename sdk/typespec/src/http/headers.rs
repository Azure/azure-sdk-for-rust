// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP headers.

// cspell:ignore hasher traceparent tracestate

use crate::error::{Error, ErrorKind, ResultExt};
use std::{
    borrow::Cow, collections::HashSet, convert::Infallible, fmt, str::FromStr, sync::LazyLock,
};

/// Default set of allowed headers. Headers not in this list will be redacted.
pub static DEFAULT_ALLOWED_HEADER_NAMES: LazyLock<HashSet<Cow<'static, str>>> =
    LazyLock::new(|| {
        [
            "accept",
            "cache-control",
            "connection",
            "content-length",
            "content-type",
            "date",
            "etag",
            "expires",
            "if-match",
            "if-modified-since",
            "if-none-match",
            "if-unmodified-since",
            "last-modified",
            "ms-cv",
            "pragma",
            "request-id",
            "retry-after",
            "server",
            "traceparent",
            "tracestate",
            "transfer-encoding",
            "user-agent",
            "www-authenticate",
            "x-ms-request-id",
            "x-ms-client-request-id",
            "x-ms-return-client-request-id",
        ]
        .iter()
        .map(|s| Cow::Borrowed(*s))
        .collect()
    });

/// Default pattern for redacted headers or query parameters.
pub const REDACTED_PATTERN: &str = "REDACTED";

/// A trait for converting a type into request headers.
pub trait AsHeaders {
    /// The error type which can occur when converting the type into headers.
    type Error: std::error::Error + Send + Sync + 'static;

    /// The iterator type which yields header name/value pairs.
    type Iter: Iterator<Item = (HeaderName, HeaderValue)>;

    /// Iterate over all the header name/value pairs.
    fn as_headers(&self) -> Result<Self::Iter, Self::Error>;
}

impl<T> AsHeaders for T
where
    T: Header,
{
    type Error = Infallible;
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    /// Iterate over all the header name/value pairs.
    fn as_headers(&self) -> Result<Self::Iter, Self::Error> {
        Ok(vec![(self.name(), self.value())].into_iter())
    }
}

impl<T> AsHeaders for Option<T>
where
    T: AsHeaders<Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>>,
{
    type Error = T::Error;
    type Iter = T::Iter;

    /// Iterate over all the header name/value pairs.
    fn as_headers(&self) -> Result<Self::Iter, T::Error> {
        match self {
            Some(h) => h.as_headers(),
            None => Ok(vec![].into_iter()),
        }
    }
}

/// Extract a value from the [`Headers`] collection.
///
/// The [`FromHeaders::from_headers()`] method is usually used implicitly, through [`Headers::get()`] or [`Headers::get_optional()`].
pub trait FromHeaders: Sized {
    /// The error type which can occur when extracting the value from headers.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Gets a list of the header names that [`FromHeaders::from_headers`] expects.
    ///
    /// Used by [`Headers::get()`] to generate an error if the headers are not present.
    fn header_names() -> &'static [&'static str];

    /// Extracts the value from the provided [`Headers`] collection.
    ///
    /// This method returns one of the following three values:
    /// * `Ok(Some(...))` if the relevant headers are present and could be parsed into the value.
    /// * `Ok(None)` if the relevant headers are not present, so no attempt to parse them can be made.
    /// * `Err(...)` if an error occurred when trying to parse the headers. This likely indicates that the headers are present but invalid.
    fn from_headers(headers: &Headers) -> Result<Option<Self>, Self::Error>;
}

/// View a type as an HTTP header.
///
// Ad interim there are two default functions: `add_to_builder` and `add_to_request`.
//
// While not restricted by the type system, please add HTTP headers only. In particular, do not
// interact with the body of the request.
//
// As soon as the migration to the pipeline architecture will be complete we will phase out
// `add_to_builder`.
pub trait Header {
    /// Get the name of the header.
    fn name(&self) -> HeaderName;
    /// Get the value of the header.
    fn value(&self) -> HeaderValue;
}

/// A collection of headers.
#[derive(Clone, PartialEq, Eq, Default)]
pub struct Headers(std::collections::HashMap<HeaderName, HeaderValue>);

impl Headers {
    /// Create a new headers collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the headers represented by `H`, or return an error if the header is not found.
    pub fn get<H: FromHeaders>(&self) -> crate::Result<H> {
        match H::from_headers(self) {
            Ok(Some(x)) => Ok(x),
            Ok(None) => Err(crate::Error::with_message(
                ErrorKind::DataConversion,
                || {
                    let required_headers = H::header_names();
                    format!(
                        "required header(s) not found: {}",
                        required_headers.join(", ")
                    )
                },
            )),
            Err(e) => Err(crate::Error::new(ErrorKind::DataConversion, e)),
        }
    }

    /// Gets the headers represented by `H`, if they are present.
    ///
    /// This method returns one of the following three values:
    /// * `Ok(Some(...))` if the relevant headers are present and could be parsed into the value.
    /// * `Ok(None)` if the relevant headers are not present, so no attempt to parse them can be made.
    /// * `Err(...)` if an error occurred when trying to parse the headers. This likely indicates that the headers are present but invalid.
    pub fn get_optional<H: FromHeaders>(&self) -> Result<Option<H>, H::Error> {
        H::from_headers(self)
    }

    /// Optionally get a header value as a `String`.
    pub fn get_optional_string(&self, key: &HeaderName) -> Option<String> {
        self.get_as(key).ok()
    }

    /// Get a header value as a `str`, or err if it is not found.
    pub fn get_str(&self, key: &HeaderName) -> crate::Result<&str> {
        self.get_with(key, |s| crate::Result::Ok(s.as_str()))
    }

    /// Optionally get a header value as a `str`.
    pub fn get_optional_str(&self, key: &HeaderName) -> Option<&str> {
        self.get_str(key).ok()
    }

    /// Get a header value parsing it as the type, or err if it's not found or it fails to parse.
    pub fn get_as<V, E>(&self, key: &HeaderName) -> crate::Result<V>
    where
        V: FromStr<Err = E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.get_with(key, |s| s.as_str().parse())
    }

    /// Optionally get a header value parsing it as the type, or err if it fails to parse.
    pub fn get_optional_as<V, E>(&self, key: &HeaderName) -> crate::Result<Option<V>>
    where
        V: FromStr<Err = E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.get_optional_with(key, |s| s.as_str().parse())
    }

    /// Get a header value using the parser, or err if it is not found or fails to parse.
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

    /// Optionally get a header value using the parser, or err if it fails to parse.
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

    /// Insert a header name/value pair.
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        self.0.insert(key.into(), value.into());
    }

    /// Add headers to the headers collection.
    ///
    /// ## Errors
    ///
    /// The error this returns depends on the type `H`.
    /// Many header types are infallible, return a `Result` with [`Infallible`] as the error type.
    /// In this case, you can safely `.unwrap()` the value without risking a panic.
    pub fn add<H>(&mut self, header: H) -> Result<(), H::Error>
    where
        H: AsHeaders,
    {
        for (key, value) in header.as_headers()? {
            self.insert(key, value);
        }
        Ok(())
    }

    /// Iterate over all the header name/value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&HeaderName, &HeaderValue)> {
        self.0.iter()
    }

    /// Remove a header by name, returning the previous value if present.
    pub fn remove<K>(&mut self, key: K) -> Option<HeaderValue>
    where
        K: Into<HeaderName>,
    {
        self.0.remove(&key.into())
    }
}

impl fmt::Debug for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Sanitize all but safe headers.
        f.debug_map()
            .entries(self.0.iter().map(|(k, v)| {
                (
                    k.as_str(),
                    if DEFAULT_ALLOWED_HEADER_NAMES.contains(k.as_str()) {
                        v.as_str()
                    } else {
                        REDACTED_PATTERN
                    },
                )
            }))
            .finish()
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

/// A header name.
#[derive(Clone, Debug, Eq, PartialOrd, Ord)]
pub struct HeaderName {
    /// Name of the header.
    name: Cow<'static, str>,

    /// Marker indicating if the header is a standard header or not.
    /// Note that this field is not part of equality or hashing.
    pub(crate) is_standard: bool,
}

impl HeaderName {
    /// Create a header name from a static `str`.
    pub const fn from_static(s: &'static str) -> Self {
        ensure_no_uppercase(s);
        Self {
            name: Cow::Borrowed(s),
            is_standard: false,
        }
    }

    /// Create a header name from a static `str`.
    pub const fn from_static_standard(s: &'static str) -> Self {
        ensure_no_uppercase(s);
        Self {
            name: Cow::Borrowed(s),
            is_standard: true,
        }
    }

    fn from_cow<C>(c: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        let c = c.into();
        assert!(
            c.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()),
            "header names must be lowercase: {c}"
        );
        Self {
            name: c,
            is_standard: false,
        }
    }

    /// Get a header name as a `str`.
    pub fn as_str(&self) -> &str {
        self.name.as_ref()
    }

    /// Get whether the header was defined as a standard HTTP header.
    pub fn is_standard(&self) -> bool {
        self.is_standard
    }
}

impl PartialEq for HeaderName {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq_ignore_ascii_case(&other.name)
    }
}

impl PartialEq<&str> for HeaderName {
    fn eq(&self, other: &&str) -> bool {
        self.name.eq_ignore_ascii_case(other)
    }
}

impl std::hash::Hash for HeaderName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Keep hashing consistent with PartialEq: include the case-insensitive name.
        std::hash::Hash::hash(&self.name, state);
    }
}

/// Ensures the supplied string does not contain any uppercase ascii characters
const fn ensure_no_uppercase(s: &str) {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        assert!(
            !(byte >= 65u8 && byte <= 90u8),
            "header names must not contain uppercase letters"
        );
        i += 1;
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

/// A header value.
#[derive(Clone, PartialEq, Eq)]
pub struct HeaderValue(Cow<'static, str>);

impl HeaderValue {
    /// Create a header value from a static `str`.
    pub const fn from_static(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }

    /// Create a header value from a [`Cow`].
    pub fn from_cow<C>(c: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self(c.into())
    }

    /// Get a header value as a `str`.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Debug for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("HeaderValue")
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

#[cfg(test)]
mod tests {
    use crate::error::ErrorKind;
    use url::Url;

    use super::{FromHeaders, HeaderName, Headers};

    // Just in case we add a ContentLocation struct later, this one is named "ForTest" to indicate it's just here for this test.
    #[derive(Debug)]
    struct ContentLocationForTest(Url);

    impl FromHeaders for ContentLocationForTest {
        type Error = url::ParseError;

        fn header_names() -> &'static [&'static str] {
            &["content-location"]
        }

        fn from_headers(headers: &super::Headers) -> Result<Option<Self>, Self::Error> {
            let Some(loc) = headers.get_optional_str(&HeaderName::from("content-location")) else {
                return Ok(None);
            };

            Ok(Some(ContentLocationForTest(loc.parse()?)))
        }
    }

    #[test]
    pub fn headers_get_optional_returns_ok_some_if_header_present_and_valid() {
        let mut headers = Headers::new();
        headers.insert("content-location", "https://example.com");
        let content_location: ContentLocationForTest = headers.get_optional().unwrap().unwrap();
        assert_eq!("https://example.com/", content_location.0.as_str())
    }

    #[test]
    pub fn headers_get_optional_returns_ok_none_if_header_not_present() {
        let headers = Headers::new();
        let content_location: Option<ContentLocationForTest> = headers.get_optional().unwrap();
        assert!(content_location.is_none())
    }

    #[test]
    pub fn headers_get_optional_returns_err_if_conversion_fails() {
        let mut headers = Headers::new();
        headers.insert("content-location", "not a URL");
        let err = headers
            .get_optional::<ContentLocationForTest>()
            .unwrap_err();
        assert_eq!(url::ParseError::RelativeUrlWithoutBase, err)
    }

    #[test]
    pub fn headers_get_returns_ok_if_header_present_and_valid() {
        let mut headers = Headers::new();
        headers.insert("content-location", "https://example.com");
        let content_location: ContentLocationForTest = headers.get().unwrap();
        assert_eq!("https://example.com/", content_location.0.as_str())
    }

    #[test]
    pub fn headers_get_returns_err_if_header_not_present() {
        let headers = Headers::new();
        let err = headers.get::<ContentLocationForTest>().unwrap_err();
        assert_eq!(&ErrorKind::DataConversion, err.kind());

        // The "Display" implementation is the canonical way to get an error's "message"
        assert_eq!(
            "required header(s) not found: content-location",
            format!("{}", err)
        );
    }

    #[test]
    pub fn headers_get_returns_err_if_header_requiring_multiple_headers_not_present() {
        #[derive(Debug)]
        struct HasTwoHeaders;

        impl FromHeaders for HasTwoHeaders {
            type Error = std::convert::Infallible;

            fn header_names() -> &'static [&'static str] {
                &["header-a", "header-b"]
            }

            fn from_headers(_: &Headers) -> Result<Option<Self>, Self::Error> {
                Ok(None)
            }
        }

        let headers = Headers::new();
        let err = headers.get::<HasTwoHeaders>().unwrap_err();
        assert_eq!(&ErrorKind::DataConversion, err.kind());

        // The "Display" implementation is the canonical way to get an error's "message"
        assert_eq!(
            "required header(s) not found: header-a, header-b",
            format!("{}", err)
        );
    }

    #[test]
    pub fn headers_get_returns_err_if_conversion_fails() {
        let mut headers = Headers::new();
        headers.insert("content-location", "not a URL");
        let err = headers.get::<ContentLocationForTest>().unwrap_err();
        assert_eq!(&ErrorKind::DataConversion, err.kind());
        let inner: Box<url::ParseError> = err.into_inner().unwrap().downcast().unwrap();
        assert_eq!(Box::new(url::ParseError::RelativeUrlWithoutBase), inner)
    }

    #[test]
    pub fn headers_remove_existing_header_returns_value() {
        let mut headers = Headers::new();
        headers.insert("test-header", "test-value");

        // Verify the header is present
        assert_eq!(
            headers.get_optional_str(&HeaderName::from("test-header")),
            Some("test-value")
        );

        // Remove the header and verify it returns the previous value
        let removed_value = headers.remove("test-header");
        assert!(removed_value.is_some());
        assert_eq!(removed_value.unwrap().as_str(), "test-value");

        // Verify the header is no longer present
        assert_eq!(
            headers.get_optional_str(&HeaderName::from("test-header")),
            None
        );
    }

    #[test]
    pub fn headers_remove_nonexistent_header_returns_none() {
        let mut headers = Headers::new();

        // Try to remove a header that doesn't exist
        let removed_value = headers.remove("nonexistent-header");
        assert_eq!(removed_value, None);
    }

    #[test]
    pub fn headers_remove_works_with_different_key_types() {
        let mut headers = Headers::new();
        headers.insert("test-header", "test-value");

        // Test removing with &str
        let removed_value = headers.remove("test-header");
        assert!(removed_value.is_some());
        assert_eq!(removed_value.unwrap().as_str(), "test-value");

        // Re-add the header
        headers.insert("test-header", "test-value");

        // Test removing with HeaderName
        let removed_value = headers.remove(HeaderName::from("test-header"));
        assert!(removed_value.is_some());
        assert_eq!(removed_value.unwrap().as_str(), "test-value");

        // Re-add the header
        headers.insert("test-header", "test-value");

        // Test removing with String
        let removed_value = headers.remove("test-header".to_string());
        assert!(removed_value.is_some());
        assert_eq!(removed_value.unwrap().as_str(), "test-value");
    }
}
