// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types for working with transactional batch operations in Cosmos DB.
//!
//! Transactional batches allow you to group multiple operations (create, read, upsert, replace, delete)
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

use crate::PartitionKey;
use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Options for batch upsert operations.
///
/// Upsert supports both conditional options for optimistic concurrency control.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchUpsertOptions {
    /// Only perform the operation if the item's ETag matches this value.
    if_match: Option<String>,
    /// Only perform the operation if the item's ETag does not match this value.
    if_none_match: Option<String>,
}

impl BatchUpsertOptions {
    /// Sets the `if_match` condition for optimistic concurrency control.
    pub fn with_if_match(mut self, etag: impl Into<String>) -> Self {
        self.if_match = Some(etag.into());
        self
    }

    /// Sets the `if_none_match` condition for optimistic concurrency control.
    pub fn with_if_none_match(mut self, etag: impl Into<String>) -> Self {
        self.if_none_match = Some(etag.into());
        self
    }
}

/// Options for batch replace operations.
///
/// Replace only supports `if_match` for optimistic concurrency control,
/// since the item must exist to be replaced.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchReplaceOptions {
    /// Only replace if the item's current ETag matches this value.
    if_match: Option<String>,
}

impl BatchReplaceOptions {
    /// Sets the `if_match` condition for optimistic concurrency control.
    pub fn with_if_match(mut self, etag: impl Into<String>) -> Self {
        self.if_match = Some(etag.into());
        self
    }
}

/// Options for batch read operations.
///
/// Read supports both conditional options, commonly used for cache validation.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchReadOptions {
    /// Only return the item if its ETag matches this value.
    if_match: Option<String>,
    /// Only return the item if its ETag does not match (useful for caching).
    if_none_match: Option<String>,
}

impl BatchReadOptions {
    /// Sets the `if_match` condition.
    pub fn with_if_match(mut self, etag: impl Into<String>) -> Self {
        self.if_match = Some(etag.into());
        self
    }

    /// Sets the `if_none_match` condition (useful for caching).
    pub fn with_if_none_match(mut self, etag: impl Into<String>) -> Self {
        self.if_none_match = Some(etag.into());
        self
    }
}

/// Options for batch delete operations.
///
/// Delete only supports `if_match` for optimistic concurrency control,
/// since the item must exist to be deleted.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchDeleteOptions {
    /// Only delete if the item's current ETag matches this value.
    if_match: Option<String>,
}

impl BatchDeleteOptions {
    /// Sets the `if_match` condition for optimistic concurrency control.
    pub fn with_if_match(mut self, etag: impl Into<String>) -> Self {
        self.if_match = Some(etag.into());
        self
    }
}

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
    /// * `item` - The item to create. Must implement [`Serialize`].
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
        });
        Ok(self)
    }

    /// Adds an upsert operation to the batch.
    ///
    /// # Arguments
    /// * `item` - The item to upsert. Must implement [`Serialize`].
    /// * `options` - Optional conditional options for the operation.
    pub fn upsert_item<T: Serialize>(
        mut self,
        item: T,
        options: Option<BatchUpsertOptions>,
    ) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(item)?;
        self.operations.push(TransactionalBatchOperation::Upsert {
            resource_body,
            id: None,
            if_match: options.as_ref().and_then(|o| o.if_match.clone()),
            if_none_match: options.as_ref().and_then(|o| o.if_none_match.clone()),
        });
        Ok(self)
    }

    /// Adds a replace operation to the batch.
    ///
    /// # Arguments
    /// * `item_id` - The id of the item to replace.
    /// * `item` - The new item data. Must implement [`Serialize`].
    /// * `options` - Optional conditional options for the operation (e.g., `if_match` for optimistic concurrency).
    pub fn replace_item<T: Serialize>(
        mut self,
        item_id: impl Into<Cow<'static, str>>,
        item: T,
        options: Option<BatchReplaceOptions>,
    ) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(item)?;
        self.operations.push(TransactionalBatchOperation::Replace {
            id: item_id.into(),
            resource_body,
            if_match: options.and_then(|o| o.if_match),
        });
        Ok(self)
    }

    /// Adds a read operation to the batch.
    ///
    /// # Arguments
    /// * `item_id` - The id of the item to read.
    /// * `options` - Optional conditional options for the operation.
    pub fn read_item(
        mut self,
        item_id: impl Into<Cow<'static, str>>,
        options: Option<BatchReadOptions>,
    ) -> Self {
        self.operations.push(TransactionalBatchOperation::Read {
            id: item_id.into(),
            if_match: options.as_ref().and_then(|o| o.if_match.clone()),
            if_none_match: options.as_ref().and_then(|o| o.if_none_match.clone()),
        });
        self
    }

    /// Adds a delete operation to the batch.
    ///
    /// # Arguments
    /// * `item_id` - The id of the item to delete.
    /// * `options` - Optional conditional options for the operation (e.g., `if_match` to only delete if ETag matches).
    pub fn delete_item(
        mut self,
        item_id: impl Into<Cow<'static, str>>,
        options: Option<BatchDeleteOptions>,
    ) -> Self {
        self.operations.push(TransactionalBatchOperation::Delete {
            id: item_id.into(),
            if_match: options.and_then(|o| o.if_match),
        });
        self
    }
}

