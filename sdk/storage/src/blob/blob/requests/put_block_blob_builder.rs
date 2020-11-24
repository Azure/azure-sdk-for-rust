use crate::blob::blob::generate_blob_uri;
use crate::blob::blob::responses::PutBlockBlobResponse;
use crate::core::client::Client;
use crate::core::ClientRequired;
use azure_core::errors::AzureError;
use azure_core::headers::BLOB_TYPE;
use azure_core::lease::LeaseId;
use azure_core::modify_conditions::IfMatchCondition;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_body: PhantomData<BodySet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    body: Option<&'a [u8]>,
    timeout: Option<u64>,
    content_type: Option<&'a str>,
    content_encoding: Option<&'a str>,
    content_language: Option<&'a str>,
    cache_control: Option<&'a str>,
    content_md5: Option<&'a [u8]>,
    content_disposition: Option<&'a str>,
    metadata: Option<&'a HashMap<&'a str, &'a str>>,
    lease_id: Option<&'a LeaseId>,
    if_match_condition: Option<IfMatchCondition<'a>>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> PutBlockBlobBuilder<'a, C, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> PutBlockBlobBuilder<'a, C, No, No, No> {
        PutBlockBlobBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_body: PhantomData {},
            body: None,
            timeout: None,
            content_type: None,
            content_encoding: None,
            content_language: None,
            cache_control: None,
            content_md5: None,
            content_disposition: None,
            metadata: None,
            lease_id: None,
            if_match_condition: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ClientRequired<'a, C>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, BodySet> ContainerNameRequired<'a>
    for PutBlockBlobBuilder<'a, C, Yes, BlobNameSet, BodySet>
where
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BodySet> BlobNameRequired<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, Yes, BodySet>
where
    ContainerNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> BodyRequired<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn body(&self) -> &'a [u8] {
        self.body.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> TimeoutOption
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ContentTypeOption<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_type(&self) -> Option<&'a str> {
        self.content_type
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ContentEncodingOption<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_encoding(&self) -> Option<&'a str> {
        self.content_encoding
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ContentLanguageOption<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_language(&self) -> Option<&'a str> {
        self.content_language
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> CacheControlOption<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn cache_control(&self) -> Option<&'a str> {
        self.cache_control
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ContentMD5Option<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_md5(&self) -> Option<&'a [u8]> {
        self.content_md5
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ContentDispositionOption<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_disposition(&self) -> Option<&'a str> {
        self.content_disposition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> MetadataOption<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> LeaseIdOption<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> IfMatchConditionOption<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ClientRequestIdOption<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet, BodySet> ContainerNameSupport<'a>
    for PutBlockBlobBuilder<'a, C, No, BlobNameSet, BodySet>
where
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, Yes, BlobNameSet, BodySet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BodySet> BlobNameSupport<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, No, BodySet>
where
    ContainerNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, Yes, BodySet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            body: self.body,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> BodySupport<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_body(self, body: &'a [u8]) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: Some(body),
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> TimeoutSupport
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: Some(timeout),
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ContentTypeSupport<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_content_type(self, content_type: &'a str) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_type: Some(content_type),
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ContentEncodingSupport<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_content_encoding(self, content_encoding: &'a str) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: Some(content_encoding),
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ContentLanguageSupport<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_content_language(self, content_language: &'a str) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: Some(content_language),
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> CacheControlSupport<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_cache_control(self, cache_control: &'a str) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: Some(cache_control),
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ContentMD5Support<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_content_md5(self, content_md5: &'a [u8]) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: Some(content_md5),
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ContentDispositionSupport<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_content_disposition(self, content_disposition: &'a str) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: Some(content_disposition),
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> MetadataSupport<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: Some(metadata),
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> LeaseIdSupport<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: Some(lease_id),
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> IfMatchConditionSupport<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: Some(if_match_condition),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BodySet> ClientRequestIdSupport<'a>
    for PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
    C: Client,
{
    type O = PutBlockBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        PutBlockBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_md5: self.content_md5,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> PutBlockBlobBuilder<'a, C, Yes, Yes, Yes>
where
    C: Client,
{
    #[inline]
    pub async fn finalize(self) -> Result<PutBlockBlobResponse, AzureError> {
        let mut uri =
            generate_blob_uri(self.client(), self.container_name(), self.blob_name(), None);

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}?{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let (headers, _) = self
            .client()
            .perform_request(
                &uri,
                &Method::PUT,
                &|mut request| {
                    request = ContentTypeOption::add_header(&self, request);
                    request = ContentEncodingOption::add_header(&self, request);
                    request = ContentLanguageOption::add_header(&self, request);
                    request = ContentMD5Option::add_header(&self, request);
                    request = CacheControlOption::add_header(&self, request);
                    request = ContentDispositionOption::add_header(&self, request);
                    request = MetadataOption::add_header(&self, request);
                    request = request.header(BLOB_TYPE, "BlockBlob");
                    request = LeaseIdOption::add_header(&self, request);
                    request = IfMatchConditionOption::add_header(&self, request);
                    request = ClientRequestIdOption::add_header(&self, request);
                    request
                },
                Some(self.body()),
            )?
            .check_status_extract_headers_and_body(StatusCode::CREATED)
            .await?;
        PutBlockBlobResponse::from_headers(&headers)
    }
}
