// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Query execution implementations.

mod gateway;

#[cfg(feature = "preview_query_engine")]
mod query_engine;

pub use gateway::GatewayExecutor;

#[cfg(feature = "preview_query_engine")]
pub use query_engine::QueryEngineExecutor;

use serde::de::DeserializeOwned;
use std::sync::Arc;

use crate::{
    conditional_send::ConditionalSend, pipeline::CosmosPipeline, resource_context::ResourceLink,
    FeedItemIterator, FeedPage, Query, QueryOptions,
};

#[cfg(feature = "preview_query_engine")]
use crate::query::QueryEngineRef;

/// A query executor that handles fetching pages of query results.
///
/// This enum provides two execution strategies:
/// - [`Gateway`](QueryExecutor::Gateway): Direct gateway execution for single-partition queries
/// - [`QueryEngine`](QueryExecutor::QueryEngine): Cross-partition query support via external query engine
pub enum QueryExecutor<T: DeserializeOwned + ConditionalSend> {
    /// Executes queries directly against the gateway endpoint.
    Gateway(GatewayExecutor<T>),

    /// Executes queries using an external query engine for cross-partition support.
    #[cfg(feature = "preview_query_engine")]
    QueryEngine(QueryEngineExecutor<T>),
}

impl<T: DeserializeOwned + ConditionalSend + 'static> QueryExecutor<T> {
    /// Creates a new query executor using the query engine for cross-partition support.
    ///
    /// This method is only available when the `preview_query_engine` feature is enabled.
    #[cfg(feature = "preview_query_engine")]
    pub fn query_engine(
        http_pipeline: Arc<CosmosPipeline>,
        container_link: ResourceLink,
        query: Query,
        options: QueryOptions<'_>,
        query_engine: QueryEngineRef,
    ) -> azure_core::Result<Self> {
        let context = options.method_options.context.into_owned();
        let executor =
            QueryEngineExecutor::new(http_pipeline, container_link, context, query, query_engine)?;
        Ok(QueryExecutor::QueryEngine(executor))
    }

    /// Creates a new gateway query executor for single-partition queries.
    pub fn gateway(
        http_pipeline: Arc<CosmosPipeline>,
        items_link: ResourceLink,
        query: Query,
        options: QueryOptions<'_>,
        apply_headers: impl FnOnce(&mut azure_core::http::Request) -> azure_core::Result<()>,
    ) -> azure_core::Result<Self> {
        let context = options.method_options.context.into_owned();
        let executor =
            GatewayExecutor::new(http_pipeline, items_link, context, query, apply_headers)?;
        Ok(QueryExecutor::Gateway(executor))
    }

    /// Converts the executor into a stream of pages.
    pub fn into_stream(self) -> azure_core::Result<FeedItemIterator<T>> {
        Ok(FeedItemIterator::new(futures::stream::try_unfold(
            self,
            |mut state| async move {
                let val = state.next_page().await?;
                Ok(val.map(|item| (item, state)))
            },
        )))
    }

    /// Fetches the next page of query results.
    ///
    /// Returns `None` if there are no more pages to fetch.
    pub async fn next_page(&mut self) -> azure_core::Result<Option<FeedPage<T>>> {
        match self {
            QueryExecutor::Gateway(executor) => executor.next_page().await,
            #[cfg(feature = "preview_query_engine")]
            QueryExecutor::QueryEngine(executor) => executor.next_page().await,
        }
    }
}
