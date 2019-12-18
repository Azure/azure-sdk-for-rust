use crate::blob::generate_blob_uri;
use crate::blob::responses::UpdatePageResponse;
use azure_sdk_core::ba512_range::BA512Range;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::headers::PAGE_WRITE;
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::modify_conditions::{
    IfMatchCondition, IfSinceCondition, SequenceNumberCondition,
};
use azure_sdk_core::{
    BA512RangeRequired, BA512RangeSupport, BlobNameRequired, BlobNameSupport, BodyRequired,
    BodySupport, ClientRequestIdOption, ClientRequestIdSupport, ContainerNameRequired,
    ContainerNameSupport, ContentMD5Option, ContentMD5Support, IfMatchConditionOption,
    IfMatchConditionSupport, IfSinceConditionOption, IfSinceConditionSupport, LeaseIdOption,
    LeaseIdSupport, No, SequenceNumberConditionOption, SequenceNumberConditionSupport,
    TimeoutOption, TimeoutSupport, ToAssign, Yes,
};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_ba512_range: PhantomData<BA512RangeSet>,
    p_body: PhantomData<BodySet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    ba512_range: Option<&'a BA512Range>,
    body: Option<&'a [u8]>,
    content_md5: Option<&'a [u8]>,
    lease_id: Option<&'a LeaseId>,
    sequence_number_condition: Option<SequenceNumberCondition>,
    if_since_condition: Option<IfSinceCondition>,
    if_match_condition: Option<IfMatchCondition<'a>>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
}

impl<'a> UpdatePageBuilder<'a, No, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> UpdatePageBuilder<'a, No, No, No, No> {
        UpdatePageBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_ba512_range: PhantomData {},
            ba512_range: None,
            p_body: PhantomData {},
            body: None,
            content_md5: None,
            lease_id: None,
            sequence_number_condition: None,
            if_since_condition: None,
            if_match_condition: None,
            client_request_id: None,
            timeout: None,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> ClientRequired<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, BA512RangeSet, BodySet> ContainerNameRequired<'a>
    for UpdatePageBuilder<'a, Yes, BlobNameSet, BA512RangeSet, BodySet>
where
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BA512RangeSet, BodySet> BlobNameRequired<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, Yes, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BodySet> BA512RangeRequired<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, Yes, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn ba512_range(&self) -> &'a BA512Range {
        self.ba512_range.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> BodyRequired<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    #[inline]
    fn body(&self) -> &'a [u8] {
        self.body.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> ContentMD5Option<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn content_md5(&self) -> Option<&'a [u8]> {
        self.content_md5
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> LeaseIdOption<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> SequenceNumberConditionOption
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn sequence_number_condition(&self) -> Option<SequenceNumberCondition> {
        self.sequence_number_condition
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> IfSinceConditionOption
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn if_since_condition(&self) -> Option<IfSinceCondition> {
        self.if_since_condition.clone()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> IfMatchConditionOption<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> ClientRequestIdOption<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> TimeoutOption
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> ContainerNameSupport<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = UpdatePageBuilder<'a, Yes, BlobNameSet, BA512RangeSet, BodySet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        UpdatePageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            p_body: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            body: self.body,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> BlobNameSupport<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = UpdatePageBuilder<'a, ContainerNameSet, Yes, BA512RangeSet, BodySet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        UpdatePageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            ba512_range: self.ba512_range,
            body: self.body,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> BA512RangeSupport<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, Yes, BodySet>;

    #[inline]
    fn with_ba512_range(self, ba512_range: &'a BA512Range) -> Self::O {
        UpdatePageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: Some(ba512_range),
            body: self.body,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> BodySupport<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, Yes>;

    #[inline]
    fn with_body(self, body: &'a [u8]) -> Self::O {
        UpdatePageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            body: Some(body),
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> ContentMD5Support<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>;

    #[inline]
    fn with_content_md5(self, content_md5: &'a [u8]) -> Self::O {
        UpdatePageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            body: self.body,
            content_md5: Some(content_md5),
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> LeaseIdSupport<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        UpdatePageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            body: self.body,
            content_md5: self.content_md5,
            lease_id: Some(lease_id),
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> SequenceNumberConditionSupport
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>;

    #[inline]
    fn with_sequence_number_condition(
        self,
        sequence_number_condition: SequenceNumberCondition,
    ) -> Self::O {
        UpdatePageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            body: self.body,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            sequence_number_condition: Some(sequence_number_condition),
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> IfSinceConditionSupport
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>;

    #[inline]
    fn with_if_since_condition(self, if_since_condition: IfSinceCondition) -> Self::O {
        UpdatePageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            body: self.body,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: Some(if_since_condition),
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> IfMatchConditionSupport<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        UpdatePageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            body: self.body,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: Some(if_match_condition),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> ClientRequestIdSupport<'a>
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        UpdatePageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            body: self.body,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet> TimeoutSupport
    for UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
    type O = UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        UpdatePageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            p_body: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            body: self.body,
            content_md5: self.content_md5,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: Some(timeout),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
    UpdatePageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet, BodySet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    BodySet: ToAssign,
{
}

impl<'a> UpdatePageBuilder<'a, Yes, Yes, Yes, Yes> {
    pub async fn finalize(self) -> Result<UpdatePageResponse, AzureError> {
        let mut uri = generate_blob_uri(&self, Some("comp=page"));

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let upper = self.ba512_range().size() as usize;
        trace!("upper == {}", upper);
        let b = &self.body()[0..upper];

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            |mut request| {
                request = BA512RangeRequired::add_header(&self, request);
                request = ContentMD5Option::add_header(&self, request);
                request = request.header(PAGE_WRITE, "update");
                request = LeaseIdOption::add_header(&self, request);
                request = SequenceNumberConditionOption::add_header(&self, request);
                request = IfSinceConditionOption::add_header(&self, request);
                request = IfMatchConditionOption::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(b),
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;
        UpdatePageResponse::from_headers(&headers)
    }
}
