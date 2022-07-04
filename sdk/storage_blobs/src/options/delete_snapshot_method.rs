use azure_core::headers::{self, Header};

create_enum!(DeleteSnapshotsMethod, (Include, "include"), (Only, "only"));

impl Header for DeleteSnapshotsMethod {
    fn name(&self) -> headers::HeaderName {
        azure_core::headers::DELETE_SNAPSHOTS
    }

    fn value(&self) -> headers::HeaderValue {
        self.to_string().into()
    }
}
