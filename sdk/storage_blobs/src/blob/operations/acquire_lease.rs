use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use time::OffsetDateTime;

operation! {
    AcquireLease,
    client: BlobClient,
    lease_duration: LeaseDuration,
    ?lease_id: LeaseId,
    ?proposed_lease_id: ProposedLeaseId
}

impl AcquireLeaseBuilder {
    pub fn into_future(mut self) -> AcquireLease {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "lease");

            let mut headers = Headers::new();
            headers.insert(LEASE_ACTION, "acquire");
            headers.add(self.lease_duration);
            headers.add(self.proposed_lease_id);
            headers.add(self.lease_id);

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            AcquireLeaseResponse::from_headers(response.headers())
        })
    }
}

azure_storage::response_from_headers!(AcquireLeaseResponse,
    etag_from_headers => etag: String,
    last_modified_from_headers => last_modified: OffsetDateTime,
    lease_id_from_headers => lease_id: LeaseId,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: OffsetDateTime
);
