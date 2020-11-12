use crate::collection::IndexingPolicy;
use crate::collection::PartitionKey;
use crate::prelude::*;
use crate::responses::CreateCollectionResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    collection_client: &'a dyn CollectionClient<C, D>,
    p_partition_key: PhantomData<PartitionKeysSet>,
    p_indexing_policy: PhantomData<IndexingPolicySet>,
    partition_key: Option<&'a PartitionKey>,
    indexing_policy: Option<&'a IndexingPolicy>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D> ReplaceCollectionBuilder<'a, 'b, C, D, No, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    pub(crate) fn new(
        collection_client: &'a dyn CollectionClient<C, D>,
    ) -> ReplaceCollectionBuilder<'a, 'b, C, D, No, No> {
        ReplaceCollectionBuilder {
            collection_client,
            p_partition_key: PhantomData {},
            partition_key: None,
            p_indexing_policy: PhantomData {},
            indexing_policy: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet> CollectionClientRequired<'a, C, D>
    for ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn collection_client(&self) -> &'a dyn CollectionClient<C, D> {
        self.collection_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, C, D, IndexingPolicySet> PartitionKeyRequired<'a>
    for ReplaceCollectionBuilder<'a, 'b, C, D, Yes, IndexingPolicySet>
where
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn partition_key(&self) -> &'a PartitionKey {
        self.partition_key.unwrap()
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> IndexingPolicyRequired<'a>
    for ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, Yes>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn indexing_policy(&self) -> &'a IndexingPolicy {
        self.indexing_policy.unwrap()
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet> UserAgentOption<'b>
    for ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet> ActivityIdOption<'b>
    for ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet> ConsistencyLevelOption<'b>
    for ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D, IndexingPolicySet> PartitionKeySupport<'a>
    for ReplaceCollectionBuilder<'a, 'b, C, D, No, IndexingPolicySet>
where
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceCollectionBuilder<'a, 'b, C, D, Yes, IndexingPolicySet>;

    fn with_partition_key(self, partition_key: &'a PartitionKey) -> Self::O {
        ReplaceCollectionBuilder {
            collection_client: self.collection_client,
            p_partition_key: PhantomData {},
            p_indexing_policy: PhantomData {},
            partition_key: Some(partition_key),
            indexing_policy: self.indexing_policy,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> IndexingPolicySupport<'a>
    for ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, No>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, Yes>;

    fn with_indexing_policy(self, indexing_policy: &'a IndexingPolicy) -> Self::O {
        ReplaceCollectionBuilder {
            collection_client: self.collection_client,
            p_partition_key: PhantomData {},
            p_indexing_policy: PhantomData {},
            partition_key: self.partition_key,
            indexing_policy: Some(indexing_policy),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet> UserAgentSupport<'b>
    for ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet>;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ReplaceCollectionBuilder {
            collection_client: self.collection_client,
            p_partition_key: PhantomData {},
            p_indexing_policy: PhantomData {},
            partition_key: self.partition_key,
            indexing_policy: self.indexing_policy,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet> ActivityIdSupport<'b>
    for ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet>;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ReplaceCollectionBuilder {
            collection_client: self.collection_client,
            p_partition_key: PhantomData {},
            p_indexing_policy: PhantomData {},
            partition_key: self.partition_key,
            indexing_policy: self.indexing_policy,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet> ConsistencyLevelSupport<'b>
    for ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceCollectionBuilder<'a, 'b, C, D, PartitionKeysSet, IndexingPolicySet>;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ReplaceCollectionBuilder {
            collection_client: self.collection_client,
            p_partition_key: PhantomData {},
            p_indexing_policy: PhantomData {},
            partition_key: self.partition_key,
            indexing_policy: self.indexing_policy,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D> ReplaceCollectionBuilder<'a, 'b, C, D, Yes, Yes>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    pub async fn execute(&self) -> Result<CreateCollectionResponse, AzureError> {
        trace!("ReplaceCollectionBuilder::execute called");

        let req = self
            .collection_client
            .prepare_request_with_collection_name(hyper::Method::PUT);

        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Clone, Serialize)]
        struct Request<'k> {
            id: &'k str,
            #[serde(rename = "indexingPolicy")]
            indexing_policy: &'k IndexingPolicy,
            #[serde(rename = "partitionKey")]
            partition_key: &'k crate::collection::PartitionKey,
        };

        let request = Request {
            id: self.collection_client().collection_name(),
            indexing_policy: self.indexing_policy(),
            partition_key: self.partition_key(),
        };

        let body = serde_json::to_string(&request)?;
        debug!("body == {}", body);

        let req = req.body(hyper::Body::from(body))?;
        debug!("\nreq == {:?}", req);

        // the docs are wrong here
        // [https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-collection](https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-collection).
        // They say you should receive 201 instead azure returns 200 upon success. I've filed a PR
        // to correct it.
        let (headers, body) = check_status_extract_headers_and_body(
            self.collection_client.http_client().request(req),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
