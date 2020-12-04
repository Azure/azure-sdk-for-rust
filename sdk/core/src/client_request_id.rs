use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct ClientRequestId<'a>(Cow<'a, str>);

impl<'a> ClientRequestId<'a> {
    pub fn new<CRI: Into<Cow<'a, str>>>(client_request_id: CRI) -> Self {
        Self(client_request_id.into())
    }
}

impl<'a> AddAsHeader for ClientRequestId<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(CLIENT_REQUEST_ID, self.0.as_ref())
    }
}
