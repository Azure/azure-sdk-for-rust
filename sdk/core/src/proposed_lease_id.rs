use crate::headers::*;
use crate::{AddAsHeader, LeaseId};
use http::request::Builder;

#[derive(Debug, Clone, Copy)]
pub struct ProposedLeaseId(LeaseId);

impl AddAsHeader for ProposedLeaseId {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(PROPOSED_LEASE_ID, &format!("{}", self.0))
    }
}

impl From<LeaseId> for ProposedLeaseId {
    fn from(lease_id: LeaseId) -> Self {
        Self(lease_id)
    }
}
