use crate::{blob::operations::DeleteBlobResponse, prelude::*};
use azure_core::{headers::Headers, prelude::*};

operation! {
    DeleteBlobVersion,
    client: BlobClient,
    version_id: VersionId,
    ?permanent: bool,
    ?lease_id: LeaseId
}

impl DeleteBlobVersionBuilder {
    pub fn into_future(mut self) -> DeleteBlobVersion {
        Box::pin(async move {
            let mut url = self.client.url()?;

            self.version_id.append_to_url_query(&mut url);
            if self.permanent.unwrap_or_default() {
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

type DeleteBlobVersionResponse = DeleteBlobResponse;
