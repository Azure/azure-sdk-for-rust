use super::*;
use crate::operations::*;
use crate::resources::ResourceType;
use crate::{requests, ReadonlyString};
use azure_core::{HttpClient, Pipeline, Request};

/// A client for Cosmos user defined function resources.
#[derive(Debug, Clone)]
pub struct UserDefinedFunctionClient {
    collection_client: CollectionClient,
    user_defined_function_name: ReadonlyString,
}

impl UserDefinedFunctionClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        collection_client: CollectionClient,
        user_defined_function_name: S,
    ) -> Self {
        Self {
            collection_client,
            user_defined_function_name: user_defined_function_name.into(),
        }
    }

    /// Get a [`CosmosClient`]
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection_client.cosmos_client()
    }

    /// Get a [`DatabaseClient`]
    pub fn database_client(&self) -> &DatabaseClient {
        self.collection_client.database_client()
    }

    /// Get a [`CollectionClient`]
    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection_client
    }

    /// Get the user defined function's name
    pub fn user_defined_function_name(&self) -> &str {
        &self.user_defined_function_name
    }

    /// Create the user defined function
    pub fn create_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, true)
    }

    /// Replace the user defined function
    pub fn replace_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, false)
    }

    /// Delete the user defined function
    pub fn delete_user_defined_function(&self) -> DeleteUserDefinedFunctionBuilder {
        DeleteUserDefinedFunctionBuilder::new(self.clone())
    }

    pub(crate) fn prepare_request(&self, method: http::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/udfs",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
            ),
            method,
            ResourceType::UserDefinedFunctions,
        )
    }

    pub(crate) fn prepare_request_with_user_defined_function_name(
        &self,
        method: http::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/udfs/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.user_defined_function_name()
            ),
            method,
            ResourceType::UserDefinedFunctions,
        )
    }

    pub(crate) fn prepare_pipeline_with_user_defined_function_name(
        &self,
        method: http::Method,
    ) -> Request {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/udfs/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.user_defined_function_name()
            ),
            method,
        )
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }

    /// Get a [`Pipeline`]
    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }
}
