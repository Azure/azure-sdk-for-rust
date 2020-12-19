use crate::blob::blob::responses::PutBlockResponse;
use crate::blob::prelude::*;
use crate::core::prelude::*;
use azure_core::headers::{add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;

#[derive(Debug, Clone)]
pub struct AppendBlockBuilder<'a> {
    blob_client: &'a BlobClient,
    body: &'a [u8],
    hash: Option<&'a Hash>,
    condition_max_size: Option<ConditionMaxSize>,
    condition_append_position: Option<ConditionAppendPosition>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> AppendBlockBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, body: &'a [u8]) -> Self {
        Self {
            blob_client,
            body,
            hash: None,
            condition_max_size: None,
            condition_append_position: None,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        hash: &'a Hash => Some(hash),
        condition_max_size: ConditionMaxSize => Some(condition_max_size),
        condition_append_position: ConditionAppendPosition => Some(condition_append_position),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(
        &self,
    ) -> Result<PutBlockResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self
            .blob_client
            .storage_account_client()
            .blob_storage_url()
            .to_owned();
        url.path_segments_mut()
            .map_err(|_| "Invalid blob URL")?
            .push(self.blob_client.container_client().container_name())
            .push(self.blob_client.blob_name());

        self.timeout.append_to_url_query(&mut url);
        url.query_pairs_mut().append_pair("comp", "appendblock");

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = add_optional_header_ref(&self.hash, request);
                request = add_optional_header(&self.condition_max_size, request);
                request = add_optional_header(&self.condition_append_position, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            Some(self.body),
        )?;

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::CREATED)
            .await?;

        debug!("response.headers() == {:#?}", response.headers());

        Ok(PutBlockResponse::from_headers(response.headers())?)
    }
}
