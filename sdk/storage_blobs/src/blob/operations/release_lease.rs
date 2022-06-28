use crate::prelude::*;
use azure_core::{
    headers::{LEASE_ACTION, *},
    prelude::*,
    RequestId,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ReleaseLeaseBuilder {
    blob_lease_client: BlobLeaseClient,
    timeout: Option<Timeout>,
    context: Context,
}

impl ReleaseLeaseBuilder {
    pub(crate) fn new(blob_lease_client: BlobLeaseClient) -> Self {
        Self {
            blob_lease_client,
            context: Context::new(),
            timeout: None,
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
                    .prepare_request(url, azure_core::Method::Put, None)?;
            request.insert_header(LEASE_ACTION, "release");
            request.add_mandatory_header(self.blob_lease_client.lease_id());

            let response = self
                .blob_lease_client
                .send(&mut self.context, &mut request)
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
