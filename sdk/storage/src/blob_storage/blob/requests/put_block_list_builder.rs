use crate::blob_storage::blob::responses::PutBlockListResponse;
use crate::blob_storage::prelude::*;
use azure_core::headers::{add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;
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
    client_request_id: Option<ClientRequestId<'a>>,
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
            access_tier: Some(AccessTier::Hot),
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
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(
        &self,
    ) -> Result<PutBlockListResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self
            .blob_client
            .storage_account_client()
            .blob_storage_url()
            .to_owned();
        url.path_segments_mut()
            .map_err(|_| "Invalid blob URL")?
            .push(self.blob_client.container_client().container_name())
            .push(self.blob_client.blob_name());

        url.query_pairs_mut().append_pair("comp", "blocklist");
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let body = self.block_list.to_xml();
        let body_bytes = Bytes::from(body);

        // calculate the xml MD5. This can be made optional
        // if needed, but i think it's best to calculate it.
        let md5 = {
            let hash = md5::compute(body_bytes.clone());
            debug!("md5 hash: {:02X}", hash);
            base64::encode(hash.0)
        };

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = request.header("Content-MD5", &md5);
                request = add_optional_header(&self.content_type, request);
                request = add_optional_header(&self.content_encoding, request);
                request = add_optional_header(&self.content_language, request);
                request = add_optional_header(&self.content_disposition, request);
                request = add_optional_header(&self.content_md5, request);
                request = add_optional_header(&self.metadata, request);
                request = add_optional_header(&self.access_tier, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            Some(body_bytes),
        )?;

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::CREATED)
            .await?;

        debug!("response.headers() == {:#?}", response.headers());

        Ok(PutBlockListResponse::from_headers(response.headers())?)
    }
}
