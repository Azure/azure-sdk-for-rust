use azure::core::lease_id::LeaseId;

#[derive(Debug, Clone, PartialEq)]
pub struct PutBlockOptions {
    pub lease_id: Option<LeaseId>,
    pub timeout: Option<u64>,
    pub request_id: Option<String>,
}

pub const PUT_BLOCK_OPTIONS_DEFAULT: PutBlockOptions = PutBlockOptions {
    timeout: None,
    lease_id: None,
    request_id: None,
};
