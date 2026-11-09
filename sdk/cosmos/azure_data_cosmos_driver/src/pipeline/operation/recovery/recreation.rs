// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Determines whether a stale container generation can be recovered.

use std::sync::atomic::{AtomicU8, Ordering};

use azure_core::http::headers::{HeaderName, HeaderValue};

use crate::{
    models::{request_header_names, CosmosOperation},
    pipeline::{
        operation::{
            recovery::{recreation, state::OperationRetryState},
            types::OperationOverrides,
        },
        transport::types::{TransportOutcome, TransportResult},
    },
};

/// How the caller can resume after refreshing a recreated container.
pub(crate) enum Restart {
    InPlace,
    RebuildPlan,
}

/// Refreshes an eligible stale container once and resets generation-specific retry progress.
///
/// Refresh failure retains the original service response. A successful refresh
/// either restarts this invocation or asks the outer dataflow plan to rebuild.
pub(crate) async fn recover(
    operation: &mut CosmosOperation,
    context: &crate::pipeline::operation::OperationContext<'_>,
    result: &TransportResult,
    retry_state: &mut OperationRetryState,
) -> Option<Restart> {
    let restart = self::plan(
        operation,
        context.overrides,
        context.config.custom_headers,
        retry_state.container_recreation_retry_attempted,
        result,
        retry_state,
    )?;
    retry_state.container_recreation_retry_attempted = true;
    if !self::refresh(
        operation,
        context.overrides,
        &restart,
        context.config,
        context.services.driver,
    )
    .await
    {
        return None;
    }

    if matches!(restart, Restart::InPlace) {
        retry_state.pending_write_effects.clear();
        retry_state.partition_key_range_id = None;
        retry_state.throughput_control = operation.container().map(|_| {
            crate::driver::CosmosDriver::effective_throughput_control(context.config.options)
        });
        tracing::info!(
            activity_id = %context.config.activity_id,
            container_rid = operation.container().map(|container| container.rid()),
            "retrying operation after container recreation",
        );
    }
    Some(restart)
}

/// Selects recovery only when retrying cannot invalidate caller-owned session or continuation state.
pub(crate) fn plan(
    operation: &CosmosOperation,
    overrides: &OperationOverrides,
    custom_headers: Option<&std::collections::HashMap<HeaderName, HeaderValue>>,
    attempted: bool,
    result: &TransportResult,
    retry_state: &OperationRetryState,
) -> Option<Restart> {
    if !self::container_recreation_refresh_eligible(operation, overrides, custom_headers, attempted)
        || !self::is_container_recreation_signal(result, retry_state)
    {
        return None;
    }

    Some(
        if self::container_recreation_retry_eligible(
            operation,
            overrides,
            custom_headers,
            attempted,
        ) && overrides.container_recreation_recovery_tracker.is_none()
        {
            Restart::InPlace
        } else {
            Restart::RebuildPlan
        },
    )
}

/// Allows one metadata refresh unless the request is tied to a specific container generation.
pub(crate) fn container_recreation_refresh_eligible(
    operation: &CosmosOperation,
    overrides: &OperationOverrides,
    custom_headers: Option<&std::collections::HashMap<HeaderName, HeaderValue>>,
    retry_attempted: bool,
) -> bool {
    if retry_attempted
        || overrides.container_recreation_recovery_disabled
        || operation.resource_type() == crate::models::ResourceType::StoredProcedure
        || operation
            .container()
            .is_none_or(|container| container.is_by_rid())
        || operation.request_headers().session_token.is_some()
        || overrides.continuation.is_some()
        || overrides.region_pin.is_some()
    {
        return false;
    }

    #[cfg(feature = "unstable_dtx")]
    if operation.resource_type() == crate::models::ResourceType::DistributedTransactionBatch {
        return false;
    }

    let session_header = HeaderName::from_static(request_header_names::SESSION_TOKEN);
    !custom_headers.is_some_and(|headers| headers.contains_key(&session_header))
}

