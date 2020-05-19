use crate::clients::{Client, CollectionClient, CosmosUriBuilder, ResourceType};
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use crate::requests;
use crate::user_defined_function::UserDefinedFunctionName;
use crate::{CollectionTrait, UserDefinedFunctionBuilderTrait, UserDefinedFunctionTrait};
use azure_sdk_core::No;

#[derive(Debug, Clone)]
pub struct UserDefinedFunctionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    user_defined_function_name: &'a dyn UserDefinedFunctionName,
}

impl<'a, CUB> UserDefinedFunctionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
        user_defined_function_name: &'a dyn UserDefinedFunctionName,
    ) -> Self {
        UserDefinedFunctionClient {
            collection_client,
            user_defined_function_name,
        }
    }

    pub(crate) fn main_client(&self) -> &Client<CUB> {
        self.collection_client.main_client()
    }

    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.main_client().hyper_client()
    }
}

impl<'a, CUB> UserDefinedFunctionTrait<'a, CUB> for UserDefinedFunctionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.collection_client.database_name()
    }

    fn collection_name(&self) -> &'a dyn CollectionName {
        self.collection_client.collection_name()
    }

    fn user_defined_function_name(&self) -> &'a dyn UserDefinedFunctionName {
        self.user_defined_function_name
    }

    fn create_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, CUB, No> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, true)
    }

    fn replace_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, CUB, No> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, false)
    }

    fn delete_user_defined_function(&self) -> requests::DeleteUserDefinedFunctionBuilder<'_, CUB> {
        requests::DeleteUserDefinedFunctionBuilder::new(self)
    }
}

impl<'a, CUB> UserDefinedFunctionBuilderTrait<'a, CUB> for UserDefinedFunctionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(
        &self,
        method: hyper::Method,
        specify_user_defined_function_name: bool,
    ) -> http::request::Builder {
        if specify_user_defined_function_name {
            self.main_client().prepare_request(
                &format!(
                    "dbs/{}/colls/{}/udfs/{}",
                    self.database_name().name(),
                    self.collection_name().name(),
                    self.user_defined_function_name().name()
                ),
                method,
                ResourceType::UserDefinedFunctions,
            )
        } else {
            self.main_client().prepare_request(
                &format!(
                    "dbs/{}/colls/{}/udfs",
                    self.database_name().name(),
                    self.collection_name().name(),
                ),
                method,
                ResourceType::UserDefinedFunctions,
            )
        }
    }
}
