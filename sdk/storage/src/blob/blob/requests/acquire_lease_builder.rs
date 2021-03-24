use crate::blob::blob::responses::AcquireBlobLeaseResponse;
use crate::blob::prelude::*;
use azure_core::headers::LEASE_ACTION;
use azure_core::headers::{add_mandatory_header, add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;

#[derive(Debug, Clone)]
pub struct AcquireLeaseBuilder<'a> {
    blob_client: &'a BlobClient,
    lease_duration: LeaseDuration,
    lease_id: Option<&'a LeaseId>,
    proposed_lease_id: Option<&'a ProposedLeaseId>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> AcquireLeaseBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, lease_duration: LeaseDuration) -> Self {
        Self {
            blob_client,
            lease_duration,
            lease_id: None,
            proposed_lease_id: None,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        lease_id: &'a LeaseId => Some(lease_id),
        proposed_lease_id: &'a ProposedLeaseId => Some(proposed_lease_id),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(
        self,
    ) -> Result<AcquireBlobLeaseResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self
            .blob_client
            .storage_account_client()
            .blob_storage_url()
            .to_owned();
        url.path_segments_mut()
            .map_err(|_| "Invalid blob URL")?
            .push(self.blob_client.container_client().container_name())
            .push(self.blob_client.blob_name());

        url.query_pairs_mut().append_pair("comp", "lease");
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = request.header(LEASE_ACTION, "acquire");
                request = add_mandatory_header(&self.lease_duration, request);
                request = add_optional_header_ref(&self.proposed_lease_id, request);
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request
            },
            None,
        )?;

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::CREATED)
            .await?;

        Ok(AcquireBlobLeaseResponse::from_headers(response.headers())?)
    }
}
