use crate::clients::{FileClient, PathClient};
use azure_core::error::ResultExt;
use azure_core::headers::{etag_from_headers, last_modified_from_headers};
use azure_core::prelude::*;
use azure_core::{
    collect_pinned_stream, error::ErrorKind, AppendToUrlQuery, Response as HttpResponse,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use std::convert::TryInto;
use std::str::FromStr;

/// A future of a delete file response
type GetFile = futures::future::BoxFuture<'static, azure_core::Result<GetFileResponse>>;

#[derive(Debug, Clone)]
pub struct GetFileBuilder {
    client: FileClient,
    timeout: Option<Timeout>,
    range: Option<Range>,
    if_match_condition: Option<IfMatchCondition>,
    if_modified_since: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl GetFileBuilder {
    pub(crate) fn new(client: FileClient, context: Context) -> Self {
        Self {
            client,
            timeout: None,
            range: None,
            if_match_condition: None,
            if_modified_since: None,
            client_request_id: None,
            lease_id: None,
            context,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        range: Range => Some(range),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        if_modified_since: IfModifiedSinceCondition => Some(if_modified_since),
        client_request_id: ClientRequestId => Some(client_request_id),
        lease_id: LeaseId => Some(lease_id),
        context: Context => context,
    }

    pub fn into_future(self) -> GetFile {
        let this = self.clone();
        let ctx = self.context.clone();

        Box::pin(async move {
            let mut url = this.client.url()?;

            self.timeout.append_to_url_query(&mut url);

            let mut request = this.client.prepare_request(url.as_str(), http::Method::GET);

            let requested_range = self.range.unwrap_or_else(|| Range::new(0, u64::MAX));
            request.insert_headers(&requested_range);

            request.insert_headers(&this.client_request_id);
            request.insert_headers(&this.if_match_condition);
            request.insert_headers(&this.if_modified_since);
            request.insert_headers(&this.lease_id);

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            GetFileResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetFileResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub data: Bytes,
    pub content_range: Option<ContentRange>,
}

impl GetFileResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();

        let data = collect_pinned_stream(pinned_stream).await?;
        let content_range_header = headers.get(http::header::CONTENT_RANGE);
        let content_range = match content_range_header {
            Some(hv) => Some(
                ContentRange::from_str(hv.to_str().map_kind(ErrorKind::DataConversion)?)
                    .map_kind(ErrorKind::DataConversion)?,
            ),
            None => None,
        };

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            etag: etag_from_headers(&headers)?,
            last_modified: last_modified_from_headers(&headers)?,
            data,
            content_range,
        })
    }
}
