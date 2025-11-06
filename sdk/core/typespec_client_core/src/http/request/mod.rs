// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP requests.

pub mod options;

#[cfg(not(target_arch = "wasm32"))]
use crate::stream::SeekableStream;
#[cfg(feature = "json")]
use crate::{http::JsonFormat, json::to_json};
use crate::{
    http::{
        headers::{AsHeaders, Header, HeaderName, HeaderValue, Headers},
        Method, Sanitizer, Url, DEFAULT_ALLOWED_QUERY_PARAMETERS,
    },
    Bytes,
};
#[cfg(any(feature = "json", feature = "xml"))]
use crate::{time::OffsetDateTime, Value};
#[cfg(any(feature = "json", feature = "xml"))]
use serde::Serialize;
#[cfg(any(feature = "json", feature = "xml"))]
use std::collections::HashMap;
use std::{fmt, marker::PhantomData};
#[cfg(any(feature = "json", feature = "xml"))]
use time::format_description::well_known::Rfc3339;

/// An HTTP Body.
#[derive(Clone)]
pub enum Body {
    /// A body of a known size.
    Bytes(crate::Bytes),

    /// A streaming body.
    ///
    /// This is not currently supported on WASM targets.
    // We cannot currently implement `Body::SeekableStream` for WASM
    // because `reqwest::Body::wrap_stream()` is not implemented for WASM.
    #[cfg(not(target_arch = "wasm32"))]
    SeekableStream(Box<dyn SeekableStream>),
}

impl Body {
    /// Returns the length of the body in bytes.
    pub fn len(&self) -> usize {
        match self {
            Body::Bytes(bytes) => bytes.len(),
            #[cfg(not(target_arch = "wasm32"))]
            Body::SeekableStream(stream) => stream.len(),
        }
    }

    /// Returns `true` if the body is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Resets the body to the beginning, if it is a seekable stream.
    ///
    /// This function is a no-op for byte bodies.
    pub async fn reset(&mut self) -> crate::Result<()> {
        match self {
            Body::Bytes(_) => Ok(()),
            #[cfg(not(target_arch = "wasm32"))]
            Body::SeekableStream(stream) => stream.reset().await,
        }
    }

    #[cfg(test)]
    fn from_static(value: &'static [u8]) -> Self {
        Self::Bytes(Bytes::from_static(value))
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(test)]
            Self::Bytes(v) if !v.is_empty() => f
                .debug_struct("Bytes")
                .field("len", &v.len())
                .field("data", &v)
                .finish_non_exhaustive(),
            #[cfg(not(test))]
            Self::Bytes(v) if !v.is_empty() => f.write_str("Bytes { .. }"),
            Self::Bytes(_) => f.write_str("Bytes {}"),
            #[cfg(not(target_arch = "wasm32"))]
            Self::SeekableStream(v) if !v.is_empty() => f.write_str("SeekableStream { .. }"),
            #[cfg(not(target_arch = "wasm32"))]
            Self::SeekableStream(_) => f.write_str("SeekableStream {}"),
        }
    }
}

impl<B> From<B> for Body
where
    B: Into<Bytes>,
{
    fn from(bytes: B) -> Self {
        Self::Bytes(bytes.into())
    }
}

