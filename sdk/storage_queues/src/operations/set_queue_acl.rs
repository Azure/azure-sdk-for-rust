use crate::{clients::QueueClient, QueueStoredAccessPolicy};
use azure_core::{error::Error, headers::Headers, Method, Response as AzureResponse};
use azure_storage::{headers::CommonStorageResponseHeaders, StoredAccessPolicyList};
use std::convert::TryInto;

operation! {
    SetQueueACL,
    client: QueueClient,
    policies: Vec<QueueStoredAccessPolicy>,
}

impl SetQueueACLBuilder {
    pub fn into_future(mut self) -> SetQueueACL {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "acl");

            // convert the queue_stored_access_policies slice
            // in a StoredAccessPolicyList to get its XML
            // representation.
            let xml_body = {
                let stored_access = self.policies.into_iter().map(Into::into).collect();
                StoredAccessPolicyList::new(stored_access).to_xml()
            };

            let mut request = QueueClient::finalize_request(
                url,
                Method::Put,
                Headers::new(),
                Some(xml_body.into()),
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            response.try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetQueueACLResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<AzureResponse> for SetQueueACLResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        Ok(SetQueueACLResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
