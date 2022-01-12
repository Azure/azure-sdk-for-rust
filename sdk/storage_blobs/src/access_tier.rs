use azure_core::AddAsHeader;
use http::request::Builder;

create_enum!(
    AccessTier,
    (Hot, "Hot"),
    (Cool, "Cool"),
    (Archive, "Archive")
);

impl AddAsHeader for AccessTier {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(azure_core::headers::BLOB_ACCESS_TIER, self.as_ref())
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        request.headers_mut().append(
            azure_core::headers::BLOB_ACCESS_TIER,
            http::header::HeaderValue::from_str(self.as_ref())?,
        );

        Ok(())
    }
}
