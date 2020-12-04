use crate::blob::blob::generate_blob_uri;
use crate::blob::blob::responses::PutBlobResponse;
use crate::core::prelude::*;
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
pub struct PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    timeout: Option<u64>,
    content_type: Option<&'a str>,
    content_encoding: Option<&'a str>,
    content_language: Option<&'a str>,
    cache_control: Option<&'a str>,
    content_disposition: Option<&'a str>,
    metadata: Option<&'a HashMap<&'a str, &'a str>>,
    lease_id: Option<&'a LeaseId>,
    if_match_condition: Option<IfMatchCondition<'a>>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> PutAppendBlobBuilder<'a, C, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> PutAppendBlobBuilder<'a, C, No, No> {
        PutAppendBlobBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            timeout: None,
            content_type: None,
            content_encoding: None,
            content_language: None,
            cache_control: None,
            content_disposition: None,
            metadata: None,
            lease_id: None,
            if_match_condition: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ClientRequired<'a, C>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet> ContainerNameRequired<'a> for PutAppendBlobBuilder<'a, C, Yes, BlobNameSet>
where
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet> BlobNameRequired<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> TimeoutOption
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ContentTypeOption<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_type(&self) -> Option<&'a str> {
        self.content_type
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ContentEncodingOption<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_encoding(&self) -> Option<&'a str> {
        self.content_encoding
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ContentLanguageOption<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_language(&self) -> Option<&'a str> {
        self.content_language
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> CacheControlOption<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn cache_control(&self) -> Option<&'a str> {
        self.cache_control
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ContentDispositionOption<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn content_disposition(&self) -> Option<&'a str> {
        self.content_disposition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> MetadataOption<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseIdOption<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> IfMatchConditionOption<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ClientRequestIdOption<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet> ContainerNameSupport<'a> for PutAppendBlobBuilder<'a, C, No, BlobNameSet>
where
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, Yes, BlobNameSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet> BlobNameSupport<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, No>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, ContainerNameSet, Yes>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> TimeoutSupport
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            timeout: Some(timeout),
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ContentTypeSupport<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_content_type(self, content_type: &'a str) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            timeout: self.timeout,
            content_type: Some(content_type),
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ContentEncodingSupport<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_content_encoding(self, content_encoding: &'a str) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: Some(content_encoding),
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ContentLanguageSupport<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_content_language(self, content_language: &'a str) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: Some(content_language),
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> CacheControlSupport<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_cache_control(self, cache_control: &'a str) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: Some(cache_control),
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ContentDispositionSupport<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_content_disposition(self, content_disposition: &'a str) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: Some(content_disposition),
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> MetadataSupport<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: Some(metadata),
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseIdSupport<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: Some(lease_id),
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> IfMatchConditionSupport<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: Some(if_match_condition),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ClientRequestIdSupport<'a>
    for PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = PutAppendBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        PutAppendBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            timeout: self.timeout,
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            metadata: self.metadata,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> PutAppendBlobBuilder<'a, C, Yes, Yes>
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

        let (headers, _) = self
            .client()
            .perform_request(
                &uri,
                &Method::PUT,
                &|mut request| {
                    request = ContentTypeOption::add_optional_header(&self, request);
                    request = ContentEncodingOption::add_optional_header(&self, request);
                    request = ContentLanguageOption::add_optional_header(&self, request);
                    request = CacheControlOption::add_optional_header(&self, request);
                    request = ContentDispositionOption::add_optional_header(&self, request);
                    request = MetadataOption::add_optional_header(&self, request);
                    request = request.header(BLOB_TYPE, "AppendBlob");
                    request = LeaseIdOption::add_optional_header(&self, request);
                    request = IfMatchConditionOption::add_optional_header(&self, request);
                    request = ClientRequestIdOption::add_optional_header(&self, request);
                    request
                },
                None,
            )?
            .check_status_extract_headers_and_body(StatusCode::CREATED)
            .await?;
        PutBlobResponse::from_headers(&headers)
    }
}
