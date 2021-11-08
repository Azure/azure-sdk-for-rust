use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenameSource<'a>(&'a str);

impl<'a> RenameSource<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl<'a, S> From<S> for RenameSource<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> AddAsHeader for RenameSource<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(RENAME_SOURCE, self.0)
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HTTPHeaderError> {
        request
            .headers_mut()
            .append(RENAME_SOURCE, http::HeaderValue::from_str(self.0)?);

        Ok(())
    }
}
