use azure_core::Header;

create_enum!(DeleteSnapshotsMethod, (Include, "include"), (Only, "only"));

impl Header for DeleteSnapshotsMethod {
    fn name(&self) -> &'static str {
        azure_core::headers::DELETE_SNAPSHOTS
    }

    fn value(&self) -> String {
        self.to_string()
    }
}
