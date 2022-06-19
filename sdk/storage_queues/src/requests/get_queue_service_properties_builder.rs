use crate::responses::*;
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use http::method::Method;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueServicePropertiesBuilder<'a> {
    storage_client: &'a StorageClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
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
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<GetQueueServicePropertiesResponse> {
        let mut url = self
            .storage_client
            .storage_account_client()
            .queue_storage_url()
            .to_owned();

        url.query_pairs_mut().append_pair("restype", "service");
        url.query_pairs_mut().append_pair("comp", "properties");

        self.timeout.append_to_url_query(&mut url);

        let mut request = self
            .storage_client
            .prepare_request(url.as_str(), Method::GET, None)?;
        request.add_optional_header(&self.client_request_id);

        let response = self
            .storage_client
            .storage_account_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}
