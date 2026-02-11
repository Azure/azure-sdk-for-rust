// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Point read operation.

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use rand::Rng;

use super::Operation;

/// Reads a random seeded item by ID and partition key.
pub struct ReadItemOperation {
    seed_count: usize,
}

impl ReadItemOperation {
    /// Creates a new read operation targeting items in the seeded range.
    pub fn new(seed_count: usize) -> Self {
        Self { seed_count }
    }
}

#[async_trait]
impl Operation for ReadItemOperation {
    fn name(&self) -> &'static str {
        "ReadItem"
    }

    async fn execute(&self, container: &ContainerClient) -> azure_core::Result<()> {
        let idx = rand::rng().random_range(0..self.seed_count);
        let id = format!("perf-item-{idx}");
        let pk = format!("pk-{idx}");

        container
            .read_item::<serde_json::Value>(&pk, &id, None)
            .await?;
        Ok(())
    }
}
