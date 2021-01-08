use crate::prelude::*;
use crate::resources::collection::{IndexingPolicy, PartitionKey};
use crate::responses::CreateCollectionResponse;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    collection_client: &'a CollectionClient,
    partition_key: Option<PartitionKey>,
    indexing_policy: Option<&'a IndexingPolicy>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    p_partition_key: PhantomData<PartitionKeysSet>,
    p_indexing_policy: PhantomData<IndexingPolicySet>,
}

impl<'a, 'b> ReplaceCollectionBuilder<'a, 'b, No, No> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            partition_key: None,
            indexing_policy: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            p_partition_key: PhantomData,
            p_indexing_policy: PhantomData,
        }
    }
}

impl<'a, 'b, PartitionKeysSet, IndexingPolicySet>
    ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ReplaceCollectionBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute(&self) -> Result<CreateCollectionResponse, CosmosError> {
        trace!("ReplaceCollectionBuilder::execute called");

        let req = self
            .collection_client
            .prepare_request_with_collection_name(http::Method::PUT);

        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Clone, Serialize)]
        struct Request<'k> {
            id: &'k str,
            #[serde(rename = "indexingPolicy")]
            indexing_policy: &'k IndexingPolicy,
            #[serde(rename = "partitionKey")]
            partition_key: &'k PartitionKey,
        };

        let request = Request {
            id: self.collection_client.collection_name(),
            indexing_policy: self.indexing_policy.unwrap(),
            partition_key: self.partition_key.as_ref().unwrap(),
        };

        let body = serde_json::to_string(&request)?;
        debug!("body == {}", body);

        let req = req.body(body.as_bytes())?;
        debug!("\nreq == {:?}", req);

        // the docs are wrong here
        // [https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-collection](https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-collection).
        // They say you should receive 201 instead azure returns 200 upon success. I've filed a PR
        // to correct it.
        Ok(self
            .collection_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }
}

impl<'a, 'b, IndexingPolicySet> ReplaceCollectionBuilder<'a, 'b, No, IndexingPolicySet>
where
    IndexingPolicySet: ToAssign,
{
    pub fn partition_key<P: Into<PartitionKey>>(
        self,
        partition_key: P,
    ) -> ReplaceCollectionBuilder<'a, 'b, Yes, IndexingPolicySet> {
        ReplaceCollectionBuilder {
            collection_client: self.collection_client,
            p_partition_key: PhantomData,
            p_indexing_policy: PhantomData,
            partition_key: Some(partition_key.into()),
            indexing_policy: self.indexing_policy,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, PartitionKeysSet> ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, No>
where
    PartitionKeysSet: ToAssign,
{
    pub fn indexing_policy(
        self,
        indexing_policy: &'a IndexingPolicy,
    ) -> ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, Yes> {
        ReplaceCollectionBuilder {
            collection_client: self.collection_client,
            p_partition_key: PhantomData,
            p_indexing_policy: PhantomData,
            partition_key: self.partition_key,
            indexing_policy: Some(indexing_policy),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}
