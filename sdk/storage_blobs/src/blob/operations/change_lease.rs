use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ChangeLeaseBuilder {
    blob_lease_client: BlobLeaseClient,
    proposed_lease_id: ProposedLeaseId,
    timeout: Option<Timeout>,
    context: Context,
}

impl ChangeLeaseBuilder {
    pub(crate) fn new(
        blob_lease_client: BlobLeaseClient,
        proposed_lease_id: ProposedLeaseId,
    ) -> Self {
        Self {
            blob_lease_client,
            proposed_lease_id,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_lease_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "lease");
            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.blob_lease_client
                    .prepare_request(url.as_str(), http::Method::PUT, None)?;
            request.insert_header(LEASE_ACTION, "change");
            request.add_mandatory_header(self.blob_lease_client.lease_id());
            request.add_mandatory_header(&self.proposed_lease_id);

            let response = self
                .blob_lease_client
                .send(&mut self.context, &mut request)
                .await?;

            ChangeLeaseResponse::from_headers(response.headers())
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<ChangeLeaseResponse>>;

azure_storage::response_from_headers!(ChangeLeaseResponse ,
    etag_from_headers => etag: String,
    last_modified_from_headers => last_modified: DateTime<Utc>,
    lease_id_from_headers => lease_id: LeaseId,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: DateTime<Utc>
);

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for ChangeLeaseBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
