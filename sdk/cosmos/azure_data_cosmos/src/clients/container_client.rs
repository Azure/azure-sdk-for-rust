// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    constants,
    models::{ContainerProperties, QueryResults},
    options::{QueryOptions, ReadContainerOptions},
    pipeline::{CosmosPipeline, ResourceType},
    utils::AppendPathSegments,
    Query, QueryPartitionStrategy,
};

use azure_core::{Context, Request};
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
    /// # use azure_data_cosmos::clients::{ContainerClient, ContainerClientMethods};
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// let response = container_client.read(None).await.unwrap();
    /// # }
    /// ```
    #[allow(async_fn_in_trait)] // REASON: See https://github.com/Azure/azure-sdk-for-rust/issues/1796 for detailed justification
    fn read(
        &self,
        options: Option<ReadContainerOptions>,
    ) -> azure_core::ResponseFuture<ContainerProperties>;

    /// Executes a single-partition query against items in the container.
    ///
    /// The resulting document will be deserialized into the type provided as `T`.
    /// If you want to deserialize the document to a direct representation of the JSON returned, use [`serde_json::Value`] as the target type.
    ///
    /// We recommend using ["turbofish" syntax](https://doc.rust-lang.org/book/appendix-02-operators.html#:~:text=turbofish) (`query_items::<SomeTargetType>(...)`) to specify the target type, as it makes type inference easier.
    ///
    /// **NOTE:** Currently, the Azure Cosmos DB SDK for Rust only supports single-partition querying. Cross-partition queries may be supported in the future.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to execute.
    /// * `partition_key_strategy` - The partition key to scope the query on.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// The `query` and `partition_key_strategy` parameters accept anything that can be transformed [`Into`] their relevant types.
    /// This allows simple queries without parameters to be expressed easily:
    ///
    /// ```rust,no_run
    /// # async fn doc() {
    /// # use azure_data_cosmos::clients::{ContainerClient, ContainerClientMethods};
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// #[derive(serde::Deserialize)]
    /// struct Customer {
    ///     id: u64,
    ///     name: String,
    /// }
    /// let items = container_client.query_items::<Customer>(
    ///     "SELECT * FROM c",
    ///     "some_partition_key",
    ///     None).unwrap();
    /// # }
    /// ```
    ///
    /// You can specify parameters by using [`Query::from()`] and [`Query::with_parameter()`]:
    ///
    /// ```rust,no_run
    /// # async fn doc() {
    /// # use azure_data_cosmos::{Query, clients::{ContainerClient, ContainerClientMethods}};
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// #[derive(serde::Deserialize)]
    /// struct Customer {
    ///     id: u64,
    ///     name: String,
    /// }
    /// let query = Query::from("SELECT COUNT(*) FROM c WHERE c.customer_id = @customer_id")
    ///     .with_parameter("@customer_id", 42).unwrap();
    /// let items = container_client.query_items::<Customer>(query, "some_partition_key", None).unwrap();
    /// # }
    /// ```
    ///
    /// See [`PartitionKey`](crate::PartitionKey) for more information on how to specify a partition key, and [`Query`] for more information on how to specify a query.
    fn query_items<T: DeserializeOwned + Send>(
        &self,
        query: impl Into<Query>,
        partition_key: impl Into<QueryPartitionStrategy>,
        options: Option<QueryOptions>,
    ) -> azure_core::Result<azure_core::Pageable<QueryResults<T>, azure_core::Error>>;
}

/// A client for working with a specific container in a Cosmos DB account.
///
/// You can get a `Container` by calling [`DatabaseClient::container_client()`](crate::clients::DatabaseClient::container_client()).
pub struct ContainerClient {
    container_url: Url,
    pipeline: CosmosPipeline,
}

impl ContainerClient {
    pub(crate) fn new(pipeline: CosmosPipeline, database_url: &Url, container_name: &str) -> Self {
        let mut container_url = database_url.clone();
        container_url.append_path_segments(["colls", container_name]);

        Self {
            container_url,
            pipeline,
        }
    }
}

impl ContainerClientMethods for ContainerClient {
    fn read(
        &self,

        #[allow(unused_variables)]
        // This is a documented public API so prefixing with '_' is undesirable.
        options: Option<ReadContainerOptions>,
    ) -> azure_core::ResponseFuture<ContainerProperties> {
        let req = Request::new(self.container_url.clone(), azure_core::Method::Get);
        self.pipeline
            .send(Context::new(), req, ResourceType::Containers)
    }

    fn query_items<T: DeserializeOwned + Send>(
        &self,
        query: impl Into<Query>,
        partition_key: impl Into<QueryPartitionStrategy>,

        #[allow(unused_variables)]
        // This is a documented public API so prefixing with '_' is undesirable.
        options: Option<QueryOptions>,
    ) -> azure_core::Result<azure_core::Pageable<QueryResults<T>, azure_core::Error>> {
        // Represents the raw response model from the server.
        // We'll use this to deserialize the response body and then convert it to a more user-friendly model.
        #[derive(Deserialize)]
        struct QueryResponseModel<M> {
            #[serde(rename = "Documents")]
            documents: Vec<M>,
        }

        let mut url = self.container_url.clone();
        url.append_path_segments(["docs"]);
        let mut base_req = Request::new(url, azure_core::Method::Post);

        base_req.insert_header(constants::QUERY, "True");
        base_req.add_mandatory_header(&constants::QUERY_CONTENT_TYPE);

        let QueryPartitionStrategy::SinglePartition(partition_key) = partition_key.into();
        base_req.insert_headers(&partition_key)?;

        base_req.set_json(&query.into())?;

        // We have to double-clone here.
        // First we clone the pipeline to pass it in to the closure
        let pipeline = self.pipeline.clone();
        Ok(azure_core::Pageable::new(move |continuation| {
            // Then we have to clone it again to pass it in to the async block.
            // This is because Pageable can't borrow any data, it has to own it all.
            // That's probably good, because it means a Pageable can outlive the client that produced it, but it requires some extra cloning.
            let pipeline = pipeline.clone();
            let mut req = base_req.clone();
            async move {
                if let Some(continuation) = continuation {
                    req.insert_header(constants::CONTINUATION, continuation);
                }

                let resp = pipeline
                    .send(Context::new(), req, ResourceType::Items)
                    .await?;

                let query_metrics = resp
                    .headers()
                    .get_optional_string(&constants::QUERY_METRICS);
                let index_metrics = resp
                    .headers()
                    .get_optional_string(&constants::INDEX_METRICS);
                let continuation_token =
                    resp.headers().get_optional_string(&constants::CONTINUATION);

                let query_response: QueryResponseModel<T> = resp.into_body();

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
