use crate::clients::QueueClient;
use azure_core::{error::Error, headers::Headers, prelude::*, Method, Response as AzureResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

operation! {
    SetQueueMetadata,
    client: QueueClient,
    metadata: Metadata,
}

impl SetQueueMetadataBuilder {
    pub fn into_future(mut self) -> SetQueueMetadata {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "metadata");

            let mut headers = Headers::new();
            for m in self.metadata.iter() {
                headers.add(m);
            }

            let mut request = QueueClient::finalize_request(url, Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            response.try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetQueueMetadataResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<AzureResponse> for SetQueueMetadataResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        Ok(SetQueueMetadataResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
