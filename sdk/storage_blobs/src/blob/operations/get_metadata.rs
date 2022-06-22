use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct GetMetadataBuilder {
    blob_client: BlobClient,
    blob_versioning: Option<BlobVersioning>,
    lease_id: Option<LeaseId>,
    context: Context,
    timeout: Option<Timeout>,
}

impl GetMetadataBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            blob_versioning: None,
            lease_id: None,
            context: Context::new(),
            timeout: None,
        }
    }

    setters! {
        blob_versioning: BlobVersioning => Some(blob_versioning),
        lease_id: LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),

    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "metadata");
            self.blob_versioning.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.blob_client
                    .prepare_request(url.as_str(), http::Method::GET, None)?;

            request.add_optional_header(&self.lease_id);

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
            server: server_from_headers(headers)?.to_owned(),
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
