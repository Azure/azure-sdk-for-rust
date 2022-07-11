use crate::{blob::operations::DeleteBlobResponse, prelude::*};
use azure_core::{headers::Headers, prelude::*};

#[derive(Debug, Clone)]
pub struct DeleteBlobSnapshotBuilder {
    blob_client: BlobClient,
    snapshot: Snapshot,
    permanent: bool,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl DeleteBlobSnapshotBuilder {
    pub(crate) fn new(blob_client: BlobClient, snapshot: Snapshot) -> Self {
        Self {
            blob_client,
            snapshot,
            permanent: false,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        permanent: bool => permanent,
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            (&self.snapshot).append_to_url_query(&mut url);
            if self.permanent {
                url.query_pairs_mut().append_pair("deletetype", "permanent");
            }

            let mut headers = Headers::new();
            headers.add(self.lease_id);

            let mut request = self.blob_client.finalize_request(
                url,
                azure_core::Method::Delete,
                headers,
                None,
            )?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;

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
