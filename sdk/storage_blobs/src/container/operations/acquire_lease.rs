use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};
use http::method::Method;

#[derive(Debug, Clone)]
pub struct AcquireLeaseBuilder {
    container_client: ContainerClient,
    lease_duration: LeaseDuration,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    proposed_lease_id: Option<ProposedLeaseId>,
    context: Context,
}

impl AcquireLeaseBuilder {
    pub(crate) fn new(container_client: ContainerClient, lease_duration: LeaseDuration) -> Self {
        AcquireLeaseBuilder {
            container_client,
            lease_duration,
            context: Context::new(),
            timeout: None,
            lease_id: None,
            proposed_lease_id: None,
        }
    }

    setters! {
        lease_id: LeaseId => Some(lease_id),
        proposed_lease_id: ProposedLeaseId => Some(proposed_lease_id),

        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("restype", "container");
            url.query_pairs_mut().append_pair("comp", "lease");

            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.container_client
                    .prepare_request(url.as_str(), Method::PUT, None)?;
            request.insert_header(LEASE_ACTION, "acquire");
            request.add_mandatory_header(&self.lease_duration);
            request.add_optional_header(&self.lease_id);
            request.add_optional_header(&self.proposed_lease_id);

            let response = self
                .container_client
                .send(&mut self.context, &mut request)
                .await?;

            AcquireLeaseResponse::from_headers(response.headers())
        })
    }
}

azure_storage::response_from_headers!(AcquireLeaseResponse ,
    etag_from_headers => etag: String,
    last_modified_from_headers => last_modified: DateTime<Utc>,
    lease_id_from_headers => lease_id: LeaseId,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: DateTime<Utc>
);

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<AcquireLeaseResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for AcquireLeaseBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
