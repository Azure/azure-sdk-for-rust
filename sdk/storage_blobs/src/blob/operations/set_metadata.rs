use crate::prelude::*;
use azure_core::{headers::*, prelude::*, Method, RequestId};
use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};

operation! {
    SetMetadata,
    client: BlobClient,
    ?lease_id: LeaseId,
    ?metadata: Metadata
}

impl SetMetadataBuilder {
    pub fn into_future(mut self) -> SetMetadata {
        Box::pin(async move {
            let mut url = self.client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "metadata");

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }

            let mut request = self
                .client
                .finalize_request(url, Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            response.headers().try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetMetadataResponse {
    pub request_id: RequestId,
    pub etag: String,
    pub server: String,
    pub date: DateTime<Utc>,
}

impl TryFrom<&Headers> for SetMetadataResponse {
    type Error = crate::Error;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        Ok(SetMetadataResponse {
            request_id: request_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            server: server_from_headers(headers)?,
            date: date_from_headers(headers)?,
        })
    }
}
