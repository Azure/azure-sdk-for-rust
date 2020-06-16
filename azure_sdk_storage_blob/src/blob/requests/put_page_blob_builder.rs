use crate::blob::generate_blob_uri;
use crate::blob::responses::PutBlobResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::headers::BLOB_TYPE;
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::prelude::*;
use hyper::{Method, StatusCode};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_content_length: PhantomData<PageBlobLengthSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    content_length: u64,
    sequence_number: u64,
    access_tier: Option<&'a str>,
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

impl<'a, C> PutPageBlobBuilder<'a, C, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> PutPageBlobBuilder<'a, C, No, No, No> {
        PutPageBlobBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_content_length: PhantomData {},
            content_length: 0,
            sequence_number: 0,
            access_tier: None,
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> ClientRequired<'a, C>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, PageBlobLengthSet> ContainerNameRequired<'a>
    for PutPageBlobBuilder<'a, C, Yes, BlobNameSet, PageBlobLengthSet>
where
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, PageBlobLengthSet> BlobNameRequired<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, Yes, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> PageBlobLengthRequired
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_length(&self) -> u64 {
        self.content_length
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> SequenceNumberOption
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn sequence_number(&self) -> u64 {
        self.sequence_number
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> AccessTierOption<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn access_tier(&self) -> Option<&'a str> {
        self.access_tier
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> TimeoutOption
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> ContentTypeOption<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_type(&self) -> Option<&'a str> {
        self.content_type
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> ContentEncodingOption<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_encoding(&self) -> Option<&'a str> {
        self.content_encoding
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> ContentLanguageOption<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_language(&self) -> Option<&'a str> {
        self.content_language
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> CacheControlOption<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn cache_control(&self) -> Option<&'a str> {
        self.cache_control
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> ContentDispositionOption<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_disposition(&self) -> Option<&'a str> {
        self.content_disposition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> MetadataOption<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> LeaseIdOption<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> ClientRequestIdOption<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet, PageBlobLengthSet> ContainerNameSupport<'a>
    for PutPageBlobBuilder<'a, C, No, BlobNameSet, PageBlobLengthSet>
where
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, Yes, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, PageBlobLengthSet> BlobNameSupport<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, No, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, Yes, PageBlobLengthSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, BlobNameSet> PageBlobLengthSupport
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_content_length(self, content_length: u64) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> SequenceNumberSupport
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_sequence_number(self, sequence_number: u64) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> AccessTierSupport<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_access_tier(self, access_tier: &'a str) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: Some(access_tier),
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> TimeoutSupport
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> ContentTypeSupport<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_content_type(self, content_type: &'a str) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> ContentEncodingSupport<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_content_encoding(self, content_encoding: &'a str) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> ContentLanguageSupport<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_content_language(self, content_language: &'a str) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> CacheControlSupport<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_cache_control(self, cache_control: &'a str) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> ContentDispositionSupport<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_content_disposition(self, content_disposition: &'a str) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> MetadataSupport<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> LeaseIdSupport<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet> ClientRequestIdSupport<'a>
    for PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    PageBlobLengthSet: ToAssign,
    C: Client,
{
    type O = PutPageBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, PageBlobLengthSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        PutPageBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_content_length: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            content_length: self.content_length,
            sequence_number: self.sequence_number,
            access_tier: self.access_tier,
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

impl<'a, C> PutPageBlobBuilder<'a, C, Yes, Yes, Yes>
where
    C: Client,
{
    #[inline]
    pub async fn finalize(self) -> Result<PutBlobResponse, AzureError> {
        let mut uri =
            generate_blob_uri(self.client(), self.container_name(), self.blob_name(), None);

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}?{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            &|mut request| {
                request = PageBlobLengthRequired::add_header(&self, request);
                request = SequenceNumberOption::add_header(&self, request);
                request = AccessTierOption::add_header(&self, request);
                request = ContentTypeOption::add_header(&self, request);
                request = ContentEncodingOption::add_header(&self, request);
                request = ContentLanguageOption::add_header(&self, request);
                request = CacheControlOption::add_header(&self, request);
                request = ContentDispositionOption::add_header(&self, request);
                request = MetadataOption::add_header(&self, request);
                request = request.header(BLOB_TYPE, "PageBlob");
                request = LeaseIdOption::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;
        PutBlobResponse::from_headers(&headers)
    }
}
