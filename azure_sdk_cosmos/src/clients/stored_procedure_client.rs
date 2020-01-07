use crate::clients::{Client, CollectionClient, CosmosUriBuilder, ResourceType};
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use crate::requests;
use crate::stored_procedure::StoredProcedureName;
use crate::{CollectionTrait, StoredProcedureBuilderTrait, StoredProcedureTrait};

#[derive(Debug, Clone)]
pub struct StoredProcedureClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    stored_procedure_name: &'a dyn StoredProcedureName,
}

impl<'a, CUB> StoredProcedureClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
        stored_procedure_name: &'a dyn StoredProcedureName,
    ) -> Self {
        StoredProcedureClient {
            collection_client,
            stored_procedure_name,
        }
    }

    pub(crate) fn main_client(&self) -> &Client<CUB> {
        self.collection_client.main_client()
    }

    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.main_client().hyper_client()
    }
}

impl<'a, CUB> StoredProcedureTrait<'a, CUB> for StoredProcedureClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.collection_client.database_name()
    }

    fn collection_name(&self) -> &'a dyn CollectionName {
        self.collection_client.collection_name()
    }

    fn stored_procedure_name(&self) -> &'a dyn StoredProcedureName {
        self.stored_procedure_name
    }

    fn execute_stored_procedure(&self) -> requests::ExecuteStoredProcedureBuilder<'_, '_, CUB> {
        requests::ExecuteStoredProcedureBuilder::new(self)
    }
}

impl<'a, CUB> StoredProcedureBuilderTrait<'a, CUB> for StoredProcedureClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/sprocs/{}",
                self.database_name().name(),
                self.collection_name().name(),
                self.stored_procedure_name().name()
            ),
            method,
            ResourceType::StoredProcedures,
        )
    }
}
