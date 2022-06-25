use crate::clients::*;
use crate::operations::*;
use crate::ReadonlyString;
use azure_core::{Pipeline, Request};

/// A client for Cosmos user defined function resources.
#[derive(Debug, Clone)]
pub struct UserDefinedFunctionClient {
    collection: CollectionClient,
    user_defined_function_name: ReadonlyString,
}

impl UserDefinedFunctionClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        collection: CollectionClient,
        user_defined_function_name: S,
    ) -> Self {
        Self {
            collection,
            user_defined_function_name: user_defined_function_name.into(),
        }
    }

    /// Create the user defined function.
    pub fn create_user_defined_function<B>(
        &self,
        body: B,
    ) -> CreateOrReplaceUserDefinedFunctionBuilder
    where
        B: Into<String>,
    {
        CreateOrReplaceUserDefinedFunctionBuilder::new(self.clone(), true, body.into())
    }

    /// Replace the user defined function.
    pub fn replace_user_defined_function<B>(
        &self,
        body: B,
    ) -> CreateOrReplaceUserDefinedFunctionBuilder
    where
        B: Into<String>,
    {
        CreateOrReplaceUserDefinedFunctionBuilder::new(self.clone(), false, body.into())
    }

    /// Delete the user defined function.
    pub fn delete_user_defined_function(&self) -> DeleteUserDefinedFunctionBuilder {
        DeleteUserDefinedFunctionBuilder::new(self.clone())
    }

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection.cosmos_client()
    }

    /// Get a [`DatabaseClient`].
    pub fn database_client(&self) -> &DatabaseClient {
        self.collection.database_client()
    }

    /// Get a [`CollectionClient`].
    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection
    }

    /// Get the user defined function's name.
    pub fn user_defined_function_name(&self) -> &str {
        &self.user_defined_function_name
    }

    pub(crate) fn udfs_request(&self, method: azure_core::Method) -> Request {
        self.cosmos_client().request(
            &format!(
                "dbs/{}/colls/{}/udfs",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
            ),
            method,
        )
    }

    pub(crate) fn udf_request(&self, method: azure_core::Method) -> Request {
        self.cosmos_client().request(
            &format!(
                "dbs/{}/colls/{}/udfs/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.user_defined_function_name()
            ),
            method,
        )
    }

    /// Get a [`Pipeline`]
    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }
}
