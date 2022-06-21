use crate::prelude::*;
use azure_core::{
    headers::{add_mandatory_header, add_optional_header, add_optional_header_ref, *},
    prelude::*,
    RequestId,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DeleteBlobBuilder {
    blob_client: BlobClient,
    delete_snapshots_method: DeleteSnapshotsMethod,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    client_request_id: Option<ClientRequestId>,
}

impl DeleteBlobBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            delete_snapshots_method: DeleteSnapshotsMethod::Include,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }

    setters! {
        delete_snapshots_method: DeleteSnapshotsMethod => delete_snapshots_method,
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);

            trace!("delete_blob url == {:?}", url);

            let (request, _url) = self.blob_client.prepare_request(
                url.as_str(),
                &http::Method::DELETE,
                &|mut request| {
                    request = add_optional_header_ref(&self.lease_id.as_ref(), request);
                    request = add_optional_header(&self.client_request_id, request);
                    request = add_mandatory_header(&self.delete_snapshots_method, request);
                    request
                },
                None,
            )?;

            let response = self
                .blob_client
                .http_client()
                .execute_request_check_status(request, http::StatusCode::ACCEPTED)
                .await?;

            debug!("response.headers() == {:#?}", response.headers());

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
