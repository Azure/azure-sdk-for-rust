use crate::requests;
use crate::traits::*;
use azure_sdk_core::No;
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct UserDefinedFunctionStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    collection_client: Cow<'a, COLL>,
    user_defined_function_name: Cow<'a, str>,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
}

impl<'a, C, D, COLL> UserDefinedFunctionStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    pub(crate) fn new(
        collection_client: Cow<'a, COLL>,
        user_defined_function_name: Cow<'a, str>,
    ) -> Self {
        Self {
            collection_client,
            user_defined_function_name,
            p_c: PhantomData {},
            p_d: PhantomData {},
        }
    }
}

impl<'a, C, D, COLL> HasHyperClient for UserDefinedFunctionStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.collection_client.hyper_client()
    }
}

impl<'a, C, D, COLL> HasCosmosClient<C> for UserDefinedFunctionStruct<'a, C, D, COLL>
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

impl<'a, C, D, COLL> HasDatabaseClient<C, D> for UserDefinedFunctionStruct<'a, C, D, COLL>
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

impl<'a, C, D, COLL> HasCollectionClient<C, D, COLL> for UserDefinedFunctionStruct<'a, C, D, COLL>
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

impl<'a, C, D, COLL> UserDefinedFunctionClient<C, D, COLL>
    for UserDefinedFunctionStruct<'a, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    fn user_defined_function_name(&self) -> &str {
        &self.user_defined_function_name
    }

    fn create_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_, C, D, COLL, No> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, true)
    }

    fn replace_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_, C, D, COLL, No> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, false)
    }

    fn delete_user_defined_function(
        &self,
    ) -> requests::DeleteUserDefinedFunctionBuilder<'_, '_, C, D, COLL> {
        requests::DeleteUserDefinedFunctionBuilder::new(self)
    }
}
