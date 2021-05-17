use crate::errors::HttpError;

pub struct Request {
    inner: http::Request<bytes::Bytes>,
}

impl Request {
    /// Get the inner http::Request object, replacing it
    /// with an empty one.
    /// Note: this method will soon be replaced
    pub fn take_inner(&mut self) -> http::Request<bytes::Bytes> {
        std::mem::replace(&mut self.inner, http::Request::new(bytes::Bytes::new()))
    }

    pub fn body<T: serde::Serialize>(&mut self, body: T) -> Result<(), HttpError> {
        let b = self.inner.body_mut();
        *b = crate::to_json(&body).map_err(HttpError::BodySerializationError)?;
        Ok(())
    }
}

impl From<http::Request<bytes::Bytes>> for Request {
    fn from(inner: http::Request<bytes::Bytes>) -> Self {
        Self { inner }
    }
}

pub struct Response {
    inner: http::Response<bytes::Bytes>,
}

impl Response {
    /// TODO: get rid of this
    pub fn into_inner(self) -> http::Response<bytes::Bytes> {
        self.inner
    }
}

impl From<http::Response<bytes::Bytes>> for Response {
    fn from(inner: http::Response<bytes::Bytes>) -> Self {
        Self { inner }
    }
}
