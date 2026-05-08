// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation targeting for Cosmos DB operations.

use crate::models::{FeedRange, PartitionKey};

/// Describes how an operation targets the partition key space.
///
/// Every [`CosmosOperation`](crate::models::CosmosOperation) carries an `OperationTarget`
/// that determines how the driver routes the request:
///
/// - [`None`](Self::None) — account/database/container-level operations that have no
///   partition scope (e.g., create database, read container).
/// - [`PartitionKey`](Self::PartitionKey) — operations scoped to a single logical
///   partition. Always executed as a single request (point operation).
/// - [`FeedRange`](Self::FeedRange) — operations scoped to an EPK range that may
///   span one or more physical partitions (e.g., cross-partition queries).
#[derive(Clone, Debug)]
pub enum OperationTarget {
    /// No partition scope. Used for account, database, and container-level operations.
    ///
    /// It is illegal to use this target for item-level operations inside a container.
    None,

    /// Scoped to a single logical partition key.
    ///
    /// This can always be satisfied by a single request node — no fan-out required.
    PartitionKey(PartitionKey),

    /// Scoped to a feed range (EPK range).
    ///
    /// The range may cover one or more physical partitions, including the full
    /// container key space ([`FeedRange::full()`]).
    FeedRange(FeedRange),
}

impl OperationTarget {
    /// Returns `true` if the target has a partition reference (i.e., it is not [`None`](Self::None)).
    pub fn has_partition_reference(&self) -> bool {
        !matches!(self, OperationTarget::None)
    }
}
