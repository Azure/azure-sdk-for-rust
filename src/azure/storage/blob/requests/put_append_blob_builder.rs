use azure::core::errors::{check_status_extract_headers_and_body, AzureError};
use azure::core::headers::BLOB_TYPE;
use azure::core::lease::LeaseId;
use azure::core::{
    BlobNameRequired, BlobNameSupport, CacheControlOption, CacheControlSupport, ClientRequestIdOption, ClientRequestIdSupport,
    ClientRequired, ContainerNameRequired, ContainerNameSupport, ContentDispositionOption, ContentDispositionSupport,
    ContentEncodingOption, ContentEncodingSupport, ContentLanguageOption, ContentLanguageSupport, ContentTypeOption, ContentTypeSupport,
    LeaseIdOption, LeaseIdSupport, MetadataOption, MetadataSupport, No, TimeoutOption, TimeoutSupport, ToAssign, Yes,
};
use azure::storage::blob::generate_blob_uri;
use azure::storage::blob::responses::PutBlobResponse;
use azure::storage::client::Client;
use futures::future::{done, ok};
use futures::prelude::*;
use hyper::{Method, StatusCode};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    client: &'a Client,
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
    client_request_id: Option<&'a str>,
}

impl<'a> PutAppendBlobBuilder<'a, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> PutAppendBlobBuilder<'a, No, No> {
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
            client_request_id: None,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ClientRequired<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet> ContainerNameRequired<'a> for PutAppendBlobBuilder<'a, Yes, BlobNameSet>
where
    BlobNameSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet> BlobNameRequired<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> TimeoutOption for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ContentTypeOption<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn content_type(&self) -> Option<&'a str> {
        self.content_type
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ContentEncodingOption<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn content_encoding(&self) -> Option<&'a str> {
        self.content_encoding
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ContentLanguageOption<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn content_language(&self) -> Option<&'a str> {
        self.content_language
    }
}

impl<'a, ContainerNameSet, BlobNameSet> CacheControlOption<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn cache_control(&self) -> Option<&'a str> {
        self.cache_control
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ContentDispositionOption<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn content_disposition(&self) -> Option<&'a str> {
        self.content_disposition
    }
}

impl<'a, ContainerNameSet, BlobNameSet> MetadataOption<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, ContainerNameSet, BlobNameSet> LeaseIdOption<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ClientRequestIdOption<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ContainerNameSupport<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = PutAppendBlobBuilder<'a, Yes, BlobNameSet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> BlobNameSupport<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = PutAppendBlobBuilder<'a, ContainerNameSet, Yes>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> TimeoutSupport for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ContentTypeSupport<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ContentEncodingSupport<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ContentLanguageSupport<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> CacheControlSupport<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ContentDispositionSupport<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> MetadataSupport<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> LeaseIdSupport<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ClientRequestIdSupport<'a> for PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet> PutAppendBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{}

impl<'a> PutAppendBlobBuilder<'a, Yes, Yes> {
    #[inline]
    pub fn finalize(self) -> impl Future<Item = PutBlobResponse, Error = AzureError> {
        let mut uri = generate_blob_uri(&self, None);

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}?{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let req = self.client().perform_request(
            &uri,
            Method::PUT,
            |ref mut request| {
                ContentTypeOption::add_header(&self, request);
                ContentEncodingOption::add_header(&self, request);
                ContentLanguageOption::add_header(&self, request);
                CacheControlOption::add_header(&self, request);
                ContentDispositionOption::add_header(&self, request);
                MetadataOption::add_header(&self, request);
                request.header(BLOB_TYPE, "AppendBlob");
                LeaseIdOption::add_header(&self, request);
                ClientRequestIdOption::add_header(&self, request);
            },
            None,
        );

        done(req)
            .from_err()
            .and_then(move |response| check_status_extract_headers_and_body(response, StatusCode::CREATED))
            .and_then(move |(headers, _body)| done(PutBlobResponse::from_headers(&headers)).and_then(|pbbr| ok(pbbr)))
    }
}
