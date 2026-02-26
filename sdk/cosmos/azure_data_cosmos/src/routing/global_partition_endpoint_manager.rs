//------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------

use azure_core::time::Duration;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Arc, Mutex, RwLock, Weak};
use std::time::Instant;

use crate::background_task_manager::BackgroundTaskManager;
use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use crate::resource_context::ResourceType;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::partition_key_range::PartitionKeyRange;
use tracing::info;
use url::Url;

/// Default duration (in seconds) a partition must remain marked unavailable before
/// the background failback loop considers it eligible for health re-evaluation.
const DEFAULT_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_SECS: i64 = 5;

/// Default interval (in seconds) at which the background failback loop runs to check
/// whether previously failed partitions can be restored to healthy status.
const DEFAULT_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_SECS: i64 = 300;

/// Default threshold of consecutive read failures before the circuit breaker trips.
const DEFAULT_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_READS: i32 = 2;

/// Default threshold of consecutive write failures before the circuit breaker trips.
const DEFAULT_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_WRITES: i32 = 5;

/// Default window (in minutes) after which the circuit breaker resets its failure
/// counters if no new failure has been recorded.
const DEFAULT_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_MINS: i64 = 5;

/// Represents the health status of a transport address.
/// The numeric values indicate priority for partition selection (lower = healthier).
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
#[derive(Debug)]
pub struct GlobalPartitionEndpointManager {
    /// An instance of GlobalEndpointManager.
    global_endpoint_manager: Arc<GlobalEndpointManager>,

    /// Partition unavailability duration in seconds, before it can be considered for a refresh.
    partition_unavailability_duration_secs: i64,

    /// Partition failback refresh interval in seconds. Default is 5 minutes.
    background_connection_init_interval_secs: i64,

    /// Partition key range to failover info mapping for writes in a single master account.
    partition_key_range_to_location_for_write:
        Arc<RwLock<HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>>>,

    /// Partition key range to failover info mapping for reads in single master,
    /// and both reads and writes in multi master account.
    partition_key_range_to_location_for_read_and_write:
        Arc<RwLock<HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>>>,

    /// Flag indicating if the background connection initialization task is active.
    background_connection_init_active: AtomicBool,

    /// Flag to determine if partition level failover is enabled.
    partition_level_automatic_failover_enabled: AtomicBool,

    /// Flag to determine if partition level circuit breaker is enabled.
    partition_level_circuit_breaker_enabled: AtomicBool,

    /// Manages background tasks and signals them to stop when dropped.
    background_task_manager: BackgroundTaskManager,
}

impl GlobalPartitionEndpointManager {
    /// Creates a new instance of [`GlobalPartitionEndpointManager`].
    ///
    /// Initializes partition-level failover maps, reads environment variable overrides
    /// for partition unavailability duration and refresh intervals, and spawns the
    /// background circuit-breaker failback loop if partition-level failover is enabled.
    ///
    /// # Arguments
    /// * `global_endpoint_manager` - Shared reference to the [`GlobalEndpointManager`]
    ///   used for resolving read/write endpoints and multi-write support.
    /// * `partition_level_failover_enabled` - Whether per-partition automatic failover
    ///   (for single-master write accounts) should be enabled.
    /// * `partition_level_circuit_breaker_enabled` - Whether the partition-level circuit
    ///   breaker (for multi-master accounts and reads) should be enabled.
    ///
    /// # Returns
    /// An `Arc<Self>` that can be shared across threads and async tasks.
    pub fn new(
        global_endpoint_manager: Arc<GlobalEndpointManager>,
        partition_level_failover_enabled: bool,
        partition_level_circuit_breaker_enabled: bool,
    ) -> Arc<Self> {
        let instance = Arc::new(Self {
            global_endpoint_manager,
            partition_unavailability_duration_secs:
                Self::allowed_partition_unavailability_duration_secs(
                    DEFAULT_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_SECS,
                ),
            background_connection_init_interval_secs:
                Self::stale_partition_unavailability_refresh_interval_secs(
                    DEFAULT_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_SECS,
                ),
            partition_key_range_to_location_for_write: Arc::new(RwLock::new(HashMap::new())),
            partition_key_range_to_location_for_read_and_write: Arc::new(RwLock::new(
                HashMap::new(),
            )),
            background_connection_init_active: AtomicBool::new(false),
            partition_level_automatic_failover_enabled: AtomicBool::new(
                partition_level_failover_enabled,
            ),
            partition_level_circuit_breaker_enabled: AtomicBool::new(
                partition_level_circuit_breaker_enabled,
            ),
            background_task_manager: BackgroundTaskManager::new(),
        });

        instance.initialize_and_start_circuit_breaker_failback_background_refresh();
        instance
    }

