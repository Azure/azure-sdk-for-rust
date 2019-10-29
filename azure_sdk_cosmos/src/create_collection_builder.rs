use crate::client::{Client, CosmosUriBuilder};
use crate::collection::{Collection, IndexingPolicy, PartitionKey};
use crate::Offer;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{No, ToAssign, Yes};
use futures::future::*;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DatabaseNameSet;
impl ToAssign for DatabaseNameSet {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OfferSet;
impl ToAssign for OfferSet {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IdSet;
impl ToAssign for IdSet {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IndexingPolicySet;
impl ToAssign for IndexingPolicySet {}

#[derive(Debug, Clone)]
pub struct CreateCollectionBuilder<'a, CUB, DatabaseNameSet, OfferSet, IdSet, IndexingPolicySet>
where
    CUB: CosmosUriBuilder,
    DatabaseNameSet: ToAssign,
    OfferSet: ToAssign,
    IdSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    client: &'a Client<CUB>,
    database_name: Option<&'a str>,
    offer: Option<Offer>,
    id: Option<&'a str>,
    indexing_policy: Option<IndexingPolicy>,
    partition_key: Option<PartitionKey>,
    p_database_name: PhantomData<DatabaseNameSet>,
    p_offer: PhantomData<OfferSet>,
    p_id: PhantomData<IdSet>,
    p_indexing_policy: PhantomData<IndexingPolicySet>,
}

impl<'a, CUB> CreateCollectionBuilder<'a, CUB, No, No, No, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(client: &'a Client<CUB>) -> CreateCollectionBuilder<'a, CUB, No, No, No, No> {
        CreateCollectionBuilder {
            client: client,
            database_name: None,
            offer: None,
            id: None,
            indexing_policy: None,
            partition_key: None,
            p_database_name: PhantomData {},
            p_offer: PhantomData {},
            p_id: PhantomData {},
            p_indexing_policy: PhantomData {},
        }
    }
}

impl<'a, CUB, OfferSet, IdSet, IndexingPolicySet> CreateCollectionBuilder<'a, CUB, No, OfferSet, IdSet, IndexingPolicySet>
where
    CUB: CosmosUriBuilder,
    DatabaseNameSet: ToAssign,
    OfferSet: ToAssign,
    IdSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    #[inline]
    pub fn with_database_name(self, database_name: &'a str) -> CreateCollectionBuilder<'a, CUB, Yes, OfferSet, IdSet, IndexingPolicySet> {
        CreateCollectionBuilder {
            client: self.client,
            database_name: Some(database_name),
            offer: self.offer,
            id: self.id,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            p_database_name: PhantomData {},
            p_offer: self.p_offer,
            p_id: self.p_id,
            p_indexing_policy: self.p_indexing_policy,
        }
    }
}

impl<'a, CUB, DatabaseNameSet, IdSet, IndexingPolicySet> CreateCollectionBuilder<'a, CUB, DatabaseNameSet, No, IdSet, IndexingPolicySet>
where
    CUB: CosmosUriBuilder,
    DatabaseNameSet: ToAssign,
    OfferSet: ToAssign,
    IdSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    #[inline]
    pub fn with_offer(self, offer: Offer) -> CreateCollectionBuilder<'a, CUB, DatabaseNameSet, Yes, IdSet, IndexingPolicySet> {
        CreateCollectionBuilder {
            client: self.client,
            database_name: self.database_name,
            offer: Some(offer),
            id: self.id,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            p_database_name: self.p_database_name,
            p_offer: PhantomData {},
            p_id: self.p_id,
            p_indexing_policy: self.p_indexing_policy,
        }
    }
}

impl<'a, CUB, DatabaseNameSet, OfferSet, IndexingPolicySet>
    CreateCollectionBuilder<'a, CUB, DatabaseNameSet, OfferSet, No, IndexingPolicySet>
where
    CUB: CosmosUriBuilder,
    DatabaseNameSet: ToAssign,
    OfferSet: ToAssign,
    IdSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    #[inline]
    pub fn with_id(self, id: &'a str) -> CreateCollectionBuilder<'a, CUB, DatabaseNameSet, OfferSet, Yes, IndexingPolicySet> {
        CreateCollectionBuilder {
            client: self.client,
            database_name: self.database_name,
            offer: self.offer,
            id: Some(id),
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            p_database_name: self.p_database_name,
            p_offer: self.p_offer,
            p_id: PhantomData {},
            p_indexing_policy: self.p_indexing_policy,
        }
    }
}

impl<'a, CUB, DatabaseNameSet, OfferSet, IdSet> CreateCollectionBuilder<'a, CUB, DatabaseNameSet, OfferSet, IdSet, No>
where
    CUB: CosmosUriBuilder,
    DatabaseNameSet: ToAssign,
    OfferSet: ToAssign,
    IdSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    #[inline]
    pub fn with_indexing_policy(
        self,
        indexing_policy: IndexingPolicy,
    ) -> CreateCollectionBuilder<'a, CUB, DatabaseNameSet, OfferSet, IdSet, Yes> {
        CreateCollectionBuilder {
            client: self.client,
            database_name: self.database_name,
            offer: self.offer,
            id: self.id,
            indexing_policy: Some(indexing_policy),
            partition_key: self.partition_key,
            p_database_name: self.p_database_name,
            p_offer: self.p_offer,
            p_id: self.p_id,
            p_indexing_policy: PhantomData {},
        }
    }
}

impl<'a, CUB, DatabaseNameSet, OfferSet, IdSet, IndexingPolicySet>
    CreateCollectionBuilder<'a, CUB, DatabaseNameSet, OfferSet, IdSet, IndexingPolicySet>
where
    CUB: CosmosUriBuilder,
    DatabaseNameSet: ToAssign,
    OfferSet: ToAssign,
    IdSet: ToAssign,
    IndexingPolicySet: ToAssign,
{
    #[inline]
    pub fn with_partition_key(
        self,
        partition_key: PartitionKey,
    ) -> CreateCollectionBuilder<'a, CUB, DatabaseNameSet, OfferSet, IdSet, IndexingPolicySet> {
        CreateCollectionBuilder {
            client: self.client,
            database_name: self.database_name,
            offer: self.offer,
            id: self.id,
            indexing_policy: self.indexing_policy,
            partition_key: Some(partition_key),
            p_database_name: self.p_database_name,
            p_offer: self.p_offer,
            p_id: self.p_id,
            p_indexing_policy: self.p_indexing_policy,
        }
    }
}

impl<'a, CUB> CreateCollectionBuilder<'a, CUB, Yes, Yes, Yes, Yes>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub fn finalize(self) -> impl Future<Item = Collection, Error = AzureError> {
        trace!(
            "create_collection_builder::finalize(database_name == {:?}, \
             id == {:?}, offer == {:?}, indexing_policy == {:?}, parition_key == {:?} called",
            &self.database_name,
            &self.id,
            &self.offer,
            &self.indexing_policy,
            &self.partition_key
        );

        let mut collection = Collection::new(self.id.unwrap(), self.indexing_policy.unwrap());
        collection.parition_key = self.partition_key;

        self.client
            .create_collection(self.database_name.unwrap(), self.offer.unwrap(), &collection)
    }
}
