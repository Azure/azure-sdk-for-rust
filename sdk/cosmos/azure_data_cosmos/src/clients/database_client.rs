use crate::clients::ContainerClient;
use crate::models::DatabaseProperties;
use crate::options::ReadDatabaseOptions;
use crate::pipeline::ResourceType;
use crate::utils::WithAddedPathSegments;
use crate::CosmosClient;

use azure_core::{Context, Request};
use url::Url;

#[cfg(doc)]
use crate::CosmosClientMethods;

/// Defines the methods provided by a [`DatabaseClient`]
///
/// This trait is intended to allow you to mock out the `DatabaseClient` when testing your application.
/// Rather than depending on `DatabaseClient`, you can depend on a generic parameter constrained by this trait, or an `impl DatabaseClientMethods` type.
pub trait DatabaseClientMethods {
    /// Reads the properties of the database.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::sync::Arc;
    /// # async fn doc() {
    /// use azure_data_cosmos::{CosmosClient, CosmosClientMethods, clients::DatabaseClientMethods};
    ///
    /// let credential = Arc::new(azure_identity::DefaultAzureCredential::new().unwrap());
    /// let client = CosmosClient::new("https://myaccount.documents.azure.com/", credential, None).unwrap();
    /// let db_client = client.database_client("my_database");
    /// let response = db_client.read(None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap();
    /// # }
    /// ```
    #[allow(async_fn_in_trait)] // REASON: See https://github.com/Azure/azure-sdk-for-rust/issues/1796 for detailed justification
    async fn read(
        &self,
        options: Option<ReadDatabaseOptions>,
    ) -> azure_core::Result<azure_core::Response<DatabaseProperties>>;

    /// Gets a [`ContainerClient`] that can be used to access the collection with the specified name.
    ///
    /// # Arguments
    /// * `name` - The name of the container.
    fn container_client(&self, name: impl AsRef<str>) -> ContainerClient;
}

/// A client for working with a specific database in a Cosmos DB account.
///
/// You can get a `DatabaseClient` by calling [`CosmosClient::database_client()`](CosmosClient::database_client()).
pub struct DatabaseClient {
    base_url: Url,
    root_client: CosmosClient,
}

impl DatabaseClient {
    pub(crate) fn new(root_client: CosmosClient, database_id: &str) -> Self {
        let base_url = root_client
            .endpoint()
            .with_added_path_segments(vec!["dbs", database_id]);

        Self {
            base_url,
            root_client,
        }
    }
}

impl DatabaseClientMethods for DatabaseClient {
    async fn read(
        &self,

        #[allow(unused_variables)]
        // This is a documented public API so prefixing with '_' is undesirable.
        options: Option<ReadDatabaseOptions>,
    ) -> azure_core::Result<azure_core::Response<DatabaseProperties>> {
        let mut req = Request::new(self.base_url.clone(), azure_core::Method::Get);
        self.root_client
            .pipeline
            .send(Context::new(), &mut req, ResourceType::Databases)
            .await
    }

    fn container_client(&self, name: impl AsRef<str>) -> ContainerClient {
        ContainerClient::new(self.root_client.clone(), &self.base_url, name.as_ref())
    }
}