    /// Returns the allowed partition unavailability duration in seconds.
    ///
    /// This value controls how long a partition must remain marked unavailable before
    /// the background failback loop considers it eligible for health re-evaluation.
    /// Reads from the `AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS`
    /// environment variable, falling back to `default` if the variable is unset or not parseable.
    fn allowed_partition_unavailability_duration_secs(default: i64) -> i64 {
        std::env::var("AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Returns the stale partition unavailability refresh interval in seconds.
    ///
    /// This determines how frequently the background failback loop runs to check
    /// whether previously failed partitions can be restored to healthy status.
    /// Reads from the `AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS`
    /// environment variable, falling back to `default` if the variable is unset or not parseable.
    fn stale_partition_unavailability_refresh_interval_secs(default: i64) -> i64 {
        std::env::var(
            "AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS",
        )
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
    }

    /// Initializes and starts the background circuit-breaker failback periodic refresh task.
    ///
    /// Uses an atomic `compare_exchange` on [`background_connection_init_active`] to ensure
    /// only one background task is ever spawned, regardless of how many times this method
    /// is called. The spawned task runs [`initiate_circuit_breaker_failback_loop`] indefinitely
    /// to periodically re-evaluate whether failed partitions can be marked healthy again.
    fn initialize_and_start_circuit_breaker_failback_background_refresh(self: &Arc<Self>) {
        // Atomically try to set from false to true.
        // If it was already true, another thread already started the task.
        if self
            .background_connection_init_active
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return;
        }

        let weak_self = Arc::downgrade(self);
        // Spawn via BackgroundTaskManager so the task is tracked and will be
        // canceled when the manager (and thus the client) is dropped.
        // We capture a Weak<Self> (not Arc<Self>) to avoid a reference cycle
        // that would prevent the GlobalPartitionEndpointManager from ever
        // being dropped.
        self.background_task_manager.spawn(Box::pin(async move {
            Self::initiate_circuit_breaker_failback_loop(weak_self).await;
        }));
    }

    /// Runs a loop that periodically attempts to fail back partitions to their
    /// original (previously failed) endpoints.
    ///
    /// On each iteration the loop sleeps for [`background_connection_init_interval_secs`]
    /// seconds and then calls [`initiate_failback_to_unhealthy_endpoints`]. Any errors
    /// during the failback attempt are logged but do not terminate the loop.
    ///
    /// The loop exits when `weak_self.upgrade()` returns `None` (all strong
    /// `Arc` references are gone), which happens when the owning client is
    /// dropped. Dropping the client drops the [`BackgroundTaskManager`], which
    /// drops the stored future, cancelling this task.
    async fn initiate_circuit_breaker_failback_loop(weak_self: Weak<Self>) {
        // Briefly upgrade to read the interval, then release the strong ref
        // so it does not keep Self alive across the sleep.
        let interval = match weak_self.upgrade() {
            Some(strong) => Duration::seconds(strong.background_connection_init_interval_secs),
            None => return,
        };

        loop {
            // Use the runtime-agnostic sleep from azure_core
            azure_core::async_runtime::get_async_runtime()
                .sleep(interval)
                .await;

            // Upgrade the Weak ref for this iteration only. If it fails, the
            // manager has been dropped and we should exit.
            let strong = match weak_self.upgrade() {
                Some(s) => s,
                None => {
                    info!("GlobalPartitionEndpointManager: background failback loop exiting because the client has been dropped.");
                    return;
                }
            };

            info!("GlobalPartitionEndpointManager: initiate_circuit_breaker_failback_loop() un-deterministically marking the failed partitions back to healthy.");

            if let Err(e) = strong.initiate_failback_to_unhealthy_endpoints().await {
                tracing::error!("GlobalPartitionEndpointManager: initiate_circuit_breaker_failback_loop() - failed to mark the failed partitions back to healthy. Exception: {}", e);
            }
            // `strong` is dropped here, releasing the temporary strong ref
            // before the next sleep.
        }
    }

    /// Attempts to initiate failback to previously failed endpoints non-deterministically.
    ///
    /// Scans `partition_key_range_to_location_for_read_and_write` for partitions whose
    /// first failure occurred more than [`partition_unavailability_duration_secs`] ago.
    /// Eligible partitions are marked healthy via [`mark_endpoints_to_healthy`], and their
    /// override entries are removed, allowing future requests to be routed back to the
    /// original endpoint.
    ///
    /// # Errors
    /// Returns an error if the read lock on the partition map is poisoned.
    async fn initiate_failback_to_unhealthy_endpoints(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("GlobalPartitionEndpointManager: initiate_circuit_breaker_failback_loop() - Attempting to mark the failed partitions back to healthy and initiate failback.");

        let mut pk_range_to_endpoint_mappings: HashMap<
            PartitionKeyRange,
            (String, String, PartitionHealthStatus),
        > = HashMap::new();

        // Scope the guard so it's dropped before any async operations
        {
            let guard = self
                .partition_key_range_to_location_for_read_and_write
                .read()
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
                        "Initiating failback to endpoint: {}, for partition key range: {:?}",
                        original_failed_location, pk_range
                    );
                    self.partition_key_range_to_location_for_read_and_write
                        .write()
                        .unwrap()
                        .remove(&pk_range);
                }
            }
        }

        Ok(())
    }

    /// Marks all partition key range endpoint mappings in the provided map as [`PartitionHealthStatus::Healthy`].
    ///
    /// This is a non-deterministic health restoration: once the unavailability window
    /// has elapsed, the endpoint is optimistically assumed to be healthy again.
    /// The actual verification happens when subsequent requests are routed back to
    /// the original endpoint.
    ///
    /// # Arguments
    /// * `pk_range_uri_mappings` - Mutable reference to the map of partition key ranges
    ///   to their `(collection_rid, original_failed_location, health_status)` tuples.
    fn mark_endpoints_to_healthy(
        pk_range_uri_mappings: &mut HashMap<
            PartitionKeyRange,
            (String, String, PartitionHealthStatus),
        >,
    ) {
        for (pk_range, mapping) in pk_range_uri_mappings.iter_mut() {
            info!(
                "Un-deterministically marking the original failed endpoint: {}, for the PkRange: {}, collectionRid: {} back to healthy.",
                mapping.1,
                pk_range.id,
                mapping.0
            );

            mapping.2 = PartitionHealthStatus::Healthy;
        }
    }

    /// Determines if a request is eligible for per-partition automatic failover.
    ///
    /// A request qualifies when **all** of the following are true:
    /// 1. Partition-level automatic failover is enabled.
    /// 2. The request is a **write** operation (not read-only).
    /// 3. The account is a **single-master** account (does not support multiple write locations).
    ///
    /// This path handles failover of write requests on single-master accounts to read regions.
    ///
    /// # Arguments
    /// * `request` - The Cosmos request to evaluate.
    ///
    /// # Returns
    /// `true` if the request is eligible for per-partition automatic failover, `false` otherwise.
    pub fn is_request_eligible_for_per_partition_automatic_failover(
        &self,
        request: &CosmosRequest,
    ) -> bool {
        self.partition_level_automatic_failover_enabled
            .load(Ordering::SeqCst)
            && !request.is_read_only_request()
            && !self
                .global_endpoint_manager
                .can_support_multiple_write_locations(request.resource_type, request.operation_type)
    }

    /// Determines if a request is eligible for partition-level circuit breaker.
    ///
    /// A request qualifies when **all** of the following are true:
    /// 1. Partition-level circuit breaker is enabled.
    /// 2. The request is **either** a read-only operation, **or** a write operation on a
    ///    **multi-master** account that supports multiple write locations.
    ///
    /// This path handles failover of reads (any account type) and writes on multi-master
    /// accounts to alternate preferred read regions.
    ///
    /// # Arguments
    /// * `request` - The Cosmos request to evaluate.
    ///
    /// # Returns
    /// `true` if the request is eligible for partition-level circuit breaker, `false` otherwise.
    pub fn is_request_eligible_for_partition_level_circuit_breaker(
        &self,
        request: &CosmosRequest,
    ) -> bool {
        self.partition_level_circuit_breaker_enabled
            .load(Ordering::SeqCst)
            && (request.resource_type == ResourceType::Documents
                || (request.resource_type == ResourceType::StoredProcedures
                    && request.operation_type == OperationType::Execute))
            && (request.is_read_only_request()
                || (!request.is_read_only_request()
                    && self
                        .global_endpoint_manager
                        .can_support_multiple_write_locations(
                            request.resource_type,
                            request.operation_type,
                        )))
    }

    /// Validates whether the given request is eligible for any form of partition-level failover.
    ///
    /// Performs the following checks:
    /// 1. At least one partition-level failover mode (automatic failover or circuit breaker)
    ///    must be enabled.
    /// 2. The request must target a resource type that supports partition-level failover
    ///    (see [`can_use_partition_level_failover_locations`]).
    /// 3. A resolved partition key range must exist on the request context.
    /// 4. If `should_validate_failed_location` is `true`, a valid failed location endpoint
    ///    must be present on the request context.
    ///
    /// # Arguments
    /// * `request` - The Cosmos request to validate.
    /// * `should_validate_failed_location` - When `true`, the method also extracts and
    ///   returns the failed location URL from the request context.
    ///
    /// # Returns
    /// `Some((partition_key_range, optional_failed_location))` if eligible, `None` otherwise.
    fn is_request_eligible_for_partition_failover(
        &self,
        request: &CosmosRequest,
        should_validate_failed_location: bool,
    ) -> Option<(PartitionKeyRange, Option<Url>)> {
        if !self.partition_level_automatic_failover_enabled()
            && !self.partition_level_circuit_breaker_enabled()
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

    /// Determines if partition-level failover locations can be applied to the given request.
    ///
    /// Partition-level failover only makes sense when there are multiple read endpoints
    /// available to fail over to. The request must also target one of the supported
    /// resource types:
    /// - [`ResourceType::Documents`] (CRUD on items)
    /// - [`ResourceType::StoredProcedures`] with [`OperationType::Execute`]
    ///
    /// # Arguments
    /// * `request` - The Cosmos request to check.
    ///
    /// # Returns
    /// `true` if the request can leverage partition-level failover locations, `false` otherwise.
    fn can_use_partition_level_failover_locations(&self, request: &CosmosRequest) -> bool {
        if self.global_endpoint_manager.read_endpoints().len() <= 1 {
            return false;
        }

        matches!(request.resource_type, ResourceType::Documents)
            || (request.resource_type == ResourceType::StoredProcedures
                && request.operation_type == OperationType::Execute)
    }

    /// Attempts to route the request to a partition-level override location if one exists.
    ///
    /// Looks up the partition key range in `partition_key_range_to_location_mapping`. If an
    /// override entry is found:
    /// - For circuit-breaker eligible requests, it additionally verifies that the failure
    ///   counters have exceeded the threshold before applying the override.
    /// - Updates [`request.request_context`] to route to the overridden location.
    ///
    /// # Arguments
    /// * `partition_key_range` - The partition key range to look up.
    /// * `request` - The Cosmos request whose routing will be overridden.
    /// * `partition_key_range_to_location_mapping` - The mapping to consult for override info.
    ///
    /// # Returns
    /// `true` if the request was successfully routed to an override location, `false` otherwise.
    fn try_route_request_for_partition_level_override(
        &self,
        partition_key_range: &PartitionKeyRange,
        request: &mut CosmosRequest,
        partition_key_range_to_location_mapping: &Arc<
            RwLock<HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>>,
        >,
    ) -> bool {
        if let Some(partition_key_range_failover) = partition_key_range_to_location_mapping
            .read()
            .unwrap()
            .get(partition_key_range)
        {
            if self.is_request_eligible_for_partition_level_circuit_breaker(request)
                && !partition_key_range_failover
                    .can_circuit_breaker_trigger_partition_failover(request.is_read_only_request())
            {
                return false;
            }

            // TODO - Move this to new type and capture in DiagnosticsContext when porting to driver
            let triggered_by = if self
                .partition_level_automatic_failover_enabled
                .load(Ordering::SeqCst)
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

            if let Ok(endpoint) = partition_key_range_failover.current.parse() {
                request.request_context.route_to_location_endpoint(endpoint);

                return true;
            } else {
                info!(
                    "Skipping partition level override due to invalid URI in failover info: {}",
                    partition_key_range_failover.current
                );
            }
        }

        false
    }

    /// Returns whether partition level automatic failover is enabled.
    pub fn partition_level_automatic_failover_enabled(&self) -> bool {
        self.partition_level_automatic_failover_enabled
            .load(Ordering::SeqCst)
    }

    /// Returns whether partition level circuit breaker is enabled.
    pub fn partition_level_circuit_breaker_enabled(&self) -> bool {
        self.partition_level_circuit_breaker_enabled
            .load(Ordering::SeqCst)
    }

    /// Checks if partition level failover is enabled.
    ///
    /// Returns `true` if either partition level circuit breaker or partition level
    /// automatic failover is enabled.
    pub fn partition_level_failover_enabled(&self) -> bool {
        self.partition_level_circuit_breaker_enabled()
            || self.partition_level_automatic_failover_enabled()
    }

    /// Attempts to apply a partition-level location override to the request before it is sent.
    ///
    /// This is the main entry point called by the retry pipeline to check whether an
    /// existing partition failover override exists for the request's partition key range.
    /// Depending on the request's eligibility, it delegates to
    /// [`try_route_request_for_partition_level_override`] using either:
    /// - `partition_key_range_to_location_for_read_and_write` (circuit breaker path), or
    /// - `partition_key_range_to_location_for_write` (automatic failover path).
    ///
    /// # Arguments
    /// * `request` - The mutable Cosmos request whose routing may be overridden.
    ///
    /// # Returns
    /// `true` if an override was applied and the request will be routed to an alternate
    /// location, `false` if no override exists or the request is ineligible.
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
            RwLock<HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>>,
        >,
    ) -> bool {
        if request.request_context.resolved_collection_rid.is_none() {
            return false;
        }

        let triggered_by = if self
            .partition_level_automatic_failover_enabled
            .load(Ordering::SeqCst)
        {
            "Automatic Failover"
        } else {
            "Circuit Breaker"
        };

        // Get the resolved collection RID from the request context, if available
        let collection_rid = request
            .request_context
            .resolved_collection_rid
            .clone()
            .unwrap();

        // Get or insert the partition failover info and try to move to next location
        let mut guard = partition_key_range_to_location_mapping.write().unwrap();
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

        // Remove while still holding the write guard (no need to drop and re-acquire)
        guard.remove(partition_key_range);

        false
    }

    /// Increments the request failure counter for the partition and checks whether the
    /// circuit breaker threshold has been exceeded to trigger partition-level failover.
    ///
    /// This method is called after a request to a specific partition key range fails.
    /// It records the failure in the appropriate failover info map (write-only for
    /// automatic failover, read-and-write for circuit breaker) and then evaluates
    /// whether the consecutive failure count has crossed the configured threshold.
    ///
    /// # Arguments
    /// * `request` - The failed Cosmos request, used to extract the partition key range,
    ///   collection RID, failed location, and operation type.
    ///
    /// # Returns
    /// `true` if the failure counters indicate the partition should be failed over
    /// to an alternate region, `false` otherwise.
    pub(crate) fn increment_request_failure_counter_and_check_if_partition_can_failover(
        &self,
        request: &CosmosRequest,
    ) -> bool {
        let Some((partition_key_range, Some(failed_location))) =
            self.is_request_eligible_for_partition_failover(request, true)
        else {
            return false;
        };

        if request.request_context.resolved_collection_rid.is_none() {
            return false;
        }

        let collection_rid = request
            .request_context
            .resolved_collection_rid
            .clone()
            .unwrap();

        let is_read_only = request.is_read_only_request();

        if self.is_request_eligible_for_per_partition_automatic_failover(request) {
            let mut guard = self
                .partition_key_range_to_location_for_write
                .write()
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
                .write()
                .unwrap();
            let partition_failover = guard.entry(partition_key_range).or_insert_with(|| {
                PartitionKeyRangeFailoverInfo::new(collection_rid, failed_location.to_string())
            });
            partition_failover.increment_request_failure_counts(is_read_only, Instant::now());
            partition_failover.can_circuit_breaker_trigger_partition_failover(is_read_only)
        }
    }

    /// Sets whether per partition automatic failover is enabled.
    ///
    /// Only logs when the value actually changes to avoid noisy repeated logs
    /// during periodic account refresh.
    pub fn configure_partition_level_automatic_failover(&self, is_enabled: bool) {
        let previous = self
            .partition_level_automatic_failover_enabled
            .swap(is_enabled, Ordering::SeqCst);
        if previous != is_enabled {
            info!(
                "Per partition automatic failover enablement flag changed: {} -> {}",
                previous, is_enabled
            );
        }
    }

    /// Sets whether per partition circuit breaker is enabled.
    ///
    /// Only logs when the value actually changes to avoid noisy repeated logs
    /// during periodic account refresh.
    pub fn configure_per_partition_circuit_breaker(&self, is_enabled: bool) {
        let previous = self
            .partition_level_circuit_breaker_enabled
            .swap(is_enabled, Ordering::SeqCst);
        if previous != is_enabled {
            info!(
                "Per partition circuit breaker enablement flag changed: {} -> {}",
                previous, is_enabled
            );
        }
    }
}

