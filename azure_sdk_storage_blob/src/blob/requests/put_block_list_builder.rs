use crate::blob::generate_blob_uri;
use crate::blob::responses::PutBlockListResponse;
use crate::blob::BlockList;
use crate::blob::{BlockListRequired, BlockListSupport};
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::{
    add_content_md5_header, BlobNameRequired, BlobNameSupport, CacheControlOption,
    CacheControlSupport, ClientRequestIdOption, ClientRequestIdSupport, ContainerNameRequired,
    ContainerNameSupport, ContentDispositionOption, ContentDispositionSupport,
    ContentEncodingOption, ContentEncodingSupport, ContentLanguageOption, ContentLanguageSupport,
    ContentTypeOption, ContentTypeSupport, LeaseIdOption, LeaseIdSupport, MetadataOption,
    MetadataSupport, No, TimeoutOption, TimeoutSupport, ToAssign, Yes,
};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use md5;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_block_list: PhantomData<BlockListSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    block_list: Option<&'a BlockList<T>>,
    timeout: Option<u64>,
    content_type: Option<&'a str>,
    content_encoding: Option<&'a str>,
    content_language: Option<&'a str>,
    cache_control: Option<&'a str>,
    content_disposition: Option<&'a str>,
    metadata: Option<&'a HashMap<&'a str, &'a str>>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
}

impl<'a, T> PutBlockListBuilder<'a, T, No, No, No>
where
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    pub(crate) fn new(client: &'a Client) -> PutBlockListBuilder<'a, T, No, No, No> {
        PutBlockListBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_block_list: PhantomData {},
            block_list: None,
            timeout: None,
            content_type: None,
            content_encoding: None,
            content_language: None,
            cache_control: None,
            content_disposition: None,
            metadata: None,
            lease_id: None,
            client_request_id: None,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ClientRequired<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, T, BlobNameSet, BlockListSet> ContainerNameRequired<'a>
    for PutBlockListBuilder<'a, T, Yes, BlobNameSet, BlockListSet>
where
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, T, ContainerNameSet, BlockListSet> BlobNameRequired<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, Yes, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet> BlockListRequired<'a, T>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn block_list(&self) -> &'a BlockList<T> {
        self.block_list.unwrap()
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> TimeoutOption
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ContentTypeOption<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn content_type(&self) -> Option<&'a str> {
        self.content_type
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ContentEncodingOption<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn content_encoding(&self) -> Option<&'a str> {
        self.content_encoding
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ContentLanguageOption<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn content_language(&self) -> Option<&'a str> {
        self.content_language
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> CacheControlOption<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn cache_control(&self) -> Option<&'a str> {
        self.cache_control
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ContentDispositionOption<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn content_disposition(&self) -> Option<&'a str> {
        self.content_disposition
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> MetadataOption<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> LeaseIdOption<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ClientRequestIdOption<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ContainerNameSupport<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, Yes, BlobNameSet, BlockListSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            block_list: self.block_list,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> BlobNameSupport<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, ContainerNameSet, Yes, BlockListSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            block_list: self.block_list,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> BlockListSupport<'a, T>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_block_list(self, block_list: &'a BlockList<T>) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list: Some(block_list),
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> TimeoutSupport
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list: self.block_list,
            timeout: Some(timeout),
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ContentTypeSupport<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>;

    #[inline]
    fn with_content_type(self, content_type: &'a str) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list: self.block_list,
            timeout: self.timeout,
            content_type: Some(content_type),
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ContentEncodingSupport<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>;

    #[inline]
    fn with_content_encoding(self, content_encoding: &'a str) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list: self.block_list,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: Some(content_encoding),
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ContentLanguageSupport<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>;

    #[inline]
    fn with_content_language(self, content_language: &'a str) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list: self.block_list,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: Some(content_language),
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> CacheControlSupport<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>;

    #[inline]
    fn with_cache_control(self, cache_control: &'a str) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list: self.block_list,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: Some(cache_control),
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ContentDispositionSupport<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>;

    #[inline]
    fn with_content_disposition(self, content_disposition: &'a str) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list: self.block_list,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: Some(content_disposition),
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> MetadataSupport<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>;

    #[inline]
    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list: self.block_list,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: Some(metadata),
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> LeaseIdSupport<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list: self.block_list,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet> ClientRequestIdSupport<'a>
    for PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
    type O = PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        PutBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list: self.block_list,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
    PutBlockListBuilder<'a, T, ContainerNameSet, BlobNameSet, BlockListSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockListSet: ToAssign,
    T: Borrow<[u8]> + 'a,
{
}

impl<'a, T> PutBlockListBuilder<'a, T, Yes, Yes, Yes>
where
    T: Borrow<[u8]> + 'a,
{
    #[inline]
    pub async fn finalize(self) -> Result<PutBlockListResponse, AzureError> {
        let mut uri = generate_blob_uri(&self, Some("comp=blocklist"));

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let body = BlockListRequired::to_string(&self);
        debug!("PutBlockListRequired::to_string == {}", body);
        let body_bytes = body.as_bytes();

        // calculate the xml MD5. This can be made optional
        // if needed, but i think it's best to calculate it.
        let md5 = {
            let hash = md5::compute(body_bytes);
            debug!("md5 hash: {:02X}", hash);
            hash
        };

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            |mut request| {
                request = ContentTypeOption::add_header(&self, request);
                request = ContentEncodingOption::add_header(&self, request);
                request = ContentLanguageOption::add_header(&self, request);
                request = add_content_md5_header(&md5[..], request);
                request = CacheControlOption::add_header(&self, request);
                request = ContentDispositionOption::add_header(&self, request);
                request = MetadataOption::add_header(&self, request);
                request = LeaseIdOption::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(body_bytes),
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;
        PutBlockListResponse::from_headers(&headers)
    }
}
