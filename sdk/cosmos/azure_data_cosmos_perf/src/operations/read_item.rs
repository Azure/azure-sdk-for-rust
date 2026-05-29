// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Point read operation.

use std::sync::Arc;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::options::ItemReadOptions;

use super::{extract_backend_duration, Operation, OperationOutcome};
use crate::seed::SharedItems;

/// Reads a random seeded item by ID and partition key.
pub struct ReadItemOperation {
    items: Arc<SharedItems>,
    options: Option<ItemReadOptions>,
}

impl ReadItemOperation {
    /// Creates a new read operation targeting the given seeded items.
    pub fn new(items: Arc<SharedItems>, options: Option<ItemReadOptions>) -> Self {
        Self { items, options }
    }
}

#[async_trait]
impl Operation for ReadItemOperation {
    fn name(&self) -> &'static str {
        "ReadItem"
    }

    async fn execute(
        &self,
        container: &ContainerClient,
    ) -> azure_data_cosmos::Result<OperationOutcome> {
        let item = self.items.random();

        let response = container
            .read_item(&item.partition_key, &item.id, self.options.clone())
            .await?;
        Ok(OperationOutcome::new(
            extract_backend_duration(response.headers()),
            response.diagnostics(),
        ))
    }
}
