use crate::{QueueServiceClient, QueueServiceProperties};
use azure_core::{error::Error, headers::Headers, xml::to_xml, Method, Response as AzureResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

operation! {
    SetQueueServiceProperties,
    client: QueueServiceClient,
    properties: QueueServiceProperties,
}

impl SetQueueServicePropertiesBuilder {
    pub fn into_future(mut self) -> SetQueueServiceProperties {
        Box::pin(async move {
            let mut url = self.client.url()?.clone();

            url.query_pairs_mut().append_pair("restype", "service");
            url.query_pairs_mut().append_pair("comp", "properties");

            let xml_body = to_xml(&self.properties)?;

            let mut request = QueueServiceClient::finalize_request(
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
pub struct SetQueueServicePropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<AzureResponse> for SetQueueServicePropertiesResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        Ok(SetQueueServicePropertiesResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
