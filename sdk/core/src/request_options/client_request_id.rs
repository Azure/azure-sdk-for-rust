use crate::headers;
use crate::Header;

#[derive(Debug, Clone)]
pub struct ClientRequestId(String);

impl ClientRequestId {
    pub fn new(client_request_id: String) -> Self {
        Self(client_request_id)
    }
}

impl From<String> for ClientRequestId {
    fn from(client_request_id: String) -> Self {
        Self::new(client_request_id)
    }
}

impl From<&str> for ClientRequestId {
    fn from(client_request_id: &str) -> Self {
        Self::new(client_request_id.into())
    }
}

impl Header for ClientRequestId {
    fn name(&self) -> &'static str {
        headers::CLIENT_REQUEST_ID
    }

    fn value(&self) -> String {
        self.0.to_owned()
    }
}
