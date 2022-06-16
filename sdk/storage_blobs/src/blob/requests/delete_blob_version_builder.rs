use crate::{blob::responses::DeleteBlobResponse, prelude::*};
use azure_core::{
    error::Result,
    headers::{add_optional_header, add_optional_header_ref},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct DeleteBlobVersionBuilder<'a> {
    blob_client: &'a BlobClient,
    version_id: VersionId,
    permanent: bool,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> DeleteBlobVersionBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, version_id: VersionId) -> Self {
        Self {
            blob_client,
            version_id,
            permanent: false,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }

    setters! {
        permanent: bool => permanent,
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> Result<DeleteBlobResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);
        (&self.version_id).append_to_url_query(&mut url);
        if self.permanent {
            url.query_pairs_mut().append_pair("deletetype", "permanent");
        }

        trace!("delete_blob version id url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::DELETE,
            &|mut request| {
                request = add_optional_header_ref(&self.lease_id, request);
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::ACCEPTED)
            .await?;

        debug!("response.headers() == {:#?}", response.headers());

        Ok(DeleteBlobResponse::from_headers(response.headers())?)
    }
}
