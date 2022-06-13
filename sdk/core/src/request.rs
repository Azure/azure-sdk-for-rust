use crate::error::{ErrorKind, Result, ResultExt};
use crate::headers::{AsHeaders, Headers};
use crate::SeekableStream;
use bytes::Bytes;
use http::{Method, Uri};
use std::fmt::Debug;
use std::str::FromStr;

/// An HTTP Body.
#[derive(Debug, Clone)]
pub enum Body {
    /// A body of a known size.
    Bytes(bytes::Bytes),
    /// A streaming body.
    SeekableStream(Box<dyn SeekableStream>),
}

impl<B> From<B> for Body
where
    B: Into<Bytes>,
{
    fn from(bytes: B) -> Self {
        Self::Bytes(bytes.into())
    }
}

impl From<Box<dyn SeekableStream>> for Body {
    fn from(seekable_stream: Box<dyn SeekableStream>) -> Self {
        Self::SeekableStream(seekable_stream)
    }
}

/// A pipeline request.
///
/// A pipeline request is composed by a destination (uri), a method, a collection of headers and a
/// body. Policies are expected to enrich the request by mutating it.
#[derive(Debug, Clone)]
pub struct Request {
    pub(crate) uri: Uri,
    pub(crate) method: Method,
    pub(crate) headers: Headers,
    pub(crate) body: Body,
}

impl Request {
    /// Create a new request with an empty body and no headers
    pub fn new(uri: Uri, method: Method) -> Self {
        Self {
            uri,
            method,
            headers: Headers::new(),
            body: Body::Bytes(bytes::Bytes::new()),
        }
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn insert_headers<T: AsHeaders>(&mut self, headers: &T) {
        for (name, value) in headers.as_headers() {
            self.headers_mut().insert(name, value)
        }
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn set_body(&mut self, body: impl Into<Body>) {
        self.body = body.into();
    }

    /// Parse a `Uri` from a `str`
    pub fn parse_uri(uri: &str) -> Result<Uri> {
        Uri::from_str(uri).map_kind(ErrorKind::DataConversion)
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
            headers: parts.headers.into(),
            body: Body::Bytes(body),
        }
    }
}
