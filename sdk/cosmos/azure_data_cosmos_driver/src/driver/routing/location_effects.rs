// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Location mutation effects emitted by retry evaluation.

use crate::options::Region;

use super::{CosmosEndpoint, UnavailableReason};

/// Location-state mutation emitted by retry evaluation.
#[derive(Clone, Debug)]
pub(crate) enum LocationEffect {
    /// Marks an endpoint temporarily unavailable.
    MarkEndpointUnavailable {
        endpoint: CosmosEndpoint,
        reason: UnavailableReason,
    },
    /// Marks a partition unavailable (Step 2 bridge; fully used in Step 3).
    #[allow(dead_code)]
    MarkPartitionUnavailable(UnavailablePartition),
    /// Requests a rate-limited account metadata refresh.
    RefreshAccountProperties,
}

/// Identifies a partition-region pair to mark unavailable.
#[derive(Clone, Debug)]
pub(crate) struct UnavailablePartition {
    #[allow(dead_code)]
    pub partition_key_range_id: String,
    #[allow(dead_code)]
    pub region: Option<Region>,
    #[allow(dead_code)]
    pub is_read: bool,
}
