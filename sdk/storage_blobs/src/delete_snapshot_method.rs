use azure_core::AddAsHeader;
use http::request::Builder;

create_enum!(DeleteSnapshotsMethod, (Include, "include"), (Only, "only"));

impl AddAsHeader for DeleteSnapshotsMethod {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(azure_core::headers::DELETE_SNAPSHOTS, format!("{}", self))
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        request.headers_mut().append(
            azure_core::headers::DELETE_SNAPSHOTS,
            http::header::HeaderValue::from_str(&self.to_string())?,
        );

        Ok(())
    }
}