/// Contains failover tracking information for a single partition key range.
///
/// Each instance tracks which endpoint the partition is currently routed to, which
/// locations have already been tried, and consecutive failure counts for both reads
/// and writes. The circuit breaker uses these counters to decide when to trigger
/// a partition-level failover to the next available region.
#[derive(Debug)]
pub struct PartitionKeyRangeFailoverInfo {
    /// Set of locations that have already been tried and failed, along with the
    /// timestamp when each location was marked as failed. Protected by a [`Mutex`]
    /// because it is mutated during failover transitions.
    failed_locations: Mutex<HashMap<String, Instant>>,

    /// Duration window after which consecutive failure counters are reset to zero.
    /// If the time between the current failure and the last recorded failure exceeds
    /// this window, the counters restart from zero.
    timeout_counter_reset_window: Duration,

    /// The consecutive read failure count threshold that must be exceeded
    /// before the circuit breaker triggers a partition failover for read requests.
    read_request_failure_counter_threshold: i32,

    /// The consecutive write failure count threshold that must be exceeded
    /// before the circuit breaker triggers a partition failover for write requests.
    write_request_failure_counter_threshold: i32,

    /// Timestamp of the most recent request failure for this partition key range.
    /// Protected by [`RwLock`] because it is read frequently but only written on failure.
    last_request_failure_time: RwLock<Instant>,

    /// Running count of consecutive read request failures. Atomic for lock-free
    /// incrementing across concurrent request threads.
    consecutive_read_request_failure_count: AtomicI32,

    /// Running count of consecutive write request failures. Atomic for lock-free
    /// incrementing across concurrent request threads.
    consecutive_write_request_failure_count: AtomicI32,

    /// The endpoint URI that this partition key range is currently routed to.
    pub current: String,

    /// The endpoint URI that originally failed, triggering the failover chain.
    /// Used by the failback loop to know which endpoint to restore.
    pub first_failed_location: String,

    /// The collection resource ID (RID) associated with this partition key range.
    pub collection_rid: String,

    /// Timestamp of the very first request failure that initiated this failover entry.
    /// Used by the failback loop to determine if enough time has passed for health re-evaluation.
    pub first_request_failure_time: Instant,
}

impl PartitionKeyRangeFailoverInfo {
    /// Creates a new [`PartitionKeyRangeFailoverInfo`] for the given collection and location.
    ///
    /// Initializes all failure counters to zero, reads environment variable overrides
    /// for threshold and window configurations, and records the current instant as
    /// both the first and last failure timestamps.
    ///
    /// # Arguments
    /// * `collection_rid` - The resource ID of the collection this partition belongs to.
    /// * `current_location` - The endpoint URI of the location that is currently being
    ///   used (and has just failed).
    pub fn new(collection_rid: String, current_location: String) -> Self {
        Self {
            collection_rid,
            current: current_location.clone(),
            first_failed_location: current_location,
            failed_locations: Mutex::new(HashMap::new()),
            consecutive_read_request_failure_count: AtomicI32::new(0),
            consecutive_write_request_failure_count: AtomicI32::new(0),
            read_request_failure_counter_threshold:
                Self::circuit_breaker_consecutive_failure_count_for_reads(
                    DEFAULT_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_READS,
                ),
            write_request_failure_counter_threshold:
                Self::circuit_breaker_consecutive_failure_count_for_writes(
                    DEFAULT_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_WRITES,
                ),
            timeout_counter_reset_window: Duration::seconds(
                Self::circuit_breaker_timeout_counter_reset_window_mins(
                    DEFAULT_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_MINS,
                ) * 60,
            ),
            first_request_failure_time: Instant::now(),
            last_request_failure_time: RwLock::new(Instant::now()),
        }
    }

