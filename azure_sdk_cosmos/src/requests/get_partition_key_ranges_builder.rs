use crate::clients::{CollectionClient, CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::*;
use crate::CollectionClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{IfMatchConditionOption, IfMatchConditionSupport};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, CUB> GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> GetPartitionKeyRangesBuilder<'a, 'b, CUB> {
        GetPartitionKeyRangesBuilder {
            collection_client,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, CUB> CollectionClientRequired<'a, CUB> for GetPartitionKeyRangesBuilder<'a, 'b, CUB>
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
impl<'a, 'b, CUB> IfMatchConditionOption<'b> for GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB> IfModifiedSinceOption<'b> for GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b, CUB> UserAgentOption<'b> for GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB> ActivityIdOption<'b> for GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB> ConsistencyLevelOption<'b> for GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, CUB> IfMatchConditionSupport<'b> for GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetPartitionKeyRangesBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        GetPartitionKeyRangesBuilder {
            collection_client: self.collection_client,
            if_match_condition: Some(if_match_condition),
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB> IfModifiedSinceSupport<'b> for GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetPartitionKeyRangesBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        GetPartitionKeyRangesBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: Some(if_modified_since),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB> UserAgentSupport<'b> for GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetPartitionKeyRangesBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        GetPartitionKeyRangesBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB> ActivityIdSupport<'b> for GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetPartitionKeyRangesBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        GetPartitionKeyRangesBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB> ConsistencyLevelSupport<'b> for GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetPartitionKeyRangesBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        GetPartitionKeyRangesBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> GetPartitionKeyRangesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<GetPartitionKeyRangesResponse, AzureError> {
        trace!("GetPartitionKeyRangesBuilder::execute called");

        let request = self.collection_client().main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/pkranges",
                self.collection_client.database_name().name(),
                self.collection_client.collection_name().name()
            ),
            hyper::Method::GET,
            ResourceType::PartitionKeyRanges,
        );

        let request = request.header(hyper::header::CONTENT_LENGTH, "0");
        let request = IfMatchConditionOption::add_header(self, request);
        let request = IfModifiedSinceOption::add_header(self, request);
        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        let future_response = self.collection_client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
