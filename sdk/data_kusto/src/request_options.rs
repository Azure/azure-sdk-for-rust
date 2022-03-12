use azure_core::{AddAsHeader, HttpHeaderError, Request};
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Accept<'a>(&'a str);

impl<'a> Accept<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        self.0
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcceptEncoding<'a>(&'a str);

impl<'a> AcceptEncoding<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl<'a, S> From<S> for AcceptEncoding<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> AddAsHeader for AcceptEncoding<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(http::header::ACCEPT_ENCODING, self.0)
    }

    fn add_as_header2(&self, request: &mut Request) -> Result<(), HttpHeaderError> {
        request.headers_mut().append(
            http::header::ACCEPT_ENCODING,
            http::HeaderValue::from_str(self.0)?,
        );

        Ok(())
    }
}
