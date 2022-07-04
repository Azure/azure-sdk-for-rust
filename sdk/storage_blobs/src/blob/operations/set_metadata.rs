use crate::prelude::*;
use azure_core::{headers::*, prelude::*, Method, RequestId};
use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct SetMetadataBuilder {
    blob_client: BlobClient,
    lease_id: Option<LeaseId>,
    timeout: Option<Timeout>,
    metadata: Option<Metadata>,
    context: Context,
}

impl SetMetadataBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            lease_id: None,
            context: Context::new(),
            timeout: None,
            metadata: None,
        }
    }

    setters! {
        lease_id: LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),

        metadata: Metadata => Some(metadata),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "metadata");
            self.timeout.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }

            let mut request = self
                .blob_client
                .finalize_request(url, Method::Put, headers, None)?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;
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
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<SetMetadataResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for SetMetadataBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
