use crate::{blob::responses::SetBlobTierResponse, prelude::*};
use azure_core::{
    error::{Error, ErrorKind},
    headers::{add_mandatory_header, add_optional_header},
    prelude::*,
};
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
        // Get the blob properties first. Need this to determine what HTTP status code to expect later.
        let blob_properties = self.blob_client.get_properties().execute().await?;
        let blob_tier = blob_properties.blob.properties.access_tier;
        let blob_tier = match blob_tier {
            Some(bt) => bt,
            None => {
                return Err(Error::message(
                    ErrorKind::DataConversion,
                    "Unable to determine current access tier for blob.",
                ))
            }
        };

        let mut url = self.blob_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "tier");
        self.blob_versioning.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = add_mandatory_header(&self.access_tier, request);
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header(&self.rehydrate_priority, request);

                request
            },
            None,
        )?;

        info!("request == {:?}", request);

        let expected_status: http::StatusCode;

        match blob_tier {
            AccessTier::Hot | AccessTier::Cool => expected_status = http::StatusCode::OK,
            AccessTier::Archive => {
                match &self.access_tier {
                    AccessTier::Archive => expected_status = http::StatusCode::OK,
                    _ => expected_status = http::StatusCode::ACCEPTED,
                };
            }
        }

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, expected_status)
            .await?;

        response.headers().try_into()
    }
}
