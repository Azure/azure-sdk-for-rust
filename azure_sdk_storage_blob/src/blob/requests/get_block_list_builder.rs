use crate::blob::generate_blob_uri;
use crate::blob::responses::GetBlockListResponse;
use crate::blob::{BlockListType, BlockListTypeRequired, BlockListTypeSupport};
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::{
    BlobNameRequired, BlobNameSupport, ClientRequestIdOption, ClientRequestIdSupport,
    ContainerNameRequired, ContainerNameSupport, LeaseIdOption, LeaseIdSupport, No, TimeoutOption,
    TimeoutSupport, ToAssign, Yes,
};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    client: &'a Client,
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

impl<'a> GetBlockListBuilder<'a, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> GetBlockListBuilder<'a, No, No, No> {
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

impl<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet> ClientRequired<'a>
    for GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, BlobListTypeSet> ContainerNameRequired<'a>
    for GetBlockListBuilder<'a, Yes, BlobNameSet, BlobListTypeSet>
where
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobListTypeSet> BlobNameRequired<'a>
    for GetBlockListBuilder<'a, ContainerNameSet, Yes, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> BlockListTypeRequired
    for GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn block_list_type(&self) -> BlockListType {
        self.block_list_type
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet> TimeoutOption
    for GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet> LeaseIdOption<'a>
    for GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet> ClientRequestIdOption<'a>
    for GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet> ContainerNameSupport<'a>
    for GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    type O = GetBlockListBuilder<'a, Yes, BlobNameSet, BlobListTypeSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet> BlobNameSupport<'a>
    for GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    type O = GetBlockListBuilder<'a, ContainerNameSet, Yes, BlobListTypeSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet> BlockListTypeSupport
    for GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    type O = GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

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

impl<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet> TimeoutSupport
    for GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    type O = GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet> LeaseIdSupport<'a>
    for GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    type O = GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet> ClientRequestIdSupport<'a>
    for GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
    type O = GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>;

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
impl<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
    GetBlockListBuilder<'a, ContainerNameSet, BlobNameSet, BlobListTypeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BlobListTypeSet: ToAssign,
{
}

impl<'a> GetBlockListBuilder<'a, Yes, Yes, Yes> {
    #[inline]
    pub async fn finalize(self) -> Result<GetBlockListResponse, AzureError> {
        let mut uri = generate_blob_uri(&self, Some("comp=blocklist"));

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, timeout);
        }
        uri = format!("{}&{}", uri, BlockListTypeRequired::to_uri_parameter(&self));

        trace!("uri == {:?}", uri);

        let future_response = self.client().perform_request(
            &uri,
            &Method::GET,
            |mut request| {
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
