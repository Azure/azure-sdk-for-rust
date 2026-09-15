# typespec

- **Description**: Project root for all TypeSpec-related crates.
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `http`
  - `json`
- `http`
- `json`
- `xml`

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
pub use bytes::Bytes;
pub use typespec::error::Error;
pub use typespec::error::Result;
pub mod error {
    pub struct Error {
    }
    impl Error {
        pub fn downcast_mut<T: std::error::Error + 'static>(&mut self) -> Option<&mut T>;
        pub fn downcast_ref<T: std::error::Error + 'static>(&self) -> Option<&T>;
        pub fn get_mut(&mut self) -> Option<&mut dyn std::error::Error + Send + Sync + 'static>;
        pub fn get_ref(&self) -> Option<&dyn std::error::Error + Send + Sync + 'static>;
        #[cfg(feature = "http")]
        pub fn http_status(&self) -> Option<StatusCode>;
        pub fn into_downcast<T: std::error::Error + 'static>(self) -> std::result::Result<T, Self>;
        pub fn into_inner(self) -> std::result::Result<Box<dyn std::error::Error + Send + Sync>, Self>;
        pub fn kind(&self) -> &ErrorKind;
        pub fn new<E>(kind: ErrorKind, error: E) -> Self where E: Into<Box<dyn std::error::Error + Send + Sync>>;
        #[must_use]
        pub fn with_context<C>(self, message: C) -> Self where C: Into<Cow<'static, str>>;
        #[must_use]
        pub fn with_context_fn<F, C>(self, f: F) -> Self where F: FnOnce() -> C, C: Into<Cow<'static, str>>;
        #[must_use]
        pub fn with_error<E, C>(kind: ErrorKind, error: E, message: C) -> Self where E: Into<Box<dyn std::error::Error + Send + Sync>>, C: Into<Cow<'static, str>>;
        #[must_use]
        pub fn with_error_fn<E, F, C>(kind: ErrorKind, error: E, f: F) -> Self where E: Into<Box<dyn std::error::Error + Send + Sync>>, F: FnOnce() -> C, C: Into<Cow<'static, str>>;
        #[must_use]
        pub fn with_message<C>(kind: ErrorKind, message: C) -> Self where C: Into<Cow<'static, str>>;
        #[must_use]
        pub fn with_message_fn<F, C>(kind: ErrorKind, f: F) -> Self where F: FnOnce() -> C, C: Into<Cow<'static, str>>;
    }
    impl Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Error for Error {
        fn source(&self) -> Option<&dyn std::error::Error + 'static>;
    }
    impl From<DecodeError> for Error {
        fn from(error: base64::DecodeError) -> Self;
    }
    impl From<Error> for Error {
        fn from(error: std::io::Error) -> Self;
    }
    impl From<FromUtf8Error> for Error {
        fn from(error: std::string::FromUtf8Error) -> Self;
    }
    impl From<Infallible> for Error {
        fn from(_: core::convert::Infallible) -> Self;
    }
    impl From<ParseBoolError> for Error {
        fn from(error: std::str::ParseBoolError) -> Self;
    }
    impl From<ParseError> for Error {
        fn from(error: url::ParseError) -> Self;
    }
    impl From<ParseIntError> for Error {
        fn from(error: std::num::ParseIntError) -> Self;
    }
    impl From<Utf8Error> for Error {
        fn from(error: std::str::Utf8Error) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ErrorKind {
        #[cfg(feature = "http")]
        HttpResponse { status: crate::http::StatusCode, error_code: Option<String>, raw_response: Option<Box<crate::http::RawResponse>> },
        Connection,
        Io,
        DataConversion,
        Credential,
        Other,
    }
    impl ErrorKind {
        pub fn into_error(self) -> Error;
    }
    impl Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<ErrorKind> for Error {
        fn from(kind: ErrorKind) -> Self;
    }
    pub trait ResultExt<T>: private::Sealed {
        fn with_context<C>(self, kind: ErrorKind, message: C) -> Result<T> where Self: Sized, C: Into<Cow<'static, str>>;
        fn with_context_fn<F, C>(self, kind: ErrorKind, f: F) -> Result<T> where Self: Sized, F: FnOnce() -> C, C: Into<Cow<'static, str>>;
        fn with_kind(self, kind: ErrorKind) -> Result<T> where Self: Sized;
    }
    pub type Result<T> = std::result::Result<T, Error>;
}
#[cfg(feature = "http")]
pub mod http {
    pub use typespec::http::headers::DEFAULT_ALLOWED_HEADER_NAMES;
    pub use typespec::http::response::RawResponse;
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    #[repr(u16)]
    pub enum StatusCode {
        Continue = 100,
        SwitchingProtocols = 101,
        EarlyHints = 103,
        Ok = 200,
        Created = 201,
        Accepted = 202,
        NonAuthoritativeInformation = 203,
        NoContent = 204,
        ResetContent = 205,
        PartialContent = 206,
        MultiStatus = 207,
        ImUsed = 226,
        MultipleChoice = 300,
        MovedPermanently = 301,
        Found = 302,
        SeeOther = 303,
        NotModified = 304,
        TemporaryRedirect = 307,
        PermanentRedirect = 308,
        BadRequest = 400,
        Unauthorized = 401,
        PaymentRequired = 402,
        Forbidden = 403,
        NotFound = 404,
        MethodNotAllowed = 405,
        NotAcceptable = 406,
        ProxyAuthenticationRequired = 407,
        RequestTimeout = 408,
        Conflict = 409,
        Gone = 410,
        LengthRequired = 411,
        PreconditionFailed = 412,
        PayloadTooLarge = 413,
        UriTooLong = 414,
        UnsupportedMediaType = 415,
        RequestedRangeNotSatisfiable = 416,
        ExpectationFailed = 417,
        ImATeapot = 418,
        MisdirectedRequest = 421,
        UnprocessableEntity = 422,
        Locked = 423,
        FailedDependency = 424,
        TooEarly = 425,
        UpgradeRequired = 426,
        PreconditionRequired = 428,
        TooManyRequests = 429,
        RequestHeaderFieldsTooLarge = 431,
        UnavailableForLegalReasons = 451,
        InternalServerError = 500,
        NotImplemented = 501,
        BadGateway = 502,
        ServiceUnavailable = 503,
        GatewayTimeout = 504,
        HttpVersionNotSupported = 505,
        VariantAlsoNegotiates = 506,
        InsufficientStorage = 507,
        LoopDetected = 508,
        NotExtended = 510,
        NetworkAuthenticationRequired = 511,
        UnknownValue(u16),
    }
    impl StatusCode {
        pub fn canonical_reason(&self) -> Cow<'static, str>;
        pub fn is_client_error(&self) -> bool;
        pub fn is_informational(&self) -> bool;
        pub fn is_redirection(&self) -> bool;
        pub fn is_server_error(&self) -> bool;
        pub fn is_success(&self) -> bool;
    }
    impl Deref for StatusCode {
        type Target = u16;
        fn deref(&self) -> &<Self as >::Target;
    }
    impl Display for StatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<StatusCode> for u16 {
        fn from(code: StatusCode) -> u16;
    }
    impl From<u16> for StatusCode {
        fn from(num: u16) -> Self;
    }
    impl PartialEq<StatusCode> for u16 {
        fn eq(&self, other: &StatusCode) -> bool;
    }
    impl PartialEq<u16> for StatusCode {
        fn eq(&self, other: &u16) -> bool;
    }
    impl Serialize for super::StatusCode {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::StatusCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    pub const REDACTED_PATTERN: &str = "REDACTED";
    pub mod headers {
        #[derive(Clone, Debug, Eq, Ord, PartialOrd)]
        pub struct HeaderName {
        }
        impl HeaderName {
            pub fn as_str(&self) -> &str;
            pub const fn from_static(s: &'static str) -> Self;
            pub const fn from_static_standard(s: &'static str) -> Self;
            pub fn is_standard(&self) -> bool;
        }
        impl From<&'static str> for HeaderName {
            fn from(s: &'static str) -> Self;
        }
        impl From<String> for HeaderName {
            fn from(s: String) -> Self;
        }
        impl Hash for HeaderName {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H);
        }
        impl PartialEq<&str> for HeaderName {
            fn eq(&self, other: &&str) -> bool;
        }
        impl PartialEq for HeaderName {
            fn eq(&self, other: &Self) -> bool;
        }
        #[derive(Clone, Eq, PartialEq)]
        pub struct HeaderValue(/* private fields */);
        impl HeaderValue {
            pub fn as_str(&self) -> &str;
            pub fn from_cow<C>(c: C) -> Self where C: Into<Cow<'static, str>>;
            pub const fn from_static(s: &'static str) -> Self;
        }
        impl Debug for HeaderValue {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl From<&'static str> for HeaderValue {
            fn from(s: &'static str) -> Self;
        }
        impl From<&String> for HeaderValue {
            fn from(s: &String) -> Self;
        }
        impl From<String> for HeaderValue {
            fn from(s: String) -> Self;
        }
        #[derive(Clone, Default, Eq, PartialEq)]
        pub struct Headers(/* private fields */);
        impl Headers {
            pub fn add<H>(&mut self, header: H) -> Result<(), <H as >::Error> where H: AsHeaders;
            pub fn get<H: FromHeaders>(&self) -> crate::Result<H>;
            pub fn get_as<V, E>(&self, key: &HeaderName) -> crate::Result<V> where V: FromStr<Err = E>, E: std::error::Error + Send + Sync + 'static;
            pub fn get_optional<H: FromHeaders>(&self) -> Result<Option<H>, <H as >::Error>;
            pub fn get_optional_as<V, E>(&self, key: &HeaderName) -> crate::Result<Option<V>> where V: FromStr<Err = E>, E: std::error::Error + Send + Sync + 'static;
            pub fn get_optional_str(&self, key: &HeaderName) -> Option<&str>;
            pub fn get_optional_string(&self, key: &HeaderName) -> Option<String>;
            pub fn get_optional_with<'a, V, F, E>(&self, key: &HeaderName, parser: F) -> crate::Result<Option<V>> where F: FnOnce(&'a HeaderValue) -> Result<V, E>, E: std::error::Error + Send + Sync + 'static;
            pub fn get_str(&self, key: &HeaderName) -> crate::Result<&str>;
            pub fn get_with<'a, V, F, E>(&self, key: &HeaderName, parser: F) -> crate::Result<V> where F: FnOnce(&'a HeaderValue) -> Result<V, E>, E: std::error::Error + Send + Sync + 'static;
            pub fn insert<K, V>(&mut self, key: K, value: V) where K: Into<HeaderName>, V: Into<HeaderValue>;
            pub fn iter(&self) -> impl Iterator<Item = (&HeaderName, &HeaderValue)>;
            pub fn new() -> Self;
            pub fn remove<K>(&mut self, key: K) -> Option<HeaderValue> where K: Into<HeaderName>;
        }
        impl Debug for Headers {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl From<HashMap<HeaderName, HeaderValue>> for Headers {
            fn from(c: std::collections::HashMap<HeaderName, HeaderValue>) -> Self;
        }
        impl IntoIterator for Headers {
            type IntoIter = IntoIter<HeaderName, HeaderValue>;
            type Item = (HeaderName, HeaderValue);
            fn into_iter(self) -> <Self as >::IntoIter;
        }
        pub trait AsHeaders {
            type Error: std::error::Error + Send + Sync + 'static;
            type Iter: Iterator<Item = (HeaderName, HeaderValue)>;
            fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
        }
        pub trait FromHeaders: Sized {
            type Error: std::error::Error + Send + Sync + 'static;
            fn from_headers(headers: &Headers) -> Result<Option<Self>, <Self as >::Error>;
            fn header_names() -> &'static [&'static str];
        }
        pub trait Header {
            fn name(&self) -> HeaderName;
            fn value(&self) -> HeaderValue;
        }
        pub static DEFAULT_ALLOWED_HEADER_NAMES: std::sync::LazyLock<std::collections::HashSet<std::borrow::Cow<'static, str>>> = _;
    }
    pub mod response {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct RawResponse {
        }
        impl RawResponse {
            pub fn body(&self) -> &ResponseBody;
            pub fn deconstruct(self) -> (StatusCode, Headers, ResponseBody);
            pub fn from_bytes<impl Into<Bytes>: Into<Bytes>>(status: StatusCode, headers: Headers, body: impl Into<Bytes>) -> Self;
            pub fn headers(&self) -> &Headers;
            pub fn into_body(self) -> ResponseBody;
            pub fn status(&self) -> StatusCode;
        }
        #[derive(Clone, Eq, PartialEq)]
        pub struct ResponseBody(/* private fields */);
        impl ResponseBody {
            pub fn from_bytes<impl Into<Bytes>: Into<Bytes>>(bytes: impl Into<Bytes>) -> Self;
            pub fn into_string(self) -> crate::Result<String>;
            #[cfg(feature = "json")]
            pub fn json<T>(&self) -> crate::Result<T> where T: DeserializeOwned;
            #[cfg(feature = "xml")]
            pub fn xml<T>(&self) -> crate::Result<T> where T: DeserializeOwned;
        }
        impl AsRef<[u8]> for ResponseBody {
            #[inline]
            fn as_ref(&self) -> &[u8];
        }
        impl Debug for ResponseBody {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl Deref for ResponseBody {
            type Target = [u8];
            #[inline]
            fn deref(&self) -> &<Self as >::Target;
        }
        impl From<ResponseBody> for crate::Bytes {
            fn from(body: ResponseBody) -> Self;
        }
    }
}
#[cfg(feature = "json")]
pub mod json {
    pub fn from_json<S, T>(body: S) -> crate::error::Result<T> where S: AsRef<[u8]>, T: DeserializeOwned;
    pub fn to_json<T>(value: &T) -> crate::error::Result<bytes::Bytes> where T: ?Sized + Serialize;
}
#[cfg(feature = "xml")]
pub mod xml {
    pub use quick_xml::serde_helpers::text_content as content;
    pub fn from_xml<S, T>(body: S) -> crate::error::Result<T> where S: AsRef<[u8]>, T: DeserializeOwned;
    pub fn to_xml<T>(value: &T) -> crate::error::Result<bytes::Bytes> where T: serde::Serialize;
    pub fn to_xml_with_root<T>(root_tag: &str, value: &T) -> crate::error::Result<bytes::Bytes> where T: serde::Serialize;
}
```
