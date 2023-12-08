use crate::prelude::BlobServiceClient;
use azure_core::headers::Headers;
use azure_core::{Method, Response};
use azure_storage::headers::CommonStorageResponseHeaders;
use azure_svc_blobstorage::models::StorageServiceProperties;

operation! {
    GetBlobServiceProperties,
    client: BlobServiceClient,
}

impl GetBlobServicePropertiesBuilder {
    pub fn into_future(mut self) -> GetBlobServiceProperties {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut()
                .extend_pairs([("restype", "service"), ("comp", "properties")]);

            let mut request =
                BlobServiceClient::finalize_request(url, Method::Get, Headers::new(), None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            GetBlobServicePropertiesResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetBlobServicePropertiesResponse {
    pub common: CommonStorageResponseHeaders,
    pub properties: StorageServiceProperties,
}

impl GetBlobServicePropertiesResponse {
    pub(crate) async fn try_from(
        response: Response,
    ) -> azure_core::Result<GetBlobServicePropertiesResponse> {
        let common = CommonStorageResponseHeaders::try_from(response.headers())?;
        let properties = response.xml().await?;

        Ok(GetBlobServicePropertiesResponse { common, properties })
    }
}
