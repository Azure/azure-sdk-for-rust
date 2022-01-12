use crate::AddAsHeader;
use chrono::{DateTime, Utc};
use http::header::IF_MODIFIED_SINCE;
use http::request::Builder;

#[derive(Debug, Clone, Copy)]
pub struct IfModifiedSince<'a>(&'a DateTime<Utc>);

impl<'a> IfModifiedSince<'a> {
    pub fn new(time: &'a DateTime<Utc>) -> Self {
        Self(time)
    }
}

impl AddAsHeader for IfModifiedSince<'_> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(IF_MODIFIED_SINCE, self.0.to_rfc2822())
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HttpHeaderError> {
        request.headers_mut().append(
            IF_MODIFIED_SINCE,
            http::HeaderValue::from_str(&self.0.to_rfc2822())?,
        );

        Ok(())
    }
}
