use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy)]
pub struct ContentLanguage<'a>(&'a str);

impl<'a> AddAsHeader for ContentLanguage<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(http::header::CONTENT_LANGUAGE, self.0)
    }
}

impl<'a, S> From<S> for ContentLanguage<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}
