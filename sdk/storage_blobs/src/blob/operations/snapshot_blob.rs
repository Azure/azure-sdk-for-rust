use crate::prelude::*;
use azure_core::headers::etag_from_headers;
use azure_core::{
    headers::{date_from_headers, last_modified_from_headers, request_id_from_headers, Headers},
    prelude::*,
    Method::Put,
    RequestId,
};
use time::OffsetDateTime;

operation! {
    SnapshotBlob,
    client: BlobClient,
    ?metadata: Metadata,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?lease_id: LeaseId
}

impl SnapshotBlobBuilder {
    pub fn into_future(mut self) -> SnapshotBlob {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "snapshot");

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }

            let mut request = self.client.finalize_request(url, Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            response.headers().try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotBlobResponse {
    pub request_id: RequestId,
    pub etag: String,
    pub date: OffsetDateTime,
    pub snapshot: Snapshot,
    pub last_modified: OffsetDateTime,
}

impl TryFrom<&Headers> for SnapshotBlobResponse {
    type Error = crate::Error;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        Ok(SnapshotBlobResponse {
            request_id: request_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            date: date_from_headers(headers)?,
            snapshot: Snapshot::new(headers.get_str(&SNAPSHOT)?.to_string()),
            last_modified: last_modified_from_headers(headers)?,
        })
    }
}
