// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Upsert operation.

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use rand::Rng;

use super::{Operation, PerfItem};

/// Upserts an item into a random seeded partition.
pub struct UpsertItemOperation {
    seed_count: usize,
}

impl UpsertItemOperation {
    /// Creates a new upsert operation targeting partitions in the seeded range.
    pub fn new(seed_count: usize) -> Self {
        Self { seed_count }
    }
}

#[async_trait]
impl Operation for UpsertItemOperation {
    fn name(&self) -> &'static str {
        "UpsertItem"
    }

    async fn execute(&self, container: &ContainerClient) -> azure_core::Result<()> {
        let (pk, id, value) = {
            let mut rng = rand::rng();
            let idx = rng.random_range(0..self.seed_count);
            let pk = format!("pk-{idx}");
            let id = format!("perf-item-{idx}");
            let value = rng.random_range(0..u64::MAX);
            (pk, id, value)
        };

        let item = PerfItem {
            id,
            partition_key: pk.clone(),
            value,
            payload: "perf-test-payload".to_string(),
        };

        container.upsert_item(&pk, &item, None).await?;
        Ok(())
    }
}
