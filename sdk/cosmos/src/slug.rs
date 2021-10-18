use azure_core::AddAsHeader;
use http::request::Builder;

/// The identifier of the resource.
#[derive(Debug, Clone, Copy)]
pub struct Slug<'a>(&'a str);

impl<'a> Slug<'a> {
    /// Create a `Slug` from a `&str`
    pub fn new(slug: &'a str) -> Self {
        Self(slug)
    }
}

impl<'a> AddAsHeader for Slug<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header("Slug", self.0)
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HTTPHeaderError> {
        request
            .headers_mut()
            .append("Slug", http::header::HeaderValue::from_str(self.0)?);
        Ok(())
    }
}
