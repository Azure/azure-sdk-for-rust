use crate::{blob::Blob, prelude::*};
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct GetPropertiesBuilder {
    blob_client: BlobClient,
    blob_versioning: Option<BlobVersioning>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl GetPropertiesBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            blob_versioning: None,
            timeout: None,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        blob_versioning: BlobVersioning => Some(blob_versioning),
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);
            self.blob_versioning.append_to_url_query(&mut url);

            let mut request =
                self.blob_client
                    .prepare_request(url.as_str(), http::Method::HEAD, None)?;
            request.add_optional_header(&self.lease_id);

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;
            // TODO: Fix this
            //let blob = Blob::from_headers(&blob_name, &container_name, snapshot_time, &headers)?;
            let blob = Blob::from_headers(self.blob_client.blob_name(), response.headers())?;
            GetPropertiesResponse::from_response(response.headers(), blob)
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPropertiesResponse {
    pub blob: Blob,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
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
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<GetPropertiesResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetPropertiesBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
