use crate::clients::{CollectionClient, CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::ListDocumentsResponse;
use crate::CollectionClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{IfMatchConditionOption, IfMatchConditionSupport};
use futures::stream::{unfold, Stream};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct ListDocumentsBuilder<'a, 'b, CUB>
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
    a_im: bool,
    partition_range_id: Option<&'b str>,
}

impl<'a, 'b, CUB> Clone for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn clone(&self) -> Self {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> ListDocumentsBuilder<'a, 'b, CUB> {
        ListDocumentsBuilder {
            collection_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
            a_im: false,
            partition_range_id: None,
        }
    }
}

impl<'a, 'b, CUB> CollectionClientRequired<'a, CUB> for ListDocumentsBuilder<'a, 'b, CUB>
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
impl<'a, 'b, CUB> IfMatchConditionOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB> UserAgentOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB> ActivityIdOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB> ConsistencyLevelOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level
    }
}

impl<'a, 'b, CUB> ContinuationOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b, CUB> MaxItemCountOption for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b, CUB> AIMOption for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn a_im(&self) -> bool {
        self.a_im
    }
}

impl<'a, 'b, CUB> PartitionRangeIdOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_range_id(&self) -> Option<&'b str> {
        self.partition_range_id
    }
}

impl<'a, 'b, CUB> IfMatchConditionSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: Some(if_match_condition),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> UserAgentSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> ActivityIdSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> ConsistencyLevelSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> ContinuationSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_continuation(self, continuation: &'b str) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: Some(continuation),
            max_item_count: self.max_item_count,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> MaxItemCountSupport for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> AIMSupport for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_a_im(self, a_im: bool) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> PartitionRangeIdSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_partition_range_id(self, partition_range_id: &'b str) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
            partition_range_id: Some(partition_range_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute<T>(&self) -> Result<ListDocumentsResponse<T>, AzureError>
    where
        T: DeserializeOwned,
    {
        let mut req = self.collection_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs",
                self.collection_client.database_name().name(),
                self.collection_client.collection_name().name()
            ),
            hyper::Method::GET,
            ResourceType::Documents,
        );

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);
        req = ContinuationOption::add_header(self, req);
        req = MaxItemCountOption::add_header(self, req);
        req = AIMOption::add_header(self, req);
        req = PartitionRangeIdOption::add_header(self, req);

        let req = req.body(hyper::Body::empty())?;

        let (headers, whole_body) = check_status_extract_headers_and_body(
            self.collection_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        let resp = ListDocumentsResponse::try_from((&headers, &whole_body as &[u8]))?;
        Ok(resp)
    }

    pub fn stream<T>(&self) -> impl Stream<Item = Result<ListDocumentsResponse<T>, AzureError>> + '_
    where
        T: DeserializeOwned,
    {
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

                    let continuation_token = match &response.additional_headers.continuation_token {
                        Some(ct) => Some(States::Continuation(ct.to_owned())),
                        None => None,
                    };

                    Some((Ok(response), continuation_token))
                }
            },
        )
    }
}
