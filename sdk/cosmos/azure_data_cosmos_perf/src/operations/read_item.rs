// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Point read operation.

use std::sync::Arc;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use rand::Rng;

use super::Operation;
use crate::seed::SeededItem;

/// Reads a random seeded item by ID and partition key.
pub struct ReadItemOperation {
    items: Arc<Vec<SeededItem>>,
}

impl ReadItemOperation {
    /// Creates a new read operation targeting the given seeded items.
    pub fn new(items: Arc<Vec<SeededItem>>) -> Self {
        Self { items }
    }
}

#[async_trait]
impl Operation for ReadItemOperation {
    fn name(&self) -> &'static str {
        "ReadItem"
    }

    async fn execute(&self, container: &ContainerClient) -> azure_core::Result<()> {
        let idx = rand::rng().random_range(0..self.items.len());
        let item = &self.items[idx];

        container
            .read_item::<serde_json::Value>(&item.partition_key, &item.id, None)
            .await?;
        Ok(())
    }
}
