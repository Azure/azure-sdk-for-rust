// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Create operation.

use std::sync::Arc;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::options::ItemOptions;
use rand::RngExt;
use uuid::Uuid;

use super::{Operation, PerfItem};
use crate::seed::{SeededItem, SharedItems};

/// Creates a new item with a unique ID and partition key.
///
/// On success the new item is appended to the shared items list so it
/// becomes available to subsequent read, query, and upsert operations.
pub struct CreateItemOperation {
    items: Arc<SharedItems>,
    options: Option<ItemOptions>,
}

impl CreateItemOperation {
    /// Creates a new create operation backed by the shared items list.
    pub fn new(items: Arc<SharedItems>, options: Option<ItemOptions>) -> Self {
        Self { items, options }
    }
}

#[async_trait]
impl Operation for CreateItemOperation {
    fn name(&self) -> &'static str {
        "CreateItem"
    }

    async fn execute(&self, container: &ContainerClient) -> azure_core::Result<()> {
        let id = Uuid::new_v4().to_string();
        let partition_key = Uuid::new_v4().to_string();
        let value = rand::rng().random_range(0..u64::MAX);

        let item = PerfItem {
            id: id.clone(),
            partition_key: partition_key.clone(),
            value,
            payload: "perf-test-created".to_string(),
        };

        container
            .create_item(&item.partition_key, &item, self.options.clone())
            .await?;

        self.items.push(SeededItem { id, partition_key });
        Ok(())
    }
}
