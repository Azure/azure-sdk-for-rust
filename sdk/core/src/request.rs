use crate::SeekableStream;
use http::{HeaderMap, Method, Uri};
use std::fmt::Debug;

/// An HTTP Body.
#[derive(Debug, Clone)]
pub enum Body {
    /// A body of a known size.
    Bytes(bytes::Bytes),
    /// A streaming body.
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

/// A pipeline request.
///
/// A pipeline request is composed by a destination (uri), a method, a collection of headers and a
/// body. Policies are expected to enrich the request by mutating it.
#[derive(Debug, Clone)]
pub struct Request {
    pub(crate) uri: Uri,
    pub(crate) method: Method,
    pub(crate) headers: HeaderMap,
    pub(crate) body: Body,
}

impl Request {
    /// Create a new request with an empty body and no headers
    pub fn new(uri: Uri, method: Method) -> Self {
        Self {
            uri,
            method,
            headers: HeaderMap::default(),
            body: Body::Bytes(bytes::Bytes::new()),
        }
    }

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
