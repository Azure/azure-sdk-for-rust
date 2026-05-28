// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Create operation.

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::options::ItemWriteOptions;
use rand::RngExt;
use uuid::Uuid;

use super::{extract_backend_duration, Operation, PerfItem};
use crate::seed::{SeededItem, SharedItems};

/// Creates a new item with a unique ID and partition key.
///
/// On success the new item is appended to the shared items list so it
/// becomes available to subsequent read, query, and upsert operations.
pub struct CreateItemOperation {
    items: Arc<SharedItems>,
    options: Option<ItemWriteOptions>,
}

impl CreateItemOperation {
    /// Creates a new create operation backed by the shared items list.
    pub fn new(items: Arc<SharedItems>, options: Option<ItemWriteOptions>) -> Self {
        Self { items, options }
    }
}

#[async_trait]
impl Operation for CreateItemOperation {
    fn name(&self) -> &'static str {
        "CreateItem"
    }

    async fn execute(
        &self,
        container: &ContainerClient,
    ) -> azure_data_cosmos::Result<Option<Duration>> {
        let id = Uuid::new_v4().to_string();
        let partition_key = Uuid::new_v4().to_string();
        let value = rand::rng().random_range(0..u64::MAX);

        let item = PerfItem {
            id: id.clone(),
            partition_key: partition_key.clone(),
            value,
            payload: "perf-test-created".to_string(),
        };

        let response = container
            .create_item(&item.partition_key, &id, &item, self.options.clone())
            .await?;
        let backend = extract_backend_duration(response.headers());

        self.items.push(SeededItem { id, partition_key });
        Ok(backend)
    }
}
