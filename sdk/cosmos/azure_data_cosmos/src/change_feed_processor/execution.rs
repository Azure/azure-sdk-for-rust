// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    ChangeFeedProcessor, ChangeFeedProcessorContext, ChangeFeedProcessorMode, ProcessorStartFrom,
};
use crate::{
    models::ChangeFeedItem, options::ChangeFeedMode, ContainerClient, CosmosError, CosmosStatus,
};
use azure_core::{async_runtime::get_async_runtime, http::StatusCode};
use azure_data_cosmos_driver::{
    change_feed_processor::{
        Batch, BootstrapOptions, ChangeFeedProcessorMode as DriverMode, Processor, ProcessorOptions,
    },
    models::ChangeFeedStartFrom,
};
use futures::{
    channel::oneshot,
    future::{BoxFuture, Shared},
    stream::FuturesUnordered,
    FutureExt, StreamExt,
};
use serde::de::DeserializeOwned;
use std::{
    future::Future,
    panic::AssertUnwindSafe,
    sync::{atomic::Ordering, Arc},
};

pub(super) struct Running {
    driver: Processor,
    stopped: Shared<BoxFuture<'static, crate::Result<()>>>,
}

impl Drop for Running {
    fn drop(&mut self) {
        self.driver.request_stop();
    }
}

pub(super) async fn start<M, T, H, Fut>(
    processor: &ChangeFeedProcessor<M, T, H>,
) -> crate::Result<()>
where
    M: ChangeFeedProcessorMode,
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
    let mode = match config.mode.mode() {
        ChangeFeedMode::LatestVersion => DriverMode::LatestVersion,
        ChangeFeedMode::AllVersionsAndDeletes => DriverMode::AllVersionsAndDeletes,
    };
    let mut options = ProcessorOptions::new(
        BootstrapOptions::new(mode, &config.processor_name, &config.instance_name)
            .with_lease_verification_enabled_on_restart(
                config.lease_verification_on_restart_enabled,
            ),
    )
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
    let driver = config
        .monitored_container
        .start_change_feed_processor(&config.lease_container, options)
        .await?;
    let dispatcher = driver.clone();
    let container = config.monitored_container.clone();
    let handler = config.handler.clone();
    let started = processor.started.clone();
    started.store(true, Ordering::Release);
    let (done, stopped) = oneshot::channel();
    let task = get_async_runtime().spawn(Box::pin(async move {
        let dispatching = async {
        let mut callbacks = FuturesUnordered::<BoxFuture<'static, ()>>::new();
        loop {
            futures::select! {
                batch = dispatcher.next_batch().fuse() => {
                    let Some(batch) = batch else { break; };
                    callbacks.push(dispatch::<T, H, Fut>(container.clone(), handler.clone(), batch).boxed());
                },
                _ = async {
                    if callbacks.is_empty() { futures::future::pending().await }
                    else { callbacks.next().await }
                }.fuse() => {},
            }
        }
        while callbacks.next().await.is_some() {}
        };
        let result = AssertUnwindSafe(dispatching).catch_unwind().await.map_err(|_| {
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
    drop(task);
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

async fn dispatch<T, H, Fut>(container: ContainerClient, handler: Arc<H>, batch: Batch)
where
    T: DeserializeOwned + Send + 'static,
    H: Fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = crate::Result<()>> + Send + 'static,
{
    let (lease_token, response, receipt) = batch.into_parts();
    let work = async {
        let items = container.deserialize_processor_batch::<T>(response)?;
        if !items.is_empty() {
            handler(
                ChangeFeedProcessorContext {
                    lease_token: lease_token.clone(),
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
            if let Err(err) = receipt.checkpoint().await {
                tracing::warn!(lease = %lease_token, error = %err, "Callback succeeded but checkpoint failed");
            }
        }
        Some(Ok(Err(err))) => {
            tracing::warn!(lease = %lease_token, error = %err, "Change feed callback or deserialization failed; batch will be replayed")
        }
        Some(Err(_)) => {
            tracing::error!(lease = %lease_token, "Change feed callback panicked; batch will be replayed")
        }
        None => {
            tracing::debug!(lease = %lease_token, "Change feed callback canceled with its lease")
        }
    }
}

pub(super) async fn stop<M: ChangeFeedProcessorMode, T, H>(
    processor: &ChangeFeedProcessor<M, T, H>,
    force: bool,
) -> crate::Result<()> {
    let running = processor
        .running
        .lock()
        .await
        .as_ref()
        .map(|running| (running.driver.clone(), running.stopped.clone()));
    let Some((driver, stopped)) = running else {
        return Ok(());
    };
    let result = if force {
        driver.abort().await
    } else {
        driver.stop().await
    };
    let dispatched = stopped.await;
    result.and(dispatched)
}
