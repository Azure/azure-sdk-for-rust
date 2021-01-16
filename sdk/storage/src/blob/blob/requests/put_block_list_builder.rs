use crate::blob::blob::responses::PutBlockListResponse;
use crate::blob::prelude::*;
use crate::core::prelude::*;
use azure_core::headers::{add_mandatory_header, add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;
use bytes::Bytes;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct PutBlockListBuilder<'a, T>
where
    T: Borrow<[u8]> + 'a,
{
    blob_client: &'a BlobClient,
    block_list: &'a BlockList<T>,
    hash: Option<&'a Hash>,
    content_type: Option<ContentType<'a>>,
    content_encoding: Option<ContentEncoding<'a>>,
    content_language: Option<ContentLanguage<'a>>,
    content_disposition: Option<ContentDisposition<'a>>,
    metadata: Option<&'a Metadata>,
    access_tier: AccessTier,
    // TODO: Support tags
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a, T> PutBlockListBuilder<'a, T>
where
    T: Borrow<[u8]> + 'a,
{
    pub(crate) fn new(blob_client: &'a BlobClient, block_list: &'a BlockList<T>) -> Self {
        Self {
            blob_client,
            block_list,
            hash: None,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            metadata: None,
            access_tier: AccessTier::Hot,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
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
                request = add_optional_header_ref(&self.hash, request);
                request = add_optional_header(&self.content_type, request);
                request = add_optional_header(&self.content_encoding, request);
                request = add_optional_header(&self.content_language, request);
                request = add_optional_header(&self.content_disposition, request);
                request = add_optional_header(&self.metadata, request);
                request = add_mandatory_header(&self.access_tier, request);
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
