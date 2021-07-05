use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy)]
pub struct ContentEncoding<'a>(&'a str);

impl<'a, S> From<S> for ContentEncoding<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> AddAsHeader for ContentEncoding<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(http::header::CONTENT_ENCODING, self.0)
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        request.headers_mut().append(
            http::header::CONTENT_ENCODING,
            http::HeaderValue::from_str(self.0)?,
        );

        Ok(())
    }
}
