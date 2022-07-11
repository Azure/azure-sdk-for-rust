use crate::prelude::*;
use azure_core::{headers::*, prelude::*, Method, RequestId};
use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct GetMetadataBuilder {
    blob_client: BlobClient,
    blob_versioning: Option<BlobVersioning>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl GetMetadataBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            blob_versioning: None,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        blob_versioning: BlobVersioning => Some(blob_versioning),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "metadata");
            self.blob_versioning.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.lease_id);

            let mut request = self
                .blob_client
                .finalize_request(url, Method::Get, headers, None)?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;

            response.headers().try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetMetadataResponse {
    pub request_id: RequestId,
    pub etag: String,
    pub server: String,
    pub date: DateTime<Utc>,
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
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<GetMetadataResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetMetadataBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
