use crate::collection::CollectionName;
use crate::collection::{Collection, IndexingPolicy, PartitionKey};
use crate::prelude::*;
use crate::responses::CreateCollectionResponse;
use crate::{Offer, ResourceType};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateCollectionBuilder<
    'a,
    C,
    OfferSet,
    CollectionNameSet,
    IndexingPolicySet,
    PartitionKeySet,
> where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    database_client: &'a dyn DatabaseClient<C>,
    p_offer: PhantomData<OfferSet>,
    p_collection_name: PhantomData<CollectionNameSet>,
    p_indexing_policy: PhantomData<IndexingPolicySet>,
    p_partition_key: PhantomData<PartitionKeySet>,
    offer: Option<Offer>,
    collection_name: Option<&'a dyn CollectionName>,
    indexing_policy: Option<&'a IndexingPolicy>,
    partition_key: Option<&'a PartitionKey>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, C> CreateCollectionBuilder<'a, C, No, No, No, No>
where
    C: CosmosClient,
{
    #[inline]
    pub(crate) fn new(
        database_client: &'a dyn DatabaseClient<C>,
    ) -> CreateCollectionBuilder<'a, C, No, No, No, No> {
        CreateCollectionBuilder {
            database_client,
            p_offer: PhantomData {},
            offer: None,
            p_collection_name: PhantomData {},
            collection_name: None,
            p_indexing_policy: PhantomData {},
            indexing_policy: None,
            p_partition_key: PhantomData {},
            partition_key: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    DatabaseClientRequired<'a, C>
    for CreateCollectionBuilder<
        'a,
        C,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    #[inline]
    fn database_client(&self) -> &'a dyn DatabaseClient<C> {
        self.database_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, CollectionNameSet, IndexingPolicySet, PartitionKeySet> OfferRequired
    for CreateCollectionBuilder<'a, C, Yes, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    #[inline]
    fn offer(&self) -> Offer {
        self.offer.unwrap()
    }
}

impl<'a, C, OfferSet, IndexingPolicySet, PartitionKeySet> CollectionNameRequired<'a>
    for CreateCollectionBuilder<'a, C, OfferSet, Yes, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    #[inline]
    fn collection_name(&self) -> &'a dyn CollectionName {
        self.collection_name.unwrap()
    }
}

impl<'a, C, OfferSet, CollectionNameSet, PartitionKeySet> IndexingPolicyRequired<'a>
    for CreateCollectionBuilder<'a, C, OfferSet, CollectionNameSet, Yes, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    #[inline]
    fn indexing_policy(&self) -> &'a IndexingPolicy {
        self.indexing_policy.unwrap()
    }
}

impl<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet> PartitionKeyRequired<'a>
    for CreateCollectionBuilder<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet, Yes>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
{
    #[inline]
    fn partition_key(&self) -> &'a PartitionKey {
        self.partition_key.unwrap()
    }
}

impl<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet> UserAgentOption<'a>
    for CreateCollectionBuilder<
        'a,
        C,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet> ActivityIdOption<'a>
    for CreateCollectionBuilder<
        'a,
        C,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    ConsistencyLevelOption<'a>
    for CreateCollectionBuilder<
        'a,
        C,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level.clone()
    }
}

impl<'a, C, CollectionNameSet, IndexingPolicySet, PartitionKeySet> OfferSupport
    for CreateCollectionBuilder<'a, C, No, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    type O =
        CreateCollectionBuilder<'a, C, Yes, CollectionNameSet, IndexingPolicySet, PartitionKeySet>;

    #[inline]
    fn with_offer(self, offer: Offer) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: Some(offer),
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, C, OfferSet, IndexingPolicySet, PartitionKeySet> CollectionNameSupport<'a>
    for CreateCollectionBuilder<'a, C, OfferSet, No, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    type O = CreateCollectionBuilder<'a, C, OfferSet, Yes, IndexingPolicySet, PartitionKeySet>;

    #[inline]
    fn with_collection_name(self, collection_name: &'a dyn CollectionName) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: self.offer,
            collection_name: Some(collection_name),
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, C, OfferSet, CollectionNameSet, PartitionKeySet> IndexingPolicySupport<'a>
    for CreateCollectionBuilder<'a, C, OfferSet, CollectionNameSet, No, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    type O = CreateCollectionBuilder<'a, C, OfferSet, CollectionNameSet, Yes, PartitionKeySet>;

    #[inline]
    fn with_indexing_policy(self, indexing_policy: &'a IndexingPolicy) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
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

impl<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet> PartitionKeySupport<'a>
    for CreateCollectionBuilder<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet, No>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    C: CosmosClient,
{
    type O = CreateCollectionBuilder<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet, Yes>;

    #[inline]
    fn with_partition_key(self, partition_key: &'a PartitionKey) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: self.offer,
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            partition_key: Some(partition_key),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet> UserAgentSupport<'a>
    for CreateCollectionBuilder<
        'a,
        C,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    type O = CreateCollectionBuilder<
        'a,
        C,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: self.offer,
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet> ActivityIdSupport<'a>
    for CreateCollectionBuilder<
        'a,
        C,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    type O = CreateCollectionBuilder<
        'a,
        C,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: self.offer,
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, C, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    ConsistencyLevelSupport<'a>
    for CreateCollectionBuilder<
        'a,
        C,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    C: CosmosClient,
{
    type O = CreateCollectionBuilder<
        'a,
        C,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: self.offer,
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> CreateCollectionBuilder<'a, C, Yes, Yes, Yes, Yes>
where
    C: CosmosClient,
{
    pub async fn execute(&self) -> Result<CreateCollectionResponse, CosmosError> {
        trace!("CreateCollectionBuilder::execute called");

        let mut req = self.database_client.cosmos_client().prepare_request(
            &format!("dbs/{}/colls", self.database_client.database_name()),
            http::Method::POST,
            ResourceType::Collections,
        );

        req = req.header(http::header::CONTENT_TYPE, "application/json");

        // add trait headers
        let req = OfferRequired::add_header(self, req);
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let mut collection = Collection::new(
            self.collection_name().name(),
            self.indexing_policy().to_owned(),
        );
        collection.parition_key = self.partition_key().to_owned();

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
