use super::LeaseId;
use crate::{headers, AddAsHeader};
use http::request::Builder;

#[derive(Debug, Clone, Copy)]
pub struct ProposedLeaseId(LeaseId);

impl From<LeaseId> for ProposedLeaseId {
    fn from(lease_id: LeaseId) -> Self {
        Self(lease_id)
    }
}

impl AddAsHeader for ProposedLeaseId {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::PROPOSED_LEASE_ID, &format!("{}", self.0))
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HttpHeaderError> {
        request.headers_mut().append(
            crate::PROPOSED_LEASE_ID,
            http::HeaderValue::from_str(&format!("{}", self.0))?,
        );

        Ok(())
    }
}