impl From<&Body> for Bytes {
    fn from(value: &Body) -> Self {
        match value {
            Body::Bytes(bytes) => bytes.clone(),
            #[cfg(not(target_arch = "wasm32"))]
            Body::SeekableStream(_) => unimplemented!(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Box<dyn SeekableStream>> for Body {
    fn from(seekable_stream: Box<dyn SeekableStream>) -> Self {
        Self::SeekableStream(seekable_stream)
    }
}

#[cfg(test)]
impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        #[cfg_attr(target_arch = "wasm32", allow(irrefutable_let_patterns))]
        if let Self::Bytes(this) = self {
            if let Self::Bytes(other) = other {
                return this.eq(other);
            }
        }
        false
    }
}

/// A pipeline request.
///
/// A pipeline request is composed by a destination (uri), a method, a collection of headers and a
/// body. Policies are expected to enrich the request by mutating it.
#[derive(Clone)]
pub struct Request {
    pub(crate) url: Url,
    pub(crate) method: Method,
    pub(crate) headers: Headers,
    pub(crate) body: Body,
}

impl Request {
    /// Create a new request with an empty body and no headers
    pub fn new(url: Url, method: Method) -> Self {
        Self {
            url,
            method,
            headers: Headers::new(),
            body: Body::Bytes(Bytes::new()),
        }
    }

    /// Gets the request [`Url`].
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Gets a mutable request [`Url`].
    pub fn url_mut(&mut self) -> &mut Url {
        &mut self.url
    }

    /// Gets the request URL path and query string.
    pub fn path_and_query(&self) -> String {
        let mut result = self.url.path().to_owned();
        if let Some(query) = self.url.query() {
            result.push('?');
            result.push_str(query);
        }
        result
    }

    /// Gets the request HTTP method.
    pub fn method(&self) -> Method {
        self.method
    }

    /// Sets the request HTTP method.
    pub fn set_method(&mut self, method: Method) {
        self.method = method;
    }

    /// Inserts zero or more headers from a type that implements [`AsHeaders`].
    pub fn insert_headers<T: AsHeaders>(&mut self, headers: &T) -> Result<(), T::Error> {
        for (name, value) in headers.as_headers()? {
            self.insert_header(name, value);
        }
        Ok(())
    }

    /// Gets the request [`Headers`].
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Gets a mutable request [`Headers`].
    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    /// Gets the request body.
    pub fn body(&self) -> &Body {
        &self.body
    }

    /// Sets request body JSON.
    #[cfg(feature = "json")]
    pub fn set_json<T>(&mut self, data: &T) -> crate::Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.set_body(to_json(data)?);
        Ok(())
    }

    /// Sets the request body.
    pub fn set_body(&mut self, body: impl Into<Body>) {
        self.body = body.into();
    }

    /// Inserts a header from the `key` and `value`.
    pub fn insert_header<K, V>(&mut self, key: K, value: V)
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        self.headers.insert(key, value);
    }

    /// Inserts a [`Header`] if `item` is `Some`.
    pub fn add_optional_header<T: Header>(&mut self, item: &Option<T>) {
        if let Some(item) = item {
            self.insert_header(item.name(), item.value());
        }
    }

    /// Inserts a [`Header`].
    pub fn add_mandatory_header<T: Header>(&mut self, item: &T) {
        self.insert_header(item.name(), item.value());
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            // Format URL as simple string instead of struct. Sanitize all query parameters because we don't have an allow list.
            .field("url", &self.url.sanitize(&DEFAULT_ALLOWED_QUERY_PARAMETERS))
            .field("method", &self.method)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}

/// The body content of a service client request.
///
/// This allows callers to pass a model to serialize or raw content to client methods.
#[cfg(feature = "json")]
#[derive(Clone, Debug)]
pub struct RequestContent<T, F = JsonFormat> {
    body: Body,
    phantom: PhantomData<(T, F)>,
}

/// The body content of a service client request.
///
/// This allows callers to pass a model to serialize or raw content to client methods.
#[cfg(not(feature = "json"))]
#[derive(Clone, Debug)]
pub struct RequestContent<T, F> {
    body: Body,
    phantom: PhantomData<(T, F)>,
}

impl<T, F> RequestContent<T, F> {
    /// Gets the body of the request.
    pub fn body(&self) -> &Body {
        &self.body
    }

    /// Create a new `RequestContent` from a `Vec<u8>`.
    ///
    /// Allocation may be avoided in some cases.
    pub fn from(body: Vec<u8>) -> Self {
        Self {
            body: Body::Bytes(Bytes::from(body)),
            phantom: PhantomData,
        }
    }

    /// Copies bytes into a new `RequestContent`.
    pub fn from_slice(body: &[u8]) -> Self {
        Self {
            body: Body::Bytes(Bytes::copy_from_slice(body)),
            phantom: PhantomData,
        }
    }

