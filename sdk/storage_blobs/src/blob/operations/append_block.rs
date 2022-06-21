use crate::{blob::operations::put_block::PutBlockResponse, prelude::*};
use azure_core::prelude::*;
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

            let mut request = self.blob_client.prepare_request(
                url.as_str(),
                http::Method::PUT,
                Some(self.body.clone()),
            )?;
            request.add_optional_header(&self.hash);
            request.add_optional_header(&self.condition_max_size);
            request.add_optional_header(&self.condition_append_position);
            request.add_optional_header(&self.lease_id);
            request.add_optional_header(&self.client_request_id);

            let response = self
                .blob_client
                .http_client()
                .execute_request_check_status(&request)
                .await?;

            PutBlockResponse::from_headers(response.headers())
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<PutBlockResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for AppendBlockBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
