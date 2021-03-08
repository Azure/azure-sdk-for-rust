use crate::prelude::*;
use crate::resources::document::Query;
use crate::resources::ResourceType;
use crate::responses::QueryDocumentsResponse;
use azure_core::prelude::*;
use chrono::{DateTime, Utc};
use futures::stream::{unfold, Stream};
use http::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct QueryDocumentsBuilder<'a, 'b> {
    collection_client: &'a CollectionClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<IfModifiedSince<'b>>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<Continuation<'b>>,
    max_item_count: MaxItemCount,
    partition_key_serialized: Option<String>,
    query_cross_partition: QueryCrossPartition,
    parallelize_cross_partition_query: ParallelizeCrossPartition,
}

impl<'a, 'b> QueryDocumentsBuilder<'a, 'b> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: MaxItemCount::new(-1),
            partition_key_serialized: None,
            query_cross_partition: QueryCrossPartition::No,
            // TODO: use this in request
            parallelize_cross_partition_query: ParallelizeCrossPartition::No,
        }
    }
}

impl<'a, 'b> QueryDocumentsBuilder<'a, 'b> {
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
        continuation: &'b str => Some(Continuation::new(continuation)),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
        if_modified_since: &'b DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
        query_cross_partition: bool => if query_cross_partition { QueryCrossPartition::Yes } else { QueryCrossPartition::No },
        parallelize_cross_partition_query: bool => if parallelize_cross_partition_query { ParallelizeCrossPartition::Yes } else { ParallelizeCrossPartition::No },
    }

    pub fn partition_key<PK: serde::Serialize>(self, pk: &PK) -> Result<Self, serde_json::Error> {
        Ok(Self {
            partition_key_serialized: Some(
                crate::cosmos_entity::serialize_partition_key_to_string(pk)?,
            ),
            ..self
        })
    }

    pub async fn execute<T, Q>(&self, query: Q) -> Result<QueryDocumentsResponse<T>, CosmosError>
    where
        T: DeserializeOwned,
        Q: Into<Query<'a>>,
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

        let req = if let Some(partition_key_serialized) = self.partition_key_serialized.as_ref() {
            crate::cosmos_entity::add_as_partition_key_header_serialized(
                &partition_key_serialized,
                req,
            )
        } else {
            req
        };

        // signal that this is a query
        let req = req.header(crate::headers::HEADER_DOCUMENTDB_ISQUERY, true.to_string());
        let req = req.header(http::header::CONTENT_TYPE, "application/query+json");

        // add trait headers
        let req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        let req = azure_core::headers::add_optional_header(&self.if_modified_since, req);
        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);
        let req = azure_core::headers::add_optional_header(&self.continuation, req);
        let req = azure_core::headers::add_mandatory_header(&self.max_item_count, req);
        let req = azure_core::headers::add_mandatory_header(&self.query_cross_partition, req);

        let body = azure_core::to_json(&query.into())?;
        debug!("body == {:?}", body);

        let req = req.body(body)?;
        debug!("{:?}", req);

        Ok(self
            .collection_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }

    pub fn stream<T, Q>(
        &'a self,
        query: Q,
    ) -> impl Stream<Item = Result<QueryDocumentsResponse<T>, CosmosError>> + 'a
    where
        T: DeserializeOwned,
        Q: Into<Query<'a>> + 'a + Copy,
    {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            Continuation(String),
        }

        unfold(
            Some(States::Init),
            move |continuation_token: Option<States>| async move {
                debug!("continuation_token == {:?}", &continuation_token);
                let response = match continuation_token {
                    Some(States::Init) => self.execute(query).await,
                    Some(States::Continuation(continuation_token)) => {
                        self.clone()
                            .continuation(continuation_token.as_str())
                            .execute(query)
                            .await
                    }
                    None => return None,
                };

                let response = match response {
                    Ok(response) => response,
                    Err(err) => return Some((Err(err), None)),
                };

                let continuation_token = response
                    .continuation_token
                    .as_ref()
                    .map(|ct| States::Continuation(ct.to_owned()));

                Some((Ok(response), continuation_token))
            },
        )
    }
}
