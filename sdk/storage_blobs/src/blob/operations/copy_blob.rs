use crate::{
    blob::{copy_status_from_headers, CopyStatus},
    prelude::*,
};
use azure_core::{headers::*, prelude::*, RequestId};
use azure_storage::core::{copy_id_from_headers, CopyId};
use std::convert::{TryFrom, TryInto};
use time::OffsetDateTime;
use url::Url;

operation! {
    CopyBlob,
    client: BlobClient,
    source_url: Url,
    ?metadata: Metadata,
    ?if_sequence_number: IfSequenceNumber,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_tags: IfTags,
    ?access_tier: AccessTier,
    ?lease_id: LeaseId,
    ?if_source_since: IfSourceModifiedSinceCondition,
    ?if_source_match: IfSourceMatchCondition,
    ?source_lease_id: SourceLeaseId,
    ?rehydrate_priority: RehydratePriority
}

impl CopyBlobBuilder {
    pub fn into_future(mut self) -> CopyBlob {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut headers = Headers::new();
            headers.insert(COPY_SOURCE, self.source_url.as_str().to_owned());
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }
            headers.add(self.if_sequence_number);
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.access_tier);
            headers.add(self.lease_id);
            headers.add(self.if_source_since);
            headers.add(self.if_source_match);
            headers.add(self.source_lease_id);
            headers.add(
                self.rehydrate_priority
                    .unwrap_or(RehydratePriority::Standard),
            );

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            (response.headers()).try_into()
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CopyBlobResponse {
    pub etag: String,
    pub last_modified: OffsetDateTime,
    pub request_id: RequestId,
    pub version: String,
    pub server: String,
    pub date: OffsetDateTime,
    pub copy_id: CopyId,
    pub copy_status: CopyStatus,
}

impl TryFrom<&Headers> for CopyBlobResponse {
    type Error = crate::Error;

    fn try_from(headers: &Headers) -> azure_core::Result<Self> {
        Ok(Self {
            etag: etag_from_headers(headers)?,
            last_modified: last_modified_from_headers(headers)?,
            request_id: request_id_from_headers(headers)?,
            version: version_from_headers(headers)?,
            server: server_from_headers(headers)?,
            date: date_from_headers(headers)?,
            copy_id: copy_id_from_headers(headers)?,
            copy_status: copy_status_from_headers(headers)?,
        })
    }
}
