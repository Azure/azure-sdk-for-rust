use crate::blob::generate_blob_uri;
use crate::blob::responses::GetBlockListResponse;
use crate::blob::{BlockListType, BlockListTypeRequired, BlockListTypeSupport};
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

pub struct GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_block_list_type: PhantomData<BlobListTypeSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    block_list_type: BlockListType,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> GetBlockListBuilder<'a, C, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> GetBlockListBuilder<'a, C, No, No, No> {
        GetBlockListBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_block_list_type: PhantomData {},
            block_list_type: BlockListType::Committed,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet> ClientRequired<'a, C>
    for GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, BlobListTypeSet> ContainerNameRequired<'a>
    for GetBlockListBuilder<'a, C, Yes, BlobNameSet, BlobListTypeSet>
where
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobListTypeSet> BlobNameRequired<'a>
    for GetBlockListBuilder<'a, C, ContainerNameSet, Yes, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> BlockListTypeRequired
    for GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn block_list_type(&self) -> BlockListType {
        self.block_list_type
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet> TimeoutOption
    for GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet> LeaseIdOption<'a>
    for GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet> ClientRequestIdOption<'a>
    for GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet, BlobListTypeSet> ContainerNameSupport<'a>
    for GetBlockListBuilder<'a, C, No, BlobNameSet, BlobListTypeSet>
where
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    type O = GetBlockListBuilder<'a, C, Yes, BlobNameSet, BlobListTypeSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        GetBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list_type: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            block_list_type: self.block_list_type,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobListTypeSet> BlobNameSupport<'a>
    for GetBlockListBuilder<'a, C, ContainerNameSet, No, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    type O = GetBlockListBuilder<'a, C, ContainerNameSet, Yes, BlobListTypeSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        GetBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list_type: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            block_list_type: self.block_list_type,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> BlockListTypeSupport
    for GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_block_list_type(self, block_list_type: BlockListType) -> Self::O {
        GetBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list_type: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list_type,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet> TimeoutSupport
    for GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    type O = GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        GetBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list_type: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list_type: self.block_list_type,
            timeout: Some(timeout),
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet> LeaseIdSupport<'a>
    for GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    type O = GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        GetBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list_type: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list_type: self.block_list_type,
            timeout: self.timeout,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet> ClientRequestIdSupport<'a>
    for GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
    type O = GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        GetBlockListBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_block_list_type: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            block_list_type: self.block_list_type,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>
    GetBlockListBuilder<'a, C, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
    C: Client,
{
}

// methods callable only when every mandatory field has been filled
impl<'a, C> GetBlockListBuilder<'a, C, Yes, Yes, Yes>
where
    C: Client,
{
    #[inline]
    pub async fn finalize(self) -> Result<GetBlockListResponse, AzureError> {
        let mut uri = generate_blob_uri(
            self.client,
            self.container_name(),
            self.blob_name(),
            Some("comp=blocklist"),
        );

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, timeout);
        }
        uri = format!("{}&{}", uri, BlockListTypeRequired::to_uri_parameter(&self));

        trace!("uri == {:?}", uri);

        let future_response = self.client().perform_request(
            &uri,
            &Method::GET,
            &|mut request| {
                request = LeaseIdOption::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;
        GetBlockListResponse::from_response(&headers, &body)
    }
}
