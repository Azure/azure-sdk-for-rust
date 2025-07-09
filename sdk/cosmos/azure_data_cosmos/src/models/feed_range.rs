// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};

/// Represents a feed range for a container.
///
/// A feed range represents a contiguous range of partition key values that can be used
/// to scope queries or change feed operations to a specific subset of data.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeedRange {
    /// The minimum partition key value (inclusive) for this feed range.
    pub min_inclusive: String,
    /// The maximum partition key value (exclusive) for this feed range.
    pub max_exclusive: String,
}

/// Represents a partition key range with its ID and feed range information.
///
/// This is used internally to represent the full partition key range information
/// returned by the Cosmos DB service.
///
/// Partition Key Ranges are only used by the query engine as part of executing queries.
/// Applications should use [`FeedRange`](crate::models::FeedRange) when specifying ranges for queries or change feed operations.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PartitionKeyRange {
    /// The ID of the partition key range.
    pub id: String,
    /// The feed range information for this partition key range.
    #[serde(flatten)]
    pub range: FeedRange,
}
