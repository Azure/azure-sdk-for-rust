use crate::{blob::responses::PutBlobResponse, prelude::*};
use azure_core::{
    error::Result,
    headers::{
        add_mandatory_header, add_optional_header, add_optional_header_ref, BLOB_CONTENT_LENGTH,
        BLOB_TYPE,
    },
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct PutPageBlobBuilder<'a> {
    blob_client: &'a BlobClient,
    length: u128,
    content_type: Option<ContentType<'a>>,
    content_encoding: Option<ContentEncoding<'a>>,
    content_language: Option<ContentLanguage<'a>>,
    content_disposition: Option<ContentDisposition<'a>>,
    metadata: Option<&'a Metadata>,
    // TODO: Support tags
    lease_id: Option<&'a LeaseId>,
    sequence_number: Option<SequenceNumber>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> PutPageBlobBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, length: u128) -> Self {
        Self {
            blob_client,
            length,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            metadata: None,
            lease_id: None,
            sequence_number: None,
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
        sequence_number: SequenceNumber => Some(sequence_number),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> Result<PutBlobResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = request.header(BLOB_TYPE, "PageBlob");
                request = request.header(BLOB_CONTENT_LENGTH, &format!("{}", self.length));
                request = add_optional_header(&self.content_type, request);
                request = add_optional_header(&self.content_encoding, request);
                request = add_optional_header(&self.content_language, request);
                request = add_optional_header(&self.content_disposition, request);
                if let Some(metadata) = &self.metadata {
                    for m in metadata.iter() {
                        request = add_mandatory_header(&m, request);
                    }
                }
                request = add_optional_header_ref(&self.lease_id, request);
                request = add_optional_header(&self.sequence_number, request);
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
