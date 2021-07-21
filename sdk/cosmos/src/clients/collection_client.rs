use super::{DatabaseClient, UserDefinedFunctionClient};
use crate::clients::*;
use crate::operations::*;
use crate::requests;
use crate::resources::ResourceType;
use crate::ReadonlyString;
use azure_core::{pipeline::Pipeline, Context, HttpClient};
use serde::Serialize;

/// A client for Cosmos collection resources.
#[derive(Debug, Clone)]
pub struct CollectionClient {
    database_client: DatabaseClient,
    collection_name: ReadonlyString,
}

impl CollectionClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        database_client: DatabaseClient,
        collection_name: S,
    ) -> Self {
        Self {
            database_client,
            collection_name: collection_name.into(),
        }
    }

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.database_client.cosmos_client()
    }

    /// Get a [`DatabaseClient`].
    pub fn database_client(&self) -> &DatabaseClient {
        &self.database_client
    }

    /// Get the collection name
    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    /// Get a collection
    pub fn get_collection(&self) -> requests::GetCollectionBuilder<'_> {
        requests::GetCollectionBuilder::new(self)
    }

    /// Delete a collection
    pub fn delete_collection(&self) -> requests::DeleteCollectionBuilder<'_> {
        requests::DeleteCollectionBuilder::new(self)
    }

    /// Replace a collection
    pub fn replace_collection(&self) -> requests::ReplaceCollectionBuilder<'_, '_> {
        requests::ReplaceCollectionBuilder::new(self)
    }

    /// list documents in a collection
    pub fn list_documents(&self) -> requests::ListDocumentsBuilder<'_, '_> {
        requests::ListDocumentsBuilder::new(self)
    }

    /// create a document in a collection
    pub async fn create_document<D: Serialize>(
        &self,
        ctx: Context,
        document: &D,
        options: CreateDocumentOptions<'_>,
    ) -> Result<CreateDocumentResponse, crate::Error> {
        let request = self.prepare_request_with_collection_name(http::Method::POST);
        let mut request = request.body(bytes::Bytes::new()).unwrap().into();
        let mut ctx = ctx.clone();
        options.decorate_request(&mut request, document)?;
        let response = self
            .pipeline()
            .send(&mut ctx, &mut request)
            .await?
            .validate(http::StatusCode::CREATED)
            .await?;

        Ok(CreateDocumentResponse::try_from(response).await?)
    }

    /// query documents in a collection
    pub fn query_documents(&self) -> requests::QueryDocumentsBuilder<'_, '_> {
        requests::QueryDocumentsBuilder::new(self)
    }

    /// list stored procedures in a collection
    pub fn list_stored_procedures(&self) -> requests::ListStoredProceduresBuilder<'_, '_> {
        requests::ListStoredProceduresBuilder::new(self)
    }

    /// list user defined functions in a collection
    pub fn list_user_defined_functions(&self) -> requests::ListUserDefinedFunctionsBuilder<'_, '_> {
        requests::ListUserDefinedFunctionsBuilder::new(self)
    }

    /// list triggers in a collection
    pub fn list_triggers(&self) -> requests::ListTriggersBuilder<'_, '_> {
        requests::ListTriggersBuilder::new(self)
    }

    /// list the partition key ranges in a collection
    pub fn get_partition_key_ranges(&self) -> requests::GetPartitionKeyRangesBuilder<'_, '_> {
        requests::GetPartitionKeyRangesBuilder::new(self)
    }

    /// convert into a [`DocumentClient`]
    pub fn into_document_client<S: Into<String>, PK: Serialize>(
        self,
        document_name: S,
        partition_key: &PK,
    ) -> Result<DocumentClient, serde_json::Error> {
        DocumentClient::new(self, document_name, partition_key)
    }

    /// convert into a [`TriggerClient`]
    pub fn into_trigger_client<S: Into<ReadonlyString>>(self, trigger_name: S) -> TriggerClient {
        TriggerClient::new(self, trigger_name)
    }

    /// convert into a [`UserDefinedFunctionClient`]
    pub fn into_user_defined_function_client<S: Into<ReadonlyString>>(
        self,
        user_defined_function_name: S,
    ) -> UserDefinedFunctionClient {
        UserDefinedFunctionClient::new(self, user_defined_function_name)
    }

    /// convert into a [`StoredProcedureClient`]
    pub fn into_stored_procedure_client<S: Into<ReadonlyString>>(
        self,
        stored_procedure_name: S,
    ) -> StoredProcedureClient {
        StoredProcedureClient::new(self, stored_procedure_name)
    }

    pub(crate) fn prepare_request_with_collection_name(
        &self,
        method: http::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}",
                self.database_client().database_name(),
                self.collection_name()
            ),
            method,
            ResourceType::Collections,
        )
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }
}
