use crate::{
    constants,
    models::{ContainerProperties, QueryResults},
    pipeline::ResourceType,
    utils::WithAddedPathSegments,
    CosmosClient, PartitionKey, Query, QueryItemsOptions, ReadContainerOptions,
};

use azure_core::{headers::HeaderValue, Context, Request};
use serde::{de::DeserializeOwned, Deserialize};
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
    /// let container_client = db_client.container_client("my_container");
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

    /// Executes a single-partition query against items in the container.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to execute.
    /// * `partition_key` - The partition key to scope the query on.
    /// * `options` - Optional parameters for the request.
    fn query_items<T: DeserializeOwned + Send>(
        &self,
        query: impl Into<Query>,
        partition_key: impl Into<PartitionKey>,
        options: Option<QueryItemsOptions>,
    ) -> azure_core::Result<azure_core::Pageable<QueryResults<T>, azure_core::Error>>;
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
        let base_url = database_url.with_added_path_segments(vec!["colls", container_name]);

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

    fn query_items<T: DeserializeOwned + Send>(
        &self,
        query: impl Into<Query>,
        partition_key: impl Into<PartitionKey>,

        #[allow(unused_variables)]
        // This is a documented public API so prefixing with '_' is undesirable.
        options: Option<QueryItemsOptions>,
    ) -> azure_core::Result<azure_core::Pageable<QueryResults<T>, azure_core::Error>> {
        // Represents the raw response model from the server.
        // We'll use this to deserialize the response body and then convert it to a more user-friendly model.
        #[derive(Deserialize)]
        struct QueryResponseModel<M> {
            #[serde(rename = "Documents")]
            documents: Vec<M>,
        }

        // We have to manually implement Model, because the derive macro doesn't support auto-inferring type and lifetime bounds.
        // See https://github.com/Azure/azure-sdk-for-rust/issues/1803
        impl<M: DeserializeOwned> azure_core::Model for QueryResponseModel<M> {
            async fn from_response_body(
                body: azure_core::ResponseBody,
            ) -> typespec_client_core::Result<Self> {
                body.json().await
            }
        }

        let url = self.base_url.with_added_path_segments(vec!["docs"]);
        let mut base_req = Request::new(url, azure_core::Method::Post);

        base_req.insert_header(constants::QUERY, "True");
        base_req.add_mandatory_header(&constants::QUERY_CONTENT_TYPE);
        base_req.insert_header(
            constants::PARTITION_KEY,
            HeaderValue::from_cow(partition_key.into().into_header_value()?),
        );
        base_req.set_json(&query.into())?;

        Ok(azure_core::Pageable::new(move |continuation| {
            let mut req = base_req.clone();
            async move {
                if let Some(continuation) = continuation {
                    req.insert_header(constants::CONTINUATION, continuation);
                }

                let resp = self
                    .root_client
                    .pipeline
                    .send(Context::new(), &mut req, ResourceType::Items)
                    .await?;

                let query_metrics = resp
                    .headers()
                    .get_optional_string(&constants::QUERY_METRICS);
                let index_metrics = resp
                    .headers()
                    .get_optional_string(&constants::INDEX_METRICS);
                let continuation_token =
                    resp.headers().get_optional_string(&constants::CONTINUATION);

                let query_response: QueryResponseModel<T> = resp.deserialize_body().await?;

                let query_results = QueryResults {
                    items: query_response.documents,
                    query_metrics,
                    index_metrics,
                    continuation_token,
                };

                Ok(query_results)
            }
        }))
    }
}