/// Allows in-place retry only when no partition targeting or PATCH plan must be rebuilt.
pub(crate) fn container_recreation_retry_eligible(
    operation: &CosmosOperation,
    overrides: &OperationOverrides,
    custom_headers: Option<&std::collections::HashMap<HeaderName, HeaderValue>>,
    retry_attempted: bool,
) -> bool {
    recreation::container_recreation_refresh_eligible(
        operation,
        overrides,
        custom_headers,
        retry_attempted,
    ) && !operation.is_patch_sub_operation()
        && overrides.partition_key_range_id.is_none()
        && overrides.feed_range.is_none()
        && overrides.pkrange_bounds.is_none()
}

/// Recognizes stale-container responses, including exhausted session-unavailable retries.
pub(crate) fn is_container_recreation_signal(
    result: &TransportResult,
    retry_state: &OperationRetryState,
) -> bool {
    let TransportOutcome::HttpError { status, .. } = &result.outcome else {
        return false;
    };

    (status.status_code() == azure_core::http::StatusCode::BadRequest
        && status.sub_status()
            == Some(crate::error::status_codes::substatus::COLLECTION_RID_MISMATCH))
        || (status.status_code() == azure_core::http::StatusCode::Gone
            && status.sub_status() == Some(crate::error::status_codes::substatus::NAME_CACHE_STALE))
        || (status.is_read_session_not_available() && !retry_state.can_retry_session())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContainerRecreationRecoveryOutcome {
    NotAttempted = 0,
    Attempted = 1,
    PlanRebuildRequired = 2,
}

#[derive(Debug, Default)]
pub(crate) struct ContainerRecreationRecoveryTracker {
    pub(crate) outcome: AtomicU8,
}

impl ContainerRecreationRecoveryTracker {
    /// Records a consumed recovery attempt without erasing a pending plan rebuild.
    pub(crate) fn mark_attempted(&self) {
        self.outcome.fetch_max(
            ContainerRecreationRecoveryOutcome::Attempted as u8,
            Ordering::SeqCst,
        );
    }

    /// Publishes that refreshed container identity requires the outer plan to be rebuilt.
    pub(crate) fn mark_plan_rebuild_required(&self) {
        self.outcome.store(
            ContainerRecreationRecoveryOutcome::PlanRebuildRequired as u8,
            Ordering::SeqCst,
        );
    }

    /// Reads the recovery checkpoint shared with the outer execution plan.
    pub(crate) fn outcome(&self) -> ContainerRecreationRecoveryOutcome {
        match self.outcome.load(Ordering::SeqCst) {
            0 => ContainerRecreationRecoveryOutcome::NotAttempted,
            1 => ContainerRecreationRecoveryOutcome::Attempted,
            2 => ContainerRecreationRecoveryOutcome::PlanRebuildRequired,
            value => unreachable!("invalid container recreation recovery outcome: {value}"),
        }
    }
}

/// Attempts the planned metadata refresh, preserving the original service result on failure.
///
/// The tracker is shared with the outer dataflow plan, so its mutations belong
/// at this checkpoint rather than in the pure recreation decision.
pub(crate) async fn refresh(
    operation: &mut CosmosOperation,
    overrides: &crate::pipeline::operation::types::OperationOverrides,
    restart: &crate::pipeline::operation::recovery::recreation::Restart,
    config: &crate::pipeline::operation::config::OperationPipelineConfig<'_>,
    driver: &crate::driver::CosmosDriver,
) -> bool {
    if let Some(tracker) = &overrides.container_recreation_recovery_tracker {
        tracker.mark_attempted();
    }
    match driver
        .try_recover_recreated_container(operation, config.operation_options)
        .await
    {
        Ok(true) => {
            if matches!(
                restart,
                crate::pipeline::operation::recovery::recreation::Restart::RebuildPlan
            ) {
                if let Some(tracker) = &overrides.container_recreation_recovery_tracker {
                    tracker.mark_plan_rebuild_required();
                }
            }
            true
        }
        Ok(false) => false,
        Err(error) => {
            tracing::warn!(
                activity_id = %config.activity_id,
                error = %error,
                "container metadata refresh failed; preserving the original service response",
            );
            false
        }
    }
}
