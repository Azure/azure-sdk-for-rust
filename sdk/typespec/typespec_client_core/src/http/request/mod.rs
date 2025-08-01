// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP requests.

pub mod options;

#[cfg(not(target_arch = "wasm32"))]
use crate::stream::SeekableStream;
use crate::{
    http::{
        headers::{AsHeaders, Header, HeaderName, HeaderValue, Headers},
        Format, JsonFormat, Method, SerializeWith, Url,
    },
    json::to_json,
};
use bytes::Bytes;
use serde::Serialize;
use std::{collections::HashMap, convert::Infallible, fmt, marker::PhantomData, str::FromStr};

/// An HTTP Body.
#[derive(Clone)]
pub enum Body {
    /// A body of a known size.
    Bytes(bytes::Bytes),

    /// A streaming body.
    ///
    /// This is not currently supported on WASM targets.
    // We cannot currently implement `Body::SeekableStream` for WASM
    // because `reqwest::Body::wrap_stream()` is not implemented for WASM.
    #[cfg(not(target_arch = "wasm32"))]
    SeekableStream(Box<dyn SeekableStream>),
}

impl Body {
    pub fn len(&self) -> usize {
        match self {
            Body::Bytes(bytes) => bytes.len(),
            #[cfg(not(target_arch = "wasm32"))]
            Body::SeekableStream(stream) => stream.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

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
            body: Body::Bytes(bytes::Bytes::new()),
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
            // Format URL as simple string instead of struct.
            .field("url", &self.url.as_str())
            .field("method", &self.method)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}

/// The body content of a service client request.
/// This allows callers to pass a model to serialize or raw content to client methods.
#[derive(Clone, Debug)]
pub struct RequestContent<T, F = JsonFormat> {
    body: Body,
    phantom: PhantomData<(T, F)>,
}

impl<T, F> RequestContent<T, F> {
    /// Gets the body of the request.
    pub fn body(&self) -> &Body {
        &self.body
    }

    /// Create a new `RequestContent` from byte slice.
    pub fn from(bytes: Vec<u8>) -> Self {
        Self {
            body: Body::Bytes(Bytes::from(bytes)),
            phantom: PhantomData,
        }
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

impl<T, F> TryFrom<Bytes> for RequestContent<T, F> {
    type Error = crate::Error;
    fn try_from(body: Bytes) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Body::Bytes(body),
            phantom: PhantomData,
        })
    }
}

impl<'a, T, F> TryFrom<&'a [u8]> for RequestContent<T, F> {
    type Error = crate::Error;
    fn try_from(body: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::copy_from_slice(body).into(),
            phantom: PhantomData,
        })
    }
}

impl<T, F> TryFrom<&'static str> for RequestContent<T, F> {
    type Error = crate::Error;
    fn try_from(body: &'static str) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from_static(body.as_bytes()).into(),
            phantom: PhantomData,
        })
    }
}

impl<T, F> TryFrom<bool> for RequestContent<T, F> {
    type Error = Infallible;
    fn try_from(body: bool) -> Result<Self, Infallible> {
        Ok(Self {
            body: Bytes::from(body.to_string()).into(),
            phantom: PhantomData,
        })
    }
}

impl<T: SerializeWith<F>, F: Format> TryFrom<Option<T>> for RequestContent<Option<T>, F>
where
    Option<T>: SerializeWith<F>,
{
    type Error = crate::Error;
    fn try_from(value: Option<T>) -> Result<Self, Self::Error> {
        Ok(<Option<T> as SerializeWith<F>>::serialize_with(value)?.into())
    }
}

impl<T: SerializeWith<F>, F: Format> TryFrom<Vec<T>> for RequestContent<Vec<T>, F>
where
    Vec<T>: SerializeWith<F>,
{
    type Error = crate::Error;
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Ok(<Vec<T> as SerializeWith<F>>::serialize_with(value)?.into())
    }
}

impl<T: SerializeWith<F>, F: Format> TryFrom<HashMap<String, T>>
    for RequestContent<HashMap<String, T>, F>
where
    HashMap<String, T>: SerializeWith<F>,
{
    type Error = crate::Error;
    fn try_from(value: HashMap<String, T>) -> Result<Self, Self::Error> {
        Ok(<HashMap<String, T> as SerializeWith<F>>::serialize_with(value)?.into())
    }
}

