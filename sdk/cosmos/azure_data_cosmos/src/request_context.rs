// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::regions::RegionName;
use crate::PartitionKey;
use azure_core::http::RawResponse;
use std::collections::HashMap;
use url::Url;

/// Placeholder for a resolved physical partition key range.
///
/// In a fuller implementation this would include identifiers and possibly
/// the min/max effective partition key values that define the range.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct PartitionKeyRange;

/// Carries per-request routing, partition resolution, retry and regional state.
///
/// `RequestContext` is mutated during pipeline execution to track which
/// endpoints have failed, how the request should be routed (by region index
/// or explicit endpoint), resolved partition ranges, session tokens, and
/// various internal flags influencing retries and cache refresh behavior.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct RequestContext {
    pub force_refresh_address_cache: bool,
    pub original_request_consistency_level: Option<String>, // Use enum if available
    pub quorum_selected_lsn: i64,
    pub global_committed_selected_lsn: i64,
    pub store_response: Option<RawResponse>,
    pub perform_local_refresh_on_gone_exception: bool,
    pub effective_partition_key: Option<PartitionKey>,
    pub resolved_partition_key_range: Option<PartitionKeyRange>,
    pub session_token: Option<String>,
    pub performed_background_address_refresh: bool,
    pub resolved_collection_rid: Option<String>,
    pub region_name: Option<RegionName>,
    pub local_region_request: bool,
    pub is_retry: bool,
    pub is_partition_failover_retry: bool,
    pub failed_endpoints: HashMap<Url, bool>,
    pub use_preferred_locations: Option<bool>,
    pub location_index_to_route: Option<i32>,
    pub location_endpoint_to_route: Option<Url>,
}

#[allow(dead_code)]
impl RequestContext {
    /// Marks a store endpoint as failed so subsequent retries can avoid it.
    /// In a full implementation the provided error would be inspected for
    /// status codes (e.g. 410 Gone) to trigger targeted cache refresh logic.
    pub fn add_to_failed_endpoints(
        &mut self,
        _store_exception: &str, // Replace with actual error type
        target_uri: Url,
    ) {
        self.failed_endpoints.insert(target_uri, true);
    }

    /// Routes the request to a region by its index within the preferred
    /// locations list. Clears any explicit endpoint routing state.
    pub fn route_to_location_index(&mut self, location_index: i32, use_preferred_locations: bool) {
        self.location_index_to_route = Some(location_index);
        self.use_preferred_locations = Some(use_preferred_locations);
        self.location_endpoint_to_route = None;
    }

    /// Routes the request to an explicit regional endpoint URL, disabling
    /// index-based preferred location routing for this attempt.
    pub fn route_to_location_endpoint(&mut self, location_endpoint: Url) {
        self.location_endpoint_to_route = Some(location_endpoint);
        self.location_index_to_route = None;
        self.use_preferred_locations = None;
    }

    /// Removes any explicit routing decisions, allowing default resolution.
    pub fn clear_route_to_location(&mut self) {
        self.location_index_to_route = None;
        self.location_endpoint_to_route = None;
        self.use_preferred_locations = None;
    }
}
