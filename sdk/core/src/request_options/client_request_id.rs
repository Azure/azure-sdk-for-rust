use crate::headers;
use crate::Header;

/// A unique identifier for the request
#[derive(Debug, Clone)]
pub struct ClientRequestId(String);

impl ClientRequestId {
    pub fn new(client_request_id: String) -> Self {
        Self(client_request_id)
    }
}

impl<S> From<S> for ClientRequestId
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Header for ClientRequestId {
    fn name(&self) -> headers::HeaderName {
        headers::CLIENT_REQUEST_ID
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
