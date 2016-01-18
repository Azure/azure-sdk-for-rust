use azure::core::lease::LeaseId;

#[derive(Debug, Clone, PartialEq)]
pub struct PutOptions {
    pub lease_id: Option<LeaseId>,
    pub timeout: Option<u64>,
}

pub const PUT_OPTIONS_DEFAULT: PutOptions = PutOptions {
    timeout: None,
    lease_id: None,
};
