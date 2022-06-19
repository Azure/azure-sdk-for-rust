use crate::{blob::responses::SetBlobTierResponse, prelude::*};
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetBlobTierBuilder<'a> {
    blob_client: &'a BlobClient,
    // Request Headers
    access_tier: AccessTier,
    client_request_id: Option<ClientRequestId>,
    rehydrate_priority: Option<RehydratePriority>,

    // URI Parameters
    blob_versioning: Option<&'a BlobVersioning>,
    timeout: Option<Timeout>,
}

impl<'a> SetBlobTierBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient) -> Self {
        Self {
            blob_client,
            access_tier: AccessTier::Archive,
            client_request_id: None,
            rehydrate_priority: Some(RehydratePriority::Standard),
            blob_versioning: None,
            timeout: None,
        }
    }

    setters! {
        access_tier: AccessTier => access_tier,
        client_request_id: ClientRequestId => Some(client_request_id),
        rehydrate_priority: RehydratePriority => Some(rehydrate_priority),
        blob_versioning: &'a BlobVersioning => Some(blob_versioning),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(self) -> azure_core::Result<SetBlobTierResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;
        url.query_pairs_mut().append_pair("comp", "tier");
        self.blob_versioning.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::PUT, None)?;
        request.add_mandatory_header(&self.access_tier);
        request.add_optional_header(&self.client_request_id);
        request.add_optional_header(&self.rehydrate_priority);

        let response = self
            .blob_client
            .execute_request_check_status(&request)
            .await?;

        response.headers().try_into()
    }
}
