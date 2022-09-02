use crate::clients::{FileClient, PathClient};
use azure_core::{
    headers::{self, etag_from_headers, last_modified_from_headers},
    prelude::*,
    Request, Response,
};
use azure_storage::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use std::convert::TryInto;
use time::OffsetDateTime;

operation! {
    GetFile,
    client: FileClient,
    ?range: Range,
    ?if_match_condition: IfMatchCondition,
    ?if_modified_since: IfModifiedSince,
    ?lease_id: LeaseId
}

impl GetFileBuilder {
    pub fn into_future(self) -> GetFile {
        let this = self.clone();
        let ctx = self.context.clone();

        Box::pin(async move {
            let url = this.client.url()?;

            let mut request = Request::new(url, azure_core::Method::Get);

            let requested_range = self.range.unwrap_or_else(|| Range::new(0, u64::MAX));
            request.insert_headers(&requested_range);

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
    pub last_modified: OffsetDateTime,
    pub data: Bytes,
    pub content_range: Option<ContentRange>,
}

impl GetFileResponse {
    pub async fn try_from(response: Response) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let data = body.collect().await?;

        let content_range = headers.get_optional_as(&headers::CONTENT_RANGE)?;

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            etag: etag_from_headers(&headers)?,
            last_modified: last_modified_from_headers(&headers)?,
            data,
            content_range,
        })
    }
}
