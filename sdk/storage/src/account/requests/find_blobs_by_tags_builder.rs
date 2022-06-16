use crate::account::responses::ListBlobsByTagsResponse;
use crate::core::prelude::*;
use azure_core::error::Result;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct FindBlobsByTagsBuilder<'a> {
    client: &'a StorageClient,
    expression: String,

    #[allow(unused)]
    lease_id: Option<&'a str>,
    #[allow(unused)]
    next_marker: Option<NextMarker>,
    #[allow(unused)]
    max_results: Option<MaxResults>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> FindBlobsByTagsBuilder<'a> {
    pub(crate) fn new(client: &'a StorageClient) -> Self {
        Self {
            client,
            expression: "".to_string(),
            lease_id: None,
            next_marker: None,
            max_results: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        expression: String => expression,
        next_marker: NextMarker => Some(next_marker),
        max_results: MaxResults => Some(max_results),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> Result<ListBlobsByTagsResponse> {
        let mut url = self
            .client
            .storage_account_client()
            .blob_storage_url()
            .to_owned();

        self.timeout.append_to_url_query(&mut url);
        url.query_pairs_mut().append_pair("comp", "blobs");
        url.query_pairs_mut().append_pair("where", &self.expression);

        trace!("url == {:?}", url);

        let (request, _url) = self.client.prepare_request(
            url.as_str(),
            &http::Method::GET,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::OK)
            .await?;

        debug!("response.headers() == {:#?}", response.headers());

        (&response).try_into()
    }
}
