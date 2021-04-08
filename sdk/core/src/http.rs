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
