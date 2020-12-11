use crate::blob::blob::generate_blob_uri;
use crate::blob::blob::responses::PutBlockResponse;
use crate::core::prelude::*;
use azure_core::errors::AzureError;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_body: PhantomData<BodySet>,
    p_block_id: PhantomData<BlockIdSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    body: Option<&'a [u8]>,
    block_id: Option<&'a [u8]>,
    timeout: Option<u64>,
    content_md5: Option<&'a [u8]>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> PutBlockBuilder<'a, C, No, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> PutBlockBuilder<'a, C, No, No, No, No> {
        PutBlockBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_body: PhantomData {},
            body: None,
            p_block_id: PhantomData {},
            block_id: None,
            timeout: None,
            content_md5: None,
            lease_id: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet> ClientRequired<'a, C>
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, BodySet, BlockIdSet> ContainerNameRequired<'a>
    for PutBlockBuilder<'a, C, Yes, BlobNameSet, BodySet, BlockIdSet>
where
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BodySet, BlockIdSet> BlobNameRequired<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, Yes, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BlockIdSet> BodyRequired<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn body(&self) -> &'a [u8] {
        self.body.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> BlockIdRequired<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn block_id(&self) -> &'a [u8] {
        self.block_id.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet> TimeoutOption
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet> ContentMD5Option<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_md5(&self) -> Option<&'a [u8]> {
        self.content_md5
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet> LeaseIdOption<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet> ClientRequestIdOption<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet, BodySet, BlockIdSet> ContainerNameSupport<'a>
    for PutBlockBuilder<'a, C, No, BlobNameSet, BodySet, BlockIdSet>
where
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    type O = PutBlockBuilder<'a, C, Yes, BlobNameSet, BodySet, BlockIdSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        PutBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            p_block_id: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            body: self.body,
            block_id: self.block_id,
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BodySet, BlockIdSet> BlobNameSupport<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, No, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    type O = PutBlockBuilder<'a, C, ContainerNameSet, Yes, BodySet, BlockIdSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        PutBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            p_block_id: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            body: self.body,
            block_id: self.block_id,
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BlockIdSet> BodySupport<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, No, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    type O = PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes, BlockIdSet>;

    #[inline]
    fn with_body(self, body: &'a [u8]) -> Self::O {
        PutBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            p_block_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: Some(body),
            block_id: self.block_id,
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> BlockIdSupport<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, Yes>;

    #[inline]
    fn with_block_id(self, block_id: &'a [u8]) -> Self::O {
        PutBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            p_block_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            block_id: Some(block_id),
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet> TimeoutSupport
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    type O = PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        PutBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            p_block_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            block_id: self.block_id,
            timeout: Some(timeout),
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet> ContentMD5Support<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    type O = PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>;

    #[inline]
    fn with_content_md5(self, content_md5: &'a [u8]) -> Self::O {
        PutBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            p_block_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            block_id: self.block_id,
            timeout: self.timeout,
            content_md5: Some(content_md5),
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet> LeaseIdSupport<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    type O = PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        PutBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            p_block_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            block_id: self.block_id,
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet> ClientRequestIdSupport<'a>
    for PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    BlockIdSet: ToAssign,
    C: Client,
{
    type O = PutBlockBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet, BlockIdSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        PutBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            p_block_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            block_id: self.block_id,
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> PutBlockBuilder<'a, C, Yes, Yes, Yes, Yes>
where
    C: Client,
{
    #[inline]
    pub async fn finalize(self) -> Result<PutBlockResponse, AzureError> {
        let mut uri = generate_blob_uri(
            self.client(),
            self.container_name(),
            self.blob_name(),
            Some("comp=block"),
        );

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, timeout);
        }
        uri = format!("{}&{}", uri, BlockIdRequired::to_uri_parameter(&self));

        trace!("uri == {:?}", uri);

        let perform_request_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            &|mut request| {
                request = ContentMD5Option::add_optional_header(&self, request);
                request = LeaseIdOption::add_optional_header(&self, request);
                request = ClientRequestIdOption::add_optional_header(&self, request);
                request
            },
            Some(self.body()),
        )?;

        let (headers, _body) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::CREATED)
            .await?;
        PutBlockResponse::from_headers(&headers)
    }
}
