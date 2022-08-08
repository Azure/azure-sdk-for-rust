use crate::{
    blob::{BlockListType, BlockWithSizeList},
    prelude::*,
};
use azure_core::{headers::*, prelude::*, RequestId};
use std::str::from_utf8;
use time::OffsetDateTime;

operation! {
    GetBlockList,
    client: BlobClient,
    ?if_tags: IfTags,
    ?block_list_type: BlockListType,
    ?blob_versioning: BlobVersioning,
    ?lease_id: LeaseId
}

impl GetBlockListBuilder {
    pub fn into_future(mut self) -> GetBlockList {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "blocklist");
            self.blob_versioning.append_to_url_query(&mut url);

            self.block_list_type
                .unwrap_or(BlockListType::Committed)
                .append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            headers.add(self.if_tags);

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Get, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            let (_, headers, body) = response.deconstruct();
            let body = body.collect().await?;

            GetBlockListResponse::from_response(&headers, &body)
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetBlockListResponse {
    pub etag: Option<String>,
    pub last_modified: Option<OffsetDateTime>,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
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
