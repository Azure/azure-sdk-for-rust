use azure::core::lease_id::LeaseId;

#[derive(Debug, Clone, PartialEq)]
pub struct PutPageOptions {
    pub lease_id: Option<LeaseId>,
    pub timeout: Option<u64>,
}

pub const PUT_PAGE_OPTIONS_DEFAULT: PutPageOptions = PutPageOptions {
    timeout: None,
    lease_id: None,
};
