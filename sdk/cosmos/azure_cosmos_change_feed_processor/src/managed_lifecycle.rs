// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{container_binding::invalid, ChangeFeedPage, ChangeFeedProcessor};
use azure_cosmos_change_feed_processor_engine::{
    ManagedProcessor, ManagedProcessorOptions, ManagedProcessorSnapshot, ManagedProcessorState,
    ManagedShutdownReport, RawChangeHandler,
};
use serde::de::DeserializeOwned;
use std::{future::Future, sync::Arc};

pub(crate) enum Lifecycle {
    Prepared,
    Running(ManagedProcessor),
    Stopped(Arc<ManagedShutdownReport>),
}

/// The owned Rust facade's current lifecycle and its execution evidence.
#[non_exhaustive]
pub enum ProcessorLifecycleState {
    /// Resource preparation succeeded; no managed execution has started.
    Prepared,
    /// Scheduling is active or has failed; inspect the per-lease health snapshot.
    Running(ManagedProcessorSnapshot),
    /// Shutdown was requested but its completion-bearing future still needs awaiting.
    Stopping(ManagedProcessorSnapshot),
    /// The coordinator returned its joined task and release outcomes.
    Stopped(Arc<ManagedShutdownReport>),
}

impl ChangeFeedProcessor {
    /// Starts continuous typed processing using the prepared feed and lease contexts.
    ///
    /// Startup loads the winning saved plan or discovers initial positions, verifies
    /// readiness, and starts scheduling. It does not promise every lease is healthy.
    /// The same `Fn` callback is shared across leases; its concurrency limit is
    /// distinct from the host's lease capacity. Each lease awaits actual callback
    /// completion and confirmed checkpointing before reading another batch.
    ///
    /// Concurrent lifecycle calls serialize. Repeated start while running or
    /// stopping is rejected. Cancelling startup abandons that attempt; cancelling
    /// stop retains its owned completion future for a subsequent stop call.
    /// Handlers must yield cooperatively and await work they schedule.
    ///
    /// # Errors
    ///
    /// Returns missing bindings, lifecycle conflicts, configuration, source discovery,
    /// authentication, or initialization errors. No Azure resources are provisioned.
    pub async fn start<T, H, F>(
        &self,
        options: ManagedProcessorOptions,
        handler: H,
    ) -> azure_core::Result<()>
    where
        T: DeserializeOwned + Send + 'static,
        H: Fn(ChangeFeedPage<T>) -> F + Send + Sync + 'static,
        F: Future<Output = azure_data_cosmos_driver::Result<()>> + Send + 'static,
    {
        let prepared = self.prepared.as_ref().ok_or_else(|| {
            invalid("managed execution requires independently prepared feed and lease bindings")
        })?;
        let mut lifecycle = prepared.lifecycle.lock().await;
        if matches!(&*lifecycle, Lifecycle::Running(_)) {
            return Err(invalid("processor is already running or stopping").into());
        }
        if let Lifecycle::Stopped(report) = &*lifecycle {
            if !report.is_clean() {
                return Err(
                    invalid("restart requires a previously confirmed clean shutdown").into(),
                );
            }
        }
        let handler = Arc::new(handler);
        let callback: RawChangeHandler = Arc::new(move |raw| {
            let handler = handler.clone();
            Box::pin(async move {
                let page: ChangeFeedPage<T> = raw.try_into()?;
                handler(page).await
            })
        });
        let processor = self
            .engine
            .start_managed(
                prepared.leases.driver.clone(),
                prepared.leases.container.clone(),
                prepared.group.clone(),
                options,
                callback,
            )
            .await?;
        *lifecycle = Lifecycle::Running(processor);
        Ok(())
    }

    /// Stops acquisitions and deliveries, then awaits drain, task joins, and release outcomes.
    ///
    /// Returns none if managed execution was never started. Repeated stop returns
    /// the same report. Report errors and release outcomes determine whether shutdown
    /// was clean; a cancellation request alone is not successful shutdown.
    /// Cancelling this method leaves its completion future owned by the processor.
    ///
    pub async fn stop(&self) -> Option<Arc<ManagedShutdownReport>> {
        let prepared = self.prepared.as_ref()?;
        let mut lifecycle = prepared.lifecycle.lock().await;
        if let Lifecycle::Stopped(report) = &*lifecycle {
            return Some(report.clone());
        }
        let Lifecycle::Running(processor) = &mut *lifecycle else {
            return None;
        };
        let report = processor.stop().await;
        if processor.is_complete() {
            *lifecycle = Lifecycle::Stopped(report.clone());
        }
        Some(report)
    }

    /// Returns lifecycle and per-lease health without service I/O.
    ///
    /// This waits for the lifecycle mutex, not for lease processing. A cancelled
    /// shutdown is reported as stopping until another stop call finishes joining.
    pub async fn state(&self) -> ProcessorLifecycleState {
        let Some(prepared) = &self.prepared else {
            return ProcessorLifecycleState::Prepared;
        };
        match &*prepared.lifecycle.lock().await {
            Lifecycle::Prepared => ProcessorLifecycleState::Prepared,
            Lifecycle::Running(processor)
                if processor.state() == ManagedProcessorState::Stopping =>
            {
                ProcessorLifecycleState::Stopping(processor.snapshot())
            }
            Lifecycle::Running(processor) => ProcessorLifecycleState::Running(processor.snapshot()),
            Lifecycle::Stopped(report) => ProcessorLifecycleState::Stopped(report.clone()),
        }
    }
}
