use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContentType<'a>(&'a str);

impl<'a> AddAsHeader for ContentType<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(http::header::CONTENT_TYPE, self.0)
    }
}

impl<'a, S> From<S> for ContentType<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}
