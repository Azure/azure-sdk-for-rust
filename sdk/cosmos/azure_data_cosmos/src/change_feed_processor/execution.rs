// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    ChangeFeedCheckpointPolicy, ChangeFeedProcessor, ChangeFeedProcessorContext, ErrorHandler,
    ProcessorStartFrom,
};
use crate::{
    models::ChangeFeedItem, options::ChangeFeedMode, ContainerClient, CosmosError, CosmosStatus,
};
use azure_core::{async_runtime::get_async_runtime, http::StatusCode};
use azure_data_cosmos_driver::{
    change_feed_processor::{ChangeFeedBatch, CheckpointPolicy, Processor, ProcessorOptions},
    models::{ChangeFeedFactoryMode as DriverMode, ChangeFeedStartFrom},
};
use futures::{
    channel::{mpsc, oneshot},
    future::{BoxFuture, Shared},
    stream::FuturesUnordered,
    FutureExt, StreamExt,
};
use serde::de::DeserializeOwned;
use std::{
    future::Future,
    panic::AssertUnwindSafe,
    sync::{atomic::Ordering, Arc, Mutex},
};

const ERROR_QUEUE_CAPACITY: usize = 64;
type ErrorNotification = (Option<ChangeFeedProcessorContext>, CosmosError);

struct ErrorReporter {
    sender: Mutex<Option<mpsc::Sender<ErrorNotification>>>,
    cancel: Mutex<Option<oneshot::Sender<()>>>,
}

impl ErrorReporter {
    fn new(
        enabled: bool,
    ) -> (
        Arc<Self>,
        mpsc::Receiver<ErrorNotification>,
        oneshot::Receiver<()>,
    ) {
        // The single sender reserves one additional slot.
        let (sender, received) = mpsc::channel(ERROR_QUEUE_CAPACITY - 1);
        let (cancel, canceled) = oneshot::channel();
        (
            Arc::new(Self {
                sender: Mutex::new(enabled.then_some(sender)),
                cancel: Mutex::new(Some(cancel)),
            }),
            received,
            canceled,
        )
    }

    fn report(&self, lease: Option<String>, error: CosmosError) {
        let mut sender = self.sender.lock().unwrap_or_else(|err| {
            tracing::error!("Recovering poisoned error notification sender");
            err.into_inner()
        });
        if let Some(sender) = sender.as_mut() {
            let context = lease.map(|lease_token| ChangeFeedProcessorContext {
                lease_token,
                diagnostics: error.diagnostics(),
            });
            if let Err(err) = sender.try_send((context, error)) {
                tracing::warn!(error = %err, "Processor error notification was dropped; error dispatcher is full or closed");
            }
        }
    }

    fn close(&self) {
        self.sender
            .lock()
            .unwrap_or_else(|err| {
                tracing::error!("Recovering poisoned error notification sender");
                err.into_inner()
            })
            .take();
    }

    fn cancel(&self) {
        if let Some(cancel) = self
            .cancel
            .lock()
            .unwrap_or_else(|err| {
                tracing::error!("Recovering poisoned error observer cancellation");
                err.into_inner()
            })
            .take()
        {
            let _ = cancel.send(());
        }
    }
}

async fn report_errors(
    handler: Option<Arc<ErrorHandler>>,
    mut received: mpsc::Receiver<ErrorNotification>,
    canceled: oneshot::Receiver<()>,
) {
    let Some(handler) = handler else { return };
    let canceled = canceled.fuse();
    futures::pin_mut!(canceled);
    loop {
        let notification = futures::select_biased! {
            _ = canceled => return,
            notification = received.next().fuse() => notification,
        };
        let Some((context, error)) = notification else {
            return;
        };
        let reporting = AssertUnwindSafe(async { handler(context, error).await })
            .catch_unwind()
            .fuse();
        futures::pin_mut!(reporting);
        futures::select_biased! {
            _ = canceled => return,
            result = reporting => {
                if result.is_err() {
                    tracing::error!("Processor error callback panicked");
                }
            },
        }
    }
}

pub(super) struct Running {
    driver: Processor,
    stopped: Shared<BoxFuture<'static, crate::Result<()>>>,
    errors: Arc<ErrorReporter>,
}

impl Drop for Running {
    fn drop(&mut self) {
        self.driver.request_stop();
        self.errors.cancel();
    }
}

