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
use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};
use url::Url;

pub struct CopyBlobFromUrlBuilder {
    blob_client: BlobClient,
    source_url: Url,
    is_synchronous: bool,
    metadata: Option<Metadata>,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    if_match_condition: Option<IfMatchCondition>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    if_source_since_condition: Option<IfSourceModifiedSinceCondition>,
    if_source_match_condition: Option<IfSourceMatchCondition>,
    source_content_md5: Option<SourceContentMD5>,
    context: Context,
}

impl CopyBlobFromUrlBuilder {
    pub(crate) fn new(blob_client: BlobClient, source_url: Url) -> Self {
        Self {
            blob_client,
            source_url,
            is_synchronous: false,
            metadata: None,
            if_modified_since_condition: None,
            if_match_condition: None,
            timeout: None,
            lease_id: None,
            if_source_since_condition: None,
            if_source_match_condition: None,
            source_content_md5: None,
            context: Context::new(),
        }
    }

    setters! {
        is_synchronous: bool => is_synchronous,
        metadata: Metadata => Some(metadata),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
        if_source_since_condition: IfSourceModifiedSinceCondition => Some(if_source_since_condition),
        if_source_match_condition: IfSourceMatchCondition => Some(if_source_match_condition),
        source_content_md5: SourceContentMD5 => Some(source_content_md5),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.insert(COPY_SOURCE, self.source_url.to_string());
            headers.insert(REQUIRES_SYNC, format!("{}", self.is_synchronous));
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }
            headers.add(self.if_modified_since_condition);
            headers.add(self.if_match_condition);
            headers.add(self.lease_id);
            headers.add(self.if_source_since_condition);
            headers.add(self.if_source_match_condition);
            headers.add(self.source_content_md5);

            let mut request =
                self.blob_client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;

            (response.headers()).try_into()
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CopyBlobFromUrlResponse {
    pub content_md5: Option<ConsistencyMD5>,
    pub last_modified: DateTime<Utc>,
    pub etag: String,
    pub server: String,
    pub request_id: RequestId,
    pub version: String,
    pub copy_id: CopyId,
    pub copy_status: CopyStatus,
    pub date: DateTime<Utc>,
}

impl TryFrom<&Headers> for CopyBlobFromUrlResponse {
    type Error = crate::Error;
    fn try_from(headers: &Headers) -> azure_core::Result<Self> {
        Ok(Self {
            content_md5: content_md5_from_headers_optional(headers)?,
            last_modified: last_modified_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            server: server_from_headers(headers)?.to_owned(),
            request_id: request_id_from_headers(headers)?,
            version: version_from_headers(headers)?.to_owned(),
            copy_id: copy_id_from_headers(headers)?,
            copy_status: copy_status_from_headers(headers)?,
            date: date_from_headers(headers)?,
        })
    }
}

pub type Response =
    futures::future::BoxFuture<'static, azure_core::Result<CopyBlobFromUrlResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CopyBlobFromUrlBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
