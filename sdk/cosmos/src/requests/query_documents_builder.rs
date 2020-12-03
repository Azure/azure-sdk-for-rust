use crate::prelude::*;
use crate::resources::document::Query;
use crate::resources::ResourceType;
use crate::responses::QueryDocumentsResponse;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use futures::stream::{unfold, Stream};
use http::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    collection_client: &'a CollectionClient,
    p_query: PhantomData<QuerySet>,
    query: Option<&'b Query<'b>>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<IfModifiedSince<'b>>,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<Continuation<'b>>,
    max_item_count: MaxItemCount,
    partition_keys: Option<&'b PartitionKeys>,
    query_cross_partition: QueryCrossPartition,
    parallelize_cross_partition_query: ParallelizeCrossPartition,
}

impl<'a, 'b> QueryDocumentsBuilder<'a, 'b, No> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            p_query: PhantomData,
            query: None,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: MaxItemCount::new(-1),
            partition_keys: None,
            query_cross_partition: QueryCrossPartition::No,
            parallelize_cross_partition_query: ParallelizeCrossPartition::No,
        }
    }
}

impl<'a, 'b, QuerySet> QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    pub fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }

    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }

    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }

    fn max_item_count(&self) -> MaxItemCount {
        self.max_item_count
    }

    fn partition_keys(&self) -> Option<&'b PartitionKeys> {
        self.partition_keys
    }

    fn query_cross_partition(&self) -> QueryCrossPartition {
        self.query_cross_partition
    }

    // TODO: Use this in request
    #[allow(unused)]
    fn parallelize_cross_partition_query(&self) -> ParallelizeCrossPartition {
        self.parallelize_cross_partition_query
    }

    pub fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }

    pub fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self {
        Self {
            if_modified_since: Some(IfModifiedSince::new(if_modified_since)),
            ..self
        }
    }

    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'b str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }

    pub fn with_continuation(self, continuation: &'b str) -> Self {
        Self {
            continuation: Some(Continuation::new(continuation)),
            ..self
        }
    }

    pub fn with_max_item_count(self, max_item_count: i32) -> Self {
        Self {
            max_item_count: MaxItemCount::new(max_item_count),
            ..self
        }
    }

    pub fn with_partition_keys(self, partition_keys: &'b PartitionKeys) -> Self {
        Self {
            partition_keys: Some(partition_keys),
            ..self
        }
    }

    pub fn with_query_cross_partition(self, query_cross_partition: bool) -> Self {
        Self {
            query_cross_partition: if query_cross_partition {
                QueryCrossPartition::Yes
            } else {
                QueryCrossPartition::No
            },
            ..self
        }
    }

    pub fn with_parallelize_cross_partition_query(
        self,
        parallelize_cross_partition_query: bool,
    ) -> Self {
        Self {
            parallelize_cross_partition_query: if parallelize_cross_partition_query {
                ParallelizeCrossPartition::Yes
            } else {
                ParallelizeCrossPartition::No
            },
            ..self
        }
    }
}

impl<'a, 'b> QueryDocumentsBuilder<'a, 'b, Yes> {
    pub async fn execute<T>(&self) -> Result<QueryDocumentsResponse<T>, CosmosError>
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
            http::Method::POST,
            ResourceType::Documents,
        );

        // signal that this is a query
        let req = req.header(crate::headers::HEADER_DOCUMENTDB_ISQUERY, true.to_string());
        let req = req.header(http::header::CONTENT_TYPE, "application/query+json");

        // add trait headers
        let req = crate::headers::add_header(self.if_match_condition(), req);
        let req = crate::headers::add_header(self.if_modified_since(), req);
        let req = crate::headers::add_header(self.user_agent(), req);
        let req = crate::headers::add_header(self.activity_id(), req);
        let req = crate::headers::add_header(self.consistency_level(), req);
        let req = crate::headers::add_header(self.continuation(), req);
        let req = crate::headers::add_header(Some(self.max_item_count()), req);
        let req = crate::headers::add_header(self.partition_keys(), req);
        let req = crate::headers::add_header(Some(self.query_cross_partition()), req);

        let body = serde_json::to_string(self.query())?;
        debug!("body == {}", body);

        let req = req.body(body.as_bytes())?;
        debug!("{:?}", req);

        Ok(self
            .collection_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }

    pub fn stream<T>(
        &self,
    ) -> impl Stream<Item = Result<QueryDocumentsResponse<T>, CosmosError>> + '_
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

impl<'a, 'b, QuerySet> QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn continuation(&self) -> Option<Continuation<'b>> {
        self.continuation
    }
}

impl<'a, 'b, QuerySet> QueryDocumentsBuilder<'a, 'b, QuerySet>
where
    QuerySet: ToAssign,
{
    fn if_modified_since(&self) -> Option<IfModifiedSince> {
        self.if_modified_since.clone()
    }
}

impl<'a, 'b> QueryDocumentsBuilder<'a, 'b, Yes> {
    fn query(&self) -> &'b Query<'b> {
        self.query.unwrap()
    }
}
impl<'a, 'b> QueryDocumentsBuilder<'a, 'b, No> {
    pub fn with_query(self, query: &'b Query<'b>) -> QueryDocumentsBuilder<'a, 'b, Yes> {
        QueryDocumentsBuilder {
            query: Some(query),
            collection_client: self.collection_client,
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
            p_query: PhantomData,
        }
    }
}
