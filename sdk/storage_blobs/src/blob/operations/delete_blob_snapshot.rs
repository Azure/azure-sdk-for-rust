use crate::{blob::operations::DeleteBlobResponse, prelude::*};
use azure_core::{headers::Headers, prelude::*};

operation! {
    DeleteBlobSnapshot,
    client: BlobClient,
    snapshot: Snapshot,
    ?permanent: bool,
    ?lease_id: LeaseId
}

impl DeleteBlobSnapshotBuilder {
    pub fn into_future(mut self) -> DeleteBlobSnapshot {
        Box::pin(async move {
            let mut url = self.client.url()?;

            self.snapshot.append_to_url_query(&mut url);
            let permanent = self.permanent.unwrap_or(false);
            if permanent {
                url.query_pairs_mut().append_pair("deletetype", "permanent");
            }

            let mut headers = Headers::new();
            headers.add(self.lease_id);

            let mut request =
                BlobClient::finalize_request(url, azure_core::Method::Delete, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            DeleteBlobResponse::from_headers(response.headers())
        })
    }
}

type DeleteBlobSnapshotResponse = DeleteBlobResponse;
