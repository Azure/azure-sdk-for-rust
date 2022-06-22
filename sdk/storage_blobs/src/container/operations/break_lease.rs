use crate::prelude::*;
use azure_core::{
    headers::{LEASE_ACTION, *},
    prelude::*,
    RequestId,
};
use chrono::{DateTime, Utc};
use http::method::Method;

#[derive(Debug, Clone)]
pub struct BreakLeaseBuilder {
    container_client: ContainerClient,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    lease_break_period: Option<LeaseBreakPeriod>,
    lease_id: Option<LeaseId>,
}

impl BreakLeaseBuilder {
    pub(crate) fn new(container_client: ContainerClient) -> BreakLeaseBuilder {
        Self {
            container_client,
            client_request_id: None,
            timeout: None,
            lease_break_period: None,
            lease_id: None,
        }
    }

    setters! {
        lease_id: LeaseId => Some(lease_id),
        lease_break_period: LeaseBreakPeriod => Some(lease_break_period),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("restype", "container");
            url.query_pairs_mut().append_pair("comp", "lease");

            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.container_client
                    .prepare_request(url.as_str(), Method::PUT, None)?;
            request.insert_header(LEASE_ACTION, "break");
            request.add_optional_header(&self.client_request_id);
            request.add_optional_header(&self.lease_id);
            request.add_optional_header(&self.lease_break_period);

            let response = self
                .container_client
                .storage_client()
                .storage_account_client()
                .http_client()
                .execute_request_check_status(&request)
                .await?;

            BreakLeaseResponse::from_headers(response.headers())
        })
    }
}

azure_storage::response_from_headers!(BreakLeaseResponse,
    etag_from_headers => etag: String,
    last_modified_from_headers => last_modified: DateTime<Utc>,
    lease_time_from_headers => lease_time: u8,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: DateTime<Utc>
);

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<BreakLeaseResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for BreakLeaseBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
