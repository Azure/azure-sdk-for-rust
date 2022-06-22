use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct SetBlobTierBuilder {
    blob_client: BlobClient,
    // Request Headers
    access_tier: AccessTier,
    client_request_id: Option<ClientRequestId>,
    rehydrate_priority: Option<RehydratePriority>,

    // URI Parameters
    blob_versioning: Option<BlobVersioning>,
    timeout: Option<Timeout>,
}

impl SetBlobTierBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
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
        blob_versioning: BlobVersioning => Some(blob_versioning),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
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
                .http_client()
                .execute_request_check_status(&request)
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
