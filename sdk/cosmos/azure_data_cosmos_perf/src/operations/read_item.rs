// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Point read operation.

use std::sync::Arc;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::options::ItemOptions;

use super::Operation;
use crate::seed::SharedItems;

/// Reads a random seeded item by ID and partition key.
pub struct ReadItemOperation {
    items: Arc<SharedItems>,
    options: Option<ItemOptions>,
}

impl ReadItemOperation {
    /// Creates a new read operation targeting the given seeded items.
    pub fn new(items: Arc<SharedItems>, options: Option<ItemOptions>) -> Self {
        Self { items, options }
    }
}

#[async_trait]
impl Operation for ReadItemOperation {
    fn name(&self) -> &'static str {
        "ReadItem"
    }

    async fn execute(&self, container: &ContainerClient) -> azure_core::Result<()> {
        let item = self.items.random();

        container
            .read_item::<serde_json::Value>(&item.partition_key, &item.id, self.options.clone())
            .await?;
        Ok(())
    }
}
