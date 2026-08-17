// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::{
    send_client::BufferedSendClient, DeliveryHandlers, SendBatchFailedContext,
    SendBatchSucceededContext,
};
use crate::{
    error::ErrorKind, models::EventData, producer::batch::EventDataBatchInner, EventHubsError,
};
use async_lock::SemaphoreGuardArc;
use azure_core::{sleep, time::Duration};
use azure_core_amqp::{AmqpMessage, AmqpSendOutcome};
use futures::{
    channel::{mpsc, oneshot},
    FutureExt, StreamExt,
};
use std::{
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};
use tracing::{debug, trace, warn};

/// A message that the client sends to a partition worker.
pub(crate) enum Command {
    /// One event to add to the active batch.
    ///
    /// The command carries the capacity permit for the event. The permit returns
    /// to the partition semaphore once the event reaches a terminal outcome, so
    /// the buffer bound covers every event that the client accepted.
    Event {
        event: EventData,
        message: Box<AmqpMessage>,
        permit: SemaphoreGuardArc,
    },

    /// A flush barrier.
    ///
    /// The worker sends the active batch, then it completes the sender. The
    /// position of this command in the queue is the barrier: the worker
    /// processes every command in front of it first, and no command behind it
    /// can delay it.
    Flush(oneshot::Sender<()>),
}

/// One event that the worker holds, with the capacity permit for the event.
type PendingEvent = (EventData, SemaphoreGuardArc);

/// Publishes the events of one partition.
///
/// One worker owns one partition. The worker is the only reader of the
/// partition queue, so the events keep the order that the caller enqueued them,
/// and only one send is active for the partition at a time.
pub(crate) struct PartitionWorker {
    partition_id: String,
    receiver: mpsc::UnboundedReceiver<Command>,
    send_client: Arc<dyn BufferedSendClient>,
    max_wait_time: Duration,
    max_events_per_batch: usize,
    handlers: DeliveryHandlers,
    buffered: Arc<AtomicUsize>,
    total_buffered: Arc<AtomicUsize>,
    abandon: Arc<AtomicBool>,

    /// Tells the client that this worker stopped.
    ///
    /// The worker sends on this channel when it leaves [`Self::run`]. A runtime
    /// that drops the task instead cancels the channel. Either way the client
    /// learns that the worker holds nothing more, without depending on what
    /// [`AbortableTask::abort`] does on the runtime in use.
    ///
    /// [`AbortableTask::abort`]: azure_core::async_runtime::AbortableTask
    stopped: Option<oneshot::Sender<()>>,
}

