use crate::{clients::QueueClient, QueueStoredAccessPolicy};
use azure_core::{headers::Headers, Method, Response as AzureResponse};
use azure_storage::{core::headers::CommonStorageResponseHeaders, StoredAccessPolicyList};
use std::convert::TryInto;

operation! {
    GetQueueACL,
    client: QueueClient,
}

impl GetQueueACLBuilder {
    pub fn into_future(mut self) -> GetQueueACL {
        Box::pin(async move {
            let mut url = self.client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "acl");

            let mut request = self.client.storage_client().finalize_request(
                url,
                Method::Get,
                Headers::new(),
                None,
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            GetQueueACLResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetQueueACLResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub stored_access_policies: Vec<QueueStoredAccessPolicy>,
}

impl GetQueueACLResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        let a: azure_core::Result<Vec<QueueStoredAccessPolicy>> =
            StoredAccessPolicyList::from_xml(&body)?
                .stored_access
                .into_iter()
                .map(|sap| sap.try_into())
                .collect();

        Ok(GetQueueACLResponse {
            common_storage_response_headers: (&headers).try_into()?,
            stored_access_policies: a?,
        })
    }
}
