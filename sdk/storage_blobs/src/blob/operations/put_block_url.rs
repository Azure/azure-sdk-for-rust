use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId, Url};
use azure_storage::{headers::consistency_from_headers, ConsistencyCRC64, ConsistencyMD5};
use time::OffsetDateTime;

operation! {
    PutBlockUrl,
    client: BlobClient,
    block_id: BlockId,
    url: Url,
    ?hash: Hash,
    ?range: Range,
    ?lease_id: LeaseId
}

impl PutBlockUrlBuilder {
    pub fn into_future(mut self) -> PutBlockUrl {
        Box::pin(async move {
            let mut url = self.client.url()?;

            self.block_id.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("comp", "block");

            let mut headers = Headers::new();
            headers.insert(COPY_SOURCE, self.url.to_string());
            headers.add(self.lease_id);
            if let Some(range) = self.range {
                headers.insert(SOURCE_RANGE, format!("{range}"));
            }

            let mut request =
                BlobClient::finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            PutBlockUrlResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutBlockUrlResponse {
    pub content_md5: Option<ConsistencyMD5>,
    pub content_crc64: Option<ConsistencyCRC64>,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
    pub request_server_encrypted: bool,
}

impl PutBlockUrlResponse {
    pub(crate) fn from_headers(headers: &Headers) -> azure_core::Result<PutBlockUrlResponse> {
        let (content_md5, content_crc64) = consistency_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(PutBlockUrlResponse {
            content_md5,
            content_crc64,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}
