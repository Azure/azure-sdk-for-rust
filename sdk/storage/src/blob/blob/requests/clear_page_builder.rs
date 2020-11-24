use crate::blob::blob::generate_blob_uri;
use crate::blob::blob::responses::ClearPageResponse;
use crate::core::prelude::*;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::headers::PAGE_WRITE;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    client: &'a C,
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

impl<'a, C> ClearPageBuilder<'a, C, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> ClearPageBuilder<'a, C, No, No, No> {
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

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> ClientRequired<'a, C>
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, BA512RangeSet> ContainerNameRequired<'a>
    for ClearPageBuilder<'a, C, Yes, BlobNameSet, BA512RangeSet>
where
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BA512RangeSet> BlobNameRequired<'a>
    for ClearPageBuilder<'a, C, ContainerNameSet, Yes, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> BA512RangeRequired<'a>
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn ba512_range(&self) -> &'a BA512Range {
        self.ba512_range.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> LeaseIdOption<'a>
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> SequenceNumberConditionOption
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn sequence_number_condition(&self) -> Option<SequenceNumberCondition> {
        self.sequence_number_condition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> IfSinceConditionOption
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn if_since_condition(&self) -> Option<IfSinceCondition> {
        self.if_since_condition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> IfMatchConditionOption<'a>
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> ClientRequestIdOption<'a>
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> TimeoutOption
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, BlobNameSet, BA512RangeSet> ContainerNameSupport<'a>
    for ClearPageBuilder<'a, C, No, BlobNameSet, BA512RangeSet>
where
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    type O = ClearPageBuilder<'a, C, Yes, BlobNameSet, BA512RangeSet>;

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

impl<'a, C, ContainerNameSet, BA512RangeSet> BlobNameSupport<'a>
    for ClearPageBuilder<'a, C, ContainerNameSet, No, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    type O = ClearPageBuilder<'a, C, ContainerNameSet, Yes, BA512RangeSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet> BA512RangeSupport<'a>
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> LeaseIdSupport<'a>
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    type O = ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> SequenceNumberConditionSupport
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    type O = ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> IfSinceConditionSupport
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    type O = ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> IfMatchConditionSupport<'a>
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    type O = ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> ClientRequestIdSupport<'a>
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    type O = ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet> TimeoutSupport
    for ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BA512RangeSet: ToAssign,
    C: Client,
{
    type O = ClearPageBuilder<'a, C, ContainerNameSet, BlobNameSet, BA512RangeSet>;

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

// methods callable only when every mandatory field has been filled
impl<'a, C> ClearPageBuilder<'a, C, Yes, Yes, Yes>
where
    C: Client,
{
    #[inline]
    pub async fn finalize(self) -> Result<ClearPageResponse, AzureError> {
        let mut uri = generate_blob_uri(
            self.client(),
            self.container_name(),
            self.blob_name(),
            Some("comp=page"),
        );

        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, timeout);
        }

        trace!("uri == {:?}", uri);

        let perform_request_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            &|mut request| {
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

        let (headers, _body) = check_status_extract_headers_and_body(
            perform_request_response.response_future,
            StatusCode::CREATED,
        )
        .await?;
        ClearPageResponse::from_headers(&headers)
    }
}
