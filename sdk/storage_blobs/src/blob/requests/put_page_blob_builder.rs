use crate::{blob::responses::PutBlobResponse, prelude::*};
use azure_core::{
    headers::{BLOB_CONTENT_LENGTH, BLOB_TYPE},
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

    pub async fn execute(&self) -> azure_core::Result<PutBlobResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::PUT, None)?;
        request.insert_header(BLOB_TYPE, "PageBlob");
        request.insert_header(BLOB_CONTENT_LENGTH, &format!("{}", self.length));
        request.add_optional_header(self.content_type);
        request.add_optional_header(self.content_encoding);
        request.add_optional_header(self.content_language);
        request.add_optional_header(self.content_disposition);
        if let Some(metadata) = &self.metadata {
            for m in metadata.iter() {
                request.add_mandatory_header(&m);
            }
        }
        request.add_optional_header(self.lease_id);
        request.add_optional_header(self.sequence_number);
        request.add_optional_header(self.client_request_id);

        let response = self
            .blob_client
            .execute_request_check_status(&request)
            .await?;

        PutBlobResponse::from_headers(response.headers())
    }
}
