use crate::clients::{CollectionClient, CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::QueryDocumentsResponse;
use crate::CollectionClientRequired;
use crate::Query;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{IfMatchConditionOption, IfMatchConditionSupport};
use azure_sdk_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use futures::stream::{unfold, Stream};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    p_query: PhantomData<QuerySet>,
    query: Option<&'b Query<'b>>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    continuation: Option<&'b str>,
    max_item_count: i32,
    partition_keys: Option<&'b PartitionKeys>,
    query_cross_partition: bool,
    parallelize_cross_partition_query: bool,
}

impl<'a, 'b, CUB, QuerySet> Clone for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn clone(&self) -> Self {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b, CUB> QueryDocumentsBuilder<'a, 'b, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> QueryDocumentsBuilder<'a, 'b, CUB, No> {
        QueryDocumentsBuilder {
            collection_client,
            p_query: PhantomData {},
            query: None,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
            partition_keys: None,
            query_cross_partition: false,
            parallelize_cross_partition_query: false,
        }
    }
}

impl<'a, 'b, CUB, QuerySet> CollectionClientRequired<'a, CUB>
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, CUB> QueryRequired<'b> for QueryDocumentsBuilder<'a, 'b, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn query(&self) -> &'b Query<'b> {
        self.query.unwrap()
    }
}

impl<'a, 'b, CUB, QuerySet> IfMatchConditionOption<'b>
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB, QuerySet> IfModifiedSinceOption<'b>
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b, CUB, QuerySet> UserAgentOption<'b> for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB, QuerySet> ActivityIdOption<'b> for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB, QuerySet> ConsistencyLevelOption<'b>
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level
    }
}

impl<'a, 'b, CUB, QuerySet> ContinuationOption<'b> for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b, CUB, QuerySet> MaxItemCountOption for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b, CUB, QuerySet> PartitionKeysOption<'b> for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_keys(&self) -> Option<&'b PartitionKeys> {
        self.partition_keys
    }
}

impl<'a, 'b, CUB, QuerySet> QueryCrossPartitionOption
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn query_cross_partition(&self) -> bool {
        self.query_cross_partition
    }
}

impl<'a, 'b, CUB, QuerySet> ParallelizeCrossPartitionQueryOption
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn parallelize_cross_partition_query(&self) -> bool {
        self.parallelize_cross_partition_query
    }
}

impl<'a, 'b, CUB> QuerySupport<'b> for QueryDocumentsBuilder<'a, 'b, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    type O = QueryDocumentsBuilder<'a, 'b, CUB, Yes>;

    #[inline]
    fn with_query(self, query: &'b Query<'b>) -> Self::O {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: Some(query),
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b, CUB, QuerySet> IfMatchConditionSupport<'b>
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: Some(if_match_condition),
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b, CUB, QuerySet> IfModifiedSinceSupport<'b>
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>;

    #[inline]
    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: self.if_match_condition,
            if_modified_since: Some(if_modified_since),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b, CUB, QuerySet> UserAgentSupport<'b> for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b, CUB, QuerySet> ActivityIdSupport<'b> for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b, CUB, QuerySet> ConsistencyLevelSupport<'b>
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b, CUB, QuerySet> ContinuationSupport<'b> for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>;

    #[inline]
    fn with_continuation(self, continuation: &'b str) -> Self::O {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: Some(continuation),
            max_item_count: self.max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b, CUB, QuerySet> MaxItemCountSupport for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>;

    #[inline]
    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b, CUB, QuerySet> PartitionKeysSupport<'b>
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>;

    #[inline]
    fn with_partition_keys(self, partition_keys: &'b PartitionKeys) -> Self::O {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_keys: Some(partition_keys),
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b, CUB, QuerySet> QueryCrossPartitionSupport
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>;

    #[inline]
    fn with_query_cross_partition(self, query_cross_partition: bool) -> Self::O {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b, CUB, QuerySet> ParallelizeCrossPartitionQuerySupport
    for QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>
where
    QuerySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = QueryDocumentsBuilder<'a, 'b, CUB, QuerySet>;

    #[inline]
    fn with_parallelize_cross_partition_query(
        self,
        parallelize_cross_partition_query: bool,
    ) -> Self::O {
        QueryDocumentsBuilder {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> QueryDocumentsBuilder<'a, 'b, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute<T>(&self) -> Result<QueryDocumentsResponse<T>, AzureError>
    where
        T: DeserializeOwned,
    {
        trace!("QueryDocumentBuilder::execute called");

        let mut req = self.collection_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs",
                self.collection_client.database_name().name(),
                self.collection_client.collection_name().name()
            ),
            hyper::Method::POST,
            ResourceType::Documents,
        );

        // signal that this is a query
        req = req.header(crate::headers::HEADER_DOCUMENTDB_ISQUERY, true.to_string());
        req = req.header(http::header::CONTENT_TYPE, "application/query+json");

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = IfModifiedSinceOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);
        req = ContinuationOption::add_header(self, req);
        req = MaxItemCountOption::add_header(self, req);
        req = PartitionKeysOption::add_header(self, req);
        req = QueryCrossPartitionOption::add_header(self, req);

        let body = serde_json::to_string(self.query())?;
        debug!("body == {}", body);

        let req = req.body(hyper::Body::from(body))?;
        debug!("{:?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.collection_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nbody == {:#?}", body);

        let resp = QueryDocumentsResponse::try_from((&headers, &body as &[u8]))?;
        Ok(resp)
    }

    pub fn stream<T>(
        &self,
    ) -> impl Stream<Item = Result<QueryDocumentsResponse<T>, AzureError>> + '_
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
