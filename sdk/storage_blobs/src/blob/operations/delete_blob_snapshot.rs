use crate::{blob::operations::DeleteBlobResponse, prelude::*};
use azure_core::prelude::*;

#[derive(Debug, Clone)]
pub struct DeleteBlobSnapshotBuilder {
    blob_client: BlobClient,
    snapshot: Snapshot,
    permanent: bool,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    client_request_id: Option<ClientRequestId>,
}

impl DeleteBlobSnapshotBuilder {
    pub(crate) fn new(blob_client: BlobClient, snapshot: Snapshot) -> Self {
        Self {
            blob_client,
            snapshot,
            permanent: false,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }

    setters! {
        permanent: bool => permanent,
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);
            (&self.snapshot).append_to_url_query(&mut url);
            if self.permanent {
                url.query_pairs_mut().append_pair("deletetype", "permanent");
            }

            trace!("delete_blob snapshot url == {:?}", url);

            let mut request =
                self.blob_client
                    .prepare_request(url.as_str(), http::Method::DELETE, None)?;
            request.add_optional_header(&self.lease_id);
            request.add_optional_header(&self.client_request_id);

            let response = self
                .blob_client
                .http_client()
                .execute_request_check_status(&request)
                .await?;

            debug!("response.headers() == {:#?}", response.headers());

            DeleteBlobResponse::from_headers(response.headers())
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<DeleteBlobResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteBlobSnapshotBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
