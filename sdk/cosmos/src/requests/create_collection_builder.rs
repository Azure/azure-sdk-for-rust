use crate::headers;
use crate::prelude::*;
use crate::resources::collection::{Collection, IndexingPolicy, PartitionKey};
use crate::resources::ResourceType;
use crate::responses::CreateCollectionResponse;
use azure_core::prelude::*;
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
    p_offer: PhantomData<OfferSet>,
    p_collection_name: PhantomData<CollectionNameSet>,
    p_indexing_policy: PhantomData<IndexingPolicySet>,
    p_partition_key: PhantomData<PartitionKeySet>,
    offer: Option<Offer>,
    collection_name: Option<&'a str>,
    indexing_policy: Option<&'a IndexingPolicy>,
    partition_key: Option<&'a PartitionKey>,
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> CreateCollectionBuilder<'a, No, No, No, No> {
    pub(crate) fn new(database_client: &'a DatabaseClient) -> Self {
        Self {
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

impl<'a, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    pub fn database_client(&self) -> &'a DatabaseClient {
        self.database_client
    }
}

impl<'a, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    CreateCollectionBuilder<'a, Yes, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    fn offer(&self) -> Offer {
        self.offer.unwrap()
    }
}

impl<'a, OfferSet, IndexingPolicySet, PartitionKeySet> CollectionNameRequired<'a>
    for CreateCollectionBuilder<'a, OfferSet, Yes, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    fn collection_name(&self) -> &'a str {
        self.collection_name.unwrap()
    }
}

impl<'a, OfferSet, CollectionNameSet, PartitionKeySet> IndexingPolicyRequired<'a>
    for CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, Yes, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    PartitionKeySet: ToAssign,
{
    fn indexing_policy(&self) -> &'a IndexingPolicy {
        self.indexing_policy.unwrap()
    }
}

impl<'a, OfferSet, CollectionNameSet, IndexingPolicySet> PartitionKeyRequired<'a>
    for CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, IndexingPolicySet, Yes>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    fn partition_key(&self) -> &'a PartitionKey {
        self.partition_key.unwrap()
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
    fn user_agent(&self) -> Option<UserAgent<'a>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<ActivityId<'a>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, CollectionNameSet, IndexingPolicySet, PartitionKeySet> OfferSupport
    for CreateCollectionBuilder<'a, No, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    type O =
        CreateCollectionBuilder<'a, Yes, CollectionNameSet, IndexingPolicySet, PartitionKeySet>;

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

impl<'a, OfferSet, IndexingPolicySet, PartitionKeySet> CollectionNameSupport<'a>
    for CreateCollectionBuilder<'a, OfferSet, No, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    type O = CreateCollectionBuilder<'a, OfferSet, Yes, IndexingPolicySet, PartitionKeySet>;

    fn with_collection_name(self, collection_name: &'a str) -> Self::O {
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

impl<'a, OfferSet, CollectionNameSet, PartitionKeySet> IndexingPolicySupport<'a>
    for CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, No, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    PartitionKeySet: ToAssign,
{
    type O = CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, Yes, PartitionKeySet>;

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

impl<'a, OfferSet, CollectionNameSet, IndexingPolicySet> PartitionKeySupport<'a>
    for CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, IndexingPolicySet, No>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    type O = CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, IndexingPolicySet, Yes>;

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

impl<'a, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet> UserAgentSupport<'a>
    for CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    type O = CreateCollectionBuilder<
        'a,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        Self {
            user_agent: Some(UserAgent(user_agent)),
            ..self
        }
    }
}

impl<'a, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet> ActivityIdSupport<'a>
    for CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    type O = CreateCollectionBuilder<
        'a,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        Self {
            activity_id: Some(ActivityId(activity_id)),
            ..self
        }
    }
}

impl<'a, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    ConsistencyLevelSupport<'a>
    for CreateCollectionBuilder<'a, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
{
    type O = CreateCollectionBuilder<
        'a,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
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
        let req = headers::add_header(Some(self.offer()), req);
        let req = headers::add_header(self.user_agent(), req);
        let req = headers::add_header(self.activity_id(), req);
        let req = headers::add_header(self.consistency_level(), req);

        let mut collection =
            Collection::new(self.collection_name(), self.indexing_policy().to_owned());
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
