//------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------

use azure_core::time::Duration;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

// use tokio::sync::Notify;
use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use crate::resource_context::ResourceType;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::partition_key_range::PartitionKeyRange;
use azure_core::async_runtime::get_async_runtime;
// use tokio_util::sync::CancellationToken;
use tracing::info;
use url::Url;

/// Represents the health status of a transport address.
/// The numeric values indicate priority for replica selection (lower = healthier).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PartitionHealthStatus {
    /// The partition is marked as healthy.
    Healthy = 100,

    /// The partition is confirmed unhealthy.
    Unhealthy = 200,
}

/// This struct is used to failover single partitions to different regions.
/// The client retry policy will mark a partition as down. The PartitionKeyRangeToLocationForReadAndWrite
/// will add an override to the next read region. When the request is retried it will
/// override the default location with the new region from the PartitionKeyRangeToLocationForReadAndWrite.
pub struct GlobalPartitionEndpointManager {
    /// An instance of GlobalEndpointManager.
    global_endpoint_manager: Arc<GlobalEndpointManager>,

    /// Partition unavailability duration in seconds, before it can be considered for a refresh.
    partition_unavailability_duration_secs: i64,

    /// Partition failback refresh interval in seconds. Default is 5 minutes.
    background_connection_init_interval_secs: i64,

    /// Partition key range to failover info mapping for writes in a single master account.
    partition_key_range_to_location_for_write:
        Arc<Mutex<HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>>>,

    /// Partition key range to failover info mapping for reads in single master,
    /// and both reads and writes in multi master account.
    partition_key_range_to_location_for_read_and_write:
        Arc<Mutex<HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>>>,

    /// Flag indicating if the background connection initialization task is active.
    is_background_connection_init_active: AtomicBool,

    /// Lock for background connection initialization.
    background_connection_init_lock: Mutex<()>,

    /// Flag to determine if partition level failover is enabled (using i32 for atomic operations).
    is_partition_level_automatic_failover_enabled: AtomicI32,

    /// Flag to determine if partition level circuit breaker is enabled.
    is_partition_level_circuit_breaker_enabled: AtomicI32,
}

impl std::fmt::Debug for GlobalPartitionEndpointManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalPartitionEndpointManager")
            .field("global_endpoint_manager", &self.global_endpoint_manager)
            .field(
                "partition_unavailability_duration_secs",
                &self.partition_unavailability_duration_secs,
            )
            .field(
                "background_connection_init_interval_secs",
                &self.background_connection_init_interval_secs,
            )
            .field(
                "partition_key_range_to_location_for_write",
                &self.partition_key_range_to_location_for_write,
            )
            .field(
                "partition_key_range_to_location_for_read_and_write",
                &self.partition_key_range_to_location_for_read_and_write,
            )
            .field(
                "is_background_connection_init_active",
                &self.is_background_connection_init_active,
            )
            .field(
                "is_partition_level_automatic_failover_enabled",
                &self.is_partition_level_automatic_failover_enabled,
            )
            .field(
                "is_partition_level_circuit_breaker_enabled",
                &self.is_partition_level_circuit_breaker_enabled,
            )
            .finish()
    }
}

impl GlobalPartitionEndpointManager {
    /// Creates a new instance of GlobalPartitionEndpointManager.
    pub fn new(
        global_endpoint_manager: Arc<GlobalEndpointManager>,
        is_partition_level_failover_enabled: bool,
        is_partition_level_circuit_breaker_enabled: bool,
    ) -> Arc<Self> {
        let instance = Arc::new(Self {
            global_endpoint_manager,
            partition_unavailability_duration_secs:
                Self::get_allowed_partition_unavailability_duration_secs(5),
            background_connection_init_interval_secs:
                Self::get_stale_partition_unavailability_refresh_interval_secs(300),
            partition_key_range_to_location_for_write: Arc::from(Mutex::new(HashMap::new())),
            partition_key_range_to_location_for_read_and_write: Arc::from(Mutex::new(
                HashMap::new(),
            )),
            is_background_connection_init_active: AtomicBool::new(false),
            background_connection_init_lock: Mutex::new(()),
            is_partition_level_automatic_failover_enabled: AtomicI32::new(
                if is_partition_level_failover_enabled {
                    1
                } else {
                    0
                },
            ),
            is_partition_level_circuit_breaker_enabled: AtomicI32::new(
                if is_partition_level_circuit_breaker_enabled {
                    1
                } else {
                    0
                },
            ),
        });

        instance.initialize_and_start_circuit_breaker_failback_background_refresh();
        instance
    }

