// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Single-partition query operation.

use std::sync::Arc;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::Query;
use futures::StreamExt;

use super::Operation;
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

    async fn execute(&self, container: &ContainerClient) -> azure_core::Result<()> {
        let item = self.items.random();
        let pk = &item.partition_key;

        let query =
            Query::from("SELECT * FROM c WHERE c.partition_key = @pk").with_parameter("@pk", pk)?;

        let mut stream = container.query_items::<serde_json::Value>(query, pk, None)?;
        while let Some(result) = stream.next().await {
            result?;
        }

        Ok(())
    }
}
