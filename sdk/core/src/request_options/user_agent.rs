use crate::AddAsHeader;
use http::header::USER_AGENT;
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
        builder.header(USER_AGENT, self.0)
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HttpHeaderError> {
        request
            .headers_mut()
            .append(USER_AGENT, http::HeaderValue::from_str(self.0)?);

        Ok(())
    }
}
