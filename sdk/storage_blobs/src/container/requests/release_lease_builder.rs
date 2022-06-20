use crate::{container::responses::ReleaseLeaseResponse, prelude::*};
use azure_core::{headers::LEASE_ACTION, prelude::*};
use http::method::Method;

#[derive(Debug, Clone)]
pub struct ReleaseLeaseBuilder<'a> {
    container_lease_client: &'a ContainerLeaseClient,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> ReleaseLeaseBuilder<'a> {
    pub(crate) fn new(container_lease_client: &'a ContainerLeaseClient) -> Self {
        Self {
            container_lease_client,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> azure_core::Result<ReleaseLeaseResponse> {
        let mut url = self.container_lease_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype", "container");
        url.query_pairs_mut().append_pair("comp", "lease");

        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.container_lease_client
                .prepare_request(url.as_str(), Method::PUT, None)?;
        request.insert_header(LEASE_ACTION, "release");
        request.add_optional_header(&self.client_request_id);
        request.add_mandatory_header(self.container_lease_client.lease_id());

        let response = self
            .container_lease_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        ReleaseLeaseResponse::from_headers(response.headers())
    }
}
