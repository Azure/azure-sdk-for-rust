use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use time::OffsetDateTime;

operation! {
    DeleteBlob,
    client: BlobClient,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_tags: IfTags,
    ?delete_snapshots_method: DeleteSnapshotsMethod,
    ?lease_id: LeaseId
}

impl DeleteBlobBuilder {
    pub fn into_future(mut self) -> DeleteBlob {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            headers.add(
                self.delete_snapshots_method
                    .unwrap_or(DeleteSnapshotsMethod::Include),
            );
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.if_tags);

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Delete, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            DeleteBlobResponse::from_headers(response.headers())
        })
    }
}

#[cfg(not(feature = "azurite_workaround"))]
azure_storage::response_from_headers!(DeleteBlobResponse ,
    delete_type_permanent_from_headers => delete_type_permanent: bool,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: OffsetDateTime
);

#[cfg(feature = "azurite_workaround")]
azure_storage::response_from_headers!(DeleteBlobResponse ,
    delete_type_permanent_from_headers => delete_type_permanent: Option<bool>,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: OffsetDateTime
);
