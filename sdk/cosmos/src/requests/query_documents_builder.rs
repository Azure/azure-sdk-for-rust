use crate::prelude::*;
use crate::responses::QueryDocumentsResponse;
use crate::{Query, ResourceType};
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use futures::stream::{unfold, Stream};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    collection_client: &'a CollectionClient,
    p_query: PhantomData<QuerySet>,
    query: Option<&'b Query<'b>>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<&'b str>,
    max_item_count: i32,
    partition_keys: Option<&'b PartitionKeys>,
    query_cross_partition: bool,
    parallelize_cross_partition_query: bool,
}

impl<'a, 'b, QuerySet> Clone for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn clone(&self) -> Self {
        Self {
            collection_client: self.collection_client,
            p_query: PhantomData {},
            query: self.query,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level.clone(),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            parallelize_cross_partition_query: self.parallelize_cross_partition_query,
        }
    }
}

impl<'a, 'b> QueryDocumentsBuilder<'a, 'b, No> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
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

impl<'a, 'b, QuerySet> CollectionClientRequired<'a> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }
}

impl<'a, 'b> QueryRequired<'b> for QueryDocumentsBuilder<'a, 'b, Yes> {
    fn query(&self) -> &'b Query<'b> {
        self.query.unwrap()
    }
}

impl<'a, 'b, QuerySet> IfMatchConditionOption<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, QuerySet> IfModifiedSinceOption<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b, QuerySet> UserAgentOption<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, QuerySet> ActivityIdOption<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, QuerySet> ConsistencyLevelOption<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, QuerySet> ContinuationOption<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b, QuerySet> MaxItemCountOption for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b, QuerySet> PartitionKeysOption<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn partition_keys(&self) -> Option<&'b PartitionKeys> {
        self.partition_keys
    }
}

impl<'a, 'b, QuerySet> QueryCrossPartitionOption for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn query_cross_partition(&self) -> bool {
        self.query_cross_partition
    }
}

impl<'a, 'b, QuerySet> ParallelizeCrossPartitionQueryOption
    for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn parallelize_cross_partition_query(&self) -> bool {
        self.parallelize_cross_partition_query
    }
}

impl<'a, 'b> QuerySupport<'b> for QueryDocumentsBuilder<'a, 'b, No> {
    type O = QueryDocumentsBuilder<'a, 'b, Yes>;

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

impl<'a, 'b, QuerySet> IfMatchConditionSupport<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    type O = Self;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }
}

impl<'a, 'b, QuerySet> IfModifiedSinceSupport<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    type O = Self;

    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        Self {
            if_modified_since: Some(if_modified_since),
            ..self
        }
    }
}

impl<'a, 'b, QuerySet> UserAgentSupport<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b, QuerySet> ActivityIdSupport<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b, QuerySet> ConsistencyLevelSupport<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

impl<'a, 'b, QuerySet> ContinuationSupport<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    type O = Self;

    fn with_continuation(self, continuation: &'b str) -> Self::O {
        Self {
            continuation: Some(continuation),
            ..self
        }
    }
}

impl<'a, 'b, QuerySet> MaxItemCountSupport for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    type O = Self;

    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        Self {
            max_item_count,
            ..self
        }
    }
}

impl<'a, 'b, QuerySet> PartitionKeysSupport<'b> for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    type O = Self;

    fn with_partition_keys(self, partition_keys: &'b PartitionKeys) -> Self::O {
        Self {
            partition_keys: Some(partition_keys),
            ..self
        }
    }
}

impl<'a, 'b, QuerySet> QueryCrossPartitionSupport for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    type O = Self;

    fn with_query_cross_partition(self, query_cross_partition: bool) -> Self::O {
        Self {
            query_cross_partition,
            ..self
        }
    }
}

impl<'a, 'b, QuerySet> ParallelizeCrossPartitionQuerySupport
    for QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    type O = Self;

    fn with_parallelize_cross_partition_query(
        self,
        parallelize_cross_partition_query: bool,
    ) -> Self::O {
        Self {
            parallelize_cross_partition_query,
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> QueryDocumentsBuilder<'a, 'b, Yes> {
    pub async fn execute<T>(&self) -> Result<QueryDocumentsResponse<T>, AzureError>
    where
        T: DeserializeOwned,
    {
        trace!("QueryDocumentBuilder::execute called");

        let req = self.collection_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name()
            ),
            hyper::Method::POST,
            ResourceType::Documents,
        );

        // signal that this is a query
        let req = req.header(crate::headers::HEADER_DOCUMENTDB_ISQUERY, true.to_string());
        let req = req.header(http::header::CONTENT_TYPE, "application/query+json");

        // add trait headers
        let req = IfMatchConditionOption::add_header(self, req);
        let req = IfModifiedSinceOption::add_header(self, req);
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);
        let req = ContinuationOption::add_header(self, req);
        let req = MaxItemCountOption::add_header(self, req);
        let req = PartitionKeysOption::add_header(self, req);
        let req = QueryCrossPartitionOption::add_header(self, req);

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

        Ok((&headers, &body as &[u8]).try_into()?)
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
            move |continuation_token: Option<States>| async move {
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

                let continuation_token = match &response.continuation_token {
                    Some(ct) => Some(States::Continuation(ct.to_owned())),
                    None => None,
                };

                Some((Ok(response), continuation_token))
            },
        )
    }
}
