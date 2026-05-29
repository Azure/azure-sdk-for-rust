// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Single-partition query operation.

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use azure_data_cosmos::Query;
use azure_data_cosmos::{clients::ContainerClient, query::FeedScope};
use futures::StreamExt;

use super::{extract_backend_duration, Operation, OperationOutcome};
use crate::seed::SharedItems;

/// Runs a single-partition query against a random seeded partition key.
pub struct QueryItemsOperation {
    items: Arc<SharedItems>,
}

impl QueryItemsOperation {
    /// Creates a new query operation targeting the given seeded items.
    pub fn new(items: Arc<SharedItems>) -> Self {
        Self { items }
    }
}

#[async_trait]
impl Operation for QueryItemsOperation {
    fn name(&self) -> &'static str {
        "QueryItems"
    }

    async fn execute(
        &self,
        container: &ContainerClient,
    ) -> azure_data_cosmos::Result<OperationOutcome> {
        let item = self.items.random();
        let pk = &item.partition_key;

        let query =
            Query::from("SELECT * FROM c WHERE c.partition_key = @pk").with_parameter("@pk", pk)?;

        let mut stream = container
            .query_items::<serde_json::Value>(query, FeedScope::partition(pk), None)
            .await?
            .into_pages();

        // Sum backend durations across pages so a multi-page query reports
        // the total server processing time, mirroring how the client-observed
        // elapsed wraps the entire stream consumption. Keep the last page's
        // diagnostics for shard observation — pages typically share a shard,
        // and the last page is the freshest signal of the shard's state.
        let mut backend_total: Option<Duration> = None;
        let mut last_diagnostics = None;
        while let Some(result) = stream.next().await {
            let page = result?;
            if let Some(d) = extract_backend_duration(page.headers()) {
                backend_total = Some(backend_total.unwrap_or_default() + d);
            }
            last_diagnostics = Some(page.diagnostics());
        }

        Ok(OperationOutcome {
            backend_duration: backend_total,
            diagnostics: last_diagnostics,
        })
    }
}
