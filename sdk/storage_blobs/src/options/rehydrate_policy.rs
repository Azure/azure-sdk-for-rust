use azure_core::Header;

create_enum!(RehydratePriority, (High, "High"), (Standard, "Standard"));

impl Header for RehydratePriority {
    fn name(&self) -> azure_core::headers::HeaderName {
        "x-ms-rehydrate-priority".into()
    }

    fn value(&self) -> azure_core::headers::HeaderValue {
        self.to_string().into()
    }
}
