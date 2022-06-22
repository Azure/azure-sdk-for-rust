use azure_core::headers::HeaderName;

pub const REHYDRATE_PRIORITY: HeaderName = HeaderName::from_static("x-ms-rehydrate-priority");
