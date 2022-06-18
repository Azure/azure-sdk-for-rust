use crate::{blob::responses::PutBlockBlobResponse, prelude::*};
use azure_core::{
    headers::{add_mandatory_header, add_optional_header, add_optional_header_ref, BLOB_TYPE},
    prelude::*,
};
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct PutBlockBlobBuilder<'a> {
    blob_client: &'a BlobClient,
    body: Bytes,
    hash: Option<&'a Hash>,
    content_type: Option<ContentType<'a>>,
    content_encoding: Option<ContentEncoding<'a>>,
    content_language: Option<ContentLanguage<'a>>,
    content_disposition: Option<ContentDisposition<'a>>,
    metadata: Option<&'a Metadata>,
    access_tier: Option<AccessTier>,
    // TODO: Support tags
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> PutBlockBlobBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, body: impl Into<Bytes>) -> Self {
        Self {
            blob_client,
            body: body.into(),
            hash: None,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            metadata: None,
            access_tier: None,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        hash: &'a Hash => Some(hash),
        content_type: ContentType<'a> => Some(content_type),
        content_encoding: ContentEncoding<'a> => Some(content_encoding),
        content_language: ContentLanguage<'a> => Some(content_language),
        content_disposition: ContentDisposition<'a> => Some(content_disposition),
        metadata: &'a Metadata => Some(metadata),
        access_tier: AccessTier => Some(access_tier),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> azure_core::Result<PutBlockBlobResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request.insert_header(BLOB_TYPE, "BlockBlob");
                request.add_optional_header_ref(&self.hash, request);
                request.add_optional_header(&self.content_type, request);
                request.add_optional_header(&self.content_encoding, request);
                request.add_optional_header(&self.content_language, request);
                request.add_optional_header(&self.content_disposition, request);
                if let Some(metadata) = &self.metadata {
                    for m in metadata.iter() {
                        request.add_mandatory_header(&m, request);
                    }
                }
                request.add_optional_header(&self.access_tier, request);
                request.add_optional_header_ref(&self.lease_id, request);
                request.add_optional_header(&self.client_request_id, request);
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

        PutBlockBlobResponse::from_headers(response.headers())
    }
}
