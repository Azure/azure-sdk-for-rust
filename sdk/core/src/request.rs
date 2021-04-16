use http::{HeaderMap, Method, Uri};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Body {
    Bytes(bytes::Bytes),
}

impl From<bytes::Bytes> for Body {
    fn from(bytes: bytes::Bytes) -> Self {
        Self::Bytes(bytes)
    }
}

/// A pipeline request.
///
/// A pipeline request is composed by a destination (uri), a method, a collection of headers and a
/// body. Policies are expected to enrich the request by mutating it.
#[derive(Debug)]
pub struct Request {
    uri: Uri,
    method: Method,
    headers: HeaderMap,
    body: Body,
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

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn set_body(&mut self, body: Body) {
        self.body = body;
    }

    /// Swaps the body with an empty one an returns ownership on the internal one. Care must be
    /// taken to make sure the body is preserved between retries.
    pub fn extract_body(&mut self) -> Body {
        let mut b = Body::Bytes(bytes::Bytes::new());
        std::mem::swap(&mut self.body, &mut b);
        b
    }
}

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
