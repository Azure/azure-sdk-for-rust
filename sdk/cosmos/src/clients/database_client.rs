use super::*;
use crate::operations::*;
use crate::requests;
use crate::resources::ResourceType;
use crate::CosmosError;
use crate::ReadonlyString;
use azure_core::pipeline::Pipeline;
use azure_core::{Context, HttpClient, Request};

/// A client for Cosmos database resources.
#[derive(Debug, Clone)]
pub struct DatabaseClient {
    cosmos_client: CosmosClient,
    database_name: ReadonlyString,
}

impl DatabaseClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        cosmos_client: CosmosClient,
        database_name: S,
    ) -> Self {
        Self {
            cosmos_client,
            database_name: database_name.into(),
        }
    }

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        &self.cosmos_client
    }

    /// Get the database's name
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Get the database
    pub fn get_database(&self) -> requests::GetDatabaseBuilder<'_, '_> {
        requests::GetDatabaseBuilder::new(self)
    }

    /// List collections in the database
    pub fn list_collections(&self) -> requests::ListCollectionsBuilder<'_> {
        requests::ListCollectionsBuilder::new(self)
    }

    /// Delete the database
    pub fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_> {
        requests::DeleteDatabaseBuilder::new(self)
    }

    /// Create a collection
    pub async fn create_collection<S: AsRef<str>>(
        &self,
        ctx: Context,
        collection_name: S,
        options: CreateCollectionOptions,
    ) -> Result<CreateCollectionResponse, CosmosError> {
        let request = self.cosmos_client().prepare_request(
            &format!("dbs/{}/colls", self.database_name()),
            http::Method::POST,
            ResourceType::Collections,
        );
        let mut request: Request = request.body(bytes::Bytes::new()).unwrap().into();

        let mut ctx = ctx.clone();
        options.decorate_request(&mut request, collection_name.as_ref())?;
        let response = self
            .pipeline()
            .send(&mut ctx, &mut request)
            .await
            .map_err(CosmosError::PolicyError)?;

        Ok(CreateCollectionResponse::try_from(response).await?)
    }

    /// List users
    pub fn list_users(&self) -> requests::ListUsersBuilder<'_, '_> {
        requests::ListUsersBuilder::new(self)
    }

    /// Convert into a [`CollectionClient`]
    pub fn into_collection_client<S: Into<ReadonlyString>>(
        self,
        collection_name: S,
    ) -> CollectionClient {
        CollectionClient::new(self, collection_name)
    }

    /// Convert into a [`UserClient`]
    pub fn into_user_client<S: Into<ReadonlyString>>(self, user_name: S) -> UserClient {
        UserClient::new(self, user_name)
    }

    pub(crate) fn prepare_request_with_database_name(
        &self,
        method: http::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!("dbs/{}", self.database_name()),
            method,
            ResourceType::Databases,
        )
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }

    fn pipeline(&self) -> &Pipeline {
        self.cosmos_client.pipeline()
    }
}
