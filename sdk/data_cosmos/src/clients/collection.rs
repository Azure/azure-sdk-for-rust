use super::{DatabaseClient, UserDefinedFunctionClient};
use crate::clients::*;
use crate::operations::*;
use crate::resources::collection::PartitionKey;
use crate::resources::document::Query;
use crate::CosmosEntity;
use crate::ReadonlyString;
use azure_core::{Pipeline, Request};
use serde::Serialize;

/// A client for Cosmos collection resources.
#[derive(Debug, Clone)]
pub struct CollectionClient {
    database: DatabaseClient,
    collection_name: ReadonlyString,
}

impl CollectionClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        database: DatabaseClient,
        collection_name: S,
    ) -> Self {
        Self {
            database,
            collection_name: collection_name.into(),
        }
    }

    /// Get a collection.
    pub fn get_collection(&self) -> GetCollectionBuilder {
        GetCollectionBuilder::new(self.clone())
    }

    /// Delete a collection.
    pub fn delete_collection(&self) -> DeleteCollectionBuilder {
        DeleteCollectionBuilder::new(self.clone())
    }

    /// Replace a collection.
    pub fn replace_collection<P: Into<PartitionKey>>(
        &self,
        partition_key: P,
    ) -> ReplaceCollectionBuilder {
        ReplaceCollectionBuilder::new(self.clone(), partition_key.into())
    }

    /// List documents in a collection.
    pub fn list_documents(&self) -> ListDocumentsBuilder {
        ListDocumentsBuilder::new(self.clone())
    }

    /// Create a document in a collection.
    pub fn create_document<D: Serialize + CosmosEntity + Send + 'static>(
        &self,
        document: D,
    ) -> CreateDocumentBuilder<D> {
        CreateDocumentBuilder::new(self.clone(), document)
    }

    /// Query documents in a collection.
    pub fn query_documents<Q: Into<Query>>(&self, query: Q) -> QueryDocumentsBuilder {
        QueryDocumentsBuilder::new(self.clone(), query.into())
    }

    /// List stored procedures in a collection.
    pub fn list_stored_procedures(&self) -> ListStoredProceduresBuilder {
        ListStoredProceduresBuilder::new(self.clone())
    }

    /// List user defined functions in a collection.
    pub fn list_user_defined_functions(&self) -> ListUserDefinedFunctionsBuilder {
        ListUserDefinedFunctionsBuilder::new(self.clone())
    }

    /// List triggers in a collection.
    pub fn list_triggers(&self) -> ListTriggersBuilder {
        ListTriggersBuilder::new(self.clone())
    }

    /// List the partition key ranges in a collection.
    pub fn get_partition_key_ranges(&self) -> GetPartitionKeyRangesBuilder {
        GetPartitionKeyRangesBuilder::new(self.clone())
    }

    /// Convert into a [`DocumentClient`].
    pub fn document_client<S: Into<String>, PK: Serialize>(
        &self,
        document_name: S,
        partition_key: &PK,
    ) -> azure_core::Result<DocumentClient> {
        DocumentClient::new(self.clone(), document_name, partition_key)
    }

    /// Convert into a [`TriggerClient`].
    pub fn trigger_client<S: Into<ReadonlyString>>(&self, trigger_name: S) -> TriggerClient {
        TriggerClient::new(self.clone(), trigger_name)
    }

    /// Convert into a [`UserDefinedFunctionClient`].
    pub fn user_defined_function_client<S: Into<ReadonlyString>>(
        &self,
        user_defined_function_name: S,
    ) -> UserDefinedFunctionClient {
        UserDefinedFunctionClient::new(self.clone(), user_defined_function_name)
    }

    /// Convert into a [`StoredProcedureClient`].
    pub fn stored_procedure_client<S: Into<ReadonlyString>>(
        &self,
        stored_procedure_name: S,
    ) -> StoredProcedureClient {
        StoredProcedureClient::new(self.clone(), stored_procedure_name)
    }

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.database.cosmos_client()
    }

    /// Get a [`DatabaseClient`].
    pub fn database_client(&self) -> &DatabaseClient {
        &self.database
    }

    /// Get the collection name.
    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub(crate) fn collection_request(&self, http_method: http::Method) -> Request {
        let path = &format!(
            "dbs/{}/colls/{}",
            self.database_client().database_name(),
            self.collection_name()
        );
        self.cosmos_client().request(path, http_method)
    }

    pub(crate) fn docs_request(&self, http_method: http::Method) -> Request {
        let path = &format!(
            "dbs/{}/colls/{}/docs",
            self.database_client().database_name(),
            self.collection_name()
        );
        self.cosmos_client().request(path, http_method)
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }
}
