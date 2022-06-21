use crate::QueueStoredAccessPolicy;
use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_core::CollectedResponse;
use azure_storage::core::headers::CommonStorageResponseHeaders;
use azure_storage::StoredAccessPolicyList;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueACLResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub stored_access_policies: Vec<QueueStoredAccessPolicy>,
}

impl std::convert::TryFrom<CollectedResponse> for GetQueueACLResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let headers = response.headers();
        let body = response.body();

        let a: azure_core::Result<Vec<QueueStoredAccessPolicy>> =
            StoredAccessPolicyList::from_xml(body)
                .map_kind(ErrorKind::DataConversion)?
                .stored_access
                .into_iter()
                .map(|sap| sap.try_into().map_kind(ErrorKind::DataConversion))
                .collect();

        Ok(GetQueueACLResponse {
            common_storage_response_headers: headers.try_into()?,
            stored_access_policies: a?,
        })
    }
}