    /// Returns the consecutive read failure count threshold for the circuit breaker.
    ///
    /// Reads from the `AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_READS`
    /// environment variable, falling back to `default` if the variable is unset or not parseable.
    fn circuit_breaker_consecutive_failure_count_for_reads(default: i32) -> i32 {
        std::env::var("AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_READS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Returns the consecutive write failure count threshold for the circuit breaker.
    ///
    /// Reads from the `AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_WRITES`
    /// environment variable, falling back to `default` if the variable is unset or not parseable.
    fn circuit_breaker_consecutive_failure_count_for_writes(default: i32) -> i32 {
        std::env::var("AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_WRITES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Returns the timeout counter reset window in minutes for the circuit breaker.
    ///
    /// If the elapsed time between two consecutive failures exceeds this window,
    /// the read and write failure counters are reset to zero. Reads from the
    /// `AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES`
    /// environment variable, falling back to `default` if unset or not parseable.
    fn circuit_breaker_timeout_counter_reset_window_mins(default: i64) -> i64 {
        std::env::var("AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Attempts to move this partition's routing to the next available location.
    ///
    /// If `failed_location` no longer matches `self.current` (another thread already
    /// moved it), returns `true` immediately. Otherwise, iterates through `locations`
    /// and picks the first one that:
    /// - Is not the current location.
    /// - Has not already been tried (not in `failed_locations`).
    ///
    /// Records `failed_location` in the `failed_locations` set and updates `self.current`
    /// to the new location.
    ///
    /// # Arguments
    /// * `locations` - Ordered list of candidate endpoint URIs to try.
    /// * `failed_location` - The endpoint URI that just failed.
    ///
    /// # Returns
    /// `true` if a new location was selected (or someone else already moved), `false` if
    /// all locations have been exhausted.
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

    /// Checks whether the circuit breaker should trigger a partition-level failover.
    ///
    /// Compares the current consecutive failure counts against the configured thresholds.
    /// For read-only requests, checks the read counter; for write requests, checks the
    /// write counter.
    ///
    /// # Arguments
    /// * `is_read_only_request` - Whether the triggering request is read-only.
    ///
    /// # Returns
    /// `true` if the consecutive failure count exceeds the threshold, indicating that
    /// a failover should be triggered, `false` otherwise.
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

    /// Increments the consecutive request failure counter for this partition.
    ///
    /// If the time since the last recorded failure exceeds [`timeout_counter_reset_window`],
    /// both read and write counters are reset to zero before incrementing. This prevents
    /// stale failures from accumulating across long idle periods.
    ///
    /// # Arguments
    /// * `is_read_only_request` - Whether the failed request was read-only. Determines
    ///   which counter (read or write) is incremented.
    /// * `current_time` - The timestamp of the current failure.
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

    /// Returns a snapshot of the partition failover timestamps.
    ///
    /// # Returns
    /// A tuple of `(first_request_failure_time, last_request_failure_time)` capturing
    /// when the first and most recent failures occurred for this partition key range.
    pub fn snapshot_partition_failover_timestamps(&self) -> (Instant, Instant) {
        (
            self.first_request_failure_time,
            *self.last_request_failure_time.read().unwrap(),
        )
    }

    /// Returns a snapshot of the consecutive request failure counters.
    ///
    /// # Returns
    /// A tuple of `(read_failure_count, write_failure_count)` representing the current
    /// consecutive failure counts for read and write requests respectively.
    pub fn snapshot_consecutive_request_failure_count(&self) -> (i32, i32) {
        (
            self.consecutive_read_request_failure_count
                .load(Ordering::SeqCst),
            self.consecutive_write_request_failure_count
                .load(Ordering::SeqCst),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cosmos_request::CosmosRequest;
    use crate::models::AccountRegion;
    use crate::operation_context::OperationType;
    use crate::regions::RegionName;
    use crate::resource_context::{ResourceLink, ResourceType};
    use crate::routing::global_endpoint_manager::GlobalEndpointManager;
    use crate::routing::partition_key_range::PartitionKeyRange;
    use azure_core::http::Pipeline;
    use std::sync::Arc;
    use std::time::Instant;

    // -----------------------------------------------------------------------
    // Helper functions
    // -----------------------------------------------------------------------

    fn create_test_pipeline() -> Pipeline {
        Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::http::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        )
    }

    fn create_single_region_manager() -> Arc<GlobalEndpointManager> {
        GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![RegionName::from("West US")],
            vec![],
            create_test_pipeline(),
        )
    }

    fn create_multi_region_manager() -> Arc<GlobalEndpointManager> {
        let manager = GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![RegionName::from("West US"), RegionName::from("East US")],
            vec![],
            create_test_pipeline(),
        );

        let west = AccountRegion {
            name: RegionName::from("West US"),
            database_account_endpoint: "https://test-westus.documents.azure.com".parse().unwrap(),
        };
        let east = AccountRegion {
            name: RegionName::from("East US"),
            database_account_endpoint: "https://test-eastus.documents.azure.com".parse().unwrap(),
        };

        manager.update_location_cache(vec![west.clone(), east.clone()], vec![west, east]);
        manager
    }

    fn create_three_region_manager() -> Arc<GlobalEndpointManager> {
        let manager = GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![
                RegionName::from("West US"),
                RegionName::from("East US"),
                RegionName::from("Central US"),
            ],
            vec![],
            create_test_pipeline(),
        );

        let west = AccountRegion {
            name: RegionName::from("West US"),
            database_account_endpoint: "https://test-westus.documents.azure.com".parse().unwrap(),
        };
        let east = AccountRegion {
            name: RegionName::from("East US"),
            database_account_endpoint: "https://test-eastus.documents.azure.com".parse().unwrap(),
        };
        let central = AccountRegion {
            name: RegionName::from("Central US"),
            database_account_endpoint: "https://test-centralus.documents.azure.com"
                .parse()
                .unwrap(),
        };

        manager.update_location_cache(
            vec![west.clone(), east.clone(), central.clone()],
            vec![west, east, central],
        );
        manager
    }

    /// Creates a multi-region manager that simulates a single-master account:
    /// one write endpoint (West US) and two read endpoints (West US + East US).
    fn create_single_master_multi_region_manager() -> Arc<GlobalEndpointManager> {
        let manager = GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![RegionName::from("West US"), RegionName::from("East US")],
            vec![],
            create_test_pipeline(),
        );

        let west = AccountRegion {
            name: RegionName::from("West US"),
            database_account_endpoint: "https://test-westus.documents.azure.com".parse().unwrap(),
        };
        let east = AccountRegion {
            name: RegionName::from("East US"),
            database_account_endpoint: "https://test-eastus.documents.azure.com".parse().unwrap(),
        };

        // Single write location, multiple read locations
        manager.update_location_cache(vec![west.clone()], vec![west, east]);
        manager
    }

    fn create_read_request() -> CosmosRequest {
        let resource_link = ResourceLink::root(ResourceType::Documents);
        let mut request = CosmosRequest::builder(OperationType::Read, resource_link)
            .build()
            .unwrap();
        request.request_context.location_endpoint_to_route =
            Some("https://test-westus.documents.azure.com/".parse().unwrap());
        request.request_context.resolved_partition_key_range =
            Some(PartitionKeyRange::new("0".into(), "".into(), "FF".into()));
        request.request_context.resolved_collection_rid = Some("dbs/db1/colls/coll1".into());
        request
    }

    fn create_write_request() -> CosmosRequest {
        let resource_link = ResourceLink::root(ResourceType::Documents);
        let mut request = CosmosRequest::builder(OperationType::Create, resource_link)
            .build()
            .unwrap();
        request.request_context.location_endpoint_to_route =
            Some("https://test-westus.documents.azure.com/".parse().unwrap());
        request.request_context.resolved_partition_key_range =
            Some(PartitionKeyRange::new("0".into(), "".into(), "FF".into()));
        request.request_context.resolved_collection_rid = Some("dbs/db1/colls/coll1".into());
        request
    }

    fn create_stored_procedure_execute_request() -> CosmosRequest {
        let resource_link = ResourceLink::root(ResourceType::StoredProcedures);
        let mut request = CosmosRequest::builder(OperationType::Execute, resource_link)
            .build()
            .unwrap();
        request.request_context.location_endpoint_to_route =
            Some("https://test-westus.documents.azure.com/".parse().unwrap());
        request.request_context.resolved_partition_key_range =
            Some(PartitionKeyRange::new("0".into(), "".into(), "FF".into()));
        request.request_context.resolved_collection_rid = Some("dbs/db1/colls/coll1".into());
        request
    }

    fn create_database_request() -> CosmosRequest {
        let resource_link = ResourceLink::root(ResourceType::Databases);
        let mut request = CosmosRequest::builder(OperationType::Read, resource_link)
            .build()
            .unwrap();
        request.request_context.location_endpoint_to_route =
            Some("https://test-westus.documents.azure.com/".parse().unwrap());
        request.request_context.resolved_partition_key_range =
            Some(PartitionKeyRange::new("0".into(), "".into(), "FF".into()));
        request.request_context.resolved_collection_rid = Some("dbs/db1/colls/coll1".into());
        request
    }

    // -----------------------------------------------------------------------
    // PartitionHealthStatus tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_health_status_values() {
        assert_eq!(PartitionHealthStatus::Healthy as i32, 100);
        assert_eq!(PartitionHealthStatus::Unhealthy as i32, 200);
    }

    #[tokio::test]
    async fn test_health_status_equality() {
        assert_eq!(
            PartitionHealthStatus::Healthy,
            PartitionHealthStatus::Healthy
        );
        assert_ne!(
            PartitionHealthStatus::Healthy,
            PartitionHealthStatus::Unhealthy
        );
    }

    // -----------------------------------------------------------------------
    // PartitionKeyRangeFailoverInfo tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_failover_info_new_initializes_correctly() {
        let info = PartitionKeyRangeFailoverInfo::new(
            "rid1".to_string(),
            "https://loc1.documents.azure.com/".to_string(),
        );

        assert_eq!(info.collection_rid, "rid1");
        assert_eq!(info.current, "https://loc1.documents.azure.com/");
        assert_eq!(
            info.first_failed_location,
            "https://loc1.documents.azure.com/"
        );

        let (read_count, write_count) = info.snapshot_consecutive_request_failure_count();
        assert_eq!(read_count, 0);
        assert_eq!(write_count, 0);
    }

    #[tokio::test]
    async fn test_failover_info_timestamps_initialized_to_now() {
        let before = Instant::now();
        let info = PartitionKeyRangeFailoverInfo::new("rid".into(), "https://loc.com/".into());
        let after = Instant::now();

        let (first, last) = info.snapshot_partition_failover_timestamps();
        assert!(first >= before && first <= after);
        assert!(last >= before && last <= after);
    }

    #[tokio::test]
    async fn test_try_move_next_location_moves_to_first_available() {
        let mut info =
            PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        let locations = vec![
            "https://loc1.com/".to_string(),
            "https://loc2.com/".to_string(),
            "https://loc3.com/".to_string(),
        ];

        let result = info.try_move_next_location(&locations, "https://loc1.com/");
        assert!(result);
        assert_eq!(info.current, "https://loc2.com/");
    }

    #[tokio::test]
    async fn test_try_move_next_location_skips_current_location() {
        let mut info =
            PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        // Only the current location is available
        let locations = vec!["https://loc1.com/".to_string()];

        let result = info.try_move_next_location(&locations, "https://loc1.com/");
        assert!(!result);
        assert_eq!(info.current, "https://loc1.com/");
    }

    #[tokio::test]
    async fn test_try_move_next_location_returns_true_if_already_moved() {
        let mut info =
            PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        // Move to loc2 first
        let locations = vec![
            "https://loc1.com/".to_string(),
            "https://loc2.com/".to_string(),
        ];
        info.try_move_next_location(&locations, "https://loc1.com/");
        assert_eq!(info.current, "https://loc2.com/");

        // Now try to move again with loc1 as failed — but current is already loc2
        let result = info.try_move_next_location(&locations, "https://loc1.com/");
        assert!(result);
        // Current should remain loc2 (already moved away from loc1)
        assert_eq!(info.current, "https://loc2.com/");
    }

    #[tokio::test]
    async fn test_try_move_next_location_sequential_failover() {
        let mut info =
            PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        let locations = vec![
            "https://loc1.com/".to_string(),
            "https://loc2.com/".to_string(),
            "https://loc3.com/".to_string(),
        ];

        // First failover: loc1 -> loc2
        assert!(info.try_move_next_location(&locations, "https://loc1.com/"));
        assert_eq!(info.current, "https://loc2.com/");

        // Second failover: loc2 -> loc3
        assert!(info.try_move_next_location(&locations, "https://loc2.com/"));
        assert_eq!(info.current, "https://loc3.com/");

        // Third failover: loc3 -> no more locations
        assert!(!info.try_move_next_location(&locations, "https://loc3.com/"));
        assert_eq!(info.current, "https://loc3.com/");
    }

    #[tokio::test]
    async fn test_try_move_next_location_empty_locations() {
        let mut info =
            PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        let result = info.try_move_next_location(&[], "https://loc1.com/");
        assert!(!result);
    }

    #[tokio::test]
    async fn test_can_circuit_breaker_trigger_reads_below_threshold() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        // Default read threshold is 2, counter starts at 0
        assert!(!info.can_circuit_breaker_trigger_partition_failover(true));
    }

