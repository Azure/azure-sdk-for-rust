use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId, Response};
use std::convert::{TryFrom, TryInto};

operation! {
    SetBlobExpiry,
    client: BlobClient,
    blob_expiry: BlobExpiry,
    ?lease_id: LeaseId
}

impl SetBlobExpiryBuilder {
    pub fn into_future(mut self) -> SetBlobExpiry {
        Box::pin(async move {
            let mut url = self.client.url()?;
            url.query_pairs_mut().append_pair("comp", "expiry");

            let mut headers = self.blob_expiry.to_headers();
            headers.add(self.lease_id);

            let mut request =
                BlobClient::finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            response.try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetBlobExpiryResponse {
    pub request_id: RequestId,
    pub client_request_id: Option<String>,
    pub version: String,
}

impl TryFrom<Response> for SetBlobExpiryResponse {
    type Error = azure_core::Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        let headers = response.headers();
        Ok(SetBlobExpiryResponse {
            request_id: request_id_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
            version: version_from_headers(headers)?,
        })
    }
}
