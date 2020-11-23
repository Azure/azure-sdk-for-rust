use super::*;
use crate::collection::{CollectionName, IndexingPolicy, PartitionKey};
use crate::{requests, Offer, ReadonlyString, ResourceType};

use azure_core::HttpClient;

#[derive(Debug, Clone)]
pub struct DatabaseClient {
    cosmos_client: CosmosClient,
    database_name: ReadonlyString,
}

impl DatabaseClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        cosmos_client: CosmosClient,
        database_name: S,
    ) -> Self {
        Self {
            cosmos_client,
            database_name: database_name.into(),
        }
    }

    pub fn cosmos_client(&self) -> &CosmosClient {
        &self.cosmos_client
    }

    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    pub fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }

    pub fn list_collections(&self) -> requests::ListCollectionsBuilder<'_> {
        requests::ListCollectionsBuilder::new(self)
    }

    pub fn get_database(&self) -> requests::GetDatabaseBuilder<'_, '_> {
        requests::GetDatabaseBuilder::new(self)
    }

    pub fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_> {
        requests::DeleteDatabaseBuilder::new(self)
    }

    pub fn create_collection<'a>(
        &'a self,
        offer: Offer,
        collection_name: &'a dyn CollectionName,
        indexing_policy: &'a IndexingPolicy,
        partition_key: &'a PartitionKey,
    ) -> requests::CreateCollectionBuilder<'a> {
        requests::CreateCollectionBuilder::new(
            self,
            offer,
            collection_name,
            indexing_policy,
            partition_key,
        )
    }

    pub fn list_users(&self) -> requests::ListUsersBuilder<'_, '_> {
        requests::ListUsersBuilder::new(self)
    }

    pub fn into_collection_client<S: Into<ReadonlyString>>(
        self,
        collection_name: S,
    ) -> CollectionClient {
        CollectionClient::new(self, collection_name)
    }

    pub fn into_user_client<S: Into<ReadonlyString>>(self, user_name: S) -> UserClient {
        UserClient::new(self, user_name)
    }

    pub fn prepare_request(&self, method: http::Method) -> http::request::Builder {
        self.cosmos_client()
            .prepare_request("dbs", method, ResourceType::Databases)
    }

    pub fn prepare_request_with_database_name(
        &self,
        method: http::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!("dbs/{}", self.database_name()),
            method,
            ResourceType::Databases,
        )
    }
}
