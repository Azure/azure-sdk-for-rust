// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Upsert operation.

use std::sync::Arc;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::options::ItemOptions;
use rand::RngExt;

use super::{Operation, PerfItem};
use crate::seed::SharedItems;

/// Upserts an item into a random seeded partition.
pub struct UpsertItemOperation {
    items: Arc<SharedItems>,
    options: Option<ItemOptions>,
}

impl UpsertItemOperation {
    /// Creates a new upsert operation targeting the given seeded items.
    pub fn new(items: Arc<SharedItems>, options: Option<ItemOptions>) -> Self {
        Self { items, options }
    }
}

#[async_trait]
impl Operation for UpsertItemOperation {
    fn name(&self) -> &'static str {
        "UpsertItem"
    }

    async fn execute(&self, container: &ContainerClient) -> azure_core::Result<()> {
        let seeded = self.items.random();
        let value = rand::rng().random_range(0..u64::MAX);

        let item = PerfItem {
            id: seeded.id.clone(),
            partition_key: seeded.partition_key.clone(),
            value,
            payload: "perf-test-payload".to_string(),
        };

        container
            .upsert_item(&item.partition_key, &item, self.options.clone())
            .await?;
        Ok(())
    }
}