    /// Create a new `RequestContent` from a static slice.
    ///
    /// This should not allocate.
    pub fn from_static(body: &'static [u8]) -> Self {
        Self {
            body: Body::Bytes(Bytes::from_static(body)),
            phantom: PhantomData,
        }
    }

    /// Copies UTF-8 bytes into a new `RequestContent`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(body: &str) -> Self {
        Self::from_slice(body.as_bytes())
    }
}

#[cfg(test)]
impl<T, F> PartialEq for RequestContent<T, F> {
    fn eq(&self, other: &Self) -> bool {
        self.body.eq(&other.body)
    }
}

impl<T, F> From<RequestContent<T, F>> for Body {
    fn from(content: RequestContent<T, F>) -> Self {
        content.body
    }
}

impl<T, F> From<Body> for RequestContent<T, F> {
    fn from(body: Body) -> Self {
        Self {
            body,
            phantom: PhantomData,
        }
    }
}

impl<T, F> From<Bytes> for RequestContent<T, F> {
    fn from(body: Bytes) -> Self {
        Self {
            body: Body::Bytes(body),
            phantom: PhantomData,
        }
    }
}

// Represent a decimal as a string instead of relying on its `serde-str` feature which,
// if another crate enables the `serde-float` feature, will instead serialize as f64 and lose precision.
#[cfg(feature = "decimal")]
mod decimal {
    use super::*;
    use rust_decimal::Decimal;
    use std::convert::Infallible;

