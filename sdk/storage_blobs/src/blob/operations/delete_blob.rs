use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DeleteBlobBuilder {
    blob_client: BlobClient,
    delete_snapshots_method: DeleteSnapshotsMethod,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl DeleteBlobBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            delete_snapshots_method: DeleteSnapshotsMethod::Include,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        delete_snapshots_method: DeleteSnapshotsMethod => delete_snapshots_method,
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let url = self.blob_client.url_with_segments(None)?;

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            headers.add(self.delete_snapshots_method);

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

#[cfg(not(feature = "azurite_workaround"))]
azure_storage::response_from_headers!(DeleteBlobResponse ,
    delete_type_permanent_from_headers => delete_type_permanent: bool,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: DateTime<Utc>
);

#[cfg(feature = "azurite_workaround")]
azure_storage::response_from_headers!(DeleteBlobResponse ,
    delete_type_permanent_from_headers => delete_type_permanent: Option<bool>,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: DateTime<Utc>
);
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<DeleteBlobResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteBlobBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
