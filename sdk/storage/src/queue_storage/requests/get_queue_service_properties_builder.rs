use crate::core::prelude::*;
use crate::queue_storage::responses::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueServicePropertiesBuilder<'a> {
    storage_client: &'a StorageClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> GetQueueServicePropertiesBuilder<'a> {
    pub(crate) fn new(storage_client: &'a StorageClient) -> Self {
        Self {
            storage_client,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(
        &self,
    ) -> Result<GetQueueServicePropertiesResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self
            .storage_client
            .storage_account_client()
            .queue_storage_url()
            .to_owned();

        url.query_pairs_mut().append_pair("restype", "service");
        url.query_pairs_mut().append_pair("comp", "properties");

        self.timeout.append_to_url_query(&mut url);

        trace!("url == {}", url);

        let request = self.storage_client.prepare_request(
            url.as_str(),
            &Method::GET,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .storage_client
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        Ok((&response).try_into()?)
    }
}
