use crate::prelude::*;
use azure_core::{
    headers::{
        add_mandatory_header, add_optional_header, date_from_headers, etag_from_headers,
        request_id_from_headers, server_from_headers,
    },
    prelude::*,
    RequestId,
};
use chrono::{DateTime, Utc};
use http::HeaderMap;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct SetMetadataBuilder {
    blob_client: BlobClient,
    lease_id: Option<LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    metadata: Option<Metadata>,
}

impl SetMetadataBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            lease_id: None,
            client_request_id: None,
            timeout: None,
            metadata: None,
        }
    }

    setters! {
        lease_id: LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
        metadata: Metadata => Some(metadata),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "metadata");
            self.timeout.append_to_url_query(&mut url);

            trace!("url == {:?}", url);

            let (request, _url) = self.blob_client.prepare_request(
                url.as_str(),
                &http::Method::PUT,
                &|mut request| {
                    request = add_optional_header(&self.client_request_id, request);
                    request = add_optional_header(&self.lease_id, request);
                    if let Some(metadata) = &self.metadata {
                        for m in metadata.iter() {
                            request = add_mandatory_header(&m, request);
                        }
                    }
                    request
                },
                None,
            )?;

            info!("request == {:?}", request);

            let response = self
                .blob_client
                .http_client()
                .execute_request_check_status(request, http::StatusCode::OK)
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

impl TryFrom<&HeaderMap> for SetMetadataResponse {
    type Error = crate::Error;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        debug!("headers == {:#?}", headers);

        Ok(SetMetadataResponse {
            request_id: request_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            server: server_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
        })
    }
}
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<SetMetadataResponse>>;
