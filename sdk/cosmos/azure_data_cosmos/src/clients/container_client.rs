use crate::{
    models::ContainerProperties, pipeline::ResourceType, CosmosClient, ReadContainerOptions,
};

use azure_core::{Context, Request};
use url::Url;

#[cfg(doc)]
use crate::clients::DatabaseClientMethods;

/// Defines the methods provided by a [`ContainerClient`]
///
/// This trait is intended to allow you to mock out the `ContainerClient` when testing your application.
/// Rather than depending on `ContainerClient`, you can depend on a generic parameter constrained by this trait, or an `impl ContainerClientMethods` type.
pub trait ContainerClientMethods {
    /// Reads the properties of the container.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn doc() {
    /// use azure_data_cosmos::{CosmosClient, CosmosClientMethods, clients::DatabaseClientMethods, clients::ContainerClientMethods};
    ///
    /// let credential = azure_identity::create_default_credential().unwrap();
    /// let client = CosmosClient::new("https://myaccount.documents.azure.com/", credential, None).unwrap();
    /// let db_client = client.database_client("my_database");
    /// let container_client = client.container_client("my_container");
    /// let response = container_client.read(None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap();
    /// # }
    /// ```
    #[allow(async_fn_in_trait)] // REASON: See https://github.com/Azure/azure-sdk-for-rust/issues/1796 for detailed justification
    async fn read(
        &self,
        options: Option<ReadContainerOptions>,
    ) -> azure_core::Result<azure_core::Response<ContainerProperties>>;
}

/// A client for working with a specific container in a Cosmos DB account.
///
/// You can get a `Container` by calling [`DatabaseClient::container_client()`](DatabaseClient::container_client()).
pub struct ContainerClient {
    base_url: Url,
    root_client: CosmosClient,
}

impl ContainerClient {
    pub(crate) fn new(root_client: CosmosClient, database_url: &Url, container_name: &str) -> Self {
        let base_url = {
            let mut u = database_url.clone();
            {
                let mut segments = u
                    .path_segments_mut()
                    .expect("The root client should have validated the format of the URL");
                segments.push("colls");
                segments.push(container_name);
            }
            u
        };

        Self {
            base_url,
            root_client,
        }
    }
}

impl ContainerClientMethods for ContainerClient {
    async fn read(
        &self,

        #[allow(unused_variables)]
        // This is a documented public API so prefixing with '_' is undesirable.
        options: Option<ReadContainerOptions>,
    ) -> azure_core::Result<azure_core::Response<ContainerProperties>> {
        let mut req = Request::new(self.base_url.clone(), azure_core::Method::Get);
        self.root_client
            .pipeline
            .send(Context::new(), &mut req, ResourceType::Containers)
            .await
    }
}
