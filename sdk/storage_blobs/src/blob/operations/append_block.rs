use crate::{blob::operations::put_block::PutBlockResponse, prelude::*};
use azure_core::{
    headers::{add_optional_header, add_optional_header_ref},
    prelude::*,
};
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct AppendBlockBuilder {
    blob_client: BlobClient,
    body: Bytes,
    hash: Option<Hash>,
    condition_max_size: Option<ConditionMaxSize>,
    condition_append_position: Option<ConditionAppendPosition>,
    lease_id: Option<LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl AppendBlockBuilder {
    pub(crate) fn new(blob_client: BlobClient, body: impl Into<Bytes>) -> Self {
        Self {
            blob_client,
            body: body.into(),
            hash: None,
            condition_max_size: None,
            condition_append_position: None,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        hash: Hash => Some(hash),
        condition_max_size: ConditionMaxSize => Some(condition_max_size),
        condition_append_position: ConditionAppendPosition => Some(condition_append_position),
        lease_id: LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("comp", "appendblock");

            trace!("url == {:?}", url);

            let (request, _url) = self.blob_client.prepare_request(
                url.as_str(),
                &http::Method::PUT,
                &|mut request| {
                    request = add_optional_header_ref(&self.hash.as_ref(), request);
                    request = add_optional_header(&self.condition_max_size, request);
                    request = add_optional_header(&self.condition_append_position, request);
                    request = add_optional_header_ref(&self.lease_id.as_ref(), request);
                    request = add_optional_header(&self.client_request_id, request);
                    request
                },
                Some(self.body.clone()),
            )?;

            let response = self
                .blob_client
                .http_client()
                .execute_request_check_status(request, http::StatusCode::CREATED)
                .await?;

            debug!("response.headers() == {:#?}", response.headers());

            PutBlockResponse::from_headers(response.headers())
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<PutBlockResponse>>;
