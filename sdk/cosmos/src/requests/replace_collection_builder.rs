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
pub struct ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    collection_client: &'a CollectionClient,
    p_partition_key: PhantomData<PartitionKeysSet>,
    p_indexing_policy: PhantomData<IndexingPolicySet>,
    partition_key: Option<&'a PartitionKey>,
    indexing_policy: Option<&'a IndexingPolicy>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplaceCollectionBuilder<'a, 'b, No, No> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
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

impl<'a, 'b, PartitionKeysSet, IndexingPolicySet> CollectionClientRequired<'a>
    for ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }
}

impl<'a, 'b, IndexingPolicySet> PartitionKeyRequired<'a>
    for ReplaceCollectionBuilder<'a, 'b, Yes, IndexingPolicySet>
where
    IndexingPolicySet: ToAssign,
{
    fn partition_key(&self) -> &'a PartitionKey {
        self.partition_key.unwrap()
    }
}

impl<'a, 'b, PartitionKeysSet> IndexingPolicyRequired<'a>
    for ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, Yes>
where
    PartitionKeysSet: ToAssign,
{
    fn indexing_policy(&self) -> &'a IndexingPolicy {
        self.indexing_policy.unwrap()
    }
}

impl<'a, 'b, PartitionKeysSet, IndexingPolicySet> UserAgentOption<'b>
    for ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, PartitionKeysSet, IndexingPolicySet> ActivityIdOption<'b>
    for ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, PartitionKeysSet, IndexingPolicySet> ConsistencyLevelOption<'b>
    for ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, IndexingPolicySet> PartitionKeySupport<'a>
    for ReplaceCollectionBuilder<'a, 'b, No, IndexingPolicySet>
where
    IndexingPolicySet: ToAssign,
{
    type O = ReplaceCollectionBuilder<'a, 'b, Yes, IndexingPolicySet>;

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

impl<'a, 'b, PartitionKeysSet> IndexingPolicySupport<'a>
    for ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, No>
where
    PartitionKeysSet: ToAssign,
{
    type O = ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, Yes>;

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

impl<'a, 'b, PartitionKeysSet, IndexingPolicySet> UserAgentSupport<'b>
    for ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b, PartitionKeysSet, IndexingPolicySet> ActivityIdSupport<'b>
    for ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b, PartitionKeysSet, IndexingPolicySet> ConsistencyLevelSupport<'b>
    for ReplaceCollectionBuilder<'a, 'b, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ReplaceCollectionBuilder<'a, 'b, Yes, Yes> {
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
            self.collection_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
