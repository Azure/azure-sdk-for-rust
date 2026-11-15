// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    diagnostics::DiagnosticsContext,
    feed::ContinuationToken,
    models::ChangeFeedItem,
    options::{ChangeFeedMode, MaxItemCountHint},
    ContainerClient, CosmosError, CosmosStatus,
};
use azure_core::{fmt::SafeDebug, http::StatusCode};
use azure_data_cosmos_driver::models::ChangeFeedStartFrom;
use serde::de::DeserializeOwned;
use std::{
    future::Future,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use time::OffsetDateTime;

mod execution;

#[cfg(doctest)]
mod doc_tests;

/// An async function-pointer callback accepting batch context and changes; capturing closures are unsupported.
/// The returned future must resolve to [`crate::Result<()>`]; callbacks must tolerate replay.
pub type ChangeFeedHandler<T, Fut> = fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut;

/// Controls persistence of successful callback progress; defaults to five-minute deferred persistence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChangeFeedCheckpointPolicy {
    /// Persists each successful callback before reading its lease's next batch.
    Eager,
    /// Attempts persistence after `max_interval` since the first unsaved success; idle reads do not force writes.
    /// Explicit checkpoints, graceful stop, and topology recovery can flush sooner; acknowledged changes may replay.
    Deferred {
        /// Positive maximum interval before attempting persistence of successful progress.
        max_interval: Duration,
    },
}

impl Default for ChangeFeedCheckpointPolicy {
    fn default() -> Self {
        Self::deferred()
    }
}

impl ChangeFeedCheckpointPolicy {
    /// Creates a [`Self::Deferred`] policy with a five-minute interval.
    pub fn deferred() -> Self {
        Self::Deferred {
            max_interval: Duration::from_secs(300),
        }
    }
}

/// Identifies a callback batch's lease and read diagnostics.
/// Retaining this context does not prove current lease ownership or authorize checkpoint writes.
#[derive(Clone, SafeDebug)]
pub struct ChangeFeedProcessorContext {
    lease_token: String,
    #[safe(false)]
    diagnostics: Arc<DiagnosticsContext>,
}

impl ChangeFeedProcessorContext {
    /// Returns the opaque lease identifier within this workload, not a continuation token or physical partition ID.
    pub fn lease_token(&self) -> &str {
        &self.lease_token
    }

    /// Returns diagnostics for the batch's read, not callback execution or checkpoint writes.
    /// Retaining the handle does not retain the payload, render diagnostics, or log them.
    pub fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        self.diagnostics.clone()
    }
}

/// Describes observed lease state; monitoring is not implemented and currently produces no records.
#[derive(Clone, SafeDebug)]
pub struct ChangeFeedProcessorState {
    lease_token: String,
    owner_worker_name: Option<String>,
    continuation_token: Option<ContinuationToken>,
    estimated_lag: Option<u64>,
}

impl ChangeFeedProcessorState {
    /// Returns the opaque lease identifier, matching [`ChangeFeedProcessorContext::lease_token()`].
    pub fn lease_token(&self) -> &str {
        &self.lease_token
    }

    /// Returns the observed owner's worker name, or `None` for an unowned lease.
    pub fn owner_worker_name(&self) -> Option<&str> {
        self.owner_worker_name.as_deref()
    }

    /// Returns the saved lease-scoped continuation, if available; this is not a container-wide restart token.
    pub fn continuation_token(&self) -> Option<&ContinuationToken> {
        self.continuation_token.as_ref()
    }

    /// Returns estimated outstanding changes, or `None` if unavailable.
    /// `Some(0)` means a zero estimate, not an exact count of remaining documents or batches.
    pub fn estimated_lag(&self) -> Option<u64> {
        self.estimated_lag
    }
}

/// Configures a processor with a required [`ChangeFeedHandler`] supplied at construction.
/// Each worker processes batches sequentially; use additional workers for concurrency.
#[must_use]
pub struct ChangeFeedProcessorBuilder<T, Fut> {
    monitored_container: ContainerClient,
    lease_container: ContainerClient,
    workload_name: String,
    worker_name: String,
    mode: ChangeFeedMode,
    start_from: ChangeFeedStartFrom,
    start_continuation: Option<ContinuationToken>,
    max_item_count: MaxItemCountHint,
    feed_poll_delay: Duration,
    bootstrap_timeout: Duration,
    checkpoint_policy: ChangeFeedCheckpointPolicy,
    min_scale_count: u32,
    max_scale_count: u32,
    lease_verification_on_restart_enabled: bool,
    handler: ChangeFeedHandler<T, Fut>,
}

