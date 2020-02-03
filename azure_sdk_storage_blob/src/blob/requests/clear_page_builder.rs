use crate::blob::generate_blob_uri;
use crate::blob::responses::ClearPageResponse;
use azure_sdk_core::ba512_range::BA512Range;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::headers::PAGE_WRITE;
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::modify_conditions::{
    IfMatchCondition, IfSinceCondition, SequenceNumberCondition,
};
use azure_sdk_core::{
    BA512RangeRequired, BA512RangeSupport, BlobNameRequired, BlobNameSupport,
    ClientRequestIdOption, ClientRequestIdSupport, ContainerNameRequired, ContainerNameSupport,
    IfMatchConditionOption, IfMatchConditionSupport, IfSinceConditionOption,
    IfSinceConditionSupport, LeaseIdOption, LeaseIdSupport, No, SequenceNumberConditionOption,
    SequenceNumberConditionSupport, TimeoutOption, TimeoutSupport, ToAssign, Yes,
};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_ba512_range: PhantomData<BA512RangeSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    ba512_range: Option<&'a BA512Range>,
    lease_id: Option<&'a LeaseId>,
    sequence_number_condition: Option<SequenceNumberCondition>,
    if_since_condition: Option<IfSinceCondition>,
    if_match_condition: Option<IfMatchCondition<'a>>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
}

impl<'a> ClearPageBuilder<'a, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> ClearPageBuilder<'a, No, No, No> {
        ClearPageBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_ba512_range: PhantomData {},
            ba512_range: None,
            lease_id: None,
            sequence_number_condition: None,
            if_since_condition: None,
            if_match_condition: None,
            client_request_id: None,
            timeout: None,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> ClientRequired<'a>
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, BA512RangeSet> ContainerNameRequired<'a>
    for ClearPageBuilder<'a, Yes, BlobNameSet, BA512RangeSet>
where
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BA512RangeSet> BlobNameRequired<'a>
    for ClearPageBuilder<'a, ContainerNameSet, Yes, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> BA512RangeRequired<'a>
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn ba512_range(&self) -> &'a BA512Range {
        self.ba512_range.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> LeaseIdOption<'a>
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> SequenceNumberConditionOption
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    #[inline]
    fn sequence_number_condition(&self) -> Option<SequenceNumberCondition> {
        self.sequence_number_condition
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> IfSinceConditionOption
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    #[inline]
    fn if_since_condition(&self) -> Option<IfSinceCondition> {
        self.if_since_condition.clone()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> IfMatchConditionOption<'a>
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> ClientRequestIdOption<'a>
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> TimeoutOption
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> ContainerNameSupport<'a>
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    type O = ClearPageBuilder<'a, Yes, BlobNameSet, BA512RangeSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        ClearPageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> BlobNameSupport<'a>
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    type O = ClearPageBuilder<'a, ContainerNameSet, Yes, BA512RangeSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        ClearPageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            ba512_range: self.ba512_range,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> BA512RangeSupport<'a>
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    type O = ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_ba512_range(self, ba512_range: &'a BA512Range) -> Self::O {
        ClearPageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: Some(ba512_range),
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> LeaseIdSupport<'a>
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    type O = ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        ClearPageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            lease_id: Some(lease_id),
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> SequenceNumberConditionSupport
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    type O = ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>;

    #[inline]
    fn with_sequence_number_condition(
        self,
        sequence_number_condition: SequenceNumberCondition,
    ) -> Self::O {
        ClearPageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            lease_id: self.lease_id,
            sequence_number_condition: Some(sequence_number_condition),
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> IfSinceConditionSupport
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    type O = ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>;

    #[inline]
    fn with_if_since_condition(self, if_since_condition: IfSinceCondition) -> Self::O {
        ClearPageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: Some(if_since_condition),
            if_match_condition: self.if_match_condition,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> IfMatchConditionSupport<'a>
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    type O = ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        ClearPageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: Some(if_match_condition),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> ClientRequestIdSupport<'a>
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    type O = ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        ClearPageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
            lease_id: self.lease_id,
            sequence_number_condition: self.sequence_number_condition,
            if_since_condition: self.if_since_condition,
            if_match_condition: self.if_match_condition,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet> TimeoutSupport
    for ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
    type O = ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        ClearPageBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_ba512_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            ba512_range: self.ba512_range,
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
impl<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
    ClearPageBuilder<'a, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
{
}

impl<'a> ClearPageBuilder<'a, Yes, Yes, Yes> {
    #[inline]
    pub async fn finalize(self) -> Result<ClearPageResponse, AzureError> {
        let mut uri = generate_blob_uri(&self, Some("comp=page"));

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            |mut request| {
                request = BA512RangeRequired::add_header(&self, request);
                request = request.header(PAGE_WRITE, "clear");
                request = LeaseIdOption::add_header(&self, request);
                request = SequenceNumberConditionOption::add_header(&self, request);
                request = IfSinceConditionOption::add_header(&self, request);
                request = IfMatchConditionOption::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;
        ClearPageResponse::from_headers(&headers)
    }
}