    #[tokio::test]
    async fn test_can_circuit_breaker_trigger_reads_at_threshold() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        // Increment read counter to threshold (2)
        info.consecutive_read_request_failure_count
            .store(2, Ordering::SeqCst);

        // At threshold, not above — should not trigger
        assert!(!info.can_circuit_breaker_trigger_partition_failover(true));
    }

    #[tokio::test]
    async fn test_can_circuit_breaker_trigger_reads_above_threshold() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        // Default read threshold is 10; store 11 so that 11 > 10 triggers the breaker
        info.consecutive_read_request_failure_count
            .store(11, Ordering::SeqCst);

        assert!(info.can_circuit_breaker_trigger_partition_failover(true));
    }

    #[tokio::test]
    async fn test_can_circuit_breaker_trigger_writes_below_threshold() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        // Default write threshold is 5, counter starts at 0
        assert!(!info.can_circuit_breaker_trigger_partition_failover(false));
    }

    #[tokio::test]
    async fn test_can_circuit_breaker_trigger_writes_at_threshold() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        // Increment write counter to threshold (5)
        info.consecutive_write_request_failure_count
            .store(5, Ordering::SeqCst);

        // At threshold, not above — should not trigger
        assert!(!info.can_circuit_breaker_trigger_partition_failover(false));
    }

    #[tokio::test]
    async fn test_can_circuit_breaker_trigger_writes_above_threshold() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        // Increment write counter above threshold (> 5)
        info.consecutive_write_request_failure_count
            .store(6, Ordering::SeqCst);

        assert!(info.can_circuit_breaker_trigger_partition_failover(false));
    }

    #[tokio::test]
    async fn test_can_circuit_breaker_read_count_does_not_affect_write_check() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        // High read count should not trigger write failover
        info.consecutive_read_request_failure_count
            .store(100, Ordering::SeqCst);
        assert!(!info.can_circuit_breaker_trigger_partition_failover(false));

        // High write count should not trigger read failover
        info.consecutive_write_request_failure_count
            .store(100, Ordering::SeqCst);
        info.consecutive_read_request_failure_count
            .store(0, Ordering::SeqCst);
        assert!(!info.can_circuit_breaker_trigger_partition_failover(true));
    }

    #[tokio::test]
    async fn test_increment_read_failure_count() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());
        let now = Instant::now();

        info.increment_request_failure_counts(true, now);
        info.increment_request_failure_counts(true, now);
        info.increment_request_failure_counts(true, now);

        let (read_count, write_count) = info.snapshot_consecutive_request_failure_count();
        assert_eq!(read_count, 3);
        assert_eq!(write_count, 0);
    }

    #[tokio::test]
    async fn test_increment_write_failure_count() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());
        let now = Instant::now();

        info.increment_request_failure_counts(false, now);
        info.increment_request_failure_counts(false, now);

        let (read_count, write_count) = info.snapshot_consecutive_request_failure_count();
        assert_eq!(read_count, 0);
        assert_eq!(write_count, 2);
    }

    #[tokio::test]
    async fn test_increment_mixed_read_and_write_failures() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());
        let now = Instant::now();

        info.increment_request_failure_counts(true, now);
        info.increment_request_failure_counts(false, now);
        info.increment_request_failure_counts(true, now);
        info.increment_request_failure_counts(false, now);
        info.increment_request_failure_counts(false, now);

        let (read_count, write_count) = info.snapshot_consecutive_request_failure_count();
        assert_eq!(read_count, 2);
        assert_eq!(write_count, 3);
    }

    #[tokio::test]
    async fn test_increment_updates_last_failure_time() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        let (_, time_before) = info.snapshot_partition_failover_timestamps();

        // Sleep briefly to ensure time advances
        std::thread::sleep(std::time::Duration::from_millis(10));
        let later = Instant::now();
        info.increment_request_failure_counts(true, later);

        let (_, time_after) = info.snapshot_partition_failover_timestamps();
        assert!(time_after > time_before);
    }

    #[tokio::test]
    async fn test_increment_resets_counters_when_timeout_window_exceeded() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        // Add some failures at current time
        let now = Instant::now();
        info.increment_request_failure_counts(true, now);
        info.increment_request_failure_counts(true, now);
        info.increment_request_failure_counts(false, now);

        let (read_count, write_count) = info.snapshot_consecutive_request_failure_count();
        assert_eq!(read_count, 2);
        assert_eq!(write_count, 1);

        // Simulate a time far in the future (beyond the 5 min default window)
        // The timeout_counter_reset_window is 5 * 60 = 300 seconds
        let far_future = now + std::time::Duration::from_secs(400);
        info.increment_request_failure_counts(true, far_future);

        // After reset + 1 new read failure
        let (read_count, write_count) = info.snapshot_consecutive_request_failure_count();
        assert_eq!(read_count, 1);
        assert_eq!(write_count, 0);
    }

    #[tokio::test]
    async fn test_increment_does_not_reset_within_timeout_window() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        let now = Instant::now();
        info.increment_request_failure_counts(true, now);
        info.increment_request_failure_counts(true, now);

        // Within the window (< 300 seconds)
        let soon = now + std::time::Duration::from_secs(100);
        info.increment_request_failure_counts(true, soon);

        let (read_count, _) = info.snapshot_consecutive_request_failure_count();
        assert_eq!(read_count, 3);
    }

    #[tokio::test]
    async fn test_snapshot_consecutive_count_returns_current_values() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        info.consecutive_read_request_failure_count
            .store(7, Ordering::SeqCst);
        info.consecutive_write_request_failure_count
            .store(3, Ordering::SeqCst);

        let (r, w) = info.snapshot_consecutive_request_failure_count();
        assert_eq!(r, 7);
        assert_eq!(w, 3);
    }

    // -----------------------------------------------------------------------
    // mark_endpoints_to_healthy tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_mark_endpoints_to_healthy_marks_all_as_healthy() {
        let pk1 = PartitionKeyRange::new("0".into(), "".into(), "AA".into());
        let pk2 = PartitionKeyRange::new("1".into(), "AA".into(), "FF".into());

        let mut mappings = HashMap::new();
        mappings.insert(
            pk1.clone(),
            (
                "rid1".to_string(),
                "https://loc1.com/".to_string(),
                PartitionHealthStatus::Unhealthy,
            ),
        );
        mappings.insert(
            pk2.clone(),
            (
                "rid2".to_string(),
                "https://loc2.com/".to_string(),
                PartitionHealthStatus::Unhealthy,
            ),
        );

        GlobalPartitionEndpointManager::mark_endpoints_to_healthy(&mut mappings);

        assert_eq!(mappings[&pk1].2, PartitionHealthStatus::Healthy);
        assert_eq!(mappings[&pk2].2, PartitionHealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_mark_endpoints_to_healthy_empty_map() {
        let mut mappings: HashMap<PartitionKeyRange, (String, String, PartitionHealthStatus)> =
            HashMap::new();

        GlobalPartitionEndpointManager::mark_endpoints_to_healthy(&mut mappings);

        assert!(mappings.is_empty());
    }

    #[tokio::test]
    async fn test_mark_endpoints_to_healthy_already_healthy() {
        let pk = PartitionKeyRange::new("0".into(), "".into(), "FF".into());

        let mut mappings = HashMap::new();
        mappings.insert(
            pk.clone(),
            (
                "rid1".to_string(),
                "https://loc1.com/".to_string(),
                PartitionHealthStatus::Healthy,
            ),
        );

        GlobalPartitionEndpointManager::mark_endpoints_to_healthy(&mut mappings);

        assert_eq!(mappings[&pk].2, PartitionHealthStatus::Healthy);
    }

    // -----------------------------------------------------------------------
    // GlobalPartitionEndpointManager flag tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_new_both_flags_disabled() {
        let gem = create_single_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, false);

        assert!(!manager.partition_level_automatic_failover_enabled());
        assert!(!manager.partition_level_circuit_breaker_enabled());
        assert!(!manager.partition_level_failover_enabled());
    }

    #[tokio::test]
    async fn test_new_auto_failover_enabled_only() {
        let gem = create_single_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, false);

        assert!(manager.partition_level_automatic_failover_enabled());
        assert!(!manager.partition_level_circuit_breaker_enabled());
        assert!(manager.partition_level_failover_enabled());
    }

    #[tokio::test]
    async fn test_new_circuit_breaker_enabled_only() {
        let gem = create_single_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        assert!(!manager.partition_level_automatic_failover_enabled());
        assert!(manager.partition_level_circuit_breaker_enabled());
        assert!(manager.partition_level_failover_enabled());
    }

    #[tokio::test]
    async fn test_new_both_flags_enabled() {
        let gem = create_single_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, true);

        assert!(manager.partition_level_automatic_failover_enabled());
        assert!(manager.partition_level_circuit_breaker_enabled());
        assert!(manager.partition_level_failover_enabled());
    }

    // -----------------------------------------------------------------------
    // can_use_partition_level_failover_locations tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_can_use_failover_locations_with_single_endpoint() {
        let gem = create_single_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, true);

        let request = create_read_request();
        // Single region: read_endpoints().len() <= 1 → false
        assert!(!manager.can_use_partition_level_failover_locations(&request));
    }

    #[tokio::test]
    async fn test_can_use_failover_locations_with_multiple_endpoints_documents() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, true);

        let request = create_read_request();
        assert!(manager.can_use_partition_level_failover_locations(&request));
    }

    #[tokio::test]
    async fn test_can_use_failover_locations_with_stored_procedure_execute() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, true);

        let request = create_stored_procedure_execute_request();
        assert!(manager.can_use_partition_level_failover_locations(&request));
    }

    #[tokio::test]
    async fn test_can_use_failover_locations_with_database_resource() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, true);

        let request = create_database_request();
        // Databases are not eligible for partition-level failover
        assert!(!manager.can_use_partition_level_failover_locations(&request));
    }

    // -----------------------------------------------------------------------
    // is_request_eligible_for_per_partition_automatic_failover tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_auto_failover_eligible_write_on_single_master() {
        // Single master: can_support_multiple_write_locations returns false
        let gem = create_single_master_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, false);

        let request = create_write_request();
        // auto failover enabled + write request + single master = eligible
        assert!(manager.is_request_eligible_for_per_partition_automatic_failover(&request));
    }

    #[tokio::test]
    async fn test_auto_failover_not_eligible_when_disabled() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, false);

        let request = create_write_request();
        assert!(!manager.is_request_eligible_for_per_partition_automatic_failover(&request));
    }

    #[tokio::test]
    async fn test_auto_failover_not_eligible_for_read_request() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, false);

        let request = create_read_request();
        // Read-only requests are not eligible for automatic failover
        assert!(!manager.is_request_eligible_for_per_partition_automatic_failover(&request));
    }

    // -----------------------------------------------------------------------
    // is_request_eligible_for_partition_level_circuit_breaker tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_circuit_breaker_eligible_for_read_request() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let request = create_read_request();
        assert!(manager.is_request_eligible_for_partition_level_circuit_breaker(&request));
    }

    #[tokio::test]
    async fn test_circuit_breaker_not_eligible_when_disabled() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, false);

        let request = create_read_request();
        assert!(!manager.is_request_eligible_for_partition_level_circuit_breaker(&request));
    }

    #[tokio::test]
    async fn test_circuit_breaker_not_eligible_write_on_single_master() {
        // Single-master: can_support_multiple_write_locations returns false for writes
        let gem = create_single_master_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let request = create_write_request();
        // Write on single-master: circuit breaker should not be eligible
        // (can_support_multiple_write_locations is false for the default manager config)
        assert!(!manager.is_request_eligible_for_partition_level_circuit_breaker(&request));
    }

    // -----------------------------------------------------------------------
    // is_request_eligible_for_partition_failover tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_partition_failover_returns_none_when_both_disabled() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, false);

        let request = create_read_request();
        assert!(manager
            .is_request_eligible_for_partition_failover(&request, false)
            .is_none());
    }

    #[tokio::test]
    async fn test_partition_failover_returns_some_when_eligible() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let request = create_read_request();
        let result = manager.is_request_eligible_for_partition_failover(&request, false);
        assert!(result.is_some());

        let (pk_range, failed_loc) = result.unwrap();
        assert_eq!(pk_range.id, "0");
        assert!(failed_loc.is_none()); // Not validating failed location
    }

    #[tokio::test]
    async fn test_partition_failover_validates_failed_location() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let request = create_read_request();
        let result = manager.is_request_eligible_for_partition_failover(&request, true);
        assert!(result.is_some());

        let (pk_range, failed_loc) = result.unwrap();
        assert_eq!(pk_range.id, "0");
        assert!(failed_loc.is_some());
        assert_eq!(
            failed_loc.unwrap().as_str(),
            "https://test-westus.documents.azure.com/"
        );
    }

    #[tokio::test]
    async fn test_partition_failover_returns_none_without_partition_key_range() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let mut request = create_read_request();
        request.request_context.resolved_partition_key_range = None;

        assert!(manager
            .is_request_eligible_for_partition_failover(&request, false)
            .is_none());
    }

    #[tokio::test]
    async fn test_partition_failover_returns_none_for_ineligible_resource_type() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let request = create_database_request();
        assert!(manager
            .is_request_eligible_for_partition_failover(&request, false)
            .is_none());
    }

    #[tokio::test]
    async fn test_partition_failover_returns_none_without_failed_location() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let mut request = create_read_request();
        request.request_context.location_endpoint_to_route = None;

        // With should_validate_failed_location=true, missing location → None
        assert!(manager
            .is_request_eligible_for_partition_failover(&request, true)
            .is_none());
    }

    // -----------------------------------------------------------------------
    // try_mark_endpoint_unavailable_for_partition_key_range tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_mark_endpoint_unavailable_circuit_breaker_path() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let request = create_read_request();
        let result = manager.try_mark_endpoint_unavailable_for_partition_key_range(&request);
        assert!(result);

        // Verify the override was added to the read_and_write map
        let guard = manager
            .partition_key_range_to_location_for_read_and_write
            .read()
            .unwrap();
        let pk = PartitionKeyRange::new("0".into(), "".into(), "FF".into());
        assert!(guard.contains_key(&pk));

        let failover_info = guard.get(&pk).unwrap();
        assert_ne!(
            failover_info.current,
            "https://test-westus.documents.azure.com/"
        );
    }

    #[tokio::test]
    async fn test_mark_endpoint_unavailable_auto_failover_path() {
        let gem = create_single_master_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, false);

        let request = create_write_request();
        let result = manager.try_mark_endpoint_unavailable_for_partition_key_range(&request);
        assert!(result);

        // Verify the override was added to the write map
        let guard = manager
            .partition_key_range_to_location_for_write
            .read()
            .unwrap();
        let pk = PartitionKeyRange::new("0".into(), "".into(), "FF".into());
        assert!(guard.contains_key(&pk));
    }

    #[tokio::test]
    async fn test_mark_endpoint_unavailable_returns_false_when_disabled() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, false);

        let request = create_read_request();
        assert!(!manager.try_mark_endpoint_unavailable_for_partition_key_range(&request));
    }

    #[tokio::test]
    async fn test_mark_endpoint_unavailable_returns_false_without_failed_location() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let mut request = create_read_request();
        request.request_context.location_endpoint_to_route = None;

        assert!(!manager.try_mark_endpoint_unavailable_for_partition_key_range(&request));
    }

    #[tokio::test]
    async fn test_mark_endpoint_unavailable_sequential_failover_removes_on_exhaust() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let pk = PartitionKeyRange::new("0".into(), "".into(), "FF".into());

        // First failure at West US → should move to East US
        let request = create_read_request();
        assert!(manager.try_mark_endpoint_unavailable_for_partition_key_range(&request));

        {
            let guard = manager
                .partition_key_range_to_location_for_read_and_write
                .read()
                .unwrap();
            let info = guard.get(&pk).unwrap();
            assert_eq!(info.current, "https://test-eastus.documents.azure.com/");
        }

        // Second failure at East US → all exhausted, should remove override
        let mut request2 = create_read_request();
        request2.request_context.location_endpoint_to_route =
            Some("https://test-eastus.documents.azure.com/".parse().unwrap());
        let result = manager.try_mark_endpoint_unavailable_for_partition_key_range(&request2);
        assert!(!result);

        // Override should be removed
        let guard = manager
            .partition_key_range_to_location_for_read_and_write
            .read()
            .unwrap();
        assert!(!guard.contains_key(&pk));
    }

    // -----------------------------------------------------------------------
    // try_add_partition_level_location_override tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_add_override_returns_false_when_no_override_exists() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let mut request = create_read_request();
        assert!(!manager.try_add_partition_level_location_override(&mut request));
    }

    #[tokio::test]
    async fn test_add_override_routes_to_override_location() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        // First, mark endpoint as unavailable to create an override
        let request = create_read_request();
        manager.try_mark_endpoint_unavailable_for_partition_key_range(&request);

        // Bump failure counts above threshold so circuit breaker applies.
        // Default read threshold is 10; store 11 so that 11 > 10 is true.
        {
            let guard = manager
                .partition_key_range_to_location_for_read_and_write
                .read()
                .unwrap();
            let pk = PartitionKeyRange::new("0".into(), "".into(), "FF".into());
            let info = guard.get(&pk).unwrap();
            info.consecutive_read_request_failure_count
                .store(11, Ordering::SeqCst);
        }

        // Now try to add override — should route to the new location
        let mut request2 = create_read_request();
        let result = manager.try_add_partition_level_location_override(&mut request2);
        assert!(result);

        assert_eq!(
            request2
                .request_context
                .location_endpoint_to_route
                .unwrap()
                .as_str(),
            "https://test-eastus.documents.azure.com/"
        );
    }

    #[tokio::test]
    async fn test_add_override_returns_false_when_disabled() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, false);

        let mut request = create_read_request();
        assert!(!manager.try_add_partition_level_location_override(&mut request));
    }

    #[tokio::test]
    async fn test_add_override_circuit_breaker_below_threshold_returns_false() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        // Mark endpoint unavailable to create override
        let request = create_read_request();
        manager.try_mark_endpoint_unavailable_for_partition_key_range(&request);

        // Don't bump failure counts — below threshold
        let mut request2 = create_read_request();
        let result = manager.try_add_partition_level_location_override(&mut request2);
        assert!(!result);
    }

    #[tokio::test]
    async fn test_add_override_auto_failover_path() {
        let gem = create_single_master_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, false);

        // Mark write endpoint as unavailable
        let request = create_write_request();
        manager.try_mark_endpoint_unavailable_for_partition_key_range(&request);

        // Now try to add override for a write request
        let mut request2 = create_write_request();
        let result = manager.try_add_partition_level_location_override(&mut request2);
        assert!(result);
    }

    // -----------------------------------------------------------------------
    // increment_request_failure_counter_and_check_if_partition_can_failover tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_increment_failure_counter_returns_false_when_disabled() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, false);

        let request = create_read_request();
        assert!(!manager
            .increment_request_failure_counter_and_check_if_partition_can_failover(&request));
    }

    #[tokio::test]
    async fn test_increment_failure_counter_creates_entry_on_first_call() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let request = create_read_request();
        let _ =
            manager.increment_request_failure_counter_and_check_if_partition_can_failover(&request);

        let guard = manager
            .partition_key_range_to_location_for_read_and_write
            .read()
            .unwrap();
        let pk = PartitionKeyRange::new("0".into(), "".into(), "FF".into());
        assert!(guard.contains_key(&pk));
    }

    #[tokio::test]
    async fn test_increment_failure_counter_below_threshold_returns_false() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let request = create_read_request();
        // Default read threshold is 2, so 1 failure should not trigger
        let result =
            manager.increment_request_failure_counter_and_check_if_partition_can_failover(&request);
        assert!(!result);
    }

    #[tokio::test]
    async fn test_increment_failure_counter_above_threshold_returns_true() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let request = create_read_request();

        // Default read threshold is 10; we need > 10 increments so the
        // counter exceeds the threshold (strict greater-than comparison).
        for _ in 0..11 {
            manager.increment_request_failure_counter_and_check_if_partition_can_failover(&request);
        }

        let result =
            manager.increment_request_failure_counter_and_check_if_partition_can_failover(&request);
        assert!(result);
    }

    #[tokio::test]
    async fn test_increment_failure_counter_auto_failover_path_for_writes() {
        let gem = create_single_master_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, false);

        let request = create_write_request();
        let _ =
            manager.increment_request_failure_counter_and_check_if_partition_can_failover(&request);

        // Verify it went to the write map
        let guard = manager
            .partition_key_range_to_location_for_write
            .read()
            .unwrap();
        let pk = PartitionKeyRange::new("0".into(), "".into(), "FF".into());
        assert!(guard.contains_key(&pk));

        // And not in the read_and_write map
        let guard2 = manager
            .partition_key_range_to_location_for_read_and_write
            .read()
            .unwrap();
        assert!(!guard2.contains_key(&pk));
    }

    // -----------------------------------------------------------------------
    // try_add_or_update_partition_failover_info_and_move_to_next_location tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_add_or_update_moves_to_next_location() {
        let gem = create_three_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let pk = PartitionKeyRange::new("0".into(), "".into(), "FF".into());
        let request = create_read_request();
        let map = Arc::new(RwLock::new(HashMap::new()));

        let next_locations = vec![
            "https://test-westus.documents.azure.com/".to_string(),
            "https://test-eastus.documents.azure.com/".to_string(),
            "https://test-centralus.documents.azure.com/".to_string(),
        ];

        let result = manager.try_add_or_update_partition_failover_info_and_move_to_next_location(
            &pk,
            "https://test-westus.documents.azure.com/",
            &next_locations,
            &request,
            &map,
        );
        assert!(result);

        let guard = map.read().unwrap();
        let info = guard.get(&pk).unwrap();
        assert_eq!(info.current, "https://test-eastus.documents.azure.com/");
    }

    #[tokio::test]
    async fn test_add_or_update_removes_on_all_exhausted() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let pk = PartitionKeyRange::new("0".into(), "".into(), "FF".into());
        let request = create_read_request();
        let map = Arc::new(RwLock::new(HashMap::new()));

        let next_locations = vec![
            "https://test-westus.documents.azure.com/".to_string(),
            "https://test-eastus.documents.azure.com/".to_string(),
        ];

        // First move: west -> east
        let result = manager.try_add_or_update_partition_failover_info_and_move_to_next_location(
            &pk,
            "https://test-westus.documents.azure.com/",
            &next_locations,
            &request,
            &map,
        );
        assert!(result);

        // Second move: east -> exhausted
        let result2 = manager.try_add_or_update_partition_failover_info_and_move_to_next_location(
            &pk,
            "https://test-eastus.documents.azure.com/",
            &next_locations,
            &request,
            &map,
        );
        assert!(!result2);

        // Entry should be removed
        let guard = map.read().unwrap();
        assert!(!guard.contains_key(&pk));
    }

    // -----------------------------------------------------------------------
    // Multiple partition key range tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_different_partition_key_ranges_tracked_independently() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        // Create two requests with different partition key ranges
        let mut request1 = create_read_request();
        request1.request_context.resolved_partition_key_range =
            Some(PartitionKeyRange::new("0".into(), "".into(), "AA".into()));

        let mut request2 = create_read_request();
        request2.request_context.resolved_partition_key_range =
            Some(PartitionKeyRange::new("1".into(), "AA".into(), "FF".into()));

        // Mark both as unavailable
        assert!(manager.try_mark_endpoint_unavailable_for_partition_key_range(&request1));
        assert!(manager.try_mark_endpoint_unavailable_for_partition_key_range(&request2));

        // Both should have entries
        let guard = manager
            .partition_key_range_to_location_for_read_and_write
            .read()
            .unwrap();
        assert_eq!(guard.len(), 2);
    }

    // -----------------------------------------------------------------------
    // Debug trait tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_debug_formatting() {
        let gem = create_single_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, false);

        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("GlobalPartitionEndpointManager"));
        assert!(debug_str.contains("partition_level_automatic_failover_enabled"));
        assert!(debug_str.contains("partition_level_circuit_breaker_enabled"));
    }

    #[tokio::test]
    async fn test_failover_info_debug_formatting() {
        let info = PartitionKeyRangeFailoverInfo::new("rid1".into(), "https://loc1.com/".into());

        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("PartitionKeyRangeFailoverInfo"));
        assert!(debug_str.contains("rid1"));
    }

    // -----------------------------------------------------------------------
    // Three-region failover tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_three_region_sequential_failover() {
        let gem = create_three_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        let pk = PartitionKeyRange::new("0".into(), "".into(), "FF".into());

        // First failure at West US
        let request1 = create_read_request();
        assert!(manager.try_mark_endpoint_unavailable_for_partition_key_range(&request1));

        {
            let guard = manager
                .partition_key_range_to_location_for_read_and_write
                .read()
                .unwrap();
            assert_eq!(
                guard.get(&pk).unwrap().current,
                "https://test-eastus.documents.azure.com/"
            );
        }

        // Second failure at East US
        let mut request2 = create_read_request();
        request2.request_context.location_endpoint_to_route =
            Some("https://test-eastus.documents.azure.com/".parse().unwrap());
        assert!(manager.try_mark_endpoint_unavailable_for_partition_key_range(&request2));

        {
            let guard = manager
                .partition_key_range_to_location_for_read_and_write
                .read()
                .unwrap();
            assert_eq!(
                guard.get(&pk).unwrap().current,
                "https://test-centralus.documents.azure.com/"
            );
        }

        // Third failure at Central US → all exhausted, override removed
        let mut request3 = create_read_request();
        request3.request_context.location_endpoint_to_route = Some(
            "https://test-centralus.documents.azure.com/"
                .parse()
                .unwrap(),
        );
        assert!(!manager.try_mark_endpoint_unavailable_for_partition_key_range(&request3));

        let guard = manager
            .partition_key_range_to_location_for_read_and_write
            .read()
            .unwrap();
        assert!(!guard.contains_key(&pk));
    }

    // -----------------------------------------------------------------------
    // Background initialization tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_background_init_flag_set_on_construction() {
        let gem = create_single_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, true);

        assert!(manager
            .background_connection_init_active
            .load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_second_background_init_is_noop() {
        let gem = create_single_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, true);

        // Flag is already true after construction
        assert!(manager
            .background_connection_init_active
            .load(Ordering::SeqCst));

        // Calling again should be a no-op (no panic, flag stays true)
        manager.initialize_and_start_circuit_breaker_failback_background_refresh();
        assert!(manager
            .background_connection_init_active
            .load(Ordering::SeqCst));
    }

    // -----------------------------------------------------------------------
    // End-to-end: mark unavailable → add override → verify routing
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_end_to_end_failover_and_override_routing() {
        let gem = create_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, true);

        // Step 1: A read request fails at West US
        let request = create_read_request();
        assert!(manager.try_mark_endpoint_unavailable_for_partition_key_range(&request));

        // Step 2: Bump failure count above threshold.
        // Default read threshold is 10; store 11 so that 11 > 10 triggers override.
        {
            let guard = manager
                .partition_key_range_to_location_for_read_and_write
                .read()
                .unwrap();
            let pk = PartitionKeyRange::new("0".into(), "".into(), "FF".into());
            let info = guard.get(&pk).unwrap();
            info.consecutive_read_request_failure_count
                .store(11, Ordering::SeqCst);
        }

        // Step 3: New request should be routed to East US
        let mut new_request = create_read_request();
        assert!(manager.try_add_partition_level_location_override(&mut new_request));

        assert_eq!(
            new_request
                .request_context
                .location_endpoint_to_route
                .unwrap()
                .as_str(),
            "https://test-eastus.documents.azure.com/"
        );
    }

    #[tokio::test]
    async fn test_end_to_end_auto_failover_write_request() {
        let gem = create_single_master_multi_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, false);

        // Step 1: A write request fails at West US
        let request = create_write_request();
        assert!(manager.try_mark_endpoint_unavailable_for_partition_key_range(&request));

        // Step 2: New write request should be routed to East US
        // (auto failover path doesn't check circuit breaker thresholds)
        let mut new_request = create_write_request();
        assert!(manager.try_add_partition_level_location_override(&mut new_request));

        assert_eq!(
            new_request
                .request_context
                .location_endpoint_to_route
                .unwrap()
                .as_str(),
            "https://test-eastus.documents.azure.com/"
        );
    }

    // -----------------------------------------------------------------------
    // Dynamic configure_* method tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_configure_partition_level_automatic_failover_toggles_flag() {
        let gem = create_single_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, false);

        // Initially disabled
        assert!(!manager.partition_level_automatic_failover_enabled());

        // Enable it
        manager.configure_partition_level_automatic_failover(true);
        assert!(manager.partition_level_automatic_failover_enabled());

        // Disable it again
        manager.configure_partition_level_automatic_failover(false);
        assert!(!manager.partition_level_automatic_failover_enabled());
    }

    #[tokio::test]
    async fn test_configure_per_partition_circuit_breaker_toggles_flag() {
        let gem = create_single_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, false, false);

        // Initially disabled
        assert!(!manager.partition_level_circuit_breaker_enabled());

        // Enable it
        manager.configure_per_partition_circuit_breaker(true);
        assert!(manager.partition_level_circuit_breaker_enabled());

        // Disable it again
        manager.configure_per_partition_circuit_breaker(false);
        assert!(!manager.partition_level_circuit_breaker_enabled());
    }

    #[tokio::test]
    async fn test_configure_idempotent_same_value() {
        let gem = create_single_region_manager();
        let manager = GlobalPartitionEndpointManager::new(gem, true, true);

        // Setting the same value should not panic or change the flag
        assert!(manager.partition_level_automatic_failover_enabled());
        manager.configure_partition_level_automatic_failover(true);
        assert!(manager.partition_level_automatic_failover_enabled());

        assert!(manager.partition_level_circuit_breaker_enabled());
        manager.configure_per_partition_circuit_breaker(true);
        assert!(manager.partition_level_circuit_breaker_enabled());
    }
}
