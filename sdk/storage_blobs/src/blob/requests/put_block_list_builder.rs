use crate::{blob::responses::PutBlockListResponse, prelude::*};
use azure_core::{headers::add_optional_header_ref, prelude::*};
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct PutBlockListBuilder<'a> {
    blob_client: &'a BlobClient,
    block_list: &'a BlockList,
    content_type: Option<ContentType<'a>>,
    content_encoding: Option<ContentEncoding<'a>>,
    content_language: Option<ContentLanguage<'a>>,
    content_disposition: Option<ContentDisposition<'a>>,
    content_md5: Option<BlobContentMD5>,
    metadata: Option<&'a Metadata>,
    access_tier: Option<AccessTier>,
    // TODO: Support tags
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> PutBlockListBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, block_list: &'a BlockList) -> Self {
        Self {
            blob_client,
            block_list,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            content_md5: None,
            metadata: None,
            access_tier: None,
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
        content_md5: BlobContentMD5 => Some(content_md5),
        metadata: &'a Metadata => Some(metadata),
        access_tier: AccessTier => Some(access_tier),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> azure_core::Result<PutBlockListResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "blocklist");
        self.timeout.append_to_url_query(&mut url);

        let body = self.block_list.to_xml();
        let body_bytes = Bytes::from(body);

        // calculate the xml MD5. This can be made optional
        // if needed, but i think it's best to calculate it.
        let md5 = {
            let hash = md5::compute(body_bytes.clone());
            debug!("md5 hash: {:02X}", hash);
            base64::encode(hash.0)
        };

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), &http::Method::PUT, Some(body_bytes))?;
        request.insert_header("Content-MD5", &md5);
        request.add_optional_header(&self.content_type);
        request.add_optional_header(&self.content_encoding);
        request.add_optional_header(&self.content_language);
        request.add_optional_header(&self.content_disposition);
        request.add_optional_header(&self.content_md5);
        if let Some(metadata) = &self.metadata {
            for m in metadata.iter() {
                request.add_mandatory_header(&m);
            }
        }
        request.add_optional_header(&self.access_tier);
        request.add_optional_header_ref(&self.lease_id);
        request.add_optional_header(&self.client_request_id);

        let response = self
            .blob_client
            .execute_request_check_status(&request)
            .await?;

        PutBlockListResponse::from_headers(response.headers())
    }
}
