use crate::blob::generate_blob_uri;
use crate::blob::responses::PutBlockResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::{
    AppendPositionOption, AppendPositionSupport, BlobNameRequired, BlobNameSupport, BodyRequired,
    BodySupport, ClientRequestIdOption, ClientRequestIdSupport, ContainerNameRequired,
    ContainerNameSupport, ContentMD5Option, ContentMD5Support, IfMatchConditionOption,
    IfMatchConditionSupport, LeaseIdOption, LeaseIdSupport, No, TimeoutOption, TimeoutSupport,
    ToAssign, Yes,
};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
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
    content_md5: Option<&'a [u8]>,
    lease_id: Option<&'a LeaseId>,
    if_match_condition: Option<IfMatchCondition<'a>>,
    client_request_id: Option<&'a str>,
    append_position: Option<u32>,
}

impl<'a> PutAppendBlockBuilder<'a, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> PutAppendBlockBuilder<'a, No, No, No> {
        PutAppendBlockBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_body: PhantomData {},
            body: None,
            timeout: None,
            content_md5: None,
            lease_id: None,
            if_match_condition: None,
            client_request_id: None,
            append_position: None,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ClientRequired<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, BodySet> ContainerNameRequired<'a>
    for PutAppendBlockBuilder<'a, Yes, BlobNameSet, BodySet>
where
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BodySet> BlobNameRequired<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, Yes, BodySet>
where
    ContainerNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> BodyRequired<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn body(&self) -> &'a [u8] {
        self.body.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> TimeoutOption
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentMD5Option<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn content_md5(&self) -> Option<&'a [u8]> {
        self.content_md5
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> LeaseIdOption<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> IfMatchConditionOption<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ClientRequestIdOption<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> AppendPositionOption
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn append_position(&self) -> Option<u32> {
        self.append_position
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContainerNameSupport<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutAppendBlockBuilder<'a, Yes, BlobNameSet, BodySet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        PutAppendBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            append_position: self.append_position,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> BlobNameSupport<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutAppendBlockBuilder<'a, ContainerNameSet, Yes, BodySet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        PutAppendBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            body: self.body,
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            append_position: self.append_position,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> BodySupport<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_body(self, body: &'a [u8]) -> Self::O {
        PutAppendBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: Some(body),
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            append_position: self.append_position,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> TimeoutSupport
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        PutAppendBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: Some(timeout),
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            append_position: self.append_position,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ContentMD5Support<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_content_md5(self, content_md5: &'a [u8]) -> Self::O {
        PutAppendBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_md5: Some(content_md5),
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            append_position: self.append_position,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> LeaseIdSupport<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        PutAppendBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: Some(lease_id),
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            append_position: self.append_position,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> IfMatchConditionSupport<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        PutAppendBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            if_match_condition: Some(if_match_condition),
            client_request_id: self.client_request_id,
            append_position: self.append_position,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> ClientRequestIdSupport<'a>
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        PutAppendBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: Some(client_request_id),
            append_position: self.append_position,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> AppendPositionSupport
    for PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    type O = PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>;

    #[inline]
    fn with_append_position(self, append_position: u32) -> Self::O {
        PutAppendBlockBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            body: self.body,
            timeout: self.timeout,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            append_position: Some(append_position),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, BodySet>
    PutAppendBlockBuilder<'a, ContainerNameSet, BlobNameSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
}

impl<'a> PutAppendBlockBuilder<'a, Yes, Yes, Yes> {
    #[inline]
    pub async fn finalize(self) -> Result<PutBlockResponse, AzureError> {
        let mut uri = generate_blob_uri(&self, Some("comp=appendblock"));

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            |mut request| {
                request = ContentMD5Option::add_header(&self, request);
                request = LeaseIdOption::add_header(&self, request);
                request = IfMatchConditionOption::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request = AppendPositionOption::add_header(&self, request);
                request
            },
            Some(self.body()),
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;
        PutBlockResponse::from_headers(&headers)
    }
}
