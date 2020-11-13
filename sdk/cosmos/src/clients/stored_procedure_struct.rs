use crate::requests;
use crate::traits::*;
use azure_core::HttpClient;
use azure_core::No;
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct StoredProcedureStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    collection_client: Cow<'a, COLL>,
    stored_procedure_name: Cow<'a, str>,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
}

impl<'a, C, D, COLL> StoredProcedureStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    pub(crate) fn new(
        collection_client: Cow<'a, COLL>,
        stored_procedure_name: Cow<'a, str>,
    ) -> Self {
        Self {
            collection_client,
            stored_procedure_name,
            p_c: PhantomData {},
            p_d: PhantomData {},
        }
    }
}

impl<'a, C, D, COLL> HasHttpClient for StoredProcedureStruct<'a, C, D, COLL>
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

impl<'a, C, D, COLL> HasCosmosClient<C> for StoredProcedureStruct<'a, C, D, COLL>
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

impl<'a, C, D, COLL> HasDatabaseClient<C, D> for StoredProcedureStruct<'a, C, D, COLL>
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

impl<'a, C, D, COLL> HasCollectionClient<C, D, COLL> for StoredProcedureStruct<'a, C, D, COLL>
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

impl<'a, C, D, COLL> StoredProcedureClient<C, D, COLL> for StoredProcedureStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    fn stored_procedure_name(&self) -> &str {
        &self.stored_procedure_name
    }

    fn create_stored_procedure(
        &self,
    ) -> requests::CreateStoredProcedureBuilder<'_, '_, C, D, COLL, No> {
        requests::CreateStoredProcedureBuilder::new(self)
    }

    fn replace_stored_procedure(
        &self,
    ) -> requests::ReplaceStoredProcedureBuilder<'_, '_, C, D, COLL, No> {
        requests::ReplaceStoredProcedureBuilder::new(self)
    }

    fn execute_stored_procedure(
        &self,
    ) -> requests::ExecuteStoredProcedureBuilder<'_, '_, C, D, COLL> {
        requests::ExecuteStoredProcedureBuilder::new(self)
    }

    fn delete_stored_procedure(
        &self,
    ) -> requests::DeleteStoredProcedureBuilder<'_, '_, C, D, COLL> {
        requests::DeleteStoredProcedureBuilder::new(self)
    }
}
