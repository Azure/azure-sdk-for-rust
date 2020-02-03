use crate::blob::generate_blob_uri;
use crate::blob::responses::PutBlobResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::headers::BLOB_TYPE;
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::{
    AccessTierOption, AccessTierSupport, BlobNameRequired, BlobNameSupport, CacheControlOption,
    CacheControlSupport, ClientRequestIdOption, ClientRequestIdSupport, ContainerNameRequired,
    ContainerNameSupport, ContentDispositionOption, ContentDispositionSupport,
    ContentEncodingOption, ContentEncodingSupport, ContentLanguageOption, ContentLanguageSupport,
    ContentTypeOption, ContentTypeSupport, LeaseIdOption, LeaseIdSupport, MetadataOption,
    MetadataSupport, No, PageBlobLengthRequired, PageBlobLengthSupport, SequenceNumberOption,
    SequenceNumberSupport, TimeoutOption, TimeoutSupport, ToAssign, Yes,
};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_content_length: PhantomData<ContentLengthSet>,
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

impl<'a> PutPageBlobBuilder<'a, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> PutPageBlobBuilder<'a, No, No, No> {
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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ClientRequired<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, ContentLengthSet> ContainerNameRequired<'a>
    for PutPageBlobBuilder<'a, Yes, BlobNameSet, ContentLengthSet>
where
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, ContentLengthSet> BlobNameRequired<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, Yes, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> PageBlobLengthRequired
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn content_length(&self) -> u64 {
        self.content_length
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> SequenceNumberOption
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn sequence_number(&self) -> u64 {
        self.sequence_number
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> AccessTierOption<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn access_tier(&self) -> Option<&'a str> {
        self.access_tier
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> TimeoutOption
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ContentTypeOption<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn content_type(&self) -> Option<&'a str> {
        self.content_type
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ContentEncodingOption<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn content_encoding(&self) -> Option<&'a str> {
        self.content_encoding
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ContentLanguageOption<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn content_language(&self) -> Option<&'a str> {
        self.content_language
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> CacheControlOption<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn cache_control(&self) -> Option<&'a str> {
        self.cache_control
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ContentDispositionOption<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn content_disposition(&self) -> Option<&'a str> {
        self.content_disposition
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> MetadataOption<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> LeaseIdOption<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ClientRequestIdOption<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ContainerNameSupport<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, Yes, BlobNameSet, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> BlobNameSupport<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, Yes, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> PageBlobLengthSupport
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = Result<PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, Yes>, AzureError>;

    #[inline]
    fn with_content_length(self, content_length: u64) -> Self::O {
        // sadly this must be checked at runtime
        if content_length % 512 != 0 {
            Err(AzureError::Not512ByteAlignedError(content_length))
        } else {
            Ok(PutPageBlobBuilder {
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
            })
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> SequenceNumberSupport
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> AccessTierSupport<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> TimeoutSupport
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ContentTypeSupport<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ContentEncodingSupport<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ContentLanguageSupport<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> CacheControlSupport<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ContentDispositionSupport<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> MetadataSupport<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> LeaseIdSupport<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet> ClientRequestIdSupport<'a>
    for PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
    type O = PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>;

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

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
    PutPageBlobBuilder<'a, ContainerNameSet, BlobNameSet, ContentLengthSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ContentLengthSet: ToAssign,
{
}

impl<'a> PutPageBlobBuilder<'a, Yes, Yes, Yes> {
    #[inline]
    pub async fn finalize(self) -> Result<PutBlobResponse, AzureError> {
        let mut uri = generate_blob_uri(&self, None);

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}?{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            |mut request| {
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