impl PartitionWorker {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        partition_id: String,
        receiver: mpsc::UnboundedReceiver<Command>,
        send_client: Arc<dyn BufferedSendClient>,
        max_wait_time: Duration,
        max_events_per_batch: usize,
        handlers: DeliveryHandlers,
        buffered: Arc<AtomicUsize>,
        total_buffered: Arc<AtomicUsize>,
        abandon: Arc<AtomicBool>,
        stopped: oneshot::Sender<()>,
    ) -> Self {
        Self {
            partition_id,
            receiver,
            send_client,
            max_wait_time,
            max_events_per_batch,
            handlers,
            buffered,
            total_buffered,
            abandon,
            stopped: Some(stopped),
        }
    }

    /// Runs the worker until the queue closes or the client abandons the events.
    pub(crate) async fn run(mut self) {
        debug!(
            partition_id = %self.partition_id,
            "Buffered producer partition worker started."
        );

        let mut batch: Option<EventDataBatchInner> = None;
        let mut pending: Vec<PendingEvent> = Vec::new();
        let mut timer: Option<Pin<Box<dyn Future<Output = ()> + Send>>> = None;

        loop {
            if self.abandon.load(Ordering::Acquire) {
                break;
            }

            let command = match timer.as_mut() {
                // The batch is empty. Wait for a command with no timer running.
                None => self.receiver.next().await,
                // The batch holds events. Send it when the wait time expires.
                Some(deadline) => {
                    futures::select! {
                        command = self.receiver.next().fuse() => command,
                        _ = deadline.as_mut().fuse() => {
                            debug!(
                                partition_id = %self.partition_id,
                                "Maximum wait time expired; sending the partial batch."
                            );
                            self.send_batch(&mut batch, &mut pending).await;
                            timer = None;
                            continue;
                        }
                    }
                }
            };

            let Some(command) = command else {
                // The client dropped every sender. Send what is left and stop.
                // A client that abandons its events takes the same path, so
                // `send_batch` must see the flag; it returns without a send.
                debug!(
                    partition_id = %self.partition_id,
                    "Partition queue closed; draining the active batch."
                );
                self.send_batch(&mut batch, &mut pending).await;
                break;
            };

            match command {
                Command::Event {
                    event,
                    message,
                    permit,
                } => {
                    self.add_event(
                        &mut batch,
                        &mut pending,
                        &mut timer,
                        event,
                        *message,
                        permit,
                    )
                    .await;

                    if pending.len() >= self.max_events_per_batch {
                        debug!(
                            partition_id = %self.partition_id,
                            event_count = pending.len(),
                            "Batch reached the configured event count; sending it."
                        );
                        self.send_batch(&mut batch, &mut pending).await;
                        timer = None;
                    }
                }
                Command::Flush(completed) => {
                    self.send_batch(&mut batch, &mut pending).await;
                    timer = None;
                    if self.abandon.load(Ordering::Acquire) {
                        // The client abandoned the events during this flush, so
                        // the barrier cannot report success. Dropping the sender
                        // cancels the waiter.
                        drop(completed);
                        break;
                    }
                    // A dropped receiver means the caller stopped waiting.
                    let _ = completed.send(());
                }
            }
        }

        if self.abandon.load(Ordering::Acquire) {
            self.discard_remaining(pending).await;
        }

        debug!(
            partition_id = %self.partition_id,
            "Buffered producer partition worker stopped."
        );

        // Drop everything this worker owns before the client hears about it.
        // The client takes the producer out of its Arc as soon as it reads the
        // acknowledgement, and a worker that still held the send client would
        // make that step find a second reference and skip the graceful close.
        let stopped = self.stopped.take();
        drop(self);
        if let Some(stopped) = stopped {
            let _ = stopped.send(());
        }
    }

    /// Adds one event to the active batch, and sends the batch when the event
    /// does not fit.
    #[allow(clippy::too_many_arguments)]
    async fn add_event(
        &self,
        batch: &mut Option<EventDataBatchInner>,
        pending: &mut Vec<PendingEvent>,
        timer: &mut Option<Pin<Box<dyn Future<Output = ()> + Send>>>,
        event: EventData,
        message: AmqpMessage,
        permit: SemaphoreGuardArc,
    ) {
        // The batch needs the maximum message size of the link, so it is created
        // on the first event.
        if batch.is_none() {
            match self.send_client.max_message_size(&self.partition_id).await {
                Ok(max_size) => {
                    *batch = Some(EventDataBatchInner::new(max_size, None));
                }
                Err(error) => {
                    warn!(
                        partition_id = %self.partition_id,
                        "Could not read the maximum message size; failing the event."
                    );
                    self.fail_one(event, permit, error).await;
                    return;
                }
            }
        }
        let active = batch.as_mut().expect("the batch was just created");

        match active.try_add(message.clone()) {
            Ok(true) => {
                trace!(
                    partition_id = %self.partition_id,
                    "Added an event to the active batch."
                );
                pending.push((event, permit));
                if timer.is_none() {
                    *timer = Some(Box::pin(sleep(self.max_wait_time)));
                }
            }
            Ok(false) if active.is_empty() => {
                // One event on its own is larger than the whole batch. Fail that
                // event only. The worker stays healthy.
                self.fail_oversized(event, permit).await;
            }
            Ok(false) => {
                // The batch is full. Send it, then start a new batch with this event.
                debug!(
                    partition_id = %self.partition_id,
                    "The next event does not fit; sending the full batch."
                );
                self.send_batch(batch, pending).await;
                *timer = None;

                let active = batch.as_mut().expect("the batch is reused after a send");
                match active.try_add(message) {
                    Ok(true) => {
                        pending.push((event, permit));
                        *timer = Some(Box::pin(sleep(self.max_wait_time)));
                    }
                    Ok(false) => self.fail_oversized(event, permit).await,
                    Err(error) => self.fail_one(event, permit, error).await,
                }
            }
            Err(error) => {
                warn!(
                    partition_id = %self.partition_id,
                    "Could not add an event to the batch; failing that event."
                );
                self.fail_one(event, permit, error).await;
            }
        }
    }

    async fn fail_oversized(&self, event: EventData, permit: SemaphoreGuardArc) {
        warn!(
            partition_id = %self.partition_id,
            "An event is too large for an empty batch; failing that event."
        );
        self.fail_one(
            event,
            permit,
            EventHubsError::with_message(
                "The event is too large for the maximum message size of the link.",
            ),
        )
        .await;
    }

    /// Reports one event as failed and returns its capacity permit.
    async fn fail_one(&self, event: EventData, permit: SemaphoreGuardArc, error: EventHubsError) {
        // Give the capacity back before the handler runs. The outcome is
        // already terminal, and the handler runs on this task, so a handler
        // that enqueues to this partition would otherwise wait for a permit
        // that only this task can return.
        self.release(1);
        drop(permit);
        self.report_failure(vec![event], error).await;
    }

    /// Sends the active batch and reports exactly one outcome for it.
    ///
    /// The method does nothing when the batch holds no events, so the worker
    /// never sends an empty batch. It also does nothing once the client
    /// abandons its events: every path that ends the worker calls this method,
    /// and an immediate close must not publish what it promised to drop. The
    /// events stay in `pending`, and [`Self::discard_remaining`] drops them.
    async fn send_batch(
        &self,
        batch: &mut Option<EventDataBatchInner>,
        pending: &mut Vec<PendingEvent>,
    ) {
        let Some(active) = batch.as_mut() else {
            return;
        };
        if active.is_empty() {
            return;
        }
        if self.abandon.load(Ordering::Acquire) {
            debug!(
                partition_id = %self.partition_id,
                event_count = pending.len(),
                "The client abandoned its events; not sending the active batch."
            );
            return;
        }

        let batch_size_in_bytes = active.size();
        let envelope = active.take_envelope();
        let (events, permits): (Vec<EventData>, Vec<SemaphoreGuardArc>) =
            std::mem::take(pending).into_iter().unzip();
        let event_count = events.len();

        debug!(
            partition_id = %self.partition_id,
            event_count,
            batch_size_in_bytes,
            "Sending a batch of events."
        );

        let outcome = self
            .send_client
            .send_envelope(&self.partition_id, envelope)
            .await;

        // The send settled, so every event in the batch is at a terminal
        // outcome whatever that outcome is. Give the capacity back before the
        // handlers run. The handlers run on this task, so a handler that
        // enqueues to this partition would otherwise wait for a permit that
        // only this task can return.
        self.release(event_count);
        drop(permits);

        match outcome {
            Ok(AmqpSendOutcome::Accepted) => {
                debug!(
                    partition_id = %self.partition_id,
                    event_count,
                    "The service accepted the batch."
                );
                self.report_success(events).await;
            }
            Ok(AmqpSendOutcome::Modified(reason)) => {
                // Modified does not mean that the service stored the events, so
                // this is a delivery failure, not a success.
                warn!(
                    partition_id = %self.partition_id,
                    event_count,
                    modification = ?reason,
                    "The service modified the batch; it did not durably accept it."
                );
                self.report_failure(
                    events,
                    EventHubsError::from(ErrorKind::SendNotAccepted(
                        "the service returned a Modified outcome".into(),
                    )),
                )
                .await;
            }
            Ok(AmqpSendOutcome::Released) => {
                warn!(
                    partition_id = %self.partition_id,
                    event_count,
                    "The service released the batch; it did not durably accept it."
                );
                self.report_failure(
                    events,
                    EventHubsError::from(ErrorKind::SendNotAccepted(
                        "the service returned a Released outcome".into(),
                    )),
                )
                .await;
            }
            Ok(AmqpSendOutcome::Rejected(reason)) => {
                warn!(
                    partition_id = %self.partition_id,
                    event_count,
                    "The service rejected the batch."
                );
                self.report_failure(
                    events,
                    EventHubsError::from(ErrorKind::SendRejected(reason)),
                )
                .await;
            }
            Err(error) => {
                // The recoverable sender already applied the retry policy, so
                // this error is terminal.
                warn!(
                    partition_id = %self.partition_id,
                    event_count,
                    "The batch failed after the retry policy was exhausted."
                );
                self.report_failure(events, error).await;
            }
        }
    }

    async fn report_success(&self, events: Vec<EventData>) {
        if let Some(handler) = self.handlers.succeeded.as_ref() {
            handler(SendBatchSucceededContext {
                partition_id: self.partition_id.clone(),
                events,
            })
            .await;
        }
    }

    async fn report_failure(&self, events: Vec<EventData>, error: EventHubsError) {
        (self.handlers.failed)(SendBatchFailedContext {
            partition_id: self.partition_id.clone(),
            events,
            error,
        })
        .await;
    }

    /// Removes events from the buffered counts once they reach a terminal outcome.
    fn release(&self, count: usize) {
        if count == 0 {
            return;
        }
        self.buffered.fetch_sub(count, Ordering::AcqRel);
        self.total_buffered.fetch_sub(count, Ordering::AcqRel);
    }

    /// Drops every event that the worker still holds after an immediate close.
    ///
    /// Dropping a command also returns its capacity permit, so a later caller is
    /// not blocked by an abandoned event.
    async fn discard_remaining(&mut self, pending: Vec<PendingEvent>) {
        let mut abandoned = pending.len();
        drop(pending);

        // Take everything that is still in the queue. The client already took
        // the sender, so this loop ends.
        self.receiver.close();
        while let Some(command) = self.receiver.next().await {
            match command {
                Command::Event { .. } => abandoned += 1,
                // A waiting flush must not hang. Dropping the sender cancels it.
                Command::Flush(completed) => drop(completed),
            }
        }

        if abandoned > 0 {
            warn!(
                partition_id = %self.partition_id,
                event_count = abandoned,
                "Abandoned buffered events during an immediate close."
            );
        }
    }
}
