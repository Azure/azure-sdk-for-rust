use crate::clients::{Client, CollectionClient, CosmosUriBuilder, ResourceType, UserClient};
use crate::database::DatabaseName;
use crate::DatabaseBuilderTrait;
use crate::{requests, UserName};
use crate::{CollectionName, DatabaseTrait};
use azure_sdk_core::No;

#[derive(Debug, Clone)]
pub struct DatabaseClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    main_client: &'a Client<CUB>,
    database_name: &'a dyn DatabaseName,
}

impl<'a, CUB> DatabaseClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(main_client: &'a Client<CUB>, database_name: &'a dyn DatabaseName) -> Self {
        DatabaseClient {
            main_client,
            database_name,
        }
    }

    pub(crate) fn main_client(&self) -> &Client<CUB> {
        self.main_client
    }

    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.main_client().hyper_client()
    }
}

impl<'a, CUB> DatabaseTrait<'a, CUB> for DatabaseClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.database_name
    }

    fn list_collections(&self) -> requests::ListCollectionsBuilder<'_, CUB> {
        requests::ListCollectionsBuilder::new(self)
    }

    fn get_database(&self) -> requests::GetDatabaseBuilder<'_, CUB> {
        requests::GetDatabaseBuilder::new(self)
    }

    fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_, CUB> {
        requests::DeleteDatabaseBuilder::new(self)
    }

    fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, CUB, No, No, No, No> {
        requests::CreateCollectionBuilder::new(self)
    }

    fn with_collection<'c>(
        &'c self,
        collection_name: &'c dyn CollectionName,
    ) -> CollectionClient<'c, CUB> {
        CollectionClient::new(self, collection_name)
    }

    fn with_user<'c>(&'c self, user_name: &'c dyn UserName) -> UserClient<'c, CUB> {
        UserClient::new(&self, user_name)
    }
}

impl<'a, CUB> DatabaseBuilderTrait<'a, CUB> for DatabaseClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.main_client().prepare_request(
            &format!("dbs/{}", self.database_name().name()),
            method,
            ResourceType::Databases,
        )
    }
}
