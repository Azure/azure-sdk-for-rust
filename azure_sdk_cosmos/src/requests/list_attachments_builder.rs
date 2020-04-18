use crate::clients::{CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::ListAttachmentsResponse;
use crate::DocumentClient;
use crate::DocumentClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    document_client: &'a DocumentClient<'a, CUB>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    continuation: Option<&'b str>,
    max_item_count: i32,
    a_im: bool,
}

impl<'a, 'b, CUB> ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        document_client: &'a DocumentClient<'a, CUB>,
    ) -> ListAttachmentsBuilder<'a, 'b, CUB> {
        ListAttachmentsBuilder {
            document_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
            a_im: false,
        }
    }
}

impl<'a, 'b, CUB> DocumentClientRequired<'a, CUB> for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn document_client(&self) -> &'a DocumentClient<'a, CUB> {
        self.document_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, CUB> IfMatchConditionOption<'b> for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB> UserAgentOption<'b> for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB> ActivityIdOption<'b> for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB> ConsistencyLevelOption<'b> for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, CUB> ContinuationOption<'b> for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b, CUB> MaxItemCountOption for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b, CUB> AIMOption for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn a_im(&self) -> bool {
        self.a_im
    }
}

impl<'a, 'b, CUB> IfMatchConditionSupport<'b> for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            if_match_condition: Some(if_match_condition),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB> UserAgentSupport<'b> for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            if_match_condition: self.if_match_condition,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB> ActivityIdSupport<'b> for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB> ConsistencyLevelSupport<'b> for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB> ContinuationSupport<'b> for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_continuation(self, continuation: &'b str) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: Some(continuation),
            max_item_count: self.max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB> MaxItemCountSupport for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB> AIMSupport for ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_a_im(self, a_im: bool) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> ListAttachmentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<ListAttachmentsResponse, AzureError> {
        let mut req = self.document_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments",
                self.document_client.database_name().name(),
                self.document_client.collection_name().name(),
                self.document_client.document_name().name()
            ),
            hyper::Method::GET,
            ResourceType::Attachments,
        );

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);
        req = ContinuationOption::add_header(self, req);
        req = MaxItemCountOption::add_header(self, req);
        req = AIMOption::add_header(self, req);

        req = crate::add_partition_keys_header(self.document_client.partition_keys(), req);

        let req = req.body(hyper::Body::empty())?;

        let (headers, whole_body) = check_status_extract_headers_and_body(
            self.document_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        Ok((&headers, &whole_body as &[u8]).try_into()?)
    }
}
