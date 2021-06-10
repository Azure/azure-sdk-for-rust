use super::*;
use crate::operations::*;
use crate::resources::ResourceType;
use crate::{requests, ReadonlyString};

use azure_core::pipeline::Pipeline;
use azure_core::{Context, HttpClient};

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
    pub async fn get_database(
        &self,
        mut ctx: Context,
        options: GetDatabaseOptions,
    ) -> Result<GetDatabaseResponse, crate::Error> {
        let mut request = self
            .prepare_request_with_database_name(http::Method::GET)
            .body(bytes::Bytes::new())
            .unwrap()
            .into();
        options.decorate_request(&mut request)?;
        let response = self
            .pipeline()
            .send(&mut ctx, &mut request)
            .await
            .map_err(crate::Error::PolicyError)?
            .validate(http::StatusCode::OK)
            .await?;

        Ok(GetDatabaseResponse::try_from(response).await?)
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
        mut ctx: Context,
        collection_name: S,
        options: CreateCollectionOptions,
    ) -> Result<CreateCollectionResponse, crate::Error> {
        let mut request = self.cosmos_client().prepare_request2(
            &format!("dbs/{}/colls", self.database_name()),
            http::Method::POST,
            ResourceType::Collections,
        );
        options.decorate_request(&mut request, collection_name.as_ref())?;
        let response = self
            .pipeline()
            .send(&mut ctx, &mut request)
            .await
            .map_err(crate::Error::PolicyError)?
            .validate(http::StatusCode::CREATED)
            .await?;

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
