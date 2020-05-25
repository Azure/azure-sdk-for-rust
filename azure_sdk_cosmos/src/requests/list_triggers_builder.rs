use crate::clients::{CollectionClient, CosmosUriBuilder};
use crate::prelude::*;
use crate::responses::ListTriggersResponse;
use crate::CollectionClientRequired;
use crate::TriggerBuilderTrait;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{IfMatchConditionOption, IfMatchConditionSupport};
use futures::stream::{unfold, Stream};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug)]
pub struct ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    continuation: Option<&'b str>,
    max_item_count: i32,
}

impl<'a, 'b, CUB> Clone for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn clone(&self) -> Self {
        Self {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level.clone(),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, CUB> ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> ListTriggersBuilder<'a, 'b, CUB> {
        ListTriggersBuilder {
            collection_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
        }
    }
}

impl<'a, 'b, CUB> CollectionClientRequired<'a, CUB> for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, CUB> IfMatchConditionOption<'b> for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB> UserAgentOption<'b> for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB> ActivityIdOption<'b> for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB> ConsistencyLevelOption<'b> for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, CUB> ContinuationOption<'b> for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b, CUB> MaxItemCountOption for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b, CUB> IfMatchConditionSupport<'b> for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListTriggersBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        ListTriggersBuilder {
            collection_client: self.collection_client,
            if_match_condition: Some(if_match_condition),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, CUB> UserAgentSupport<'b> for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListTriggersBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ListTriggersBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, CUB> ActivityIdSupport<'b> for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListTriggersBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ListTriggersBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, CUB> ConsistencyLevelSupport<'b> for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListTriggersBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ListTriggersBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, CUB> ContinuationSupport<'b> for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListTriggersBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_continuation(self, continuation: &'b str) -> Self::O {
        ListTriggersBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: Some(continuation),
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, CUB> MaxItemCountSupport for ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListTriggersBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        ListTriggersBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> ListTriggersBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<ListTriggersResponse, AzureError> {
        trace!("ListTriggersBuilder::execute called");

        let req = self
            .collection_client
            .with_trigger(&"dummy")
            .prepare_request(hyper::Method::GET, false);

        // add trait headers
        let req = IfMatchConditionOption::add_header(self, req);
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);
        let req = ContinuationOption::add_header(self, req);
        let req = MaxItemCountOption::add_header(self, req);

        let request = req.body(hyper::Body::empty())?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.collection_client().hyper_client().request(request),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }

    pub fn stream(&self) -> impl Stream<Item = Result<ListTriggersResponse, AzureError>> + '_ {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            Continuation(String),
        };

        unfold(
            Some(States::Init),
            move |continuation_token: Option<States>| {
                async move {
                    debug!("continuation_token == {:?}", &continuation_token);
                    let response = match continuation_token {
                        Some(States::Init) => self.execute().await,
                        Some(States::Continuation(continuation_token)) => {
                            self.clone()
                                .with_continuation(&continuation_token)
                                .execute()
                                .await
                        }
                        None => return None,
                    };

                    // the ? operator does not work in async move (yet?)
                    // so we have to resort to this boilerplate
                    let response = match response {
                        Ok(response) => response,
                        Err(err) => return Some((Err(err), None)),
                    };

                    let continuation_token = match &response.continuation_token {
                        Some(ct) => Some(States::Continuation(ct.to_owned())),
                        None => None,
                    };

                    Some((Ok(response), continuation_token))
                }
            },
        )
    }
}