/// Represents a single operation within a transactional batch.
///
/// Each operation is serialized with the "operationType" field indicating the type
/// of operation (e.g., "Create", "Read", "Delete"). The variant names match the
/// Cosmos DB REST API requirements for transactional batch operations.
///
/// # Serialization Format
///
/// Operations are serialized as JSON objects with the following structure:
///
/// ```json
/// {
///   "operationType": "Create",
///   "resourceBody": { /* item data */ }
/// }
/// ```
///
/// Or for operations that reference an existing item:
///
/// ```json
/// {
///   "operationType": "Read",
///   "id": "item-id"
/// }
/// ```
#[derive(Clone, SafeDebug, Serialize, Deserialize)]
#[safe(true)]
#[serde(tag = "operationType", rename_all_fields = "camelCase")]
pub(crate) enum TransactionalBatchOperation {
    /// Create a new item.
    Create {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<Cow<'static, str>>,
        resource_body: serde_json::Value,
    },
    /// Upsert an item (create or replace).
    Upsert {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<Cow<'static, str>>,
        resource_body: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        if_match: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        if_none_match: Option<String>,
    },
    /// Replace an existing item.
    Replace {
        id: Cow<'static, str>,
        resource_body: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        if_match: Option<String>,
    },
    /// Read an item.
    Read {
        id: Cow<'static, str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        if_match: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        if_none_match: Option<String>,
    },
    /// Delete an item.
    Delete {
        id: Cow<'static, str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        if_match: Option<String>,
    },
}

/// Response from executing a transactional batch.
///
/// The Cosmos DB batch API returns a raw JSON array of operation results,
/// so we implement a custom deserializer to handle this format.
#[derive(Clone, SafeDebug)]
#[safe(true)]
#[non_exhaustive]
pub struct TransactionalBatchResponse {
    /// The results of each operation in the batch.
    results: Vec<TransactionalBatchOperationResult>,
}

impl TransactionalBatchResponse {
    /// Returns the results of each operation in the batch.
    pub fn results(&self) -> &[TransactionalBatchOperationResult] {
        &self.results
    }
}

impl<'de> Deserialize<'de> for TransactionalBatchResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // The Cosmos DB batch API returns a raw JSON array, not an object with a "results" field
        let results = Vec::<TransactionalBatchOperationResult>::deserialize(deserializer)?;
        Ok(TransactionalBatchResponse { results })
    }
}

/// Result of a single operation within a transactional batch.
#[derive(Clone, SafeDebug, Deserialize)]
#[safe(true)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TransactionalBatchOperationResult {
    /// HTTP status code for this operation.
    status_code: u16,

    /// The resource body returned by the operation, if any.
    #[serde(default)]
    resource_body: Option<serde_json::Value>,

    /// ETag of the resource after the operation.
    #[serde(rename = "eTag")]
    #[serde(default)]
    etag: Option<String>,

    /// Request charge for this operation.
    #[serde(default)]
    request_charge: Option<f64>,

    /// Retry after duration in milliseconds, if applicable.
    #[serde(default)]
    retry_after_milliseconds: Option<u64>,

    /// Substatus code for this operation, if applicable.
    #[serde(default)]
    substatus_code: Option<u32>,
}

impl TransactionalBatchOperationResult {
    /// Returns the HTTP status code for this operation.
    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    /// Returns the resource body returned by the operation, if any.
    pub fn resource_body(&self) -> Option<&serde_json::Value> {
        self.resource_body.as_ref()
    }

    /// Returns the ETag of the resource after the operation, if any.
    pub fn etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }

    /// Returns the request charge for this operation, if any.
    pub fn request_charge(&self) -> Option<f64> {
        self.request_charge
    }

    /// Returns the retry after duration in milliseconds, if applicable.
    pub fn retry_after_milliseconds(&self) -> Option<u64> {
        self.retry_after_milliseconds
    }

    /// Returns the substatus code for this operation, if applicable.
    pub fn substatus_code(&self) -> Option<u32> {
        self.substatus_code
    }

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
    fn serialize_all_operations() -> Result<(), Box<dyn std::error::Error>> {
        let item = TestItem {
            id: "item1".to_string(),
            value: 42,
        };

        let replace_options = BatchReplaceOptions::default().with_if_match("some-etag");

        let batch = TransactionalBatch::new("test_partition")
            .create_item(&item)?
            .upsert_item(&item, None)?
            .replace_item("id1", &item, Some(replace_options))?
            .read_item("id2", None)
            .delete_item("id3", None);

        assert_eq!(batch.operations().len(), 5);

        let serialized = serde_json::to_string_pretty(batch.operations())?;

        let expected = r#"[
  {
    "operationType": "Create",
    "resourceBody": {
      "id": "item1",
      "value": 42
    }
  },
  {
    "operationType": "Upsert",
    "resourceBody": {
      "id": "item1",
      "value": 42
    }
  },
  {
    "operationType": "Replace",
    "id": "id1",
    "resourceBody": {
      "id": "item1",
      "value": 42
    },
    "ifMatch": "some-etag"
  },
  {
    "operationType": "Read",
    "id": "id2"
  },
  {
    "operationType": "Delete",
    "id": "id3"
  }
]"#;

        assert_eq!(serialized, expected);

        Ok(())
    }
}
