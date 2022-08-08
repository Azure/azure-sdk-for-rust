use crate::{blob::PageRangeList, prelude::*};
use azure_core::{headers::*, prelude::*, RequestId};
use std::str::from_utf8;
use time::OffsetDateTime;

operation! {
    GetPageRanges,
    client: BlobClient,
        ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_tags: IfTags,
    ?blob_versioning: BlobVersioning,
    ?lease_id: LeaseId
}

impl GetPageRangesBuilder {
    pub fn into_future(mut self) -> GetPageRanges {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "pagelist");
            self.blob_versioning.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.if_tags);

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Get, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            let (_, headers, body) = response.deconstruct();
            let body = body.collect().await?;

            GetPageRangesResponse::from_response(&headers, &body)
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetPageRangesResponse {
    pub etag: String,
    pub last_modified: OffsetDateTime,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
    pub page_list: PageRangeList,
}

impl GetPageRangesResponse {
    pub(crate) fn from_response(
        headers: &Headers,
        body: &[u8],
    ) -> azure_core::Result<GetPageRangesResponse> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        let body = from_utf8(body)?;
        let page_list = PageRangeList::try_from_xml(&body[3..] as &str)?;

        Ok(GetPageRangesResponse {
            etag,
            last_modified,
            request_id,
            date,
            page_list,
        })
    }
}
