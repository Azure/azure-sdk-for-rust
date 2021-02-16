use crate::{headers, AddAsHeader};
use http::request::Builder;

#[derive(Debug, Clone, Copy)]
pub struct UserAgent<'a>(&'a str);

impl<'a> UserAgent<'a> {
    pub fn new(agent: &'a str) -> Self {
        Self(agent)
    }
}

impl<'a> AddAsHeader for UserAgent<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::USER_AGENT, self.0)
    }
}
