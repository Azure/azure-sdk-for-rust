// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fixed, shared test data. Specs compare results against these datasets.

use azure_data_cosmos::PartitionKey;
use serde::{Deserialize, Serialize};

/// The item model used by every built-in dataset.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Item {
    /// The item ID.
    pub id: String,
    /// The partition key value, stored at `/pk`.
    pub pk: String,
    /// A per-item value, unique within a dataset.
    pub value: i64,
}

impl Item {
    /// Creates an item.
    pub fn new(id: impl Into<String>, pk: impl Into<String>, value: i64) -> Self {
        Self {
            id: id.into(),
            pk: pk.into(),
            value,
        }
    }

    /// The item's partition key.
    pub fn partition_key(&self) -> PartitionKey {
        PartitionKey::from(self.pk.clone())
    }
}

/// An ordered, immutable set of items.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Dataset(Vec<Item>);

impl Dataset {
    /// A dataset with no items.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Every item, in dataset order.
    pub fn all(&self) -> Vec<Item> {
        self.0.clone()
    }

    /// The first `n` items in dataset order.
    pub fn take(&self, n: usize) -> Vec<Item> {
        self.0.iter().take(n).cloned().collect()
    }

    /// The item with the given ID.
    ///
    /// # Panics
    ///
    /// Panics if the dataset has no such item; that is a bug in the spec.
    pub fn get(&self, id: &str) -> Item {
        self.0
            .iter()
            .find(|item| item.id == id)
            .cloned()
            .unwrap_or_else(|| panic!("dataset has no item '{id}'"))
    }

    /// The number of items.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the dataset has no items.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Built-in item datasets.
pub mod items {
    use super::{Dataset, Item};

    /// 24 items, `item-00` through `item-23`, spread round-robin across six
    /// partition key values `pk-0` through `pk-5`. `value` equals the item number,
    /// so ordering by `id` and by `value` agree.
    pub fn basic_items() -> Dataset {
        Dataset(
            (0..24)
                .map(|n| Item::new(format!("item-{n:02}"), format!("pk-{}", n % 6), n))
                .collect(),
        )
    }
}
