use crate::blob::generate_blob_uri;
use crate::blob::responses::CopyBlobFromUrlResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::prelude::*;
use hyper::{Method, StatusCode};
use std::collections::HashMap;
use std::convert::TryInto;
use std::marker::PhantomData;

pub struct CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_source_url: PhantomData<SourceUrlSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    source_url: Option<&'a str>,
    timeout: Option<u64>,
    is_synchronous: bool,
    source_content_md5: Option<&'a [u8]>,
    lease_id: Option<&'a LeaseId>,
    content_type: Option<&'a str>,
    content_encoding: Option<&'a str>,
    content_language: Option<&'a str>,
    cache_control: Option<&'a str>,
    content_disposition: Option<&'a str>,
    metadata: Option<&'a HashMap<&'a str, &'a str>>,
    if_since_condition: Option<IfSinceCondition>,
    if_match_condition: Option<IfMatchCondition<'a>>,
    client_request_id: Option<&'a str>,
}

impl<'a> CopyBlobFromUrlBuilder<'a, No, No, No> {
    pub(crate) fn new(client: &'a Client) -> CopyBlobFromUrlBuilder<'a, No, No, No> {
        CopyBlobFromUrlBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_source_url: PhantomData {},
            source_url: None,
            timeout: None,
            is_synchronous: true,
            source_content_md5: None,
            lease_id: None,
            content_type: None,
            content_encoding: None,
            content_language: None,
            cache_control: None,
            content_disposition: None,
            metadata: None,
            if_since_condition: None,
            if_match_condition: None,
            client_request_id: None,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> ClientRequired<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn client(&self) -> &'a Client {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, BlobNameSet, SourceUrlSet> ContainerNameRequired<'a>
    for CopyBlobFromUrlBuilder<'a, Yes, BlobNameSet, SourceUrlSet>
where
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, SourceUrlSet> BlobNameRequired<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, Yes, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> SourceUrlRequired<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    fn source_url(&self) -> &'a str {
        self.source_url.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> TimeoutOption
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> IsSynchronousOption
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn is_synchronous(&self) -> bool {
        self.is_synchronous
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> SourceContentMD5Option<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn source_content_md5(&self) -> Option<&'a [u8]> {
        self.source_content_md5
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> LeaseIdOption<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> ContentTypeOption<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn content_type(&self) -> Option<&'a str> {
        self.content_type
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> ContentEncodingOption<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn content_encoding(&self) -> Option<&'a str> {
        self.content_encoding
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> ContentLanguageOption<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn content_language(&self) -> Option<&'a str> {
        self.content_language
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> CacheControlOption<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn cache_control(&self) -> Option<&'a str> {
        self.cache_control
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> ContentDispositionOption<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn content_disposition(&self) -> Option<&'a str> {
        self.content_disposition
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> MetadataOption<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> IfSinceConditionOption
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn if_since_condition(&self) -> Option<IfSinceCondition> {
        self.if_since_condition
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> IfMatchConditionOption<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> ClientRequestIdOption<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, BlobNameSet, SourceUrlSet> ContainerNameSupport<'a>
    for CopyBlobFromUrlBuilder<'a, No, BlobNameSet, SourceUrlSet>
where
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, Yes, BlobNameSet, SourceUrlSet>;

    fn with_container_name(self, container_name: &'a str) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, SourceUrlSet> BlobNameSupport<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, No, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, Yes, SourceUrlSet>;

    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> SourceUrlSupport<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

    fn with_source_url(self, source_url: &'a str) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: Some(source_url),
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> TimeoutSupport
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_timeout(self, timeout: u64) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: Some(timeout),
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> IsSynchronousSupport
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_is_synchronous(self, is_synchronous: bool) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> SourceContentMD5Support<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_source_content_md5(self, source_content_md5: &'a [u8]) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: Some(source_content_md5),
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> LeaseIdSupport<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: Some(lease_id),
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> ContentTypeSupport<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_content_type(self, content_type: &'a str) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: Some(content_type),
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> ContentEncodingSupport<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_content_encoding(self, content_encoding: &'a str) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: Some(content_encoding),
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> ContentLanguageSupport<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_content_language(self, content_language: &'a str) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: Some(content_language),
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> CacheControlSupport<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_cache_control(self, cache_control: &'a str) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: Some(cache_control),
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> ContentDispositionSupport<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_content_disposition(self, content_disposition: &'a str) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: Some(content_disposition),
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> MetadataSupport<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: Some(metadata),
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> IfSinceConditionSupport
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_if_since_condition(self, if_since_condition: IfSinceCondition) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: Some(if_since_condition),
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> IfMatchConditionSupport<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: Some(if_match_condition),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet> ClientRequestIdSupport<'a>
    for CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
    type O = CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        CopyBlobFromUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            timeout: self.timeout,
            is_synchronous: self.is_synchronous,
            source_content_md5: self.source_content_md5,
            lease_id: self.lease_id,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
    CopyBlobFromUrlBuilder<'a, ContainerNameSet, BlobNameSet, SourceUrlSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlSet: ToAssign,
{
}

// methods callable only when every mandatory field has been filled
impl<'a> CopyBlobFromUrlBuilder<'a, Yes, Yes, Yes> {
    #[inline]
    pub async fn finalize(self) -> Result<CopyBlobFromUrlResponse, AzureError> {
        let mut uri = generate_blob_uri(&self, None);

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}?{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            |mut request| {
                request = SourceUrlRequired::add_header(&self, request);
                request = IsSynchronousOption::add_header(&self, request);
                request = SourceContentMD5Option::add_header(&self, request);
                request = ContentTypeOption::add_header(&self, request);
                request = ContentEncodingOption::add_header(&self, request);
                request = ContentLanguageOption::add_header(&self, request);
                request = CacheControlOption::add_header(&self, request);
                request = ContentDispositionOption::add_header(&self, request);
                request = MetadataOption::add_header(&self, request);
                // According to the docs this is not needed. We'll keep it here
                // in case the docs are wrong.
                //request = request.header(BLOB_TYPE, "BlockBlob");
                request = IfSinceConditionOption::add_header(&self, request);
                request = IfMatchConditionOption::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::ACCEPTED).await?;
        (&headers).try_into()
    }
}
