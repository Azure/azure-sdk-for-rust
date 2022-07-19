use crate::prelude::*;
use azure_core::{collect_pinned_stream, headers::*, prelude::*, RequestId};
use azure_storage::xml::read_xml;
use chrono::{DateTime, Utc};

operation! {
    GetTags,
    client: BlobClient,
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

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Get, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            let (_, headers, body) = response.deconstruct();

            let collected = collect_pinned_stream(body).await?;
            GetTagsResponse::from_response(&headers, &collected)
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetTagsResponse {
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub tags: Tags,
}

impl GetTagsResponse {
    pub(crate) fn from_response(headers: &Headers, body: &[u8]) -> azure_core::Result<Self> {
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let tags = read_xml(body)?;

        Ok(Self {
            request_id,
            date,
            tags,
        })
    }
}
