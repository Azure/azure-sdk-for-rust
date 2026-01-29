// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{models::PatchDocument, PartitionKey};
use azure_core::{fmt::SafeDebug, http::Etag};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a transactional batch of operations to be executed against a container.
///
/// All operations in a batch must target items with the same partition key value.
/// A batch can contain up to 100 operations and the total request payload must not exceed 2 MB.
///
/// If any operation in the batch fails, the entire batch is rolled back atomically.
///
/// # Examples
///
/// ```rust
/// # use azure_data_cosmos::models::TransactionalBatch;
/// # use serde::{Deserialize, Serialize};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Debug, Deserialize, Serialize)]
/// struct Product {
///     id: String,
///     name: String,
///     price: f64,
/// }
///
/// let product1 = Product {
///     id: "product1".to_string(),
///     name: "Product 1".to_string(),
///     price: 10.0,
/// };
///
/// let product2 = Product {
///     id: "product2".to_string(),
///     name: "Product 2".to_string(),
///     price: 20.0,
/// };
///
/// let batch = TransactionalBatch::new("partition1")
///     .create_item(product1)?
///     .create_item(product2)?
///     .delete_item("product3", None);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, SafeDebug, PartialEq, Eq)]
#[safe(true)]
pub struct TransactionalBatch {
    partition_key: PartitionKey,
    operations: Vec<TransactionalBatchOperation>,
}

impl TransactionalBatch {
    /// Creates a new transactional batch for the specified partition key.
    ///
    /// All operations added to this batch must target items with this partition key.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key value for all operations in this batch.
    pub fn new(partition_key: impl Into<PartitionKey>) -> Self {
        Self {
            partition_key: partition_key.into(),
            operations: Vec::new(),
        }
    }

    /// Gets the partition key for this batch.
    pub fn partition_key(&self) -> &PartitionKey {
        &self.partition_key
    }

    /// Gets the operations in this batch.
    pub fn operations(&self) -> &[TransactionalBatchOperation] {
        &self.operations
    }

