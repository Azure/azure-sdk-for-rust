use crate::blob::responses::SetBlobMetadataResponse;
use crate::prelude::*;
use azure_core::headers::{add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetBlobMetadataBuilder<'a> {
    blob_client: &'a BlobClient,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    metadata: Option<&'a Metadata>,
}

impl<'a> SetBlobMetadataBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient) -> Self {
        Self {
            blob_client,
            lease_id: None,
            client_request_id: None,
            timeout: None,
            metadata: None,
        }
    }

    setters! {
        lease_id: &'a LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        metadata: &'a Metadata => Some(metadata),
    }

    pub async fn execute(
        self,
    ) -> Result<SetBlobMetadataResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self.blob_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "metadata");
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request = add_optional_header(&self.metadata, request);
                request
            },
            None,
        )?;

        info!("request == {:?}", request);

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::OK)
            .await?;

        Ok(response.headers().try_into()?)
    }
}
