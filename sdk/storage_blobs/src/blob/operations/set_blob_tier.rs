use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use std::convert::{TryFrom, TryInto};

operation! {
    SetBlobTier,
    client: BlobClient,
    access_tier: AccessTier,
    ?rehydrate_priority: RehydratePriority,
    ?blob_versioning: BlobVersioning,
    ?if_tags: IfTags
}

impl SetBlobTierBuilder {
    pub fn into_future(mut self) -> SetBlobTier {
        Box::pin(async move {
            let mut url = self.client.url()?;
            url.query_pairs_mut().append_pair("comp", "tier");
            self.blob_versioning.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.access_tier);
            headers.add(
                self.rehydrate_priority
                    .unwrap_or(RehydratePriority::Standard),
            );
            headers.add(self.if_tags);

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            response.headers().try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetBlobTierResponse {
    pub request_id: RequestId,
    pub client_request_id: Option<String>,
    pub version: String,
}

impl TryFrom<&Headers> for SetBlobTierResponse {
    type Error = azure_core::Error;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        Ok(SetBlobTierResponse {
            request_id: request_id_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
            version: version_from_headers(headers)?,
        })
    }
}
