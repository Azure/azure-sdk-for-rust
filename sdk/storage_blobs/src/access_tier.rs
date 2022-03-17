use azure_core::Header;

create_enum!(
    AccessTier,
    (Hot, "Hot"),
    (Cool, "Cool"),
    (Archive, "Archive")
);

impl Header for AccessTier {
    fn name(&self) -> &'static str {
        azure_core::headers::BLOB_ACCESS_TIER
    }

    fn value(&self) -> String {
        self.as_ref().to_owned()
    }
}
