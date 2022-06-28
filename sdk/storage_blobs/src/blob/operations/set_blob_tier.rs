use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct SetBlobTierBuilder {
    blob_client: BlobClient,
    // Request Headers
    access_tier: AccessTier,
    rehydrate_priority: Option<RehydratePriority>,
    // URI Parameters
    blob_versioning: Option<BlobVersioning>,
    timeout: Option<Timeout>,
    context: Context,
}

impl SetBlobTierBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            access_tier: AccessTier::Archive,
            context: Context::new(),
            rehydrate_priority: Some(RehydratePriority::Standard),
            blob_versioning: None,
            timeout: None,
        }
    }

    setters! {
        access_tier: AccessTier => access_tier,
        rehydrate_priority: RehydratePriority => Some(rehydrate_priority),
        blob_versioning: BlobVersioning => Some(blob_versioning),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;
            url.query_pairs_mut().append_pair("comp", "tier");
            self.blob_versioning.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.access_tier);
            headers.add(self.rehydrate_priority);

            let mut request =
                self.blob_client
                    .prepare_request(url, azure_core::Method::Put, headers, None)?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;
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
    type Error = crate::Error;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        Ok(SetBlobTierResponse {
            request_id: request_id_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
            version: version_from_headers(headers)?.to_owned(),
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<SetBlobTierResponse>>;
#[cfg(feature = "into_future")]
impl std::future::IntoFuture for SetBlobTierBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
