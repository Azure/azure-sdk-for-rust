// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP requests.

pub mod options;

#[cfg(not(target_arch = "wasm32"))]
use crate::stream::SeekableStream;
use crate::{
    http::{
        headers::{AsHeaders, Header, HeaderName, HeaderValue, Headers},
        Method, Url,
    },
    json::to_json,
    time::OffsetDateTime,
};
use bytes::Bytes;
use rust_decimal::Decimal;
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, fmt, marker::PhantomData, str::FromStr};
use time::format_description::well_known::Rfc3339;

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
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
    pub fn method(&self) -> &Method {
        &self.method
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
pub struct RequestContent<T> {
    body: Body,
    phantom: PhantomData<T>,
}

impl<T> RequestContent<T> {
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
impl<T> PartialEq for RequestContent<T> {
    fn eq(&self, other: &Self) -> bool {
        self.body.eq(&other.body)
    }
}

impl<T> From<RequestContent<T>> for Body {
    fn from(content: RequestContent<T>) -> Self {
        content.body
    }
}

impl<T> From<Body> for RequestContent<T> {
    fn from(body: Body) -> Self {
        Self {
            body,
            phantom: PhantomData,
        }
    }
}

impl<T> TryFrom<Bytes> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Bytes) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Body::Bytes(body),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<u8>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(body).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<&'static str> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: &'static str) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from_static(body.as_bytes()).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Value> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Value) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(serde_json::to_string(&body)?).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<bool> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: bool) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(body.to_string()).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Option<Decimal>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Option<Decimal>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(body.map(|d| d.to_string()).unwrap_or_default()).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<bool>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Vec<bool>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(format!(
                "[{}]",
                body.iter()
                    .map(|b| b.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<OffsetDateTime>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Vec<OffsetDateTime>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(format!(
                r#"["{}"]"#,
                body.iter()
                    .map(|dt| if let Ok(formatted) = dt.format(&Rfc3339) {
                        formatted.to_string()
                    } else {
                        dt.to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<String>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Vec<String>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(format!(
                r#"[{}]"#,
                body.iter()
                    .map(|s| format!(r#""{}""#, s))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<&str>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Vec<&str>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(format!(
                r#"[{}]"#,
                body.iter()
                    .map(|s| format!(r#""{}""#, s))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<f32>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Vec<f32>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(format!(
                "[{}]",
                body.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<f64>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Vec<f64>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(format!(
                "[{}]",
                body.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<i32>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Vec<i32>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(format!(
                "[{}]",
                body.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<i64>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Vec<i64>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(format!(
                "[{}]",
                body.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Vec<Value>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: Vec<Value>) -> Result<Self, Self::Error> {
        Ok(Self {
            body: Bytes::from(serde_json::to_string(&body)?).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<HashMap<String, bool>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: HashMap<String, bool>) -> Result<Self, Self::Error> {
        let body_str = body
            .into_iter()
            .map(|(k, v)| format!(r#""{}":{}"#, k, v))
            .collect::<Vec<_>>()
            .join(",");
        Ok(Self {
            body: Bytes::from(format!(r#"{{{}}}"#, body_str)).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<HashMap<String, OffsetDateTime>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: HashMap<String, OffsetDateTime>) -> Result<Self, Self::Error> {
        let body_str = body
            .into_iter()
            .map(|(k, v)| {
                if let Ok(formatted) = v.format(&Rfc3339) {
                    format!(r#""{}":"{}""#, k, formatted)
                } else {
                    format!(r#""{}":"{}""#, k, v)
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        Ok(Self {
            body: Bytes::from(format!(r#"{{{}}}"#, body_str)).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<HashMap<String, String>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: HashMap<String, String>) -> Result<Self, Self::Error> {
        let body_str = body
            .into_iter()
            .map(|(k, v)| format!(r#""{}":"{}""#, k, v))
            .collect::<Vec<_>>()
            .join(",");
        Ok(Self {
            body: Bytes::from(format!(r#"{{{}}}"#, body_str)).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<HashMap<String, f32>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: HashMap<String, f32>) -> Result<Self, Self::Error> {
        let body_str = body
            .into_iter()
            .map(|(k, v)| format!(r#""{}":{}"#, k, v))
            .collect::<Vec<_>>()
            .join(",");
        Ok(Self {
            body: Bytes::from(format!(r#"{{{}}}"#, body_str)).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<HashMap<String, f64>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: HashMap<String, f64>) -> Result<Self, Self::Error> {
        let body_str = body
            .into_iter()
            .map(|(k, v)| format!(r#""{}":{}"#, k, v))
            .collect::<Vec<_>>()
            .join(",");
        Ok(Self {
            body: Bytes::from(format!(r#"{{{}}}"#, body_str)).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<HashMap<String, i32>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: HashMap<String, i32>) -> Result<Self, Self::Error> {
        let body_str = body
            .into_iter()
            .map(|(k, v)| format!(r#""{}":{}"#, k, v))
            .collect::<Vec<_>>()
            .join(",");
        Ok(Self {
            body: Bytes::from(format!(r#"{{{}}}"#, body_str)).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<HashMap<String, i64>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: HashMap<String, i64>) -> Result<Self, Self::Error> {
        let body_str = body
            .into_iter()
            .map(|(k, v)| format!(r#""{}":{}"#, k, v))
            .collect::<Vec<_>>()
            .join(",");
        Ok(Self {
            body: Bytes::from(format!(r#"{{{}}}"#, body_str)).into(),
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<HashMap<String, Value>> for RequestContent<T> {
    type Error = crate::Error;
    fn try_from(body: HashMap<String, Value>) -> Result<Self, Self::Error> {
        let body_str = body
            .into_iter()
            .map(|(k, v)| format!(r#""{}":{}"#, k, v))
            .collect::<Vec<_>>()
            .join(",");
        Ok(Self {
            body: Bytes::from(format!(r#"{{{}}}"#, body_str)).into(),
            phantom: PhantomData,
        })
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
        assert_eq!(*EXPECTED, actual.try_into().unwrap());
    }

    #[test]
    fn tryfrom_vec() {
        let actual: Vec<u8> = r#"{"str":"test","num":1,"b":true}"#.bytes().collect();
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
}
