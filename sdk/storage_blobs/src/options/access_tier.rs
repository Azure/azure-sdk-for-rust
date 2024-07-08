use azure_core::headers::{self, Header};

create_enum!(
    AccessTier,
    (Hot, "Hot"),
    (Cold, "Cold"),
    (Cool, "Cool"),
    (Archive, "Archive")
);

impl Header for AccessTier {
    fn name(&self) -> headers::HeaderName {
        azure_core::headers::BLOB_ACCESS_TIER
    }

    fn value(&self) -> headers::HeaderValue {
        self.as_ref().to_owned().into()
    }
}
