use super::*;
use crate::requests;
use crate::{ReadonlyString, ResourceType};
use azure_core::No;

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

    pub fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.collection_client.hyper_client()
    }

    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection_client.cosmos_client()
    }

    pub fn database_client(&self) -> &DatabaseClient {
        self.collection_client.database_client()
    }

    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection_client
    }

    pub fn user_defined_function_name(&self) -> &str {
        &self.user_defined_function_name
    }

    pub fn create_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_, No> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, true)
    }

    pub fn replace_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_, No> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, false)
    }

    pub fn delete_user_defined_function(
        &self,
    ) -> requests::DeleteUserDefinedFunctionBuilder<'_, '_> {
        requests::DeleteUserDefinedFunctionBuilder::new(self)
    }

    pub fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
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

    pub fn prepare_request_with_user_defined_function_name(
        &self,
        method: hyper::Method,
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
}