impl<T, Fut> ChangeFeedProcessorBuilder<T, Fut> {
    /// Creates a builder with a required async function-pointer callback, without performing I/O.
    /// Cooperating workers share `workload_name` and use distinct `worker_name` values.
    pub fn new(
        mode: ChangeFeedMode,
        monitored_container: ContainerClient,
        lease_container: ContainerClient,
        workload_name: impl Into<String>,
        worker_name: impl Into<String>,
        handler: ChangeFeedHandler<T, Fut>,
    ) -> Self
    where
        T: DeserializeOwned + Send + 'static,
        Fut: Future<Output = crate::Result<()>> + Send + 'static,
    {
        Self {
            monitored_container,
            lease_container,
            workload_name: workload_name.into(),
            worker_name: worker_name.into(),
            mode,
            start_from: ChangeFeedStartFrom::Now,
            start_continuation: None,
            max_item_count: MaxItemCountHint::Limit(NonZeroU32::new(100).expect("100 is nonzero")),
            feed_poll_delay: Duration::from_secs(5),
            bootstrap_timeout: Duration::from_secs(120),
            checkpoint_policy: ChangeFeedCheckpointPolicy::default(),
            min_scale_count: 0,
            max_scale_count: 0,
            lease_verification_on_restart_enabled: false,
            handler,
        }
    }

    /// Selects checkpoint persistence; defaults to [`ChangeFeedCheckpointPolicy::deferred()`] with a five-minute interval.
    /// Callbacks may replay; a `Now` start can move on restart before the first durable checkpoint.
    pub fn with_checkpoint_policy(mut self, policy: ChangeFeedCheckpointPolicy) -> Self {
        self.checkpoint_policy = policy;
        self
    }

    /// Sets the per-lease page-size hint (default 100), not a hard batch-size or memory limit.
    /// Use [`MaxItemCountHint::ServerDecides`] to let the service choose.
    pub fn with_max_item_count(mut self, max_item_count: MaxItemCountHint) -> Self {
        self.max_item_count = max_item_count;
        self
    }

    /// Sets the delay after an idle read, independently per lease; defaults to five seconds.
    /// Zero disables this delay; nonempty batches do not incur it.
    pub fn with_feed_poll_delay(mut self, feed_poll_delay: Duration) -> Self {
        self.feed_poll_delay = feed_poll_delay;
        self
    }

    /// Sets this caller's bootstrap wait budget (default 120 seconds); must be positive and representable.
    /// Does not extend initialization claims or cover optional restart lease verification.
    pub fn with_bootstrap_timeout(mut self, bootstrap_timeout: Duration) -> Self {
        self.bootstrap_timeout = bootstrap_timeout;
        self
    }

    /// Sets the initial continuation, replacing previous start settings; saved lease progress takes precedence.
    /// [`build()`](Self::build) rejects blank tokens.
    pub fn with_start_continuation(mut self, continuation: ContinuationToken) -> Self {
        self.start_from = ChangeFeedStartFrom::Now;
        self.start_continuation = Some(continuation);
        self
    }

    /// Sets the minimum lease count (default zero); [`build()`](Self::build) rejects values above a nonzero maximum.
    pub fn with_min_scale_count(mut self, min_scale_count: u32) -> Self {
        self.min_scale_count = min_scale_count;
        self
    }

    /// Sets the maximum lease count; zero (the default) means uncapped.
    /// [`build()`](Self::build) rejects a nonzero maximum below the configured minimum.
    pub fn with_max_scale_count(mut self, max_scale_count: u32) -> Self {
        self.max_scale_count = max_scale_count;
        self
    }

    /// Enables missing-lease verification on restart; disabled by default.
    /// Preserves existing progress and split-descendant coverage; invalid coverage fails startup.
    pub fn with_lease_verification_enabled_on_restart(mut self, enabled: bool) -> Self {
        self.lease_verification_on_restart_enabled = enabled;
        self
    }

    /// Starts from the beginning only when no saved progress exists, replacing the previous start setting.
    /// Requires [`ChangeFeedMode::LatestVersion`]; [`build()`](Self::build) rejects other modes.
    pub fn with_start_from_beginning(mut self) -> Self {
        self.start_from = ChangeFeedStartFrom::Beginning;
        self.start_continuation = None;
        self
    }

    /// Starts after `start_time` only when no saved progress exists, replacing the previous start setting.
    /// Requires [`ChangeFeedMode::LatestVersion`]; [`build()`](Self::build) rejects other modes.
    pub fn with_start_time(mut self, start_time: OffsetDateTime) -> Self {
        self.start_from = ChangeFeedStartFrom::PointInTime(start_time);
        self.start_continuation = None;
        self
    }

