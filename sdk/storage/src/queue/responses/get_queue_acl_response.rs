use crate::{QueueStoredAccessPolicy, StoredAccessPolicyList};
use azure_core::headers::CommonStorageResponseHeaders;
use azure_core::PermissionError;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueACLResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub stored_access_policies: Vec<QueueStoredAccessPolicy>,
}

impl std::convert::TryFrom<&Response<Bytes>> for GetQueueACLResponse {
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body = response.body();

        debug!("headers == {:?}", headers);

        let a: Result<Vec<QueueStoredAccessPolicy>, PermissionError> =
            StoredAccessPolicyList::from_xml(&std::str::from_utf8(body)?[3..])?
                .stored_access
                .into_iter()
                .map(|sap| sap.try_into())
                .collect();

        Ok(GetQueueACLResponse {
            common_storage_response_headers: headers.try_into()?,
            stored_access_policies: a?,
        })
    }
}
