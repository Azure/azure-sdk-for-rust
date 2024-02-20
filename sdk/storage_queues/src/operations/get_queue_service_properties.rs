use crate::{QueueServiceClient, QueueServiceProperties};
use azure_core::{headers::Headers, Method, Response as AzureResponse};
use azure_storage::headers::CommonStorageResponseHeaders;

operation! {
    GetQueueServiceProperties,
    client: QueueServiceClient,
}

impl GetQueueServicePropertiesBuilder {
    pub fn into_future(mut self) -> GetQueueServiceProperties {
        Box::pin(async move {
            let mut url = self.client.url()?.clone();

            url.query_pairs_mut().append_pair("restype", "service");
            url.query_pairs_mut().append_pair("comp", "properties");

            let mut request =
                QueueServiceClient::finalize_request(url, Method::Get, Headers::new(), None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            GetQueueServicePropertiesResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetQueueServicePropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub queue_service_properties: QueueServiceProperties,
}

impl GetQueueServicePropertiesResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let queue_service_properties = body.xml().await?;

        Ok(GetQueueServicePropertiesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            queue_service_properties,
        })
    }
}
