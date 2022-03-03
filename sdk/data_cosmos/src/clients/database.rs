use super::*;
use crate::operations::*;
use crate::resources::collection::PartitionKey;
use crate::ReadonlyString;
use azure_core::Pipeline;

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
    pub fn client(&self) -> &CosmosClient {
        &self.client
    }

    /// Get the database's name
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Get the database
    pub fn get_database(&self) -> GetDatabaseBuilder {
        GetDatabaseBuilder::new(self.clone())
    }

    /// Delete the database
    pub fn delete_database(&self) -> DeleteDatabaseBuilder {
        DeleteDatabaseBuilder::new(self.clone())
    }

    /// List collections in the database
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
    pub fn list_users(&self) -> ListUsersBuilder {
        ListUsersBuilder::new(self.clone())
    }

    /// Convert into a [`CollectionClient`]
    pub fn collection<S: Into<ReadonlyString>>(&self, collection_name: S) -> CollectionClient {
        CollectionClient::new(self.clone(), collection_name)
    }

    /// Convert into a [`UserClient`]
    pub fn user<S: Into<ReadonlyString>>(&self, user_name: S) -> UserClient {
        UserClient::new(self.clone(), user_name)
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.client.pipeline()
    }
}
