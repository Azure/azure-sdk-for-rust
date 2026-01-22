// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal client for managing throughput offers.
//!
//! This client is not exposed to users but is used internally by container and database clients
//! to read and replace throughput offers.

use crate::{
    cosmos_request::CosmosRequest,
    models::ThroughputProperties,
    operation_context::OperationType,
    pipeline::CosmosPipeline,
    resource_context::{ResourceLink, ResourceType},
    FeedPage, Query,
};
use azure_core::http::{response::Response, Context};
use futures::TryStreamExt;
use std::sync::Arc;

/// Internal client for managing throughput offers.
#[derive(Clone)]
pub(crate) struct OffersClient {
    pipeline: Arc<CosmosPipeline>,
    resource_id: String,
}

impl OffersClient {
    /// Creates a new offers client for the given resource ID.
    ///
    /// # Arguments
    /// * `pipeline` - The Cosmos pipeline.
    /// * `resource_id` - The resource ID (RID) of the database or container.
    pub(crate) fn new(pipeline: Arc<CosmosPipeline>, resource_id: String) -> Self {
        Self {
            pipeline,
            resource_id,
        }
    }

    /// Reads the throughput offer for the resource.
    ///
    /// # Arguments
    /// * `context` - The context for the request.
    pub(crate) async fn read(
        &self,
        context: Context<'_>,
    ) -> azure_core::Result<Option<ThroughputProperties>> {
        // Query for the offer for this resource.
        let query = Query::from("SELECT * FROM c WHERE c.offerResourceId = @rid")
            .with_parameter("@rid", &self.resource_id)?;
        let offers_link = ResourceLink::root(ResourceType::Offers);

        let executor = crate::query::executor::QueryExecutor::new(
            self.pipeline.clone(),
            offers_link.clone(),
            context.into_owned(),
            query,
            azure_core::http::headers::Headers::new(),
        );

        // There should only be one offer for a given resource ID.
        let mut page_iter = executor.into_stream()?.into_pages();
        let page: Option<FeedPage<ThroughputProperties>> = page_iter.try_next().await?;
        Ok(page.and_then(|p| p.into_items().into_iter().next()))
    }

    /// Replaces the throughput offer for the resource.
    ///
    /// # Arguments
    /// * `context` - The context for the request.
    /// * `throughput` - The new throughput properties to set.
    pub(crate) async fn replace(
        &self,
        context: Context<'_>,
        throughput: ThroughputProperties,
    ) -> azure_core::Result<Response<ThroughputProperties>> {
        let response = self.read(context.clone()).await?;
        let mut current_throughput = response.unwrap_or_default();
        current_throughput.offer = throughput.offer;

        // NOTE: Offers API doesn't allow Enable Content Response On Write to be false, so once we support that option, we'll need to ignore it here.
        let offer_link =
            ResourceLink::root(ResourceType::Offers).item_by_rid(&current_throughput.offer_id);

        let cosmos_request = CosmosRequest::builder(OperationType::Replace, offer_link)
            .json(current_throughput)
            .build()?;

        self.pipeline
            .send_raw(cosmos_request, context)
            .await
            .map(Into::into)
    }
}
