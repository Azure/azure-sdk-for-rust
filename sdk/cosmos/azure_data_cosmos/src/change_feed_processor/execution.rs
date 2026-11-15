// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    ChangeFeedCheckpointPolicy, ChangeFeedHandler, ChangeFeedProcessor, ChangeFeedProcessorContext,
};
use crate::{options::ChangeFeedMode, ContainerClient, CosmosError, CosmosStatus};
use azure_core::{async_runtime::get_async_runtime, http::StatusCode};
use azure_data_cosmos_driver::{
    change_feed_processor::{ChangeFeedBatch, CheckpointPolicy, Processor, ProcessorOptions},
    models::ChangeFeedFactoryMode as DriverMode,
};
use futures::{
    channel::oneshot,
    future::{BoxFuture, Shared},
    FutureExt,
};
use serde::de::DeserializeOwned;
use std::{future::Future, panic::AssertUnwindSafe, sync::atomic::Ordering};

pub(super) struct Running {
    driver: Processor,
    stopped: Shared<BoxFuture<'static, crate::Result<()>>>,
}

impl Drop for Running {
    fn drop(&mut self) {
        self.driver.request_stop();
    }
}

impl<T, Fut> ChangeFeedProcessor<T, Fut> {
    /// Initializes the workload and starts processing; the lease container requires `/id` and default TTL `-1`.
    /// Returns startup errors or [`StatusCode::Conflict`] if already running; later per-lease failures are logged.
    // 1. Borrow this processor; T is the document type and Fut is the callback future.
    pub async fn start(&self) -> crate::Result<()>
    where
        T: DeserializeOwned + Send + 'static,
        Fut: Future<Output = crate::Result<()>> + Send + 'static,
    {
        // 2. Serialize lifecycle changes, reject duplicate startup, and map SDK options.
        let mut running = self.running.lock().await;
        if self.is_started() {
            return Err(CosmosError::builder()
                .with_status(CosmosStatus::new(StatusCode::Conflict))
                .with_message("Change feed processor is already running.")
                .build());
        }
        let config = &self.configuration;
        let mode = match config.mode {
            ChangeFeedMode::LatestVersion => DriverMode::Incremental,
            ChangeFeedMode::AllVersionsAndDeletes => DriverMode::FullFidelity,
        };
        let mut options = ProcessorOptions::new(mode, &config.workload_name, &config.worker_name)
            .with_start(config.start_from.clone())
            .with_lease_verification_enabled_on_restart(
                config.lease_verification_on_restart_enabled,
            )
            .with_checkpoint_policy(match config.checkpoint_policy {
                ChangeFeedCheckpointPolicy::Eager => CheckpointPolicy::Eager,
                ChangeFeedCheckpointPolicy::Deferred { max_interval } => {
                    CheckpointPolicy::Deferred { max_interval }
                }
            })
            .with_max_item_count(config.max_item_count)
            .with_poll_delay(config.feed_poll_delay)
            .with_bootstrap_timeout(config.bootstrap_timeout)
            .with_lease_counts(config.min_scale_count, config.max_scale_count);
        if let Some(token) = &config.start_continuation {
            options = options.with_continuation(token.clone());
        }
        // 3. Await driver startup, including bootstrap; propagate failures before launching SDK processing.
        let driver = config
            .monitored_container
            .start_change_feed_processor(&config.lease_container, options)
            .await?;
        // 4. Prepare owned task inputs: shared handles and a copied callback pointer.
        let dispatcher = driver.clone();
        let container = config.monitored_container.clone();
        let handler = config.handler;
        let started = self.started.clone();
        self.mark_started();
        // 5. Create a one-shot completion signal for shutdown callers.
        let (done, stopped) = oneshot::channel();
        // 6. Run processing and cleanup in the background, then signal completion.
        let dispatcher_task = get_async_runtime().spawn(Box::pin(async move {
            let dispatching = consume::<T, Fut>(dispatcher.clone(), container, handler);
            let result = AssertUnwindSafe(dispatching)
                .catch_unwind()
                .await
                .map_err(|_| {
                    dispatcher.request_stop();
                    CosmosError::builder()
                        .with_status(CosmosStatus::new(StatusCode::InternalServerError))
                        .with_message("Change feed dispatcher panicked.")
                        .build()
                });
            if let Err(err) = dispatcher.stop().await {
                tracing::debug!(error = %err, "Processor shutdown completed with an error");
            }
            started.store(false, Ordering::Release);
            let _ = done.send(result);
        }));
        // Discard the handle, not the running task: awaiting it here would block startup until processing ends.
        // Shutdown uses the driver; the saved completion future below confirms dispatcher cleanup.
        drop(dispatcher_task);
        // 7. Save the driver and shared shutdown future; return without waiting for processing to finish.
        *running = Some(Running {
            driver,
            stopped: async move {
                stopped.await.map_err(|_| {
                    CosmosError::builder()
                        .with_status(CosmosStatus::new(StatusCode::InternalServerError))
                        .with_message("Change feed dispatcher did not complete.")
                        .build()
                })?
            }
            .boxed()
            .shared(),
        });
        Ok(())
    }

