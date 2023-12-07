use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId, ResponseBody};
use time::OffsetDateTime;

operation! {
    GetTags,
    client: BlobClient,
    ?if_tags: IfTags,
    ?blob_versioning: BlobVersioning,
    ?lease_id: LeaseId
}

impl GetTagsBuilder {
    pub fn into_future(mut self) -> GetTags {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "tags");

            self.blob_versioning.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            headers.add(self.if_tags);

            let mut request =
                BlobClient::finalize_request(url, azure_core::Method::Get, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            let (_, headers, body) = response.deconstruct();
            GetTagsResponse::from_response(headers, body).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetTagsResponse {
    pub request_id: RequestId,
    pub date: OffsetDateTime,
    pub tags: Tags,
}

impl GetTagsResponse {
    pub(crate) async fn from_response(
        headers: Headers,
        body: ResponseBody,
    ) -> azure_core::Result<Self> {
        let request_id = request_id_from_headers(&headers)?;
        let date = date_from_headers(&headers)?;
        let tags = body.xml().await?;

        Ok(Self {
            request_id,
            date,
            tags,
        })
    }
}
