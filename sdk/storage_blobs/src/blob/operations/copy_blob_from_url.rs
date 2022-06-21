use crate::{
    blob::{copy_status_from_headers, CopyStatus, SourceContentMD5},
    prelude::*,
};
use azure_core::{
    error::{ErrorKind, ResultExt},
    headers::{
        add_mandatory_header, add_optional_header, date_from_headers, etag_from_headers,
        last_modified_from_headers, request_id_from_headers, server_from_headers,
        version_from_headers, COPY_SOURCE, REQUIRES_SYNC,
    },
    prelude::*,
    RequestId,
};
use azure_storage::{
    core::{copy_id_from_headers, CopyId},
    headers::content_md5_from_headers_optional,
    ConsistencyMD5,
};
use chrono::{DateTime, Utc};
use http::HeaderMap;
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
    client_request_id: Option<ClientRequestId>,
    if_source_since_condition: Option<IfSourceModifiedSinceCondition>,
    if_source_match_condition: Option<IfSourceMatchCondition>,
    source_content_md5: Option<SourceContentMD5>,
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
            client_request_id: None,
            if_source_since_condition: None,
            if_source_match_condition: None,
            source_content_md5: None,
        }
    }

    setters! {
        is_synchronous: bool => is_synchronous,
        metadata: Metadata => Some(metadata),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        if_source_since_condition: IfSourceModifiedSinceCondition => Some(if_source_since_condition),
        if_source_match_condition: IfSourceMatchCondition => Some(if_source_match_condition),
        source_content_md5: SourceContentMD5 => Some(source_content_md5),
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
                    request = request.header(REQUIRES_SYNC, format!("{}", self.is_synchronous));
                    if let Some(metadata) = &self.metadata {
                        for m in metadata.iter() {
                            request = add_mandatory_header(&m, request);
                        }
                    }
                    request = add_optional_header(&self.if_modified_since_condition, request);
                    request = add_optional_header(&self.if_match_condition, request);
                    request = add_optional_header(&self.lease_id, request);
                    request = add_optional_header(&self.client_request_id, request);
                    request = add_optional_header(&self.if_source_since_condition, request);
                    request = add_optional_header(&self.if_source_match_condition, request);
                    request = add_optional_header(&self.source_content_md5, request);
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

impl TryFrom<&HeaderMap> for CopyBlobFromUrlResponse {
    type Error = crate::Error;
    fn try_from(headers: &HeaderMap) -> azure_core::Result<Self> {
        debug!("headers == {:#?}", headers);
        Ok(Self {
            content_md5: content_md5_from_headers_optional(headers)
                .map_kind(ErrorKind::DataConversion)?,
            last_modified: last_modified_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            server: server_from_headers(headers)?.to_owned(),
            request_id: request_id_from_headers(headers)?,
            version: version_from_headers(headers)?.to_owned(),
            copy_id: copy_id_from_headers(headers).map_kind(ErrorKind::DataConversion)?,
            copy_status: copy_status_from_headers(headers)?,
            date: date_from_headers(headers)?,
        })
    }
}

pub type Response =
    futures::future::BoxFuture<'static, azure_core::Result<CopyBlobFromUrlResponse>>;
