// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Single-partition query operation.

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::Query;
use futures::StreamExt;
use rand::Rng;

use super::Operation;

/// Runs a single-partition query against a random seeded partition key.
pub struct QueryItemsOperation {
    seed_count: usize,
}

impl QueryItemsOperation {
    /// Creates a new query operation targeting items in the seeded range.
    pub fn new(seed_count: usize) -> Self {
        Self { seed_count }
    }
}

#[async_trait]
impl Operation for QueryItemsOperation {
    fn name(&self) -> &'static str {
        "QueryItems"
    }

    async fn execute(&self, container: &ContainerClient) -> azure_core::Result<()> {
        let idx = rand::rng().random_range(0..self.seed_count);
        let pk = format!("pk-{idx}");

        let query = Query::from("SELECT * FROM c WHERE c.partition_key = @pk")
            .with_parameter("@pk", &pk)?;

        let mut stream = container.query_items::<serde_json::Value>(query, &pk, None)?;
        while let Some(result) = stream.next().await {
            result?;
        }

        Ok(())
    }
}
