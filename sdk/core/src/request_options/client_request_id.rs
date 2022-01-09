use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;

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
        Self::new(client_request_id.to_owned())
    }
}

impl AddAsHeader for ClientRequestId {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(CLIENT_REQUEST_ID, &self.0)
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HTTPHeaderError> {
        request
            .headers_mut()
            .append(CLIENT_REQUEST_ID, http::HeaderValue::from_str(&self.0)?);

        Ok(())
    }
}
