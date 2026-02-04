// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types for working with transactional batch operations in Cosmos DB.
//!
//! Transactional batches allow you to group multiple operations (create, read, upsert, replace, delete, patch)
//! within the same partition key as a single atomic transaction.
//!
//! # Examples
//!
//! ```rust,no_run
//! use azure_data_cosmos::TransactionalBatch;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Deserialize, Serialize)]
//! struct Product {
//!     id: String,
//!     category: String,
//!     name: String,
//! }
//!
//! # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
//! # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
//! let product1 = Product {
//!     id: "product1".to_string(),
//!     category: "category1".to_string(),
//!     name: "Product #1".to_string(),
//! };
//!
//! let batch = TransactionalBatch::new("category1")
//!     .create_item(product1)?;
//!
//! let response = container_client.execute_transactional_batch(batch, None).await?;
//! # Ok(())
//! # }
//! ```

use crate::{models::PatchDocument, PartitionKey};
use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a transactional batch of operations to be executed atomically.
///
/// All operations in the batch must target the same partition key.
/// See the [module documentation](self) for more information and examples.
#[derive(Clone, SafeDebug)]
#[safe(true)]
pub struct TransactionalBatch {
    partition_key: PartitionKey,
    operations: Vec<TransactionalBatchOperation>,
}

impl TransactionalBatch {
    /// Creates a new transactional batch for the specified partition key.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key for all operations in this batch.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use azure_data_cosmos::TransactionalBatch;
    ///
    /// let batch = TransactionalBatch::new("my_partition_key");
    /// ```
    pub fn new(partition_key: impl Into<PartitionKey>) -> Self {
        Self {
            partition_key: partition_key.into(),
            operations: Vec::new(),
        }
    }

    /// Returns the partition key for this batch.
    pub fn partition_key(&self) -> &PartitionKey {
        &self.partition_key
    }

    /// Returns the operations in this batch.
    pub(crate) fn operations(&self) -> &[TransactionalBatchOperation] {
        &self.operations
    }

    /// Adds a create operation to the batch.
    ///
    /// # Arguments
    /// * `item` - The item to create. Must implement [`Serialize`](serde::Serialize).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use azure_data_cosmos::TransactionalBatch;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Product {
    ///     id: String,
    ///     name: String,
    /// }
    ///
    /// # fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// let product = Product {
    ///     id: "product1".to_string(),
    ///     name: "Product #1".to_string(),
    /// };
    ///
    /// let batch = TransactionalBatch::new("partition1")
    ///     .create_item(product)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_item<T: Serialize>(mut self, item: T) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(item)?;
        self.operations.push(TransactionalBatchOperation::Create {
            resource_body,
            id: None,
            if_match: None,
            if_none_match: None,
        });
        Ok(self)
    }

    /// Adds an upsert operation to the batch.
    ///
    /// # Arguments
    /// * `item` - The item to upsert. Must implement [`Serialize`](serde::Serialize).
    pub fn upsert_item<T: Serialize>(mut self, item: T) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(item)?;
        self.operations.push(TransactionalBatchOperation::Upsert {
            resource_body,
            id: None,
            if_match: None,
            if_none_match: None,
        });
        Ok(self)
    }

    /// Adds a replace operation to the batch.
    ///
    /// # Arguments
    /// * `item_id` - The id of the item to replace.
    /// * `item` - The new item data. Must implement [`Serialize`](serde::Serialize).
    pub fn replace_item<T: Serialize>(
        mut self,
        item_id: impl Into<Cow<'static, str>>,
        item: T,
    ) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(item)?;
        self.operations.push(TransactionalBatchOperation::Replace {
            id: item_id.into(),
            resource_body,
            if_match: None,
            if_none_match: None,
        });
        Ok(self)
    }

    /// Adds a read operation to the batch.
    ///
    /// # Arguments
    /// * `item_id` - The id of the item to read.
    pub fn read_item(mut self, item_id: impl Into<Cow<'static, str>>) -> Self {
        self.operations.push(TransactionalBatchOperation::Read {
            id: item_id.into(),
            if_match: None,
            if_none_match: None,
        });
        self
    }

    /// Adds a delete operation to the batch.
    ///
    /// # Arguments
    /// * `item_id` - The id of the item to delete.
    pub fn delete_item(mut self, item_id: impl Into<Cow<'static, str>>) -> Self {
        self.operations.push(TransactionalBatchOperation::Delete {
            id: item_id.into(),
            if_match: None,
            if_none_match: None,
        });
        self
    }

    /// Adds a patch operation to the batch.
    ///
    /// # Arguments
    /// * `item_id` - The id of the item to patch.
    /// * `patch` - The patch document to apply.
    pub fn patch_item(
        mut self,
        item_id: impl Into<Cow<'static, str>>,
        patch: PatchDocument,
    ) -> Self {
        self.operations.push(TransactionalBatchOperation::Patch {
            id: item_id.into(),
            resource_body: patch,
            if_match: None,
            if_none_match: None,
        });
        self
    }
}

