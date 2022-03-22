use crate::{AddAsHeader, HttpHeaderError, Request};
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Accept<'a>(&'a str);

impl<'a> Accept<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a, S> From<S> for Accept<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> AddAsHeader for Accept<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(http::header::ACCEPT, self.0)
    }

    fn add_as_header2(&self, request: &mut Request) -> Result<(), HttpHeaderError> {
        request
            .headers_mut()
            .append(http::header::ACCEPT, http::HeaderValue::from_str(self.0)?);

        Ok(())
    }
}