    #[allow(
        unknown_lints,
        clippy::infallible_try_from,
        reason = "maintain a consistent pattern of `try_into()`"
    )]
    impl<T, F> TryFrom<Decimal> for RequestContent<T, F> {
        type Error = Infallible;
        fn try_from(value: Decimal) -> Result<Self, Infallible> {
            Ok(Self {
                body: Bytes::from(value.to_string()).into(),
                phantom: PhantomData,
            })
        }
    }

    #[allow(
        unknown_lints,
        clippy::infallible_try_from,
        reason = "maintain a consistent pattern of `try_into()`"
    )]
    impl<T, F> TryFrom<Option<Decimal>> for RequestContent<T, F> {
        type Error = Infallible;
        fn try_from(value: Option<Decimal>) -> Result<Self, Infallible> {
            Ok(Self {
                body: Bytes::from(value.as_ref().map(ToString::to_string).unwrap_or_default())
                    .into(),
                phantom: PhantomData,
            })
        }
    }

    #[test]
    fn spector_decimal() {
        let actual: RequestContent<Decimal> = Decimal::new(314, 2).try_into().unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"3.14"#))
    }

    #[test]
    fn spector_option_decimal() {
        let actual: RequestContent<Option<Decimal>> =
            Some(Decimal::new(314, 2)).try_into().unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"3.14"#))
    }
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    #[cfg(test)]
    use std::collections::BTreeMap;
    #[cfg(test)]
    use time::macros::datetime;

    macro_rules! impl_try_from {
        ($t:ty) => {
            impl<T> ::core::convert::TryFrom<$t> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
                type Error = $crate::Error;
                fn try_from(value: $t) -> $crate::Result<Self> {
                    Ok(Self {
                        body: $crate::json::to_json(&value)?.into(),
                        phantom: ::core::marker::PhantomData,
                    })
                }
            }

            impl<T> ::core::convert::TryFrom<::std::vec::Vec<$t>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
                type Error = $crate::Error;
                fn try_from(value: ::std::vec::Vec<$t>) -> $crate::Result<Self> {
                    Ok(Self {
                        body: $crate::json::to_json(&value)?.into(),
                        phantom: ::core::marker::PhantomData,
                    })
                }
            }

            impl<T> ::core::convert::TryFrom<::std::collections::HashMap<String, $t>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
                type Error = $crate::Error;
                fn try_from(value: ::std::collections::HashMap<String, $t>) -> $crate::Result<Self> {
                    Ok(Self {
                        body: $crate::json::to_json(&value)?.into(),
                        phantom: ::core::marker::PhantomData,
                    })
                }
            }

            #[cfg(test)]
            impl<T> ::core::convert::TryFrom<::std::collections::BTreeMap<String, $t>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
                type Error = $crate::Error;
                fn try_from(value: ::std::collections::BTreeMap<String, $t>) -> $crate::Result<Self> {
                    Ok(Self {
                        body: $crate::json::to_json(&value)?.into(),
                        phantom: ::core::marker::PhantomData,
                    })
                }
            }
        };

        ($($t:ty),*) => {
            $(impl_try_from!($t);)*
        };
    }

    // We can't add a blanket implementation of TryFrom<T> for RequestContent<T, JsonFormat>,
    // so we explicit support those scenarios needed for unbranded TypeSpec mimicking the Spector test suite.
    impl_try_from!(bool);
    impl_try_from!(&str, String);
    impl_try_from!(i32, i64);
    impl_try_from!(f32, f64);
    #[cfg(any(feature = "json", feature = "xml"))]
    impl_try_from!(Value);

    impl<T, F> TryFrom<Vec<OffsetDateTime>> for RequestContent<T, F> {
        type Error = crate::Error;
        fn try_from(body: Vec<OffsetDateTime>) -> Result<Self, Self::Error> {
            Ok(Self {
                body: Bytes::from(serde_json::to_string(
                    &body
                        .iter()
                        .map(|v| v.format(&Rfc3339).unwrap_or_else(|_| v.to_string()))
                        .collect::<Vec<_>>(),
                )?)
                .into(),
                phantom: PhantomData,
            })
        }
    }

    impl<T, F> TryFrom<HashMap<String, OffsetDateTime>> for RequestContent<T, F> {
        type Error = crate::Error;
        fn try_from(body: HashMap<String, OffsetDateTime>) -> Result<Self, Self::Error> {
            let body_rfc3339: HashMap<String, String> = body
                .into_iter()
                .map(|(k, v)| {
                    let formatted = v.format(&Rfc3339).unwrap_or_else(|_| v.to_string());
                    (k, formatted)
                })
                .collect();

            Ok(Self {
                body: Bytes::from(serde_json::to_string(&body_rfc3339)?).into(),
                phantom: PhantomData,
            })
        }
    }

    #[test]
    fn spector_vec_bool() {
        let actual: RequestContent<Vec<bool>> = vec![true, false].try_into().unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"[true,false]"#));
    }

    #[test]
    fn spector_vec_offset_date_time() {
        let actual: RequestContent<Vec<OffsetDateTime>> =
            vec![datetime!(2022-08-26 18:38:00 UTC)].try_into().unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"["2022-08-26T18:38:00Z"]"#)
        );
    }

    #[test]
    fn spector_vec_duration() {
        let actual: RequestContent<Vec<String>> =
            vec!["P123DT22H14M12.011S".to_string()].try_into().unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"["P123DT22H14M12.011S"]"#)
        );
    }

    #[test]
    fn spector_vec_f32() {
        let actual: RequestContent<Vec<f32>> = vec![43.125f32].try_into().unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"[43.125]"#));
    }

    #[test]
    fn spector_vec_i64() {
        let actual: RequestContent<Vec<i64>> = vec![9007199254740991i64, -9007199254740991i64]
            .try_into()
            .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"[9007199254740991,-9007199254740991]"#)
        );
    }

    #[test]
    fn spector_vec_string() {
        let actual: RequestContent<Vec<String>> = vec!["hello".to_string(), "".to_string()]
            .try_into()
            .unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"["hello",""]"#));
    }

    #[cfg(any(feature = "json", feature = "xml"))]
    #[test]
    fn spector_vec_value() {
        let actual: RequestContent<Vec<Value>> = vec![
            Value::Number(1.into()),
            Value::String("hello".into()),
            Value::Null,
        ]
        .try_into()
        .unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"[1,"hello",null]"#));
    }

    #[test]
    fn spector_dictionary_bool() {
        let actual: RequestContent<BTreeMap<String, bool>> =
            BTreeMap::from_iter(vec![("k1".into(), true), ("k2".into(), false)])
                .try_into()
                .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":true,"k2":false}"#)
        );
    }

    #[test]
    fn spector_dictionary_offset_date_time() {
        let actual: RequestContent<HashMap<String, OffsetDateTime>> =
            HashMap::from_iter(vec![("k1".into(), datetime!(2022-08-26 18:38:00 UTC))])
                .try_into()
                .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":"2022-08-26T18:38:00Z"}"#)
        );
    }

    #[test]
    fn spector_dictionary_duration() {
        let actual: RequestContent<HashMap<String, String>> =
            HashMap::from_iter(vec![("k1".to_string(), "P123DT22H14M12.011S".to_string())])
                .try_into()
                .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":"P123DT22H14M12.011S"}"#)
        );
    }

    #[test]
    fn spector_dictionary_f32() {
        let actual: RequestContent<HashMap<String, f32>> =
            HashMap::from_iter(vec![("k1".into(), 43.125f32)])
                .try_into()
                .unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"{"k1":43.125}"#));
    }

    #[test]
    fn spector_dictionary_i64() {
        let actual: RequestContent<BTreeMap<String, i64>> = BTreeMap::from_iter(vec![
            ("k1".into(), 9007199254740991i64),
            ("k2".into(), -9007199254740991i64),
        ])
        .try_into()
        .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":9007199254740991,"k2":-9007199254740991}"#)
        );
    }

    #[test]
    fn spector_dictionary_string() {
        let actual: RequestContent<BTreeMap<String, String>> = BTreeMap::from_iter(vec![
            ("k1".to_string(), "hello".to_string()),
            ("k2".to_string(), "".to_string()),
        ])
        .try_into()
        .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":"hello","k2":""}"#)
        );
    }

    #[cfg(any(feature = "json", feature = "xml"))]
    #[test]
    fn spector_dictionary_value() {
        let actual: RequestContent<BTreeMap<String, Value>> = BTreeMap::from_iter(vec![
            ("k1".into(), Value::Number(1.into())),
            ("k2".into(), Value::String("hello".into())),
            ("k3".into(), Value::Null),
        ])
        .try_into()
        .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":1,"k2":"hello","k3":null}"#)
        );
    }
}

