use crate::clients::*;
use crate::prelude::*;
use crate::ReadonlyString;
use azure_core::{Pipeline, Request};
use serde::de::DeserializeOwned;

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

    /// Create the stored procedure.
    pub fn create_stored_procedure<S: Into<String>>(
        &self,
        function_body: S,
    ) -> CreateStoredProcedureBuilder {
        CreateStoredProcedureBuilder::new(self.clone(), function_body.into())
    }

    /// Replace the stored procedure.
    pub fn replace_stored_procedure<S: Into<String>>(
        &self,
        function_body: S,
    ) -> ReplaceStoredProcedureBuilder {
        ReplaceStoredProcedureBuilder::new(self.clone(), function_body.into())
    }

    /// Execute the stored procedure.
    pub fn execute_stored_procedure<T: DeserializeOwned + Send>(
        &self,
    ) -> ExecuteStoredProcedureBuilder<T> {
        ExecuteStoredProcedureBuilder::new(self.clone())
    }

    /// Delete the stored procedure.
    pub fn delete_stored_procedure(&self) -> DeleteStoredProcedureBuilder {
        DeleteStoredProcedureBuilder::new(self.clone())
    }

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection.cosmos_client()
    }

    /// Get a [`DatabaseClient`].
    pub fn database_client(&self) -> &DatabaseClient {
        self.collection.database_client()
    }

    /// Get the [`CollectionClient`].
    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection
    }

    /// Get the stored procedure's name.
    pub fn stored_procedure_name(&self) -> &str {
        &self.stored_procedure_name
    }

    pub(crate) fn stored_procedure_request(&self, method: azure_core::Method) -> Request {
        self.cosmos_client().request(
            &format!(
                "dbs/{}/colls/{}/sprocs/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.stored_procedure_name()
            ),
            method,
        )
    }

    pub(crate) fn stored_procedures_request(&self, method: azure_core::Method) -> Request {
        self.cosmos_client().request(
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
