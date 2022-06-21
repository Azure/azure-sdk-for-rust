use crate::{
    blob::{copy_status_from_headers, CopyStatus},
    prelude::*,
    RehydratePriority,
};
use azure_core::{
    error::{ErrorKind, ResultExt},
    headers::{
        add_mandatory_header, add_optional_header, client_request_id_from_headers_optional,
        date_from_headers, etag_from_headers, last_modified_from_headers, request_id_from_headers,
        server_from_headers, version_from_headers, COPY_SOURCE,
    },
    prelude::*,
    RequestId,
};
use azure_storage::core::{copy_id_from_headers, CopyId};
use chrono::{DateTime, Utc};
use http::HeaderMap;
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
    client_request_id: Option<ClientRequestId>,
    if_source_since_condition: Option<IfSourceModifiedSinceCondition>,
    if_source_match_condition: Option<IfSourceMatchCondition>,
    source_lease_id: Option<SourceLeaseId>,
    rehydrate_priority: RehydratePriority,
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
            client_request_id: None,
            if_source_since_condition: None,
            if_source_match_condition: None,
            source_lease_id: None,
            rehydrate_priority: RehydratePriority::Standard,
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
        client_request_id: ClientRequestId => Some(client_request_id),
        if_source_since_condition: IfSourceModifiedSinceCondition => Some(if_source_since_condition),
        if_source_match_condition: IfSourceMatchCondition => Some(if_source_match_condition),
        source_lease_id: SourceLeaseId => Some(source_lease_id),
        rehydrate_priority: RehydratePriority => rehydrate_priority,
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);

            trace!("url == {:?}", url);

            let (request, _url) = self.blob_client.prepare_request(
                url.as_str(),
                &http::Method::PUT,
                &|mut request| {
                    request = request.header(COPY_SOURCE, self.source_url.as_str());
                    if let Some(metadata) = &self.metadata {
                        for m in metadata.iter() {
                            request = add_mandatory_header(&m, request);
                        }
                    }
                    request = add_optional_header(&self.sequence_number_condition, request);
                    request = add_optional_header(&self.if_modified_since_condition, request);
                    request = add_optional_header(&self.if_match_condition, request);
                    request = add_optional_header(&self.access_tier, request);
                    request = add_optional_header(&self.lease_id, request);
                    request = add_optional_header(&self.client_request_id, request);
                    request = add_optional_header(&self.if_source_since_condition, request);
                    request = add_optional_header(&self.if_source_match_condition, request);
                    request = add_optional_header(&self.source_lease_id, request);
                    request = add_mandatory_header(&self.rehydrate_priority, request);
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
    pub client_request_id: Option<String>,
}

impl TryFrom<&HeaderMap> for CopyBlobResponse {
    type Error = crate::Error;

    fn try_from(headers: &HeaderMap) -> azure_core::Result<Self> {
        trace!("CopyBlobResponse headers == {:#?}", headers);
        Ok(Self {
            etag: etag_from_headers(headers)?,
            last_modified: last_modified_from_headers(headers)?,
            request_id: request_id_from_headers(headers)?,
            version: version_from_headers(headers)?.to_owned(),
            server: server_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
            copy_id: copy_id_from_headers(headers).map_kind(ErrorKind::DataConversion)?,
            copy_status: copy_status_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
        })
    }
}
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<CopyBlobResponse>>;
