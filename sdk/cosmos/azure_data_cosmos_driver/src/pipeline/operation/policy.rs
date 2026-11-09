// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Resolves fixed operation policy and initial retry progress.

use std::time::{Duration, Instant};

use crate::{
    diagnostics::DiagnosticsContextBuilder,
    models::{CosmosOperation, DefaultConsistencyLevel},
    options,
    options::{PartitionTopologyCacheMode, ReadConsistencyStrategy},
    pipeline::operation::{
        config::{OperationPipelineConfig, OperationPipelineServices},
        recovery,
        recovery::state::OperationRetryState,
        response, session,
        types::OperationOverrides,
    },
};

/// Fixed policy shared by all attempts in an invocation.
pub(crate) struct Policy {
    pub fault_injection_enabled: bool,
    pub endpoint_unavailability_ttl: Duration,
    pub session_capturing_disabled: bool,
    pub read_consistency_strategy: ReadConsistencyStrategy,
    pub operation_read_consistency_strategy: ReadConsistencyStrategy,
    pub effective_consistency: DefaultConsistencyLevel,
    pub session_token_resolution_active: bool,
    pub session_token_capture_active: bool,
    pub lazy_topology: bool,
    pub configured_request_timeout: Option<Duration>,
    pub deadline: Option<Instant>,
}

/// Fixed policy and owned progress produced by initialization.
pub(crate) struct Resolved {
    pub policy: Policy,
    pub retry_state: OperationRetryState,
    pub diagnostics: DiagnosticsContextBuilder,
}

/// Resolves fixed policy and seeds operation-local progress before the first attempt.
pub(crate) fn resolve(
    operation: &CosmosOperation,
    overrides: &OperationOverrides,
    config: &OperationPipelineConfig<'_>,
    services: &OperationPipelineServices<'_>,
    diagnostics: DiagnosticsContextBuilder,
) -> crate::error::Result<Resolved> {
    #[cfg(feature = "fault_injection")]
    let fault_injection_enabled = services.driver.fault_injection_enabled();
    #[cfg(not(feature = "fault_injection"))]
    let fault_injection_enabled = false;
    let location_snapshot = services.location_state_store.capture();
    let max_failover_retries = config
        .options
        .max_failover_retry_count()
        .copied()
        .unwrap_or(3);
    let endpoint_unavailability_ttl = config
        .options
        .endpoint_unavailability_ttl()
        .copied()
        .unwrap_or(Duration::from_secs(60));

    let session_capturing_disabled = config
        .options
        .session_capturing_disabled()
        .copied()
        .unwrap_or(false);
    let read_consistency_strategy = config
        .options
        .read_consistency_strategy()
        .copied()
        .unwrap_or(ReadConsistencyStrategy::Default);
    let operation_read_consistency_strategy =
        session::read_consistency_strategy_for_operation(operation, read_consistency_strategy);
    let effective_consistency = options::resolve_effective_consistency(
        operation_read_consistency_strategy,
        config.account_default_consistency,
    );
    let session_token_resolution_active = !session_capturing_disabled
        && session::operation_allows_automatic_session_token_resolution(
            operation,
            location_snapshot.account.multiple_write_locations_enabled,
        )
        && operation_read_consistency_strategy
            .is_session_effective(config.account_default_consistency);
    let session_token_capture_active = !session_capturing_disabled;
    let lazy_topology = services
        .driver
        .options()
        .partition_failover_options()
        .partition_topology_cache_mode()
        == PartitionTopologyCacheMode::Lazy;

    // GlobalStrong cannot provide its contract on a non-Strong account.
    if matches!(
        read_consistency_strategy,
        ReadConsistencyStrategy::GlobalStrong
    ) && operation.is_read_only()
        && config.account_default_consistency != DefaultConsistencyLevel::Strong
    {
        return Err(response::global_strong_account_validation_error(
            diagnostics,
        ));
    }
    let max_session_retries = config
        .options
        .max_session_retry_count()
        .copied()
        .unwrap_or_else(|| {
            // Java SDK parity: 2 for single-write, endpoints.len() for multi-write.
            // Uses the original endpoint count (before unavailability filtering).
            if location_snapshot.account.multiple_write_locations_enabled {
                let endpoints_len = location_snapshot
                    .account
                    .preferred_endpoints(operation.is_read_only())
                    .len();
                endpoints_len as u32
            } else {
                2
            }
        });

    let mut retry_state = OperationRetryState::initial(
        location_snapshot.account.generation,
        location_snapshot.account.multiple_write_locations_enabled,
        config
            .options
            .excluded_regions()
            .map(|r| r.iter().cloned().collect())
            .unwrap_or_default(),
        max_failover_retries,
        max_session_retries,
    );
    retry_state.throughput_control = config.throughput_control;
    // Dataflow resolves physical identity before execution. EPK targets carry
    // their wire-routing range ID; logical targets carry internal-only metadata.
    retry_state.partition_key_range_id = session::partition_key_range_id_from_overrides(&overrides);

    // Non-partitioned writes cannot use partition-level write-region discovery.
    retry_state.ppaf_write_retry_allowed = location_snapshot
        .partitions
        .per_partition_automatic_failover_enabled
        && !location_snapshot.account.multiple_write_locations_enabled
        && operation
            .resource_type()
            .is_partitioned(operation.operation_type());

    // Partition-level thresholds avoid evicting an otherwise healthy endpoint.
    retry_state.ppcb_active = location_snapshot
        .partitions
        .per_partition_circuit_breaker_enabled
        && location_snapshot.account.preferred_read_endpoints.len() > 1;
    recovery::state::apply_read_routing_strategy(
        &mut retry_state,
        operation.prefers_write_endpoints_for_read(),
    );

    // Metadata operations must never enter hub-region discovery.
    retry_state.is_dataplane = config.pipeline_type.is_data_plane();

    // Retry-triggered hedges use the original budget, not the remaining time.
    let configured_request_timeout = config
        .options
        .end_to_end_latency_policy()
        .map(|p| p.timeout());
    let deadline = operation
        .absolute_deadline()
        .or_else(|| configured_request_timeout.map(|t| services.clock.now() + t));

    Ok(Resolved {
        policy: Policy {
            fault_injection_enabled,
            endpoint_unavailability_ttl,
            session_capturing_disabled,
            read_consistency_strategy,
            operation_read_consistency_strategy,
            effective_consistency,
            session_token_resolution_active,
            session_token_capture_active,
            lazy_topology,
            configured_request_timeout,
            deadline,
        },
        retry_state,
        diagnostics,
    })
}
