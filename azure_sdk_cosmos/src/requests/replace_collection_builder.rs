use crate::clients::CosmosUriBuilder;
use crate::collection::IndexingPolicy;
use crate::collection::PartitionKey;
use crate::prelude::*;
use crate::responses::CreateCollectionResponse;
use crate::CollectionBuilderTrait;
use crate::CollectionClient;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceCollectionBuilder<'a, CUB, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    p_partition_key: PhantomData<PartitionKeysSet>,
    p_indexing_policy: PhantomData<IndexingPolicySet>,
    partition_key: Option<&'a PartitionKey>,
    indexing_policy: Option<&'a IndexingPolicy>,
}

impl<'a, CUB> ReplaceCollectionBuilder<'a, CUB, No, No>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> ReplaceCollectionBuilder<'a, CUB, No, No> {
        ReplaceCollectionBuilder {
            collection_client,
            p_partition_key: PhantomData {},
            partition_key: None,
            p_indexing_policy: PhantomData {},
            indexing_policy: None,
        }
    }
}

impl<'a, CUB, PartitionKeysSet, IndexingPolicySet> CollectionClientRequired<'a, CUB>
    for ReplaceCollectionBuilder<'a, CUB, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB, IndexingPolicySet> PartitionKeyRequired<'a>
    for ReplaceCollectionBuilder<'a, CUB, Yes, IndexingPolicySet>
where
    IndexingPolicySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn partition_key(&self) -> &'a PartitionKey {
        self.partition_key.unwrap()
    }
}

impl<'a, CUB, PartitionKeysSet> IndexingPolicyRequired<'a>
    for ReplaceCollectionBuilder<'a, CUB, PartitionKeysSet, Yes>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn indexing_policy(&self) -> &'a IndexingPolicy {
        self.indexing_policy.unwrap()
    }
}

impl<'a, CUB, IndexingPolicySet> PartitionKeySupport<'a>
    for ReplaceCollectionBuilder<'a, CUB, No, IndexingPolicySet>
where
    IndexingPolicySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceCollectionBuilder<'a, CUB, Yes, IndexingPolicySet>;

    fn with_partition_key(self, partition_key: &'a PartitionKey) -> Self::O {
        ReplaceCollectionBuilder {
            collection_client: self.collection_client,
            p_partition_key: PhantomData {},
            p_indexing_policy: PhantomData {},
            partition_key: Some(partition_key),
            indexing_policy: self.indexing_policy,
        }
    }
}

impl<'a, CUB, PartitionKeysSet> IndexingPolicySupport<'a>
    for ReplaceCollectionBuilder<'a, CUB, PartitionKeysSet, No>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ReplaceCollectionBuilder<'a, CUB, PartitionKeysSet, Yes>;

    fn with_indexing_policy(self, indexing_policy: &'a IndexingPolicy) -> Self::O {
        ReplaceCollectionBuilder {
            collection_client: self.collection_client,
            p_partition_key: PhantomData {},
            p_indexing_policy: PhantomData {},
            partition_key: self.partition_key,
            indexing_policy: Some(indexing_policy),
        }
    }
}

// methods callable regardless
impl<'a, CUB, PartitionKeysSet, IndexingPolicySet>
    ReplaceCollectionBuilder<'a, CUB, PartitionKeysSet, IndexingPolicySet>
where
    PartitionKeysSet: ToAssign,
    IndexingPolicySet: ToAssign,
    CUB: CosmosUriBuilder,
{
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ReplaceCollectionBuilder<'a, CUB, Yes, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<CreateCollectionResponse, AzureError> {
        trace!("ReplaceCollectionBuilder::execute called");

        let mut req = self.collection_client.prepare_request(hyper::Method::PUT);

        req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Clone, Serialize)]
        struct Request<'k> {
            id: &'k str,
            #[serde(rename = "indexingPolicy")]
            indexing_policy: &'k IndexingPolicy,
            #[serde(rename = "partitionKey")]
            partition_key: &'k crate::collection::PartitionKey,
        };

        let request = Request {
            id: self.collection_client().collection_name().name(),
            indexing_policy: self.indexing_policy(),
            partition_key: self.partition_key(),
        };

        let body = serde_json::to_string(&request)?;
        println!("body == {}", body);

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
