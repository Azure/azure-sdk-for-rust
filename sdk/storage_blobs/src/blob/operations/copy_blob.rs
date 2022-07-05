use crate::{
    blob::{copy_status_from_headers, CopyStatus},
    prelude::*,
};
use azure_core::{headers::*, prelude::*, RequestId};
use azure_storage::core::{copy_id_from_headers, CopyId};
use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};
use url::Url;

#[derive(Debug, Clone)]
pub struct CopyBlobBuilder {
    blob_client: BlobClient,
    source_url: Url,
    metadata: Option<Metadata>,
    sequence_number_condition: Option<SequenceNumberCondition>,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    if_match_condition: Option<IfMatchCondition>,
    access_tier: Option<AccessTier>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    if_source_since_condition: Option<IfSourceModifiedSinceCondition>,
    if_source_match_condition: Option<IfSourceMatchCondition>,
    source_lease_id: Option<SourceLeaseId>,
    rehydrate_priority: RehydratePriority,
    context: Context,
}

impl CopyBlobBuilder {
    pub(crate) fn new(blob_client: BlobClient, source_url: Url) -> Self {
        Self {
            blob_client,
            source_url,
            metadata: None,
            sequence_number_condition: None,
            if_modified_since_condition: None,
            if_match_condition: None,
            access_tier: None,
            timeout: None,
            lease_id: None,
            if_source_since_condition: None,
            if_source_match_condition: None,
            source_lease_id: None,
            rehydrate_priority: RehydratePriority::Standard,
            context: Context::new(),
        }
    }

    setters! {
        metadata: Metadata => Some(metadata),
        sequence_number_condition: SequenceNumberCondition => Some(sequence_number_condition),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        access_tier: AccessTier => Some(access_tier),
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
        if_source_since_condition: IfSourceModifiedSinceCondition => Some(if_source_since_condition),
        if_source_match_condition: IfSourceMatchCondition => Some(if_source_match_condition),
        source_lease_id: SourceLeaseId => Some(source_lease_id),
        rehydrate_priority: RehydratePriority => rehydrate_priority,
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);
            let mut headers = Headers::new();
            headers.insert(COPY_SOURCE, self.source_url.as_str().to_owned());
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }
            headers.add(self.sequence_number_condition);
            headers.add(self.if_modified_since_condition);
            headers.add(self.if_match_condition);
            headers.add(self.access_tier);
            headers.add(self.lease_id);
            headers.add(self.if_source_since_condition);
            headers.add(self.if_source_match_condition);
            headers.add(self.source_lease_id);
            headers.add(self.rehydrate_priority);

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
pub struct CopyBlobResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub request_id: RequestId,
    pub version: String,
    pub server: String,
    pub date: DateTime<Utc>,
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
            version: version_from_headers(headers)?.to_owned(),
            server: server_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
            copy_id: copy_id_from_headers(headers)?,
            copy_status: copy_status_from_headers(headers)?,
        })
    }
}
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<CopyBlobResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CopyBlobBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