// cspell:ignore fromstr tryfrom
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    #[derive(Debug, Serialize)]
    struct Expected {
        str: String,
        num: i32,
        b: bool,
    }

    impl TryFrom<Expected> for RequestContent<Expected> {
        type Error = crate::Error;
        fn try_from(value: Expected) -> Result<Self, Self::Error> {
            Ok(RequestContent::from(serde_json::to_vec(&value)?))
        }
    }

    static EXPECTED: LazyLock<RequestContent<Expected>> = LazyLock::new(|| RequestContent {
        body: Bytes::from(r#"{"str":"test","num":1,"b":true}"#.to_string()).into(),
        phantom: PhantomData,
    });

    #[test]
    fn tryfrom_t() {
        let actual = Expected {
            str: "test".to_string(),
            num: 1,
            b: true,
        };
        assert_eq!(*EXPECTED, actual.try_into().unwrap());
    }

    #[test]
    fn tryfrom_bytes() {
        let actual = Bytes::from(r#"{"str":"test","num":1,"b":true}"#.to_string());
        assert_eq!(*EXPECTED, actual.into());
    }

    #[test]
    fn tryfrom_vec() {
        let actual: Vec<u8> = r#"{"str":"test","num":1,"b":true}"#.bytes().collect();
        assert_eq!(*EXPECTED, RequestContent::from(actual));
    }

    #[test]
    fn tryfrom_str() {
        let actual = r#"{"str":"test","num":1,"b":true}"#;
        assert_eq!(*EXPECTED, RequestContent::from_str(actual));
    }
}