impl<T> FromStr for RequestContent<T> {
    type Err = crate::Error;
    fn from_str(body: &str) -> Result<Self, Self::Err> {
        let body: Bytes = Bytes::copy_from_slice(body.as_bytes());
        Ok(Self {
            body: Body::Bytes(body),
            phantom: PhantomData,
        })
    }
}

// cspell:ignore fromstr tryfrom
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        error::{Error, ErrorKind, ResultExt as _},
        time::OffsetDateTime,
    };
    use bytes::BufMut as _;
    use serde::Serializer;
    use serde_json::{value::RawValue, Value};
    use std::{collections::BTreeMap, sync::LazyLock};
    use time::macros::datetime;

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

    // Serialization support for an ordered HashMap i.e, BTreeMap.
    impl<T: SerializeWith<JsonFormat>> SerializeWith<JsonFormat> for BTreeMap<String, T> {
        fn serialize_with(value: Self) -> typespec::Result<Body> {
            use serde::ser::SerializeMap;

            let mut buf = vec![].writer();
            let mut ser = serde_json::Serializer::new(&mut buf);
            let mut seq = ser
                .serialize_map(Some(value.len()))
                .with_context(ErrorKind::Io, || "failed map start")?;
            for (key, elem) in value {
                let Body::Bytes(raw) = T::serialize_with(elem)? else {
                    return Err(Error::new(
                        ErrorKind::DataConversion,
                        "failed json serialization",
                    ));
                };
                let raw = RawValue::from_string(String::from_utf8(raw.to_vec())?)?;
                seq.serialize_entry(&key, &raw)
                    .with_context(ErrorKind::Io, || "failed map entry")?;
            }
            seq.end().with_context(ErrorKind::Io, || "failed map end")?;

            let buf = buf.into_inner();
            Ok(buf.into())
        }
    }

    // Serialization support for an ordered HashMap i.e, BTreeMap.
    impl<T: SerializeWith<F>, F: Format> TryFrom<std::collections::BTreeMap<String, T>>
        for RequestContent<std::collections::BTreeMap<String, T>, F>
    where
        std::collections::BTreeMap<String, T>: SerializeWith<F>,
    {
        type Error = crate::Error;
        fn try_from(value: std::collections::BTreeMap<String, T>) -> Result<Self, Self::Error> {
            Ok(
                <std::collections::BTreeMap<String, T> as SerializeWith<F>>::serialize_with(value)?
                    .into(),
            )
        }
    }

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
        assert_eq!(*EXPECTED, actual.try_into().unwrap());
    }

    #[test]
    fn tryfrom_vec() {
        let actual = br#"{"str":"test","num":1,"b":true}"#.as_ref();
        assert_eq!(*EXPECTED, actual.try_into().unwrap());
    }

    #[test]
    fn tryfrom_str() {
        let actual = r#"{"str":"test","num":1,"b":true}"#;
        assert_eq!(*EXPECTED, actual.try_into().unwrap());
    }

    #[test]
    fn fromstr_parse() {
        let actual: RequestContent<Expected> =
            r#"{"str":"test","num":1,"b":true}"#.parse().unwrap();
        assert_eq!(*EXPECTED, actual);
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
            vec!["P123DT22H14M12.011S".into()].try_into().unwrap();
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
        let actual: RequestContent<Vec<String>> =
            vec!["hello".into(), "".into()].try_into().unwrap();
        assert_eq!(actual.body(), &Body::from_static(br#"["hello",""]"#));
    }

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
        let actual: RequestContent<BTreeMap<String, OffsetDateTime>> =
            BTreeMap::from_iter(vec![("k1".into(), datetime!(2022-08-26 18:38:00 UTC))])
                .try_into()
                .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":"2022-08-26T18:38:00Z"}"#)
        );
    }

    #[test]
    fn spector_dictionary_duration() {
        let actual: RequestContent<BTreeMap<String, String>> =
            BTreeMap::from_iter(vec![("k1".into(), "P123DT22H14M12.011S".into())])
                .try_into()
                .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":"P123DT22H14M12.011S"}"#)
        );
    }

    #[test]
    fn spector_dictionary_f32() {
        let actual: RequestContent<BTreeMap<String, f32>> =
            BTreeMap::from_iter(vec![("k1".into(), 43.125f32)])
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
            ("k1".into(), "hello".into()),
            ("k2".into(), "".into()),
        ])
        .try_into()
        .unwrap();
        assert_eq!(
            actual.body(),
            &Body::from_static(br#"{"k1":"hello","k2":""}"#)
        );
    }

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
