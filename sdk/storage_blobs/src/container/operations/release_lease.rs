use crate::prelude::*;
use azure_core::{
    headers::{LEASE_ACTION, *},
    prelude::*,
    RequestId,
};
use chrono::{DateTime, Utc};
use http::method::Method;

#[derive(Debug, Clone)]
pub struct ReleaseLeaseBuilder {
    container_lease_client: ContainerLeaseClient,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl ReleaseLeaseBuilder {
    pub(crate) fn new(container_lease_client: ContainerLeaseClient) -> Self {
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

    pub fn into_future(self) -> Response {
        Box::pin(async move {
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
        })
    }
}

azure_storage::response_from_headers!(ReleaseLeaseResponse ,
               etag_from_headers => etag: String,
               last_modified_from_headers => last_modified: DateTime<Utc>,
               request_id_from_headers => request_id: RequestId,
               date_from_headers => date: DateTime<Utc>
);

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<ReleaseLeaseResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for ReleaseLeaseBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