    /// Starts from now only when no saved progress exists; this is the default and replaces the previous start setting.
    pub fn with_start_from_now(mut self) -> Self {
        self.start_from = ChangeFeedStartFrom::Now;
        self.start_continuation = None;
        self
    }
    /// Consumes the configured builder without I/O.
    /// Returns [`StatusCode::BadRequest`] for invalid names, lease limits, start/mode combinations, intervals, or timeouts.
    pub fn build(self) -> crate::Result<ChangeFeedProcessor<T, Fut>>
    where
        T: DeserializeOwned + Send + 'static,
        Fut: Future<Output = crate::Result<()>> + Send + 'static,
    {
        if self.bootstrap_timeout.is_zero()
            || std::time::Instant::now()
                .checked_add(self.bootstrap_timeout)
                .is_none()
            || azure_core::time::Duration::try_from(self.bootstrap_timeout).is_err()
        {
            return Err(CosmosError::builder()
                .with_status(CosmosStatus::new(StatusCode::BadRequest))
                .with_message("Invalid bootstrap timeout.")
                .build());
        }
        if let ChangeFeedCheckpointPolicy::Deferred { max_interval } = self.checkpoint_policy {
            let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as i64;
            if max_interval.is_zero()
                || std::time::Instant::now()
                    .checked_add(max_interval)
                    .is_none()
                || azure_core::time::Duration::try_from(max_interval).is_err()
                || max_interval
                    .checked_mul(2)
                    .and_then(|duration| {
                        i64::try_from(duration.as_nanos().div_ceil(1_000_000)).ok()
                    })
                    .and_then(|millis| now.checked_add(millis))
                    .is_none()
            {
                return Err(CosmosError::builder()
                    .with_status(CosmosStatus::new(StatusCode::BadRequest))
                    .with_message("Invalid checkpoint interval.")
                    .build());
            }
        }
        for (name, value) in [
            ("workload_name", self.workload_name.as_str()),
            ("worker_name", self.worker_name.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(CosmosError::builder()
                    .with_status(CosmosStatus::new(StatusCode::BadRequest))
                    .with_message(format!("{name} must not be blank."))
                    .build());
            }
        }
        if self.max_scale_count != 0 && self.min_scale_count > self.max_scale_count {
            return Err(CosmosError::builder()
                .with_status(CosmosStatus::new(StatusCode::BadRequest))
                .with_message("min_scale_count must not exceed a nonzero max_scale_count.")
                .build());
        }
        if self
            .start_continuation
            .as_ref()
            .is_some_and(|token| token.as_str().trim().is_empty())
        {
            return Err(CosmosError::builder()
                .with_status(CosmosStatus::new(StatusCode::BadRequest))
                .with_message("start_continuation must not be blank.")
                .build());
        }
        if self.mode == ChangeFeedMode::AllVersionsAndDeletes
            && matches!(
                &self.start_from,
                ChangeFeedStartFrom::Beginning | ChangeFeedStartFrom::PointInTime(_)
            )
        {
            return Err(CosmosError::builder()
                .with_status(CosmosStatus::new(StatusCode::BadRequest))
                .with_message(
                    "AllVersionsAndDeletes mode does not support beginning or time-based starts.",
                )
                .build());
        }
        Ok(ChangeFeedProcessor {
            configuration: self,
            running: async_lock::Mutex::new(None),
            started: Arc::new(AtomicBool::new(false)),
        })
    }
}

/// Processes documents of type `T` using a callback returning `Fut`; failures can cause replay.
/// Drop requests cancellation without waiting; use [`stop()`](Self::stop) to confirm graceful cleanup.
#[must_use]
pub struct ChangeFeedProcessor<T, Fut> {
    configuration: ChangeFeedProcessorBuilder<T, Fut>,
    running: async_lock::Mutex<Option<execution::Running>>,
    started: Arc<AtomicBool>,
}

impl<T, Fut> ChangeFeedProcessor<T, Fut> {
    /// Returns local running state, not proof of lease ownership or service connectivity.
    pub fn is_started(&self) -> bool {
        self.started.load(Ordering::Acquire)
    }

    fn mark_started(&self) {
        self.started.store(true, Ordering::Release);
    }

    /// Not implemented; always returns [`StatusCode::NotImplemented`] without performing I/O.
    pub async fn current_state(&self) -> crate::Result<Vec<ChangeFeedProcessorState>> {
        Err(CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::NotImplemented))
            .with_message("Change feed processor monitoring is not implemented.")
            .build())
    }
}

#[cfg(all(test, feature = "__internal_in_memory_emulator"))]
mod tests;
