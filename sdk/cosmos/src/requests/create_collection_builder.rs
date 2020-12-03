use crate::headers;
use crate::prelude::*;
use crate::resources::collection::{Collection, IndexingPolicy, PartitionKey};
use crate::resources::ResourceType;
use crate::responses::CreateCollectionResponse;
use azure_core::{ActivityId, No, ToAssign, UserAgent, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateCollectionBuilder<
    'a,
    OfferSet,
    CollectionNameSet,
    IndexingPolicySet,
    PartitionKeySet,
> where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    database_client: &'a DatabaseClient,
    offer: Option<Offer>,
    collection_name: Option<&'a str>,
    indexing_policy: Option<&'a IndexingPolicy>,
    partition_key: Option<&'a PartitionKey>,
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    p_offer: PhantomData<OfferSet>,
    p_collection_name: PhantomData<CollectionNameSet>,
    p_indexing_policy: PhantomData<IndexingPolicySet>,
    p_partition_key: PhantomData<PartitionKeySet>,
}

impl<'a> CreateCollectionBuilder<'a, No, No, No, No> {
    pub(crate) fn new(database_client: &'a DatabaseClient) -> Self {
        Self {
            database_client,
            offer: None,
            collection_name: None,
            indexing_policy: None,
            partition_key: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            p_indexing_policy: PhantomData,
            p_collection_name: PhantomData,
            p_partition_key: PhantomData,
            p_offer: PhantomData,
        }
    }
}

impl<'a, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    setters! {
        user_agent: &'a str => |s| Some(UserAgent::new(s)),
        activity_id: &'a str => |s| Some(ActivityId::new(s)),
        consistency_level: ConsistencyLevel => Some,
    }
}

impl<'a, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    CreateCollectionBuilder<'a, No, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    pub fn with_offer(
        self,
        offer: Offer,
    ) -> CreateCollectionBuilder<'a, Yes, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    {
        CreateCollectionBuilder {
            offer: Some(offer),
            database_client: self.database_client,
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_offer: PhantomData,
            p_collection_name: PhantomData,
            p_indexing_policy: PhantomData,
            p_partition_key: PhantomData,
        }
    }
}

impl<'a, OfferSet, IndexingPolicySet, PartitionKeySet>
    CreateCollectionBuilder<'a, OfferSet, No, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    pub fn with_collection_name(
        self,
        collection_name: &'a str,
    ) -> CreateCollectionBuilder<'a, OfferSet, Yes, IndexingPolicySet, PartitionKeySet> {
        CreateCollectionBuilder {
            collection_name: Some(collection_name),
            database_client: self.database_client,
            offer: self.offer,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_offer: PhantomData,
            p_collection_name: PhantomData,
            p_indexing_policy: PhantomData,
            p_partition_key: PhantomData,
        }
    }
}

impl<'a, OfferSet, CollectionNameSet, PartitionKeySet>
    CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, No, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    PartitionKeySet: ToAssign,
{
    pub fn with_indexing_policy(
        self,
        indexing_policy: &'a IndexingPolicy,
    ) -> CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, Yes, PartitionKeySet> {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData,
            p_collection_name: PhantomData,
            p_indexing_policy: PhantomData,
            p_partition_key: PhantomData,
            offer: self.offer,
            collection_name: self.collection_name,
            indexing_policy: Some(indexing_policy),
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, OfferSet, CollectionNameSet, IndexingPolicySet>
    CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, IndexingPolicySet, No>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    pub fn with_partition_key(
        self,
        partition_key: &'a PartitionKey,
    ) -> CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, IndexingPolicySet, Yes> {
        CreateCollectionBuilder {
            partition_key: Some(partition_key),
            database_client: self.database_client,
            offer: self.offer,
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_offer: PhantomData,
            p_collection_name: PhantomData,
            p_indexing_policy: PhantomData,
            p_partition_key: PhantomData,
        }
    }
}

impl<'a> CreateCollectionBuilder<'a, Yes, Yes, Yes, Yes> {
    pub async fn execute(&self) -> Result<CreateCollectionResponse, CosmosError> {
        trace!("CreateCollectionBuilder::execute called");

        let mut req = self.database_client.cosmos_client().prepare_request(
            &format!("dbs/{}/colls", self.database_client.database_name()),
            http::Method::POST,
            ResourceType::Collections,
        );

        req = req.header(http::header::CONTENT_TYPE, "application/json");

        // add trait headers
        let req = headers::add_header(self.offer, req);
        let req = headers::add_header(self.user_agent, req);
        let req = headers::add_header(self.activity_id, req);
        let req = headers::add_header(self.consistency_level.clone(), req);

        let mut collection = Collection::new(
            self.collection_name.unwrap(),
            self.indexing_policy.unwrap().to_owned(),
        );
        collection.parition_key = self.partition_key.unwrap().to_owned();

        let body = serde_json::to_string(&collection)?;
        debug!("body == {}", body);

        let req = req.body(body.as_bytes())?;
        debug!("\nreq == {:?}", req);

        Ok(self
            .database_client
            .http_client()
            .execute_request_check_status(req, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
