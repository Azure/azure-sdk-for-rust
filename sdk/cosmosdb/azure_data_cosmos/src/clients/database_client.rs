use crate::authorization_policy::ResourceType;
use crate::models::DatabaseProperties;
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
    /// # async fn doc() {
    /// use azure_data_cosmos::{CosmosClient, CosmosClientMethods, DatabaseClientMethods};
    ///
    /// let credential = azure_identity::create_default_credential().unwrap();
    /// let client = CosmosClient::new("https://myaccount.documents.azure.com/", credential, None).unwrap();
    /// let db_client = client.database("my_database");
    /// let response = db_client.read(None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap();
    /// # }
    /// ```
    fn read(
        &self,
        options: Option<ReadDatabaseOptions>,
    ) -> impl std::future::Future<Output = azure_core::Result<azure_core::Response<DatabaseProperties>>>;
}

/// A client for working with a specific database in a Cosmos account.
///
/// You can get a `DatabaseClient` by calling [`CosmosClient::database()`](CosmosClient::database).
pub struct DatabaseClient {
    base_url: Url,
    root_client: CosmosClient,
}

impl DatabaseClient {
    pub(crate) fn new(root_client: CosmosClient, database_id: String) -> Self {
        let base_url = {
            let mut u = root_client.endpoint().clone();
            {
                let mut segments = u
                    .path_segments_mut()
                    .expect("The root client should have validated the format of the URL");
                segments.push("dbs");
                segments.push(&database_id);
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
        let ctx = Context::new().with_value(ResourceType::Databases);
        self.root_client.pipeline.send(&ctx, &mut req).await
    }
}
