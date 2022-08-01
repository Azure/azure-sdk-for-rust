use crate::{blob::Blob, prelude::*};
use azure_core::{headers::*, prelude::*, RequestId};
use time::OffsetDateTime;

operation! {
    GetProperties,
    client: BlobClient,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_tags: IfTags,
    ?blob_versioning: BlobVersioning,
    ?lease_id: LeaseId
}

impl GetPropertiesBuilder {
    pub fn into_future(mut self) -> GetProperties {
        Box::pin(async move {
            let mut url = self.client.url()?;

            self.blob_versioning.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.if_tags);

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Head, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            // TODO: Fix this
            //let blob = Blob::from_headers(&blob_name, &container_name, snapshot_time, &headers)?;
            let blob = Blob::from_headers(self.client.blob_name(), response.headers())?;
            GetPropertiesResponse::from_response(response.headers(), blob)
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPropertiesResponse {
    pub blob: Blob,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
}

impl GetPropertiesResponse {
    pub(crate) fn from_response(
        headers: &Headers,
        blob: Blob,
    ) -> azure_core::Result<GetPropertiesResponse> {
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        Ok(GetPropertiesResponse {
            blob,
            request_id,
            date,
        })
    }
}