pub(super) async fn start<T, H, Fut>(processor: &ChangeFeedProcessor<T, H>) -> crate::Result<()>
where
    T: DeserializeOwned + Send + 'static,
    H: Fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = crate::Result<()>> + Send + 'static,
{
    let mut running = processor.running.lock().await;
    if processor.is_started() {
        return Err(CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::Conflict))
            .with_message("Change feed processor is already running.")
            .build());
    }
    let config = &processor.configuration;
    let (errors, notifications, error_canceled) =
        ErrorReporter::new(config.error_handler.is_some());
    let mode = match config.mode {
        ChangeFeedMode::LatestVersion => DriverMode::Incremental,
        ChangeFeedMode::AllVersionsAndDeletes => DriverMode::FullFidelity,
    };
    let mut options = ProcessorOptions::new(mode, &config.processor_name, &config.instance_name)
        .with_lease_verification_enabled_on_restart(config.lease_verification_on_restart_enabled)
        .with_checkpoint_policy(match config.checkpoint_policy {
            ChangeFeedCheckpointPolicy::EveryBatch => CheckpointPolicy::EveryBatch,
            ChangeFeedCheckpointPolicy::Coalesced { max_interval } => {
                CheckpointPolicy::Coalesced { max_interval }
            }
        })
        .with_max_item_count(config.max_item_count)
        .with_poll_delay(config.feed_poll_delay)
        .with_lease_counts(config.min_scale_count, config.max_scale_count);
    options = match &config.start_from {
        ProcessorStartFrom::Now => options.with_start(ChangeFeedStartFrom::Now),
        ProcessorStartFrom::Beginning => options.with_start(ChangeFeedStartFrom::Beginning),
        ProcessorStartFrom::PointInTime(time) => {
            options.with_start(ChangeFeedStartFrom::PointInTime(*time))
        }
        ProcessorStartFrom::Continuation(token) => options.with_continuation(token.clone()),
    };
    if config.error_handler.is_some() {
        let errors = errors.clone();
        options = options.with_error_observer(move |lease, error| {
            errors.report(lease, error);
        });
    }
    let driver = config
        .monitored_container
        .start_change_feed_processor(&config.lease_container, options)
        .await?;
    let dispatcher = driver.clone();
    let container = config.monitored_container.clone();
    let handler = config.handler.clone();
    let started = processor.started.clone();
    let concurrency = config.max_concurrency.get();
    let error_handler = config.error_handler.clone();
    let reporting = errors.clone();
    started.store(true, Ordering::Release);
    let (done, stopped) = oneshot::channel();
    let task = get_async_runtime().spawn(Box::pin(async move {
        let processing = async {
            let dispatching = async {
                let mut workers = FuturesUnordered::new();
                for _ in 0..concurrency {
                    workers.push(consume::<T, H, Fut>(
                        dispatcher.clone(),
                        container.clone(),
                        handler.clone(),
                        reporting.clone(),
                    ));
                }
                while workers.next().await.is_some() {}
            };
            let result = AssertUnwindSafe(dispatching)
                .catch_unwind()
                .await
                .map_err(|_| {
                    dispatcher.request_stop();
                    reporting.cancel();
                    CosmosError::builder()
                        .with_status(CosmosStatus::new(StatusCode::InternalServerError))
                        .with_message("Change feed dispatcher panicked.")
                        .build()
                });
            if let Err(err) = dispatcher.stop().await {
                tracing::debug!(error = %err, "Processor shutdown completed with an error");
            }
            reporting.close();
            result
        };
        let (result, ()) = futures::join!(
            processing,
            report_errors(error_handler, notifications, error_canceled),
        );
        started.store(false, Ordering::Release);
        let _ = done.send(result);
    }));
    drop(task);
    *running = Some(Running {
        driver,
        errors,
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

async fn consume<T, H, Fut>(
    driver: Processor,
    container: ContainerClient,
    handler: Arc<H>,
    errors: Arc<ErrorReporter>,
) where
    T: DeserializeOwned + Send + 'static,
    H: Fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = crate::Result<()>> + Send + 'static,
{
    while let Some(batch) = driver.next_batch().await {
        dispatch::<T, H, Fut>(container.clone(), handler.clone(), batch, &errors).await;
    }
}

async fn dispatch<T, H, Fut>(
    container: ContainerClient,
    handler: Arc<H>,
    batch: ChangeFeedBatch,
    errors: &ErrorReporter,
) where
    T: DeserializeOwned + Send + 'static,
    H: Fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut + Send + Sync + 'static,
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
                    diagnostics: Some(diagnostics),
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
                errors.report(Some(lease_token), err);
            }
        }
        Some(Ok(Err(err))) => {
            tracing::warn!(lease = %lease_token, error = %err, "Change feed callback or deserialization failed; batch will be replayed");
            drop(receipt);
            errors.report(Some(lease_token), err);
        }
        Some(Err(_)) => {
            tracing::error!(lease = %lease_token, "Change feed callback panicked; batch will be replayed");
            drop(receipt);
            errors.report(
                Some(lease_token),
                CosmosError::builder()
                    .with_status(CosmosStatus::new(StatusCode::InternalServerError))
                    .with_message("Change feed callback panicked.")
                    .build(),
            );
        }
        None => {
            tracing::debug!(lease = %lease_token, "Change feed callback canceled with its lease")
        }
    }
}

pub(super) async fn stop<T, H>(
    processor: &ChangeFeedProcessor<T, H>,
    force: bool,
) -> crate::Result<()> {
    let running = processor.running.lock().await.as_ref().map(|running| {
        (
            running.driver.clone(),
            running.stopped.clone(),
            running.errors.clone(),
        )
    });
    let Some((driver, stopped, errors)) = running else {
        return Ok(());
    };
    let result = if force {
        errors.cancel();
        driver.abort().await
    } else {
        driver.stop().await
    };
    let dispatched = stopped.await;
    result.and(dispatched)
}

#[cfg(test)]
mod tests;
