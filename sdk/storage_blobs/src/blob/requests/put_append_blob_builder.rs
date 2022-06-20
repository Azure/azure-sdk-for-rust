use crate::{blob::responses::PutBlobResponse, prelude::*};
use azure_core::{headers::BLOB_TYPE, prelude::*};

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

    pub async fn execute(&self) -> azure_core::Result<PutBlobResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::PUT, None)?;
        request.insert_header(BLOB_TYPE, "AppendBlob");
        request.add_optional_header(self.content_type.as_ref());
        request.add_optional_header(self.content_encoding.as_ref());
        request.add_optional_header(self.content_language.as_ref());
        request.add_optional_header(self.content_disposition.as_ref());
        if let Some(metadata) = &self.metadata {
            for m in metadata.iter() {
                request.add_mandatory_header(&m);
            }
        }
        request.add_optional_header(self.lease_id);
        request.add_optional_header(self.client_request_id.as_ref());

        let response = self
            .blob_client
            .execute_request_check_status(&request)
            .await?;

        debug!("response.headers() == {:#?}", response.headers());

        PutBlobResponse::from_headers(response.headers())
    }
}
