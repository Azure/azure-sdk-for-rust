use crate::clients::QueueClient;
use azure_core::{
    error::Error,
    headers::{HeaderName, Headers},
    prelude::*,
    Method, Response as AzureResponse,
};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

operation! {
    GetQueueMetadata,
    client: QueueClient,
}

impl GetQueueMetadataBuilder {
    pub fn into_future(mut self) -> GetQueueMetadata {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "metadata");

            let mut request =
                QueueClient::finalize_request(url, Method::Get, Headers::new(), None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            response.try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetQueueMetadataResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub approximate_messages_count: usize,
    pub metadata: Metadata,
}

impl std::convert::TryFrom<AzureResponse> for GetQueueMetadataResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let headers = response.headers();

        Ok(GetQueueMetadataResponse {
            common_storage_response_headers: headers.try_into()?,
            approximate_messages_count: headers.get_as(&APPROXIMATE_MESSAGES_COUNT)?,
            metadata: headers.into(),
        })
    }
}

const APPROXIMATE_MESSAGES_COUNT: HeaderName =
    HeaderName::from_static("x-ms-approximate-messages-count");
