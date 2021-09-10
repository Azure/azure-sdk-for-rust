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

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HTTPHeaderError> {
        request
            .headers_mut()
            .append(headers::USER_AGENT, http::HeaderValue::from_str(self.0)?);

        Ok(())
    }
}
