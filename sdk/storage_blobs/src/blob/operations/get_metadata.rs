use crate::prelude::*;
use azure_core::{headers::*, prelude::*, Method, RequestId};
use std::convert::{TryFrom, TryInto};
use time::OffsetDateTime;

operation! {
    GetMetadata,
    client: BlobClient,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_tags: IfTags,
    ?blob_versioning: BlobVersioning,
    ?lease_id: LeaseId
}

impl GetMetadataBuilder {
    pub fn into_future(mut self) -> GetMetadata {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "metadata");
            self.blob_versioning.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.if_tags);

            let mut request = self
                .client
                .finalize_request(url, Method::Get, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            response.headers().try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetMetadataResponse {
    pub request_id: RequestId,
    pub etag: String,
    pub server: String,
    pub date: OffsetDateTime,
    pub metadata: Metadata,
}

impl TryFrom<&Headers> for GetMetadataResponse {
    type Error = crate::Error;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        Ok(GetMetadataResponse {
            request_id: request_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            server: server_from_headers(headers)?,
            date: date_from_headers(headers)?,
            metadata: headers.into(),
        })
    }
}
