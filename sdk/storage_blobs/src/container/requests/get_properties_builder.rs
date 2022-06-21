use crate::{container::responses::GetPropertiesResponse, prelude::*};
use azure_core::prelude::*;
use http::method::Method;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetPropertiesBuilder<'a> {
    container_client: &'a ContainerClient,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a> GetPropertiesBuilder<'a> {
    pub(crate) fn new(container_client: &'a ContainerClient) -> Self {
        Self {
            container_client,
            client_request_id: None,
            timeout: None,
            lease_id: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
    }

    pub async fn execute(&self) -> azure_core::Result<GetPropertiesResponse> {
        let mut url = self.container_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype", "container");

        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.container_client
                .prepare_request(url.as_str(), Method::HEAD, None)?;
        request.add_optional_header(&self.client_request_id);
        request.add_optional_header_ref(&self.lease_id);

        let response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        (self.container_client.container_name(), response.headers()).try_into()
    }
}
