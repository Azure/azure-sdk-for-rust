use crate::headers;
use crate::Header;

/// The (friendly) version identifier for the client making the request
#[derive(Debug, Clone)]
pub struct ClientVersion(String);

impl ClientVersion {
    pub fn new(client_request_id: String) -> Self {
        Self(client_request_id)
    }
}

impl<S> From<S> for ClientVersion
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Header for ClientVersion {
    fn name(&self) -> headers::HeaderName {
        headers::CLIENT_VERSION.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
