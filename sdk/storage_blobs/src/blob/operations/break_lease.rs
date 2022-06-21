use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BreakLeaseBuilder {
    blob_client: BlobClient,
    lease_break_period: Option<LeaseBreakPeriod>,
    lease_id: Option<LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl BreakLeaseBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            lease_break_period: None,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        lease_break_period: LeaseBreakPeriod => Some(lease_break_period),
        lease_id: LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "lease");
            self.timeout.append_to_url_query(&mut url);

            trace!("url == {:?}", url);

            let (request, _url) = self.blob_client.prepare_request(
                url.as_str(),
                &http::Method::PUT,
                &|mut request| {
                    request = request.header(LEASE_ACTION, "break");
                    request = add_optional_header(&self.lease_break_period, request);
                    request = add_optional_header_ref(&self.lease_id.as_ref(), request);
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

            BreakLeaseResponse::from_headers(response.headers())
        })
    }
}

azure_storage::response_from_headers!(BreakLeaseResponse ,
 etag_from_headers => etag: String,
 last_modified_from_headers => last_modified: DateTime<Utc>,
 lease_time_from_headers => lease_time: u8,
 request_id_from_headers => request_id: RequestId,
 date_from_headers => date: DateTime<Utc>
);

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<BreakLeaseResponse>>;
