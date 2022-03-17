use super::LeaseId;
use crate::{headers, Header};

#[derive(Debug, Clone, Copy)]
pub struct ProposedLeaseId(LeaseId);

impl From<LeaseId> for ProposedLeaseId {
    fn from(lease_id: LeaseId) -> Self {
        Self(lease_id)
    }
}

impl Header for ProposedLeaseId {
    fn name(&self) -> &'static str {
        headers::PROPOSED_LEASE_ID
    }

    fn value(&self) -> String {
        format!("{}", self.0)
    }
}
