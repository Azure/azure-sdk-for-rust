use crate::blob::blob::responses::ReleaseBlobLeaseResponse;
use crate::core::prelude::*;
use azure_core::headers::LEASE_ACTION;
use azure_core::headers::{add_mandatory_header, add_optional_header};
use azure_core::prelude::*;

#[derive(Debug, Clone)]
pub struct ReleaseLeaseBuilder<'a> {
    blob_lease_client: &'a BlobLeaseClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> ReleaseLeaseBuilder<'a> {
    pub(crate) fn new(blob_lease_client: &'a BlobLeaseClient) -> Self {
        Self {
            blob_lease_client,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(
        &self,
    ) -> Result<ReleaseBlobLeaseResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self
            .blob_lease_client
            .storage_account_client()
            .blob_storage_url()
            .to_owned();
        url.path_segments_mut()
            .map_err(|_| "Invalid blob URL")?
            .push(self.blob_lease_client.container_client().container_name())
            .push(self.blob_lease_client.blob_client().blob_name());

        url.query_pairs_mut().append_pair("comp", "lease");
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_lease_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = request.header(LEASE_ACTION, "release");
                request = add_mandatory_header(self.blob_lease_client.lease_id(), request);
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .blob_lease_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::OK)
            .await?;

        Ok(ReleaseBlobLeaseResponse::from_headers(response.headers())?)
    }
}
