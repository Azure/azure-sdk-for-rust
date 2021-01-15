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
}
