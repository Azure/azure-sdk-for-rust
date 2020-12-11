use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone)]
pub struct ClientRequestId<'a>(&'a str);

impl<'a> ClientRequestId<'a> {
    pub fn new(client_request_id: &'a str) -> Self {
        Self(client_request_id)
    }
}

impl<'a> AddAsHeader for ClientRequestId<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(CLIENT_REQUEST_ID, self.0)
    }
}

impl<'a> From<&'a str> for ClientRequestId<'a> {
    fn from(client_request_id: &'a str) -> Self {
        Self::new(client_request_id)
    }
}
