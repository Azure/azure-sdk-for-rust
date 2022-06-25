use crate::{
    blob::{BlockListType, BlockWithSizeList},
    prelude::*,
};
use azure_core::{collect_pinned_stream, headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};
use std::str::from_utf8;

pub struct GetBlockListBuilder {
    blob_client: BlobClient,
    block_list_type: BlockListType,
    blob_versioning: Option<BlobVersioning>,
    lease_id: Option<LeaseId>,
    timeout: Option<Timeout>,
    context: Context,
}

impl GetBlockListBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            block_list_type: BlockListType::Committed,
            blob_versioning: None,
            lease_id: None,
            context: Context::new(),
            timeout: None,
        }
    }

    setters! {
        block_list_type: BlockListType => block_list_type,
        blob_versioning: BlobVersioning => Some(blob_versioning),
        lease_id: LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "blocklist");
            self.blob_versioning.append_to_url_query(&mut url);
            self.block_list_type.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.blob_client
                    .prepare_request(url.as_str(), azure_core::Method::GET, None)?;
            request.add_optional_header(&self.lease_id);

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;

            let (_, headers, body) = response.deconstruct();
            let body = collect_pinned_stream(body).await?;

            GetBlockListResponse::from_response(&headers, &body)
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetBlockListResponse {
    pub etag: Option<String>,
    pub last_modified: Option<DateTime<Utc>>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub block_with_size_list: BlockWithSizeList,
}

impl GetBlockListResponse {
    pub(crate) fn from_response(
        headers: &Headers,
        body: &[u8],
    ) -> azure_core::Result<GetBlockListResponse> {
        let etag = etag_from_headers_optional(headers)?;
        let last_modified = last_modified_from_headers_optional(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        let body = from_utf8(body)?;
        let block_with_size_list = BlockWithSizeList::try_from_xml(&body[3..] as &str)?;

        Ok(GetBlockListResponse {
            etag,
            last_modified,
            request_id,
            date,
            block_with_size_list,
        })
    }
}
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<GetBlockListResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetBlockListBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