    /// Adds a create operation to the batch.
    ///
    /// # Arguments
    /// * `item` - The item to create. Must implement [`Serialize`].
    pub fn create_item<T: Serialize>(
        mut self,
        item: &T,
    ) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(item)?;
        self.operations.push(TransactionalBatchOperation::Create {
            resource_body,
            if_match: None,
            if_none_match: None,
        });
        Ok(self)
    }

    /// Adds a create operation to the batch with conditional ETags.
    ///
    /// # Arguments
    /// * `item` - The item to create. Must implement [`Serialize`].
    /// * `if_match` - Optional ETag for conditional create (If-Match header).
    /// * `if_none_match` - Optional ETag for conditional create (If-None-Match header).
    pub fn create_item_with_options<T: Serialize>(
        mut self,
        item: &T,
        if_match: Option<Etag>,
        if_none_match: Option<Etag>,
    ) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(item)?;
        self.operations.push(TransactionalBatchOperation::Create {
            resource_body,
            if_match,
            if_none_match,
        });
        Ok(self)
    }

    /// Adds a read operation to the batch.
    ///
    /// # Arguments
    /// * `item_id` - The ID of the item to read.
    pub fn read_item(mut self, item_id: impl Into<Cow<'static, str>>) -> Self {
        self.operations.push(TransactionalBatchOperation::Read {
            id: item_id.into(),
            if_match: None,
            if_none_match: None,
        });
        self
    }

    /// Adds a read operation to the batch with conditional ETags.
    ///
    /// # Arguments
    /// * `item_id` - The ID of the item to read.
    /// * `if_match` - Optional ETag for conditional read (If-Match header).
    /// * `if_none_match` - Optional ETag for conditional read (If-None-Match header).
    pub fn read_item_with_options(
        mut self,
        item_id: impl Into<Cow<'static, str>>,
        if_match: Option<Etag>,
        if_none_match: Option<Etag>,
    ) -> Self {
        self.operations.push(TransactionalBatchOperation::Read {
            id: item_id.into(),
            if_match,
            if_none_match,
        });
        self
    }

    /// Adds a replace operation to the batch.
    ///
    /// # Arguments
    /// * `item_id` - The ID of the item to replace.
    /// * `item` - The new item content. Must implement [`Serialize`].
    pub fn replace_item<T: Serialize>(
        mut self,
        item_id: impl Into<Cow<'static, str>>,
        item: &T,
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

    /// Adds a replace operation to the batch with conditional ETags.
    ///
    /// # Arguments
    /// * `item_id` - The ID of the item to replace.
    /// * `item` - The new item content. Must implement [`Serialize`].
    /// * `if_match` - Optional ETag for conditional replace (If-Match header).
    /// * `if_none_match` - Optional ETag for conditional replace (If-None-Match header).
    pub fn replace_item_with_options<T: Serialize>(
        mut self,
        item_id: impl Into<Cow<'static, str>>,
        item: &T,
        if_match: Option<Etag>,
        if_none_match: Option<Etag>,
    ) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(item)?;
        self.operations.push(TransactionalBatchOperation::Replace {
            id: item_id.into(),
            resource_body,
            if_match,
            if_none_match,
        });
        Ok(self)
    }

    /// Adds an upsert operation to the batch.
    ///
    /// # Arguments
    /// * `item` - The item to upsert. Must implement [`Serialize`].
    pub fn upsert_item<T: Serialize>(
        mut self,
        item: &T,
    ) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(item)?;
        self.operations.push(TransactionalBatchOperation::Upsert {
            resource_body,
            if_match: None,
            if_none_match: None,
        });
        Ok(self)
    }

    /// Adds an upsert operation to the batch with conditional ETags.
    ///
    /// # Arguments
    /// * `item` - The item to upsert. Must implement [`Serialize`].
    /// * `if_match` - Optional ETag for conditional upsert (If-Match header).
    /// * `if_none_match` - Optional ETag for conditional upsert (If-None-Match header).
    pub fn upsert_item_with_options<T: Serialize>(
        mut self,
        item: &T,
        if_match: Option<Etag>,
        if_none_match: Option<Etag>,
    ) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(item)?;
        self.operations.push(TransactionalBatchOperation::Upsert {
            resource_body,
            if_match,
            if_none_match,
        });
        Ok(self)
    }

    /// Adds a delete operation to the batch.
    ///
    /// # Arguments
    /// * `item_id` - The ID of the item to delete.
    /// * `if_match` - Optional ETag for conditional delete (If-Match header).
    pub fn delete_item(
        mut self,
        item_id: impl Into<Cow<'static, str>>,
        if_match: Option<Etag>,
    ) -> Self {
        self.operations.push(TransactionalBatchOperation::Delete {
            id: item_id.into(),
            if_match,
        });
        self
    }

    /// Adds a patch operation to the batch.
    ///
    /// # Arguments
    /// * `item_id` - The ID of the item to patch.
    /// * `patch_document` - The patch document describing the modifications.
    pub fn patch_item(
        mut self,
        item_id: impl Into<Cow<'static, str>>,
        patch_document: PatchDocument,
    ) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(patch_document)?;
        self.operations.push(TransactionalBatchOperation::Patch {
            id: item_id.into(),
            resource_body,
            if_match: None,
        });
        Ok(self)
    }

    /// Adds a patch operation to the batch with conditional ETag.
    ///
    /// # Arguments
    /// * `item_id` - The ID of the item to patch.
    /// * `patch_document` - The patch document describing the modifications.
    /// * `if_match` - Optional ETag for conditional patch (If-Match header).
    pub fn patch_item_with_options(
        mut self,
        item_id: impl Into<Cow<'static, str>>,
        patch_document: PatchDocument,
        if_match: Option<Etag>,
    ) -> Result<Self, serde_json::Error> {
        let resource_body = serde_json::to_value(patch_document)?;
        self.operations.push(TransactionalBatchOperation::Patch {
            id: item_id.into(),
            resource_body,
            if_match,
        });
        Ok(self)
    }
}

