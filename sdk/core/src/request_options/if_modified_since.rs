use crate::{headers, AddAsHeader};
use chrono::{DateTime, Utc};
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
        builder.header(headers::IF_MODIFIED_SINCE, self.0.to_rfc2822())
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        request.headers_mut().append(
            headers::IF_MODIFIED_SINCE,
            http::HeaderValue::from_str(&self.0.to_rfc2822())?,
        );

        Ok(())
    }
}
