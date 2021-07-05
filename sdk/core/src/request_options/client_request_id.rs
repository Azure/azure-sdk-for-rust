use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy)]
pub struct ClientRequestId<'a>(&'a str);

impl<'a> ClientRequestId<'a> {
    pub fn new(client_request_id: &'a str) -> Self {
        Self(client_request_id)
    }
}

impl<'a> From<&'a str> for ClientRequestId<'a> {
    fn from(client_request_id: &'a str) -> Self {
        Self::new(client_request_id)
    }
}

impl<'a> AddAsHeader for ClientRequestId<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(CLIENT_REQUEST_ID, self.0)
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        request
            .headers_mut()
            .append(CLIENT_REQUEST_ID, http::HeaderValue::from_str(self.0)?);

        Ok(())
    }
}
