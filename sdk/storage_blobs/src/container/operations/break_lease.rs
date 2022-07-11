use crate::prelude::*;
use azure_core::Method;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BreakLeaseBuilder {
    container_client: ContainerClient,
    lease_break_period: Option<LeaseBreakPeriod>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl BreakLeaseBuilder {
    pub(crate) fn new(container_client: ContainerClient) -> BreakLeaseBuilder {
        Self {
            container_client,
            lease_break_period: None,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        lease_id: LeaseId => Some(lease_id),
        lease_break_period: LeaseBreakPeriod => Some(lease_break_period),
        context: Context => context,
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("restype", "container");
            url.query_pairs_mut().append_pair("comp", "lease");

            let mut headers = Headers::new();
            headers.insert(LEASE_ACTION, "break");
            headers.add(self.lease_id);
            headers.add(self.lease_break_period);

            let mut request =
                self.container_client
                    .finalize_request(url, Method::Put, headers, None)?;

            let response = self
                .container_client
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
