use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId, Response};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct SetBlobExpiryBuilder {
    blob_client: BlobClient,
    blob_expiry: BlobExpiry,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl SetBlobExpiryBuilder {
    pub(crate) fn new(blob_client: BlobClient, expiry: BlobExpiry) -> Self {
        Self {
            blob_client,
            blob_expiry: expiry,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        lease_id: LeaseId => Some(lease_id),
        context: Context => context,
    }

    pub fn into_future(mut self) -> FutureResponse {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;
            url.query_pairs_mut().append_pair("comp", "expiry");

            let mut headers = self.blob_expiry.to_headers();
            headers.add(self.lease_id);

            let mut request =
                self.blob_client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;
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
    type Error = crate::Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        let headers = response.headers();
        Ok(SetBlobExpiryResponse {
            request_id: request_id_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
            version: version_from_headers(headers)?,
        })
    }
}

pub type FutureResponse =
    futures::future::BoxFuture<'static, azure_core::Result<SetBlobExpiryResponse>>;
#[cfg(feature = "into_future")]
impl std::future::IntoFuture for SetBlobExpiryBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
