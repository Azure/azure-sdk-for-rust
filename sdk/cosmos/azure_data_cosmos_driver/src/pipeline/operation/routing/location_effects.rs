// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Location mutation effects emitted by retry evaluation.

use std::ops::{Deref, DerefMut};

use crate::{
    options::Region,
    pipeline::operation::routing::{
        partition_key_range_id::PartitionKeyRangeId, CosmosEndpoint, UnavailableReason,
    },
};

/// Ordered routing feedback whose failure evidence is local until publication.
#[derive(Clone, Debug, Default)]
pub(crate) struct LocationEffects(Vec<LocationEffect>);

impl LocationEffects {
    /// Visits failed regions without treating refresh or hub-discovery effects as failures.
    pub(crate) fn unavailable_regions(&self) -> impl Iterator<Item = &Region> {
        self.0.iter().filter_map(|effect| match effect {
            LocationEffect::MarkPartitionUnavailable(partition) => partition.region.as_ref(),
            LocationEffect::MarkEndpointUnavailable { endpoint, .. } => endpoint.region(),
            LocationEffect::RefreshAccountProperties
            | LocationEffect::CacheHubRegion { .. }
            | LocationEffect::AdvanceHubRegionDiscovery { .. } => None,
        })
    }
}

impl From<Vec<LocationEffect>> for LocationEffects {
    /// Takes ownership of feedback without copying its elements.
    fn from(effects: Vec<LocationEffect>) -> Self {
        Self(effects)
    }
}

impl FromIterator<LocationEffect> for LocationEffects {
    /// Collects feedback in observation order.
    fn from_iter<T: IntoIterator<Item = LocationEffect>>(effects: T) -> Self {
        Self(effects.into_iter().collect())
    }
}

impl IntoIterator for LocationEffects {
    type Item = LocationEffect;
    type IntoIter = std::vec::IntoIter<LocationEffect>;

    /// Transfers effects to the publication or filtering stage.
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for LocationEffects {
    type Target = Vec<LocationEffect>;

    /// Borrows the ordered feedback collection.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LocationEffects {
    /// Allows the owning stage to accumulate or clear feedback.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Whether selection can revisit regions failed by the current operation.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum InFlightFailures {
    Exclude,
    Ignore,
}

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

#[cfg(test)]
mod tests {
    use super::{
        CosmosEndpoint, LocationEffect, LocationEffects, Region, UnavailablePartition,
        UnavailableReason,
    };
    use crate::pipeline::operation::{
        recovery::state::OperationRetryState, routing::InFlightFailures,
    };

    #[test]
    fn exclusions_distinguish_failure_evidence_from_hub_discovery() {
        let east = CosmosEndpoint::regional(
            "eastus".into(),
            "https://test-eastus.documents.azure.com/".parse().unwrap(),
        );
        let west = CosmosEndpoint::regional(
            "westus".into(),
            "https://test-westus.documents.azure.com/".parse().unwrap(),
        );
        let north = CosmosEndpoint::regional(
            "northeurope".into(),
            "https://test-northeurope.documents.azure.com/"
                .parse()
                .unwrap(),
        );
        let effects: LocationEffects = vec![
            LocationEffect::MarkPartitionUnavailable(UnavailablePartition {
                partition_key_range_id: None,
                region: east.region().cloned(),
                is_read: false,
                is_partitioned_resource: true,
            }),
            LocationEffect::MarkEndpointUnavailable {
                endpoint: west.clone(),
                reason: UnavailableReason::TransportError,
            },
            LocationEffect::RefreshAccountProperties,
            LocationEffect::CacheHubRegion {
                partition_key_range_id: "0".parse().unwrap(),
                hub_endpoint: north.clone(),
            },
            LocationEffect::AdvanceHubRegionDiscovery {
                partition_key_range_id: "0".parse().unwrap(),
                failed_endpoint: north.clone(),
            },
        ]
        .into();
        assert_eq!(
            effects
                .unavailable_regions()
                .cloned()
                .collect::<Vec<Region>>(),
            vec!["eastus".into(), "westus".into()]
        );

        let mut state = OperationRetryState::initial(0, false, vec!["westus".into()], 3, 2);
        state.pending_write_effects = effects;
        assert!(state.is_region_excluded(east.region(), InFlightFailures::Exclude));
        assert!(!state.is_region_excluded(east.region(), InFlightFailures::Ignore));
        assert!(state.is_region_excluded(west.region(), InFlightFailures::Exclude));
        assert!(state.is_region_excluded(west.region(), InFlightFailures::Ignore));
        assert!(!state.is_region_excluded(north.region(), InFlightFailures::Exclude));
        assert!(!state.is_region_excluded(None, InFlightFailures::Exclude));
    }
}
