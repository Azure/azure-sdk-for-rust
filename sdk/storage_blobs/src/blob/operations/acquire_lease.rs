use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct AcquireLeaseBuilder {
    blob_client: BlobClient,
    lease_duration: LeaseDuration,
    lease_id: Option<LeaseId>,
    proposed_lease_id: Option<ProposedLeaseId>,
    timeout: Option<Timeout>,
    context: Context,
}

impl AcquireLeaseBuilder {
    pub(crate) fn new(blob_client: BlobClient, lease_duration: LeaseDuration) -> Self {
        Self {
            blob_client,
            lease_duration,
            lease_id: None,
            proposed_lease_id: None,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        lease_id: LeaseId => Some(lease_id),
        proposed_lease_id: ProposedLeaseId => Some(proposed_lease_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "lease");
            self.timeout.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.insert(LEASE_ACTION, "acquire");
            headers.add(self.lease_duration);
            headers.add(self.proposed_lease_id);
            headers.add(self.lease_id);

            let mut request =
                self.blob_client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;

            AcquireLeaseResponse::from_headers(response.headers())
        })
    }
}

azure_storage::response_from_headers!(AcquireLeaseResponse,
    etag_from_headers => etag: String,
    last_modified_from_headers => last_modified: DateTime<Utc>,
    lease_id_from_headers => lease_id: LeaseId,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: DateTime<Utc>
);

/// The future returned by calling `into_future` on the builder.
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<AcquireLeaseResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for AcquireLeaseBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
