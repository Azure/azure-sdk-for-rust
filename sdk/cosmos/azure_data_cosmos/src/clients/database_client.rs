// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::ContainerClient,
    models::{
        ContainerProperties, ContainerQueryResults, DatabaseProperties, Item, ThroughputProperties,
    },
    options::ReadDatabaseOptions,
    pipeline::CosmosPipeline,
    resource_context::{ResourceLink, ResourceType},
    CreateContainerOptions, DeleteDatabaseOptions, Query, QueryContainersOptions,
    ThroughputOptions,
};

use azure_core::{Method, Model, Pager, Request, Response};
use futures::StreamExt;
use serde::Deserialize;

/// A client for working with a specific database in a Cosmos DB account.
///
/// You can get a `DatabaseClient` by calling [`CosmosClient::database_client()`](crate::CosmosClient::database_client()).
pub struct DatabaseClient {
    link: ResourceLink,
    containers_link: ResourceLink,
    database_id: String,
    pipeline: CosmosPipeline,
}

impl DatabaseClient {
    pub(crate) fn new(pipeline: CosmosPipeline, database_id: &str) -> Self {
        let database_id = database_id.to_string();
        let link = ResourceLink::root(ResourceType::Databases).item(&database_id);
        let containers_link = link.feed(ResourceType::Containers);

        Self {
            link,
            containers_link,
            database_id,
            pipeline,
        }
    }

    /// Gets a [`ContainerClient`] that can be used to access the collection with the specified name.
    ///
    /// # Arguments
    /// * `name` - The name of the container.
    pub fn container_client(&self, name: &str) -> ContainerClient {
        ContainerClient::new(self.pipeline.clone(), &self.link, name)
    }

    /// Returns the identifier of the Cosmos database.
    pub fn id(&self) -> &str {
        &self.database_id
    }

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
    /// # use azure_data_cosmos::clients::DatabaseClient;
    /// # let database_client: DatabaseClient = panic!("this is a non-running example");
    /// let response = database_client.read(None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap();
    /// # }
    /// ```
    pub async fn read(
        &self,
        options: Option<ReadDatabaseOptions<'_>>,
    ) -> azure_core::Result<Response<DatabaseProperties>> {
        let options = options.unwrap_or_default();
        let url = self.pipeline.url(&self.link);
        let mut req = Request::new(url, Method::Get);
        self.pipeline
            .send(options.method_options.context, &mut req, self.link.clone())
            .await
    }

    /// Executes a query against containers in the database.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to execute.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// The `query` parameter accepts anything that can be transformed [`Into`] a [`Query`].
    /// This allows simple queries without parameters to be expressed easily:
    ///
    /// ```rust,no_run
    /// # async fn doc() {
    /// # use azure_data_cosmos::clients::DatabaseClient;
    /// # let db_client: DatabaseClient = panic!("this is a non-running example");
    /// let containers = db_client.query_containers(
    ///     "SELECT * FROM dbs",
    ///     None).unwrap();
    /// # }
    /// ```
    ///
    /// See [`Query`] for more information on how to specify a query.
    pub fn query_containers(
        &self,
        query: impl Into<Query>,
        options: Option<QueryContainersOptions<'_>>,
    ) -> azure_core::Result<Pager<ContainerQueryResults>> {
        let options = options.unwrap_or_default();
        let url = self.pipeline.url(&self.containers_link);
        let base_request = Request::new(url, Method::Post);

        self.pipeline.send_query_request(
            options.method_options.context,
            query.into(),
            base_request,
            self.containers_link.clone(),
        )
    }

    /// Creates a new container.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `properties` - A [`ContainerProperties`] describing the new container.
    /// * `options` - Optional parameters for the request.
    pub async fn create_container(
        &self,
        properties: ContainerProperties,
        options: Option<CreateContainerOptions<'_>>,
    ) -> azure_core::Result<Response<Item<ContainerProperties>>> {
        let options = options.unwrap_or_default();
        let url = self.pipeline.url(&self.containers_link);
        let mut req = Request::new(url, Method::Post);
        req.set_json(&properties)?;

        self.pipeline
            .send(
                options.method_options.context,
                &mut req,
                self.containers_link.clone(),
            )
            .await
    }

    /// Deletes this database.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    pub async fn delete(
        &self,
        options: Option<DeleteDatabaseOptions<'_>>,
    ) -> azure_core::Result<Response> {
        let options = options.unwrap_or_default();
        let url = self.pipeline.url(&self.link);
        let mut req = Request::new(url, Method::Delete);
        self.pipeline
            .send(options.method_options.context, &mut req, self.link.clone())
            .await
    }

    pub async fn read_throughput(
        &self,

        #[allow(unused_variables)]
        // REASON: This is a documented public API so prefixing with '_' is undesirable.
        options: Option<ThroughputOptions<'_>>,
    ) -> azure_core::Result<Option<Response<ThroughputProperties>>> {
        let options = options.unwrap_or_default();

        #[derive(Model, Deserialize)]
        struct OfferResults {
            #[serde(rename = "Offers")]
            pub offers: Vec<ThroughputProperties>,
        }

        // We only have to into_owned here in order to call send_query_request below,
        // since it returns `Pager` which must own it's data.
        // But in this case, we don't really _need_ the `Pager` to own it's data
        // because we use it and dispose of it within the body of this method.
        // If we wanted to optimize this later, we have a few options:
        // 1. Give Pager a lifetime parameter so it can borrow it's context (proliferates lifetime parameters everywhere though...)
        // 2. Don't use send_query_request. We expect the offer to be in the first page, so we could just make a regular request
        // (but what if we get an empty page for some reason? is that something the server could do?)
        //
        // For now, we'll risk cloning the context data and I'm just leaving this note to complain about it ;).
        let context = options.method_options.context.into_owned();

        // We need to get the RID for the database.
        let db = self.read(None).await?.deserialize_body().await?;
        let rid = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a database");

        // Now, query for the offer for this resource.
        let query = Query::from("SELECT * FROM c WHERE c.offerResourceId = @rid")
            .with_parameter("@rid", rid)?;
        let offers_link = ResourceLink::root(ResourceType::Offers);
        let mut results: Pager<OfferResults> = self.pipeline.send_query_request(
            context.clone(),
            query,
            Request::new(self.pipeline.url(&offers_link), Method::Post),
            offers_link.clone(),
        )?;
        let offers = results
            .next()
            .await
            .expect("the first pager result should always be Some, even when there's an error")?
            .deserialize_body()
            .await?
            .offers;

        if offers.is_empty() {
            // No offers found for this resource.
            return Ok(None);
        }
        println!("Offers");
        println!("{:#?}", offers);

        let offer_link = offers_link.item(&offers[0].offer_id);
        let offer_url = self.pipeline.url(&offer_link);

        // Now we can read the offer itself
        let mut req = Request::new(offer_url, Method::Get);
        self.pipeline
            .send(context, &mut req, offer_link)
            .await
            .map(Some)
    }
}