    fn get_allowed_partition_unavailability_duration_secs(default: i64) -> i64 {
        std::env::var("AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn get_stale_partition_unavailability_refresh_interval_secs(default: i64) -> i64 {
        std::env::var(
            "AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS",
        )
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
    }

    /// Initialize and start the background connection periodic refresh task.
    fn initialize_and_start_circuit_breaker_failback_background_refresh(self: &Arc<Self>) {
        if self
            .is_background_connection_init_active
            .load(Ordering::SeqCst)
        {
            return;
        }

        let _guard = self.background_connection_init_lock.lock();
        if self
            .is_background_connection_init_active
            .load(Ordering::SeqCst)
        {
            return;
        }

        self.is_background_connection_init_active
            .store(true, Ordering::SeqCst);

        let self_clone = Arc::clone(self);
        // Use the runtime-agnostic spawn from azure_core
        // Explicitly drop the JoinHandle since this is a fire-and-forget background task
        drop(get_async_runtime().spawn(Box::pin(async move {
            self_clone.initiate_circuit_breaker_failback_loop().await;
        })));
    }

    /// Runs a continuous loop to refresh connections to failed backend replicas.
    #[allow(dead_code)]
    async fn initiate_circuit_breaker_failback_loop(self: Arc<Self>) {
        let interval = Duration::seconds(self.background_connection_init_interval_secs);

        loop {
            // Use the runtime-agnostic sleep from azure_core
            get_async_runtime().sleep(interval).await;

            info!("GlobalPartitionEndpointManager: InitiateCircuitBreakerFailbackLoop() trying to get address and open connections for failed locations.");

            if let Err(e) = self
                .try_open_connection_to_unhealthy_endpoints_and_initiate_failback()
                .await
            {
                tracing::error!("GlobalPartitionEndpointManager: InitiateCircuitBreakerFailbackLoop() - Unable to get address and open connections. Exception: {}", e);
            }
        }
    }

    /// Attempts to open connections to unhealthy endpoints and initiates failback if successful.
    async fn try_open_connection_to_unhealthy_endpoints_and_initiate_failback(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("GlobalPartitionEndpointManager: InitiateCircuitBreakerFailbackLoop() - Attempting to open connections to unhealthy endpoints and initiate failback.");

        let mut pk_range_to_endpoint_mappings: HashMap<
            PartitionKeyRange,
            (String, String, PartitionHealthStatus),
        > = HashMap::new();

        // Scope the guard so it's dropped before any async operations
        {
            let guard = self
                .partition_key_range_to_location_for_read_and_write
                .lock()
                .map_err(|e| e.to_string())?;
            for (pk_range, partition_failover) in guard.iter() {
                let pk_range = pk_range.clone();

                let (first_request_failure_time, _) =
                    partition_failover.snapshot_partition_failover_timestamps();

                if Instant::now().duration_since(first_request_failure_time)
                    > Duration::seconds(self.partition_unavailability_duration_secs)
                {
                    let original_failed_location = partition_failover.first_failed_location.clone();

                    pk_range_to_endpoint_mappings.insert(
                        pk_range,
                        (
                            partition_failover.collection_rid.clone(),
                            original_failed_location,
                            PartitionHealthStatus::Unhealthy,
                        ),
                    );
                }
            }
        } // guard is dropped here

        if !pk_range_to_endpoint_mappings.is_empty() {
            // Mark endpoints as healthy directly
            Self::mark_endpoints_to_healthy(&mut pk_range_to_endpoint_mappings);

            for (pk_range, (_, original_failed_location, current_health_state)) in
                pk_range_to_endpoint_mappings
            {
                if current_health_state == PartitionHealthStatus::Healthy {
                    info!(
                        "Initiating Failback to endpoint: {}, for partition key range: {:?}",
                        original_failed_location, pk_range
                    );
                    self.partition_key_range_to_location_for_read_and_write
                        .lock()
                        .unwrap()
                        .remove(&pk_range);
                }
            }
        }

        Ok(())
    }

    pub fn mark_endpoints_to_healthy(
        pk_range_uri_mappings: &mut HashMap<
            PartitionKeyRange,
            (String, String, PartitionHealthStatus),
        >,
    ) {
        for (pk_range, mapping) in pk_range_uri_mappings.iter_mut() {
            info!(
                "Un-deterministically marking the original failed endpoint: {}, for the PkRange: {}, collectionRid: {} back to healthy.",
                mapping.0,
                pk_range.id,
                mapping.1
            );

            mapping.2 = PartitionHealthStatus::Healthy;
        }
    }

    /// Determines if a request is eligible for per-partition automatic failover.
    pub fn is_request_eligible_for_per_partition_automatic_failover(
        &self,
        request: &CosmosRequest,
    ) -> bool {
        self.is_partition_level_automatic_failover_enabled
            .load(Ordering::SeqCst)
            == 1
            && !request.is_read_only_request()
            && !self
                .global_endpoint_manager
                .can_support_multiple_write_locations(request.resource_type, request.operation_type)
    }

    /// Determines if a request is eligible for partition-level circuit breaker.
    pub fn is_request_eligible_for_partition_level_circuit_breaker(
        &self,
        request: &CosmosRequest,
    ) -> bool {
        self.is_partition_level_circuit_breaker_enabled
            .load(Ordering::SeqCst)
            == 1
            && (request.is_read_only_request()
                || (!request.is_read_only_request()
                    && self
                        .global_endpoint_manager
                        .can_support_multiple_write_locations(
                            request.resource_type,
                            request.operation_type,
                        )))
    }

    /// Validates if the given request is eligible for partition failover.
    fn is_request_eligible_for_partition_failover(
        &self,
        request: &CosmosRequest,
        should_validate_failed_location: bool,
    ) -> Option<(PartitionKeyRange, Option<Url>)> {
        if !self.is_partition_level_automatic_failover_enabled()
            && !self.is_partition_level_circuit_breaker_enabled()
        {
            return None;
        }

        let request_context = &request.request_context;

        if !self.can_use_partition_level_failover_locations(request) {
            return None;
        }

        let partition_key_range = request_context.resolved_partition_key_range.clone()?;

        let failed_location = if should_validate_failed_location {
            let location = request_context.location_endpoint_to_route.clone()?;
            Some(location)
        } else {
            None
        };

        Some((partition_key_range, failed_location))
    }

    /// Determines if partition level failover locations can be used for the given request.
    fn can_use_partition_level_failover_locations(&self, request: &CosmosRequest) -> bool {
        if self.global_endpoint_manager.read_endpoints().len() <= 1 {
            return false;
        }

        matches!(request.resource_type, ResourceType::Documents)
            || (request.resource_type == ResourceType::StoredProcedures
                && request.operation_type == OperationType::Execute)
    }

    /// Attempts to route the request to a partition level override location if available.
    fn try_route_request_for_partition_level_override(
        &self,
        partition_key_range: &PartitionKeyRange,
        request: &mut CosmosRequest,
        partition_key_range_to_location_mapping: &Arc<
            Mutex<HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>>,
        >,
    ) -> bool {
        if let Some(partition_key_range_failover) = partition_key_range_to_location_mapping
            .lock()
            .unwrap()
            .get(partition_key_range)
        {
            // let partition_key_range_failover = entry.value();

            if self.is_request_eligible_for_partition_level_circuit_breaker(request)
                && !partition_key_range_failover
                    .can_circuit_breaker_trigger_partition_failover(request.is_read_only_request())
            {
                return false;
            }

            let triggered_by = if self
                .is_partition_level_automatic_failover_enabled
                .load(Ordering::SeqCst)
                == 1
            {
                "Automatic Failover"
            } else {
                "Circuit Breaker"
            };

            info!(
                "Attempting to route request for partition level override triggered by {}, for operation type: {:?}. URI: {}, PartitionKeyRange: {:?}",
                triggered_by,
                request.operation_type,
                partition_key_range_failover.current,
                partition_key_range.id
            );

            request.request_context.route_to_location_endpoint(
                partition_key_range_failover
                    .current
                    .clone()
                    .parse()
                    .unwrap(),
            );

            return true;
        }

        false
    }

    /// Returns whether partition level automatic failover is enabled.
    pub fn is_partition_level_automatic_failover_enabled(&self) -> bool {
        self.is_partition_level_automatic_failover_enabled
            .load(Ordering::SeqCst)
            == 1
    }

    /// Returns whether partition level circuit breaker is enabled.
    pub fn is_partition_level_circuit_breaker_enabled(&self) -> bool {
        self.is_partition_level_circuit_breaker_enabled
            .load(Ordering::SeqCst)
            == 1
    }

    pub fn try_add_partition_level_location_override(&self, request: &mut CosmosRequest) -> bool {
        let Some((partition_key_range, _)) =
            self.is_request_eligible_for_partition_failover(request, false)
        else {
            return false;
        };

        if self.is_request_eligible_for_partition_level_circuit_breaker(request) {
            return self.try_route_request_for_partition_level_override(
                &partition_key_range,
                request,
                &self.partition_key_range_to_location_for_read_and_write,
            );
        } else if self.is_request_eligible_for_per_partition_automatic_failover(request) {
            return self.try_route_request_for_partition_level_override(
                &partition_key_range,
                request,
                &self.partition_key_range_to_location_for_write,
            );
        }

        false
    }

    // TODO: Need to implement this method.
    /// Marks the current location unavailable for write. Future requests will be routed
    /// to the next location if available.
    ///
    /// # Arguments
    /// * `request` - The document service request to process.
    ///
    /// # Returns
    /// `true` if the endpoint was successfully marked as unavailable and a new location was set,
    /// `false` otherwise.
    pub fn try_mark_endpoint_unavailable_for_partition_key_range(
        &self,
        request: &CosmosRequest,
    ) -> bool {
        // Validate request eligibility and extract partition key range and failed location
        let Some((partition_key_range, failed_location)) =
            self.is_request_eligible_for_partition_failover(request, true)
        else {
            return false;
        };

        // Ensure we have a valid failed location (required when shouldValidateFailedLocation is true)
        let Some(failed_location) = failed_location else {
            return false;
        };

        let failed_location_str = failed_location.as_str();

        if self.is_request_eligible_for_partition_level_circuit_breaker(request) {
            // For multi master write accounts, since all the regions are treated as write regions,
            // the next locations to fail over will be the preferred read regions that are configured
            // in the application preferred regions in the CosmosClientOptions.
            let next_locations: Vec<String> = self
                .global_endpoint_manager
                .read_endpoints()
                .iter()
                .map(|u| u.to_string())
                .collect();

            return self.try_add_or_update_partition_failover_info_and_move_to_next_location(
                &partition_key_range,
                failed_location_str,
                &next_locations,
                request,
                &self.partition_key_range_to_location_for_read_and_write,
            );
        } else if self.is_request_eligible_for_per_partition_automatic_failover(request) {
            // For any single master write accounts, the next locations to fail over will be
            // the read regions configured at the account level.
            let next_locations: Vec<String> = self
                .global_endpoint_manager
                .account_read_endpoints()
                .iter()
                .map(|u| u.to_string())
                .collect();

            return self.try_add_or_update_partition_failover_info_and_move_to_next_location(
                &partition_key_range,
                failed_location_str,
                &next_locations,
                request,
                &self.partition_key_range_to_location_for_write,
            );
        }

        tracing::info!(
        "Partition level override was skipped since the request did not meet the minimum requirements."
    );
        false
    }

    /// Attempts to add or update the partition failover information and move to the next available location.
    ///
    /// This method checks if the current location for the partition key range has failed and updates
    /// the failover information to route the request to the next available location. If all locations
    /// have been tried, it removes the failover information for the partition key range.
    ///
    /// # Arguments
    /// * `partition_key_range` - The partition key range for which the failover information is being updated.
    /// * `failed_location` - The URI of the failed location.
    /// * `next_locations` - A slice of URIs representing the next available locations.
    /// * `request` - The document service request being routed.
    /// * `partition_key_range_to_location_mapping` - The mapping of partition key ranges to their failover information.
    ///
    /// # Returns
    /// `true` if the failover information was successfully updated and the request was routed to a new location,
    /// `false` otherwise.
    fn try_add_or_update_partition_failover_info_and_move_to_next_location(
        &self,
        partition_key_range: &PartitionKeyRange,
        failed_location: &str,
        next_locations: &[String],
        request: &CosmosRequest,
        partition_key_range_to_location_mapping: &Arc<
            Mutex<HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>>,
        >,
    ) -> bool {
        let triggered_by = if self
            .is_partition_level_automatic_failover_enabled
            .load(Ordering::SeqCst)
            == 1
        {
            "Automatic Failover"
        } else {
            "Circuit Breaker"
        };

        // Get the resolved collection RID from the request context
        let collection_rid = request
            .clone()
            .request_context
            .resolved_collection_rid
            .unwrap();

        // Get or insert the partition failover info and try to move to next location
        let mut guard = partition_key_range_to_location_mapping.lock().unwrap();
        let partition_failover = guard.entry(partition_key_range.clone()).or_insert_with(|| {
            PartitionKeyRangeFailoverInfo::new(collection_rid, failed_location.to_string())
        });

        // Will return true if it was able to update to a new region
        if partition_failover.try_move_next_location(next_locations, failed_location) {
            tracing::info!(
                "Partition level override triggered by {}, added to new location for {:?}. \
             PartitionKeyRange: {:?}, failedLocation: {}, new location: {}",
                triggered_by,
                request.operation_type,
                partition_key_range,
                failed_location,
                partition_failover.current
            );

            return true;
        }

        // All the locations have been tried. Remove the override information
        tracing::info!(
        "Partition level override removed for {:?}. PartitionKeyRange: {:?}, failedLocation: {}",
        request.operation_type,
        partition_key_range,
        failed_location
    );

        // Need to drop the guard before re-acquiring the lock
        drop(guard);

        partition_key_range_to_location_mapping
            .lock()
            .unwrap()
            .remove(partition_key_range);

        false
    }

    pub(crate) fn increment_request_failure_counter_and_check_if_partition_can_failover(
        &self,
        request: &CosmosRequest,
    ) -> bool {
        let Some((partition_key_range, Some(failed_location))) =
            self.is_request_eligible_for_partition_failover(request, true)
        else {
            return false;
        };

        let collection_rid = request
            .request_context
            .resolved_collection_rid
            .clone()
            .unwrap();

        let is_read_only = request.is_read_only_request();

        if self.is_request_eligible_for_per_partition_automatic_failover(request) {
            let mut guard = self
                .partition_key_range_to_location_for_write
                .lock()
                .unwrap();
            let partition_failover = guard.entry(partition_key_range).or_insert_with(|| {
                PartitionKeyRangeFailoverInfo::new(
                    collection_rid.clone(),
                    failed_location.to_string(),
                )
            });
            partition_failover.increment_request_failure_counts(is_read_only, Instant::now());
            partition_failover.can_circuit_breaker_trigger_partition_failover(is_read_only)
        } else {
            let mut guard = self
                .partition_key_range_to_location_for_read_and_write
                .lock()
                .unwrap();
            let partition_failover = guard.entry(partition_key_range).or_insert_with(|| {
                PartitionKeyRangeFailoverInfo::new(collection_rid, failed_location.to_string())
            });
            partition_failover.increment_request_failure_counts(is_read_only, Instant::now());
            partition_failover.can_circuit_breaker_trigger_partition_failover(is_read_only)
        }
    }
}

/// Contains failover information for a partition key range.
#[derive(Debug)]
pub struct PartitionKeyRangeFailoverInfo {
    counter_lock: Mutex<()>,
    timestamp_lock: Mutex<()>,
    failed_locations: Arc<Mutex<HashMap<String, Instant>>>,
    timeout_counter_reset_window: Duration,
    read_request_failure_counter_threshold: i32,
    write_request_failure_counter_threshold: i32,
    last_request_failure_time: RwLock<Instant>,
    consecutive_read_request_failure_count: AtomicI32,
    consecutive_write_request_failure_count: AtomicI32,

