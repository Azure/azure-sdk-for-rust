// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Location mutation effects emitted by retry evaluation.

use crate::options::Region;

use super::{partition_key_range_id::PartitionKeyRangeId, CosmosEndpoint, UnavailableReason};

/// Location-state mutation emitted by retry evaluation.
#[derive(Clone, Debug)]
pub(crate) enum LocationEffect {
    /// Marks an endpoint temporarily unavailable.
    MarkEndpointUnavailable {
        endpoint: CosmosEndpoint,
        reason: UnavailableReason,
    },
    /// Marks a partition unavailable, applied by partition-level routing.
    MarkPartitionUnavailable(UnavailablePartition),
    /// Requests a rate-limited account metadata refresh.
    RefreshAccountProperties,
    /// Inserts or updates the hub-region cache entry for a partition.
    ///
    /// Emitted on a successful (`2xx`) response when the
    /// `x-ms-cosmos-hub-region-processing-only` latch was active and the
    /// partition key range ID is known. The next attempt of any operation
    /// on the same partition that latches the hub header will route
    /// directly to `hub_endpoint` instead of running the `403/3` discovery
    /// chain.
    CacheHubRegion {
        partition_key_range_id: PartitionKeyRangeId,
        hub_endpoint: CosmosEndpoint,
    },
    /// Advances the hub-region cache to the next preferred read endpoint
    /// for a partition after a `403/3` response with the hub-region latch
    /// active.
    ///
    /// Read-path counterpart of [`Self::MarkPartitionUnavailable`]:
    /// rotates the per-partition hub-region override without marking the
    /// endpoint generally unavailable, since the region is healthy for
    /// non-hub reads of other partitions.
    AdvanceHubRegionDiscovery {
        partition_key_range_id: PartitionKeyRangeId,
        failed_endpoint: CosmosEndpoint,
    },
}

/// Identifies a partition-region pair to mark unavailable.
#[derive(Clone, Debug)]
pub(crate) struct UnavailablePartition {
    pub partition_key_range_id: Option<PartitionKeyRangeId>,
    pub region: Option<Region>,
    pub is_read: bool,
    pub is_partitioned_resource: bool,
}
