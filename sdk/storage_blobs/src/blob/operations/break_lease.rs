use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BreakLeaseBuilder {
    blob_client: BlobClient,
    lease_break_period: Option<LeaseBreakPeriod>,
    lease_id: Option<LeaseId>,
    timeout: Option<Timeout>,
    context: Context,
}

impl BreakLeaseBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            lease_break_period: None,
            lease_id: None,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        lease_break_period: LeaseBreakPeriod => Some(lease_break_period),
        lease_id: LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "lease");
            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.blob_client
                    .prepare_request(url.as_str(), azure_core::Method::PUT, None)?;
            request.insert_header(LEASE_ACTION, "break");
            request.add_optional_header(&self.lease_break_period);
            request.add_optional_header(&self.lease_id);

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
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
