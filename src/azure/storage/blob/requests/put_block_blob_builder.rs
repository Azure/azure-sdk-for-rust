use azure::core::errors::{check_status_extract_headers_and_body, AzureError};
use azure::core::headers::BLOB_TYPE;
use azure::core::lease::LeaseId;
use azure::core::{
    BlobNameRequired, BlobNameSupport, BodyRequired, BodySupport, CacheControlOption, CacheControlSupport, ClientRequestIdOption,
    ClientRequestIdSupport, ClientRequired, ContainerNameRequired, ContainerNameSupport, ContentDispositionOption,
    ContentDispositionSupport, ContentEncodingOption, ContentEncodingSupport, ContentLanguageOption, ContentLanguageSupport,
    ContentMD5Option, ContentMD5Support, ContentTypeOption, ContentTypeSupport, LeaseIdOption, LeaseIdSupport, MetadataOption,
    MetadataSupport, No, TimeoutOption, TimeoutSupport, ToAssign, Yes,
};
use azure::storage::blob::responses::PutBlockBlobResponse;
use azure::storage::client::Client;
use futures::future::{done, ok};
use futures::prelude::*;
use hyper::{Method, StatusCode};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    client: &'a Client,
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
    client_request_id: Option<&'a str>,
}

impl<'a> PutBlockBlobBuilder<'a, No, No, No> {
    pub(crate) fn new(client: &'a Client) -> PutBlockBlobBuilder<'a, No, No, No> {
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
            client_request_id: None,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ClientRequired<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, BodySet> ContainerNameRequired<'a> for PutBlockBlobBuilder<'a, Yes, BlobNameSet, BodySet>
where
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BodySet> BlobNameRequired<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, Yes, BodySet>
where
    ContainerNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> BodyRequired<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    fn body(&self) -> &'a [u8] {
        self.body.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> TimeoutOption for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentTypeOption<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn content_type(&self) -> Option<&'a str> {
        self.content_type
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentEncodingOption<'a>
    for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn content_encoding(&self) -> Option<&'a str> {
        self.content_encoding
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentLanguageOption<'a>
    for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn content_language(&self) -> Option<&'a str> {
        self.content_language
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> CacheControlOption<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn cache_control(&self) -> Option<&'a str> {
        self.cache_control
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentMD5Option<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn content_md5(&self) -> Option<&'a [u8]> {
        self.content_md5
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentDispositionOption<'a>
    for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn content_disposition(&self) -> Option<&'a str> {
        self.content_disposition
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> MetadataOption<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> LeaseIdOption<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ClientRequestIdOption<'a>
    for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContainerNameSupport<'a>
    for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, Yes, BlobNameSet, BodySet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> BlobNameSupport<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, Yes, BodySet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> BodySupport<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> TimeoutSupport for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentTypeSupport<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentEncodingSupport<'a>
    for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentLanguageSupport<'a>
    for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> CacheControlSupport<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentMD5Support<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentDispositionSupport<'a>
    for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> MetadataSupport<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> LeaseIdSupport<'a> for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

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
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ClientRequestIdSupport<'a>
    for PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

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
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, BodySet> PutBlockBlobBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{}

impl<'a> PutBlockBlobBuilder<'a, Yes, Yes, Yes> {
    pub fn finalize(self) -> impl Future<Item = PutBlockBlobResponse, Error = AzureError> {
        let mut uri = format!(
            "https://{}.blob.core.windows.net/{}/{}",
            self.client().account(),
            self.container_name(),
            self.blob_name()
        );
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
                ContentMD5Option::add_header(&self, request);
                CacheControlOption::add_header(&self, request);
                ContentDispositionOption::add_header(&self, request);
                MetadataOption::add_header(&self, request);
                request.header(BLOB_TYPE, "BlockBlob");
                LeaseIdOption::add_header(&self, request);
                ClientRequestIdOption::add_header(&self, request);
            },
            Some(self.body()),
        );

        done(req)
            .from_err()
            .and_then(move |response| check_status_extract_headers_and_body(response, StatusCode::CREATED))
            .and_then(move |(headers, _body)| done(PutBlockBlobResponse::from_headers(&headers)).and_then(|pbbr| ok(pbbr)))
    }
}
