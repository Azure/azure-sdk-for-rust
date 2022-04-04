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
    fn name(&self) -> headers::HeaderName {
        headers::PROPOSED_LEASE_ID.into()
    }

    fn value(&self) -> headers::HeaderValue {
        format!("{}", self.0).into()
    }
}