    /// Drains callbacks, persists progress, and releases leases; cleanup failures return errors and unsaved work may replay.
    /// Has no implicit timeout; canceling this wait leaves shutdown running.
    pub async fn stop(&self) -> crate::Result<()> {
        let running = self
            .running
            .lock()
            .await
            .as_ref()
            .map(|running| (running.driver.clone(), running.stopped.clone()));
        let Some((driver, stopped)) = running else {
            return Ok(());
        };
        let result = driver.stop().await;
        let dispatched = stopped.await;
        result.and(dispatched)
    }
}

async fn consume<T, Fut>(
    driver: Processor,
    container: ContainerClient,
    handler: ChangeFeedHandler<T, Fut>,
) where
    T: DeserializeOwned + Send + 'static,
    Fut: Future<Output = crate::Result<()>> + Send + 'static,
{
    while let Some(batch) = driver.read_next_change_feed_batch().await {
        dispatch::<T, Fut>(container.clone(), handler, batch).await;
    }
}

async fn dispatch<T, Fut>(
    container: ContainerClient,
    handler: ChangeFeedHandler<T, Fut>,
    batch: ChangeFeedBatch,
) where
    T: DeserializeOwned + Send + 'static,
    Fut: Future<Output = crate::Result<()>> + Send + 'static,
{
    let (lease_token, response, receipt) = batch.into_parts();
    let diagnostics = response.diagnostics();
    let work = async {
        let items = container.deserialize_processor_batch::<T>(response)?;
        if !items.is_empty() {
            handler(
                ChangeFeedProcessorContext {
                    lease_token: lease_token.clone(),
                    diagnostics,
                },
                items,
            )
            .await?;
        }
        Ok::<_, CosmosError>(())
    };
    let result = {
        let work = AssertUnwindSafe(work).catch_unwind().fuse();
        let cancelled = receipt.cancelled().fuse();
        futures::pin_mut!(work, cancelled);
        futures::select! {
            result = work => Some(result),
            _ = cancelled => None,
        }
    };
    match result {
        Some(Ok(Ok(()))) => {
            if let Err(err) = receipt.acknowledge().await {
                tracing::warn!(lease = %lease_token, error = %err, "Callback succeeded but acknowledgment failed");
            }
        }
        Some(Ok(Err(err))) => {
            tracing::warn!(lease = %lease_token, error = %err, "Change feed callback or deserialization failed; batch will be replayed");
        }
        Some(Err(_)) => {
            tracing::error!(lease = %lease_token, "Change feed callback panicked; batch will be replayed");
        }
        None => {
            tracing::debug!(lease = %lease_token, "Change feed callback canceled with its lease")
        }
    }
}
