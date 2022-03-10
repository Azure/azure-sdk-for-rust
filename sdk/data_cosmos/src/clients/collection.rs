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

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.database.cosmos_client()
    }

    /// Get a [`DatabaseClient`].
    pub fn database_client(&self) -> &DatabaseClient {
        &self.database
    }

    /// Get the collection name
    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    /// Get a collection
    pub fn get_collection(&self) -> GetCollectionBuilder {
        GetCollectionBuilder::new(self.clone())
    }

    /// Delete a collection
    pub fn delete_collection(&self) -> DeleteCollectionBuilder {
        DeleteCollectionBuilder::new(self.clone())
    }

    /// Replace a collection
    pub fn replace_collection<P: Into<PartitionKey>>(
        &self,
        partition_key: P,
    ) -> ReplaceCollectionBuilder {
        ReplaceCollectionBuilder::new(self.clone(), partition_key.into())
    }

    /// list documents in a collection
    pub fn list_documents(&self) -> ListDocumentsBuilder {
        ListDocumentsBuilder::new(self.clone())
    }

    /// create a document in a collection
    pub fn create_document<D: Serialize + CosmosEntity + Send + 'static>(
        &self,
        document: D,
    ) -> CreateDocumentBuilder<D> {
        CreateDocumentBuilder::new(self.clone(), document)
    }

    /// query documents in a collection
    pub fn query_documents<Q: Into<Query>>(&self, query: Q) -> QueryDocumentsBuilder {
        QueryDocumentsBuilder::new(self.clone(), query.into())
    }

    /// list stored procedures in a collection
    pub fn list_stored_procedures(&self) -> ListStoredProceduresBuilder {
        ListStoredProceduresBuilder::new(self.clone())
    }

    /// list user defined functions in a collection
    pub fn list_user_defined_functions(&self) -> ListUserDefinedFunctionsBuilder {
        ListUserDefinedFunctionsBuilder::new(self.clone())
    }

    /// list triggers in a collection
    pub fn list_triggers(&self) -> ListTriggersBuilder {
        ListTriggersBuilder::new(self.clone())
    }

    /// list the partition key ranges in a collection
    pub fn get_partition_key_ranges(&self) -> GetPartitionKeyRangesBuilder {
        GetPartitionKeyRangesBuilder::new(self.clone())
    }

    /// convert into a [`DocumentClient`]
    pub fn document<S: Into<String>, PK: Serialize>(
        &self,
        document_name: S,
        partition_key: &PK,
    ) -> Result<DocumentClient, serde_json::Error> {
        DocumentClient::new(self.clone(), document_name, partition_key)
    }

    /// convert into a [`TriggerClient`]
    pub fn trigger<S: Into<ReadonlyString>>(&self, trigger_name: S) -> TriggerClient {
        TriggerClient::new(self.clone(), trigger_name)
    }

    /// convert into a [`UserDefinedFunctionClient`]
    pub fn user_defined_function<S: Into<ReadonlyString>>(
        &self,
        user_defined_function_name: S,
    ) -> UserDefinedFunctionClient {
        UserDefinedFunctionClient::new(self.clone(), user_defined_function_name)
    }

    /// convert into a [`StoredProcedureClient`]
    pub fn stored_procedure<S: Into<ReadonlyString>>(
        &self,
        stored_procedure_name: S,
    ) -> StoredProcedureClient {
        StoredProcedureClient::new(self.clone(), stored_procedure_name)
    }

    pub(crate) fn prepare_request_with_collection_name(
        &self,
        http_method: http::Method,
    ) -> Request {
        let path = &format!(
            "dbs/{}/colls/{}",
            self.database_client().database_name(),
            self.collection_name()
        );
        self.cosmos_client()
            .prepare_request_pipeline(path, http_method)
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }

    pub(crate) fn prepare_doc_request_pipeline(&self, http_method: http::Method) -> Request {
        let path = &format!(
            "dbs/{}/colls/{}/docs",
            self.database_client().database_name(),
            self.collection_name()
        );
        self.cosmos_client()
            .prepare_request_pipeline(path, http_method)
    }
}
