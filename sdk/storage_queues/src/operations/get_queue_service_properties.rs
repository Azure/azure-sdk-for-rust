use crate::{QueueServiceClient, QueueServiceProperties};
use azure_core::{headers::Headers, xml::read_xml, Method, Response as AzureResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

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
        let body = body.collect().await?;

        let queue_service_properties: QueueServiceProperties = read_xml(&body)?;

        Ok(GetQueueServicePropertiesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            queue_service_properties,
        })
    }
}
