use crate::clients::ContainerClient;
use crate::models::DatabaseProperties;
use crate::pipeline::ResourceType;
use crate::{CosmosClient, ReadDatabaseOptions};

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

    /// Gets a [`CollectionClient`] that can be used to access the collection with the specified name.
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
        let base_url = {
            let mut u = root_client.endpoint().clone();
            {
                let mut segments = u
                    .path_segments_mut()
                    .expect("The root client should have validated the format of the URL");
                segments.push("dbs");
                segments.push(database_id);
            }
            u
        };

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
