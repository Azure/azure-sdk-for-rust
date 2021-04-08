use http::{HeaderMap, Method};
use std::fmt::Debug;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub url: Url,
    pub method: Method,
    pub headers: HeaderMap,
    pub payload: bytes::Bytes,
}

impl Request {
    #[allow(dead_code)]
    pub(crate) fn into_parts(self) -> (Parts, bytes::Bytes) {
        let parts = Parts {
            url: self.url,
            method: self.method,
            headers: self.headers,
        };
        (parts, self.payload)
    }

    pub fn set_payload(&mut self, bytes: bytes::Bytes) {
        self.payload = bytes;
    }
}

impl From<http::Request<bytes::Bytes>> for Request {
    fn from(request: http::Request<bytes::Bytes>) -> Self {
        let (parts, payload) = request.into_parts();
        Self {
            url: Url::parse(&parts.uri.to_string()).unwrap(),
            method: parts.method,
            headers: parts.headers,
            payload,
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

        builder.body(request.payload).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Parts {
    pub url: Url,
    pub method: Method,
    pub headers: HeaderMap,
}
