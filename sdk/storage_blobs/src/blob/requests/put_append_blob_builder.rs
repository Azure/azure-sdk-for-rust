use crate::blob::responses::PutBlobResponse;
use crate::prelude::*;
use azure_core::headers::{add_optional_header, add_optional_header_ref, BLOB_TYPE};
use azure_core::prelude::*;

#[derive(Debug, Clone)]
pub struct PutAppendBlobBuilder<'a> {
    blob_client: &'a BlobClient,
    content_type: Option<ContentType<'a>>,
    content_encoding: Option<ContentEncoding<'a>>,
    content_language: Option<ContentLanguage<'a>>,
    content_disposition: Option<ContentDisposition<'a>>,
    metadata: Option<&'a Metadata>,
    // TODO: Support tags
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> PutAppendBlobBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient) -> Self {
        Self {
            blob_client,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            metadata: None,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        content_type: ContentType<'a> => Some(content_type),
        content_encoding: ContentEncoding<'a> => Some(content_encoding),
        content_language: ContentLanguage<'a> => Some(content_language),
        content_disposition: ContentDisposition<'a> => Some(content_disposition),
        metadata: &'a Metadata => Some(metadata),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(
        &self,
    ) -> Result<PutBlobResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = request.header(BLOB_TYPE, "AppendBlob");
                request = add_optional_header(&self.content_type, request);
                request = add_optional_header(&self.content_encoding, request);
                request = add_optional_header(&self.content_language, request);
                request = add_optional_header(&self.content_disposition, request);
                request = add_optional_header(&self.metadata, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::CREATED)
            .await?;

        debug!("response.headers() == {:#?}", response.headers());

        Ok(PutBlobResponse::from_headers(response.headers())?)
    }
}
