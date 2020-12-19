use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy)]
pub struct ContentEncoding<'a>(&'a str);

impl<'a> AddAsHeader for ContentEncoding<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(http::header::CONTENT_ENCODING, self.0)
    }
}

impl<'a, S> From<S> for ContentEncoding<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}
