use crate::{
    blob::{copy_status_from_headers, CopyStatus, SourceContentMD5},
    prelude::*,
};
use azure_core::{headers::*, prelude::*, RequestId};
use azure_storage::{
    core::{copy_id_from_headers, CopyId},
    headers::content_md5_from_headers_optional,
    ConsistencyMD5,
};
use std::convert::{TryFrom, TryInto};
use time::OffsetDateTime;
use url::Url;

operation! {
    CopyBlobFromUrl,
    client: BlobClient,
    source_url: Url,
    ?is_synchronous: bool,
    ?metadata: Metadata,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_source_since: IfSourceModifiedSinceCondition,
    ?if_source_match: IfSourceMatchCondition,
    ?lease_id: LeaseId,
    ?source_content_md5: SourceContentMD5
}

impl CopyBlobFromUrlBuilder {
    pub fn into_future(mut self) -> CopyBlobFromUrl {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut headers = Headers::new();
            headers.insert(COPY_SOURCE, self.source_url.to_string());
            headers.insert(
                REQUIRES_SYNC,
                format!("{}", self.is_synchronous.unwrap_or(false)),
            );
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.lease_id);
            headers.add(self.if_source_since);
            headers.add(self.if_source_match);
            headers.add(self.source_content_md5);

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            (response.headers()).try_into()
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CopyBlobFromUrlResponse {
    pub content_md5: Option<ConsistencyMD5>,
    pub last_modified: OffsetDateTime,
    pub etag: String,
    pub server: String,
    pub request_id: RequestId,
    pub version: String,
    pub copy_id: CopyId,
    pub copy_status: CopyStatus,
    pub date: OffsetDateTime,
}

impl TryFrom<&Headers> for CopyBlobFromUrlResponse {
    type Error = crate::Error;
    fn try_from(headers: &Headers) -> azure_core::Result<Self> {
        Ok(Self {
            content_md5: content_md5_from_headers_optional(headers)?,
            last_modified: last_modified_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            server: server_from_headers(headers)?,
            request_id: request_id_from_headers(headers)?,
            version: version_from_headers(headers)?,
            copy_id: copy_id_from_headers(headers)?,
            copy_status: copy_status_from_headers(headers)?,
            date: date_from_headers(headers)?,
        })
    }
}
