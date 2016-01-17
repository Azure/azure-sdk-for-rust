use azure::core::lease_id::LeaseId;

#[derive(Debug, Clone, PartialEq)]
pub struct LeaseBlobOptions {
    pub lease_id: Option<LeaseId>,
    pub timeout: Option<u64>,
}

pub const LEASE_BLOB_OPTIONS_DEFAULT: LeaseBlobOptions = LeaseBlobOptions {
    lease_id: None,
    timeout: None,
};