/// Represents a single operation within a transactional batch.
#[derive(Clone, SafeDebug, Serialize, Deserialize)]
#[safe(true)]
#[serde(tag = "operationType")]
pub(crate) enum TransactionalBatchOperation {
    /// Create a new item.
    Create {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<Cow<'static, str>>,
        #[serde(rename = "resourceBody")]
        resource_body: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifMatch")]
        if_match: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifNoneMatch")]
        if_none_match: Option<String>,
    },
    /// Upsert an item (create or replace).
    Upsert {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<Cow<'static, str>>,
        #[serde(rename = "resourceBody")]
        resource_body: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifMatch")]
        if_match: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifNoneMatch")]
        if_none_match: Option<String>,
    },
    /// Replace an existing item.
    Replace {
        id: Cow<'static, str>,
        #[serde(rename = "resourceBody")]
        resource_body: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifMatch")]
        if_match: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifNoneMatch")]
        if_none_match: Option<String>,
    },
    /// Read an item.
    Read {
        id: Cow<'static, str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifMatch")]
        if_match: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifNoneMatch")]
        if_none_match: Option<String>,
    },
    /// Delete an item.
    Delete {
        id: Cow<'static, str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifMatch")]
        if_match: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifNoneMatch")]
        if_none_match: Option<String>,
    },
    /// Patch an item.
    Patch {
        id: Cow<'static, str>,
        #[serde(rename = "resourceBody")]
        resource_body: PatchDocument,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifMatch")]
        if_match: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ifNoneMatch")]
        if_none_match: Option<String>,
    },
}

/// Response from executing a transactional batch.
#[derive(Clone, SafeDebug, Deserialize)]
#[safe(true)]
pub struct TransactionalBatchResponse {
    /// The results of each operation in the batch.
    #[serde(rename = "results")]
    pub results: Vec<TransactionalBatchOperationResult>,
}

/// Result of a single operation within a transactional batch.
#[derive(Clone, SafeDebug, Deserialize)]
#[safe(true)]
pub struct TransactionalBatchOperationResult {
    /// HTTP status code for this operation.
    #[serde(rename = "statusCode")]
    pub status_code: u16,

    /// The resource body returned by the operation, if any.
    #[serde(rename = "resourceBody")]
    #[serde(default)]
    pub resource_body: Option<serde_json::Value>,

    /// ETag of the resource after the operation.
    #[serde(rename = "eTag")]
    #[serde(default)]
    pub etag: Option<String>,

    /// Request charge for this operation.
    #[serde(rename = "requestCharge")]
    #[serde(default)]
    pub request_charge: Option<f64>,

    /// Retry after duration in milliseconds, if applicable.
    #[serde(rename = "retryAfterMilliseconds")]
    #[serde(default)]
    pub retry_after_milliseconds: Option<u64>,

    /// Substatus code for this operation, if applicable.
    #[serde(rename = "subStatusCode")]
    #[serde(default)]
    pub substatus_code: Option<u32>,
}

impl TransactionalBatchOperationResult {
    /// Deserializes the resource body as the specified type.
    ///
    /// Returns `None` if there is no resource body.
    ///
    /// # Errors
    ///
    /// Returns an error if the resource body cannot be deserialized as the specified type.
    pub fn deserialize_body<T: serde::de::DeserializeOwned>(
        &self,
    ) -> Result<Option<T>, serde_json::Error> {
        match &self.resource_body {
            Some(value) => Ok(Some(serde_json::from_value(value.clone())?)),
            None => Ok(None),
        }
    }

    /// Returns `true` if this operation was successful (2xx status code).
    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestItem {
        id: String,
        value: i32,
    }

    #[test]
    fn create_batch_with_partition_key() {
        let batch = TransactionalBatch::new("test_partition");
        assert_eq!(batch.partition_key(), &PartitionKey::from("test_partition"));
        assert_eq!(batch.operations().len(), 0);
    }

    #[test]
    fn add_create_operation() -> Result<(), Box<dyn std::error::Error>> {
        let item = TestItem {
            id: "item1".to_string(),
            value: 42,
        };

        let batch = TransactionalBatch::new("test_partition").create_item(item)?;

        assert_eq!(batch.operations().len(), 1);
        Ok(())
    }

    #[test]
    fn add_multiple_operations() -> Result<(), Box<dyn std::error::Error>> {
        let item1 = TestItem {
            id: "item1".to_string(),
            value: 42,
        };
        let item2 = TestItem {
            id: "item2".to_string(),
            value: 24,
        };

        let batch = TransactionalBatch::new("test_partition")
            .create_item(item1)?
            .upsert_item(item2)?
            .read_item("item3")
            .delete_item("item4");

        assert_eq!(batch.operations().len(), 4);
        Ok(())
    }

    #[test]
    fn serialize_batch_operations() -> Result<(), Box<dyn std::error::Error>> {
        let item = TestItem {
            id: "item1".to_string(),
            value: 42,
        };

        let batch = TransactionalBatch::new("test_partition")
            .create_item(item)?
            .read_item("item2")
            .delete_item("item3");

        let operations = batch.operations();
        let serialized = serde_json::to_string(operations)?;

        // Verify serialization produces valid JSON array
        assert!(serialized.starts_with('['));
        assert!(serialized.ends_with(']'));
        assert!(serialized.contains("\"operationType\""));

        Ok(())
    }

    #[test]
    fn serialize_batch_operations_format() -> Result<(), Box<dyn std::error::Error>> {
        let item = TestItem {
            id: "item1".to_string(),
            value: 42,
        };

        let batch = TransactionalBatch::new("test_partition")
            .create_item(&item)?
            .read_item("item2")
            .replace_item("item3", &item)?;

        let operations = batch.operations();
        let serialized = serde_json::to_string_pretty(operations)?;

        // Verify the structure matches Cosmos DB expectations
        assert!(serialized.contains("\"operationType\": \"Create\""));
        assert!(serialized.contains("\"operationType\": \"Read\""));
        assert!(serialized.contains("\"operationType\": \"Replace\""));
        assert!(serialized.contains("\"resourceBody\""));
        assert!(serialized.contains("\"id\": \"item2\""));
        assert!(serialized.contains("\"id\": \"item3\""));

        Ok(())
    }
}
