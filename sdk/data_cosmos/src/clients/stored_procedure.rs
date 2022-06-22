use super::*;
use crate::prelude::*;
use crate::ReadonlyString;
use azure_core::{Pipeline, Request};

/// A client for Cosmos stored procedure resources.
#[derive(Debug, Clone)]
pub struct StoredProcedureClient {
    collection: CollectionClient,
    stored_procedure_name: ReadonlyString,
}

impl StoredProcedureClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        collection: CollectionClient,
        stored_procedure_name: S,
    ) -> Self {
        Self {
            collection,
            stored_procedure_name: stored_procedure_name.into(),
        }
    }

    /// Get a [`CosmosClient`]
    #[must_use]
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection.cosmos_client()
    }

    /// Get a [`DatabaseClient`]
    #[must_use]
    pub fn database_client(&self) -> &DatabaseClient {
        self.collection.database_client()
    }

    /// Get the [`CollectionClient`]
    #[must_use]
    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection
    }

    /// Get the stored procedure's name
    #[must_use]
    pub fn stored_procedure_name(&self) -> &str {
        &self.stored_procedure_name
    }

    /// Create the stored procedure
    pub fn create_stored_procedure<S: Into<String>>(
        &self,
        function_body: S,
    ) -> CreateStoredProcedureBuilder {
        CreateStoredProcedureBuilder::new(self.clone(), function_body.into())
    }

    /// Replace the stored procedure
    pub fn replace_stored_procedure<S: Into<String>>(
        &self,
        function_body: S,
    ) -> ReplaceStoredProcedureBuilder {
        ReplaceStoredProcedureBuilder::new(self.clone(), function_body.into())
    }

    /// Execute the stored procedure
    #[must_use]
    pub fn execute_stored_procedure(&self) -> ExecuteStoredProcedureBuilder {
        ExecuteStoredProcedureBuilder::new(self.clone())
    }

    /// Delete the stored procedure
    #[must_use]
    pub fn delete_stored_procedure(&self) -> DeleteStoredProcedureBuilder {
        DeleteStoredProcedureBuilder::new(self.clone())
    }

    pub(crate) fn prepare_pipeline_with_stored_procedure_name(
        &self,
        method: http::Method,
    ) -> Request {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/sprocs/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.stored_procedure_name()
            ),
            method,
        )
    }

    pub(crate) fn prepare_request_pipeline(&self, method: http::Method) -> Request {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/sprocs",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
            ),
            method,
        )
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }
}
