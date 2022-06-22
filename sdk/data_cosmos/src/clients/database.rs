use super::*;
use crate::operations::*;
use crate::resources::collection::PartitionKey;
use crate::ReadonlyString;
use azure_core::Request;
use http::Method;

/// A client for Cosmos database resources.
#[derive(Debug, Clone)]
pub struct DatabaseClient {
    client: CosmosClient,
    database_name: ReadonlyString,
}

impl DatabaseClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(client: CosmosClient, database_name: S) -> Self {
        Self {
            client,
            database_name: database_name.into(),
        }
    }

    /// Get a [`CosmosClient`].
    #[must_use]
    pub fn cosmos_client(&self) -> &CosmosClient {
        &self.client
    }

    /// Convert into a [`CollectionClient`]
    pub fn collection_client<S: Into<ReadonlyString>>(
        &self,
        collection_name: S,
    ) -> CollectionClient {
        CollectionClient::new(self.clone(), collection_name)
    }

    /// Convert into a [`UserClient`]
    pub fn user_client<S: Into<ReadonlyString>>(&self, user_name: S) -> UserClient {
        UserClient::new(self.clone(), user_name)
    }

    /// Get the database's name
    #[must_use]
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Get the database
    #[must_use]
    pub fn get_database(&self) -> GetDatabaseBuilder {
        GetDatabaseBuilder::new(self.clone())
    }

    /// Delete the database
    #[must_use]
    pub fn delete_database(&self) -> DeleteDatabaseBuilder {
        DeleteDatabaseBuilder::new(self.clone())
    }

    /// List collections in the database
    #[must_use]
    pub fn list_collections(&self) -> ListCollectionsBuilder {
        ListCollectionsBuilder::new(self.clone())
    }

    /// Create a collection
    pub fn create_collection<S: Into<String>, P: Into<PartitionKey>>(
        &self,
        collection_name: S,
        partition_key: P,
    ) -> CreateCollectionBuilder {
        CreateCollectionBuilder::new(self.clone(), collection_name.into(), partition_key.into())
    }

    /// List users
    #[must_use]
    pub fn list_users(&self) -> ListUsersBuilder {
        ListUsersBuilder::new(self.clone())
    }

    /// Convert into a [`CollectionClient`]
    pub fn into_collection_client<S: Into<ReadonlyString>>(
        self,
        collection_name: S,
    ) -> CollectionClient {
        CollectionClient::new(self, collection_name)
    }

    /// Convert into a [`UserClient`]
    pub fn into_user_client<S: Into<ReadonlyString>>(self, user_name: S) -> UserClient {
        UserClient::new(self, user_name)
    }

    pub(crate) fn prepare_pipeline(&self, method: Method) -> Request {
        self.cosmos_client()
            .prepare_request_pipeline(&format!("dbs/{}", self.database_name()), method)
    }

    pub(crate) fn prepare_collections_pipeline(&self, method: Method) -> Request {
        self.cosmos_client()
            .prepare_request_pipeline(&format!("dbs/{}/colls", self.database_name()), method)
    }
}