/// Represents a single operation in a transactional batch.
#[derive(Clone, SafeDebug, Serialize, Deserialize, PartialEq, Eq)]
#[safe(true)]
#[serde(tag = "operationType")]
pub enum TransactionalBatchOperation {
    /// Creates a new item.
    Create {
        #[serde(rename = "resourceBody")]
        resource_body: serde_json::Value,
        #[serde(rename = "ifMatch")]
        #[serde(skip_serializing_if = "Option::is_none")]
        if_match: Option<Etag>,
        #[serde(rename = "ifNoneMatch")]
        #[serde(skip_serializing_if = "Option::is_none")]
        if_none_match: Option<Etag>,
    },
    /// Reads an existing item.
    Read {
        id: Cow<'static, str>,
        #[serde(rename = "ifMatch")]
        #[serde(skip_serializing_if = "Option::is_none")]
        if_match: Option<Etag>,
        #[serde(rename = "ifNoneMatch")]
        #[serde(skip_serializing_if = "Option::is_none")]
        if_none_match: Option<Etag>,
    },
    /// Replaces an existing item.
    Replace {
        id: Cow<'static, str>,
        #[serde(rename = "resourceBody")]
        resource_body: serde_json::Value,
        #[serde(rename = "ifMatch")]
        #[serde(skip_serializing_if = "Option::is_none")]
        if_match: Option<Etag>,
        #[serde(rename = "ifNoneMatch")]
        #[serde(skip_serializing_if = "Option::is_none")]
        if_none_match: Option<Etag>,
    },
    /// Creates or replaces an item.
    Upsert {
        #[serde(rename = "resourceBody")]
        resource_body: serde_json::Value,
        #[serde(rename = "ifMatch")]
        #[serde(skip_serializing_if = "Option::is_none")]
        if_match: Option<Etag>,
        #[serde(rename = "ifNoneMatch")]
        #[serde(skip_serializing_if = "Option::is_none")]
        if_none_match: Option<Etag>,
    },
    /// Deletes an item.
    Delete {
        id: Cow<'static, str>,
        #[serde(rename = "ifMatch")]
        #[serde(skip_serializing_if = "Option::is_none")]
        if_match: Option<Etag>,
    },
    /// Patches an item.
    Patch {
        id: Cow<'static, str>,
        #[serde(rename = "resourceBody")]
        resource_body: serde_json::Value,
        #[serde(rename = "ifMatch")]
        #[serde(skip_serializing_if = "Option::is_none")]
        if_match: Option<Etag>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PatchDocument;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestItem {
        id: String,
        name: String,
    }

    #[test]
    pub fn create_batch() -> Result<(), Box<dyn std::error::Error>> {
        let batch = TransactionalBatch::new("partition1");
        assert_eq!(batch.operations().len(), 0);
        assert_eq!(batch.partition_key(), &PartitionKey::from("partition1"));
        Ok(())
    }

    #[test]
    pub fn add_create_operation() -> Result<(), Box<dyn std::error::Error>> {
        let item = TestItem {
            id: "item1".to_string(),
            name: "Test Item".to_string(),
        };
        let batch = TransactionalBatch::new("partition1").create_item(&item)?;
        assert_eq!(batch.operations().len(), 1);
        Ok(())
    }

    #[test]
    pub fn add_multiple_operations() -> Result<(), Box<dyn std::error::Error>> {
        let item1 = TestItem {
            id: "item1".to_string(),
            name: "Item 1".to_string(),
        };
        let item2 = TestItem {
            id: "item2".to_string(),
            name: "Item 2".to_string(),
        };

        let batch = TransactionalBatch::new("partition1")
            .create_item(&item1)?
            .read_item("item3")
            .replace_item("item4", &item2)?
            .delete_item("item5", None);

        assert_eq!(batch.operations().len(), 4);
        Ok(())
    }

    #[test]
    pub fn serialize_create_operation() -> Result<(), Box<dyn std::error::Error>> {
        let item = TestItem {
            id: "item1".to_string(),
            name: "Test".to_string(),
        };
        let batch = TransactionalBatch::new("partition1").create_item(&item)?;
        let operations = batch.operations();
        let serialized = serde_json::to_string(&operations)?;

        assert!(serialized.contains("\"operationType\":\"Create\""));
        assert!(serialized.contains("\"resourceBody\""));
        Ok(())
    }

    #[test]
    pub fn serialize_read_operation() -> Result<(), Box<dyn std::error::Error>> {
        let batch = TransactionalBatch::new("partition1").read_item("item1");
        let operations = batch.operations();
        let serialized = serde_json::to_string(&operations)?;

        assert!(serialized.contains("\"operationType\":\"Read\""));
        assert!(serialized.contains("\"id\":\"item1\""));
        Ok(())
    }

    #[test]
    pub fn serialize_delete_operation() -> Result<(), Box<dyn std::error::Error>> {
        let batch = TransactionalBatch::new("partition1").delete_item("item1", None);
        let operations = batch.operations();
        let serialized = serde_json::to_string(&operations)?;

        assert!(serialized.contains("\"operationType\":\"Delete\""));
        assert!(serialized.contains("\"id\":\"item1\""));
        Ok(())
    }

    #[test]
    pub fn serialize_patch_operation() -> Result<(), Box<dyn std::error::Error>> {
        let patch = PatchDocument::default().with_add("/color", "red")?;
        let batch = TransactionalBatch::new("partition1").patch_item("item1", patch)?;
        let operations = batch.operations();
        let serialized = serde_json::to_string(&operations)?;

        assert!(serialized.contains("\"operationType\":\"Patch\""));
        assert!(serialized.contains("\"id\":\"item1\""));
        assert!(serialized.contains("\"resourceBody\""));
        Ok(())
    }
}
