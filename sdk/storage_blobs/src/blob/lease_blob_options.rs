use azure_core::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseBlobOptions {
    pub lease_id: Option<LeaseId>,
    pub timeout: Option<u64>,
    pub lease_break_period: Option<u32>,
    pub lease_duration: Option<u32>,
    pub proposed_lease_id: Option<LeaseId>,
    pub request_id: Option<String>,
}

pub const LEASE_BLOB_OPTIONS_DEFAULT: LeaseBlobOptions = LeaseBlobOptions {
    lease_id: None,
    timeout: None,
    lease_break_period: None,
    lease_duration: None,
    proposed_lease_id: None,
    request_id: None,
};
