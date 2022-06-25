use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DeleteBlobBuilder {
    blob_client: BlobClient,
    delete_snapshots_method: DeleteSnapshotsMethod,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl DeleteBlobBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            delete_snapshots_method: DeleteSnapshotsMethod::Include,
            timeout: None,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        delete_snapshots_method: DeleteSnapshotsMethod => delete_snapshots_method,
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.blob_client
                    .prepare_request(url.as_str(), azure_core::Method::DELETE, None)?;
            request.add_optional_header(&self.lease_id);
            request.add_mandatory_header(&self.delete_snapshots_method);

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