    pub current: String,
    pub first_failed_location: String,
    pub collection_rid: String,
    pub first_request_failure_time: Instant,
}

impl PartitionKeyRangeFailoverInfo {
    pub fn new(collection_rid: String, current_location: String) -> Self {
        Self {
            collection_rid,
            current: current_location.clone(),
            first_failed_location: current_location,
            failed_locations: Arc::from(Mutex::new(HashMap::new())),
            consecutive_read_request_failure_count: AtomicI32::new(0),
            consecutive_write_request_failure_count: AtomicI32::new(0),
            read_request_failure_counter_threshold:
                Self::get_circuit_breaker_consecutive_failure_count_for_reads(2),
            write_request_failure_counter_threshold:
                Self::get_circuit_breaker_consecutive_failure_count_for_writes(5),
            timeout_counter_reset_window: Duration::seconds(
                Self::get_circuit_breaker_timeout_counter_reset_window_mins(5) * 60,
            ),
            first_request_failure_time: Instant::now(),
            last_request_failure_time: RwLock::new(Instant::now()),
            counter_lock: Mutex::new(()),
            timestamp_lock: Mutex::new(()),
        }
    }

    fn get_circuit_breaker_consecutive_failure_count_for_reads(default: i32) -> i32 {
        std::env::var("AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_READS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn get_circuit_breaker_consecutive_failure_count_for_writes(default: i32) -> i32 {
        std::env::var("AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_WRITES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn get_circuit_breaker_timeout_counter_reset_window_mins(default: i64) -> i64 {
        std::env::var("AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    pub fn try_move_next_location(&mut self, locations: &[String], failed_location: &str) -> bool {
        if failed_location != self.current {
            return true;
        }

        let mut guard = self.failed_locations.lock().unwrap();

        if failed_location != self.current {
            return true;
        }

        for location in locations {
            if self.current == *location {
                continue;
            }

            if guard.contains_key(location) {
                continue;
            }

            guard.insert(failed_location.to_string(), Instant::now());
            self.current = location.clone();
            return true;
        }

        false
    }

    pub fn can_circuit_breaker_trigger_partition_failover(
        &self,
        is_read_only_request: bool,
    ) -> bool {
        let (read_count, write_count) = self.snapshot_consecutive_request_failure_count();

        if is_read_only_request {
            read_count > self.read_request_failure_counter_threshold
        } else {
            write_count > self.write_request_failure_counter_threshold
        }
    }

    pub fn increment_request_failure_counts(
        &self,
        is_read_only_request: bool,
        current_time: Instant,
    ) {
        let (_, last_failure_time) = self.snapshot_partition_failover_timestamps();

        if current_time.duration_since(last_failure_time) > self.timeout_counter_reset_window {
            self.consecutive_read_request_failure_count
                .store(0, Ordering::SeqCst);
            self.consecutive_write_request_failure_count
                .store(0, Ordering::SeqCst);
        }

        if is_read_only_request {
            self.consecutive_read_request_failure_count
                .fetch_add(1, Ordering::SeqCst);
        } else {
            self.consecutive_write_request_failure_count
                .fetch_add(1, Ordering::SeqCst);
        }

        *self.last_request_failure_time.write().unwrap() = current_time;
    }

    pub fn snapshot_partition_failover_timestamps(&self) -> (Instant, Instant) {
        let _guard = self.timestamp_lock.lock();
        (
            self.first_request_failure_time,
            *self.last_request_failure_time.read().unwrap(),
        )
    }

    pub fn snapshot_consecutive_request_failure_count(&self) -> (i32, i32) {
        let _guard = self.counter_lock.lock();
        (
            self.consecutive_read_request_failure_count
                .load(Ordering::SeqCst),
            self.consecutive_write_request_failure_count
                .load(Ordering::SeqCst),
        )
    }
}
