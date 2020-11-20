use crate::requests;
use crate::traits::*;
use azure_core::HttpClient;
use azure_core::No;
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct TriggerStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    collection_client: Cow<'a, COLL>,
    trigger_name: Cow<'a, str>,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
}

impl<'a, C, D, COLL> TriggerStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    pub(crate) fn new(collection_client: Cow<'a, COLL>, trigger_name: Cow<'a, str>) -> Self {
        Self {
            collection_client,
            trigger_name,
            p_c: PhantomData {},
            p_d: PhantomData {},
        }
    }
}

impl<'a, C, D, COLL> HasHttpClient for TriggerStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    #[inline]
    fn http_client(&self) -> &dyn HttpClient {
        self.collection_client.http_client()
    }
}

impl<'a, C, D, COLL> HasCosmosClient<C> for TriggerStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.collection_client.cosmos_client()
    }
}

impl<'a, C, D, COLL> HasDatabaseClient<C, D> for TriggerStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    #[inline]
    fn database_client(&self) -> &D {
        self.collection_client.database_client()
    }
}

impl<'a, C, D, COLL> HasCollectionClient<C, D, COLL> for TriggerStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    #[inline]
    fn collection_client(&self) -> &COLL {
        &self.collection_client
    }
}

impl<'a, C, D, COLL> TriggerClient<C, D, COLL> for TriggerStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    fn trigger_name(&self) -> &str {
        &self.trigger_name
    }

    fn create_trigger(
        &self,
    ) -> requests::CreateOrReplaceTriggerBuilder<'_, C, D, COLL, No, No, No> {
        requests::CreateOrReplaceTriggerBuilder::new(self, true)
    }

    fn replace_trigger(
        &self,
    ) -> requests::CreateOrReplaceTriggerBuilder<'_, C, D, COLL, No, No, No> {
        requests::CreateOrReplaceTriggerBuilder::new(self, false)
    }

    fn delete_trigger(&self) -> requests::DeleteTriggerBuilder<'_, '_, C, D, COLL> {
        requests::DeleteTriggerBuilder::new(self)
    }
}
