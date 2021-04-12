use http::{HeaderMap, Method};
use std::fmt::Debug;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub url: Url,
    pub method: Method,
    pub headers: HeaderMap,
    pub body: bytes::Bytes,
}

impl Request {
    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn body(&self) -> bytes::Bytes {
        self.body.clone()
    }

    pub fn set_body(&mut self, bytes: bytes::Bytes) {
        self.body = bytes;
    }
}

impl From<http::Request<bytes::Bytes>> for Request {
    fn from(request: http::Request<bytes::Bytes>) -> Self {
        let (parts, body) = request.into_parts();
        Self {
            url: Url::parse(&parts.uri.to_string()).unwrap(),
            method: parts.method,
            headers: parts.headers,
            body,
        }
    }
}

impl From<Request> for http::Request<bytes::Bytes> {
    fn from(request: Request) -> Self {
        let mut builder = http::Request::builder()
            .uri(request.url.as_str().parse::<http::Uri>().unwrap())
            .method(request.method);

        for (k, v) in request.headers {
            builder = builder.header(k.unwrap(), v);
        }

        builder.body(request.body).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Parts {
    pub url: Url,
    pub method: Method,
    pub headers: HeaderMap,
}
