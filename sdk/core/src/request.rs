use crate::SeekableStream;
use http::{HeaderMap, Method, Uri};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Body {
    Bytes(bytes::Bytes),
    SeekableStream(Box<dyn SeekableStream>),
}

impl From<bytes::Bytes> for Body {
    fn from(bytes: bytes::Bytes) -> Self {
        Self::Bytes(bytes)
    }
}

impl From<Box<dyn SeekableStream>> for Body {
    fn from(seekable_stream: Box<dyn SeekableStream>) -> Self {
        Self::SeekableStream(seekable_stream)
    }
}

const FIELDS: &[&str] = &["uri", "method", "headers", "body"];

/// A pipeline request.
///
/// A pipeline request is composed by a destination (uri), a method, a collection of headers and a
/// body. Policies are expected to enrich the request by mutating it.
#[derive(Debug, Clone)]
pub struct Request {
    uri: Uri,
    method: Method,
    headers: HeaderMap,
    body: Body,
}

impl Request {
    fn new(uri: Uri, method: Method, headers: HeaderMap, body: Body) -> Self {
        Self {
            uri,
            method,
            headers,
            body,
        }
    }
}

impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hm = std::collections::HashMap::new();
        for (h, v) in self.headers().iter() {
            if h.as_str().to_lowercase() == "authorization" {
                hm.insert(h.to_string(), "<<STRIPPED>>");
            } else {
                hm.insert(h.to_string(), v.to_str().unwrap());
            }
        }

        let mut state = serializer.serialize_struct("Request", 4)?;
        state.serialize_field(
            FIELDS[0],
            &self
                .uri
                .path_and_query()
                .map(|p| p.to_string())
                .unwrap_or_else(String::new),
        )?;
        state.serialize_field(FIELDS[1], &self.method.to_string())?;
        state.serialize_field(FIELDS[2], &hm)?;
        state.serialize_field(
            FIELDS[3],
            &match &self.body {
                Body::Bytes(bytes) => base64::encode(bytes as &[u8]),
                Body::SeekableStream(_) => unimplemented!(),
            },
        )?;

        state.end()
    }
}

impl Request {
    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn set_body(&mut self, body: Body) {
        self.body = body;
    }

    /// Clones the internal body and gives it back to the caller.
    pub fn clone_body(&self) -> Body {
        self.body.clone()
    }
}

/// Temporary hack to convert preexisting requests into the new format. It
/// will be removed as soon as we remove the dependency from `http::Request`.
impl From<http::Request<bytes::Bytes>> for Request {
    fn from(request: http::Request<bytes::Bytes>) -> Self {
        let (parts, body) = request.into_parts();
        Self {
            uri: parts.uri,
            method: parts.method,
            headers: parts.headers,
            body: Body::Bytes(body),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Parts {
    pub uri: Uri,
    pub method: Method,
    pub headers: HeaderMap,
}

use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::str::FromStr;

impl<'de> Deserialize<'de> for Request {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Request", FIELDS, RequestVisitor)
    }
}

struct RequestVisitor;

impl<'de> Visitor<'de> for RequestVisitor {
    type Value = Request;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("struct Request")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let uri: (&str, &str) = match map.next_entry()? {
            Some((a, b)) => (a, b),
            None => return Err(serde::de::Error::custom("missing uri")),
        };

        if uri.0 != FIELDS[0] {
            return Err(serde::de::Error::custom(format!(
                "unexpected field {}, expected {}",
                uri.0, FIELDS[0]
            )));
        }

        let method: (&str, &str) = match map.next_entry()? {
            Some((a, b)) => (a, b),
            None => return Err(serde::de::Error::custom("missing method")),
        };

        if method.0 != FIELDS[1] {
            return Err(serde::de::Error::custom(format!(
                "unexpected field {}, expected {}",
                method.0, FIELDS[1]
            )));
        }

        let headers: (&str, HashMap<&str, String>) = match map.next_entry()? {
            Some((a, b)) => (a, b),
            None => return Err(serde::de::Error::custom("missing header map")),
        };
        if headers.0 != FIELDS[2] {
            return Err(serde::de::Error::custom(format!(
                "unexpected field {}, expected {}",
                headers.0, FIELDS[2]
            )));
        }

        let body: (&str, String) = match map.next_entry()? {
            Some((a, b)) => (a, b),
            None => return Err(serde::de::Error::custom("missing body")),
        };
        if body.0 != FIELDS[3] {
            return Err(serde::de::Error::custom(format!(
                "unexpected field {}, expected {}",
                body.0, FIELDS[3]
            )));
        }

        let body = base64::decode(&body.1).map_err(serde::de::Error::custom)?;

        let mut hm = HeaderMap::new();
        for (k, v) in headers.1.into_iter() {
            hm.append(
                http::header::HeaderName::from_lowercase(k.as_bytes())
                    .map_err(serde::de::Error::custom)?,
                http::HeaderValue::from_str(&v).map_err(serde::de::Error::custom)?,
            );
        }

        Ok(Self::Value::new(
            Uri::from_str(uri.1).expect("expected a valid uri"),
            Method::from_str(method.1).expect("expected a valid HTTP method"),
            hm,
            bytes::Bytes::from(body).into(),
        ))
    }
}
