use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
    time::Duration as StdDuration,
};

use fe2o3_amqp::{link::RecvError, session::SessionHandle, Receiver};
use futures_util::{future::poll_fn, ready, Future, FutureExt, SinkExt, Stream};
use tokio::sync::mpsc;

use url::Url;

use crate::{
    consumer::EventPosition,
    core::{RecoverableError, RecoverableTransport, TransportClient, TransportConsumer},
    event_hubs_retry_policy::EventHubsRetryPolicy,
    util::{self, sharable::Sharable},
    ReceivedEvent,
};

use super::{
    amqp_cbs_link::Command,
    amqp_client::AmqpClient,
    error::{DisposeConsumerError, RecoverAndReceiveError},
};

pub(crate) mod multiple;

pub struct AmqpConsumer<RP> {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) endpoint: Url,
    pub(crate) receiver: Receiver,
    pub(crate) link_identifier: u32,
    pub(crate) invalidate_consumer_when_partition_stolen: bool,
    pub(crate) track_last_enqueued_event_properties: bool,
    pub(crate) last_received_event: Option<ReceivedEvent>,
    pub(crate) current_event_position: Option<EventPosition>,
    pub(crate) retry_policy: RP,
    pub(crate) prefetch_count: u32,
    pub(crate) cbs_command_sender: mpsc::Sender<Command>,
}

impl<RP> AmqpConsumer<RP> {
    pub async fn close(mut self) -> Result<(), DisposeConsumerError> {
        // There is no need to remove the refresher if CBS link is already stopped
        let _ = self
            .cbs_command_sender
            .send(Command::RemoveAuthorizationRefresher(self.link_identifier))
            .await;

        self.receiver.close().await?;
        self.session_handle.close().await?;
        drop(self.session_handle);
        Ok(())
    }

    pub(crate) async fn recv_and_accept(&mut self) -> Result<ReceivedEvent, RecvError> {
        let delivery = self.receiver.recv().await?;
        self.receiver.accept(&delivery).await?;
        let event = ReceivedEvent::from_raw_amqp_message(delivery.into_message());

        let event_offset = event.offset().unwrap_or(i64::MIN);
        if event_offset > i64::MIN {
            self.current_event_position = Some(EventPosition::from_offset(event_offset, false));
        }

        if self.track_last_enqueued_event_properties {
            self.last_received_event = Some(event.clone());
        }

        Ok(event)
    }

    #[inline]
    async fn fill_buf(&mut self, buffer: &mut VecDeque<ReceivedEvent>) -> Result<(), RecvError> {
        // Only receive messages if there is space in the buffer
        let max_messages = buffer.capacity() - buffer.len();
        // Credit mode is manual, need to set credit
        if self.prefetch_count == 0 {
            // At least one credit is needed
            // max_messages is specified as u32, so it is safe to cast to u32
            let credit = max_messages.max(1) as u32;
            self.receiver.set_credit(credit).await?;
        }

        for _ in 0..max_messages {
            let delivery = self.receiver.recv().await?;
            self.receiver.accept(&delivery).await?;
            let event = ReceivedEvent::from_raw_amqp_message(delivery.into_message());

            let event_offset = event.offset().unwrap_or(i64::MIN);
            if event_offset > i64::MIN {
                self.current_event_position = Some(EventPosition::from_offset(event_offset, false));
            }

            buffer.push_back(event);
        }

        Ok(())
    }

    #[inline]
    async fn fill_buf_with_timeout(
        &mut self,
        buffer: &mut VecDeque<ReceivedEvent>,
        max_wait_time: StdDuration,
    ) -> Result<Option<()>, RecoverAndReceiveError> {
        futures_util::select_biased! {
            _ = crate::util::time::sleep(max_wait_time).fuse() => Ok(Some(())),
            result = self.fill_buf(buffer).fuse() => {
                result?;
                Ok(Some(()))
            }
        }
    }
}

async fn recover_and_recv<RP>(
    client: &mut Sharable<AmqpClient>,
    consumer: &mut AmqpConsumer<RP>,
    should_try_recover: bool,
    buffer: &mut VecDeque<ReceivedEvent>,
    max_wait_time: StdDuration,
) -> Result<Option<()>, RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send,
{
    if should_try_recover {
        if let Err(recovery_err) = client.recover().await {
            log::error!("Failed to recover client: {:?}", recovery_err);
            if recovery_err.is_scope_disposed() {
                return Err(recovery_err.into());
            }
        }

        // reattach the link
        match client {
            Sharable::Owned(client) => client.recover_consumer(consumer).await?,
            Sharable::Shared(client) => client.lock().await.recover_consumer(consumer).await?,
            Sharable::None => return Err(RecoverAndReceiveError::ConnectionScopeDisposed),
        }
    }

    match consumer
        .fill_buf_with_timeout(buffer, max_wait_time)
        .await?
    {
        Some(_) => {
            if consumer.track_last_enqueued_event_properties {
                if let Some(event) = buffer.back().cloned() {
                    consumer.last_received_event = Some(event);
                }
            }
            Ok(Some(()))
        }
        None => Ok(None),
    }
}

pub(crate) async fn receive_event_batch<RP>(
    client: &mut Sharable<AmqpClient>,
    consumer: &mut AmqpConsumer<RP>,
    buffer: &mut VecDeque<ReceivedEvent>,
    max_wait_time: Option<StdDuration>,
) -> Option<Result<(), RecoverAndReceiveError>>
where
    RP: EventHubsRetryPolicy + Send,
{
    let mut failed_attempts = 0;
    let mut try_timeout = consumer.retry_policy.calculate_try_timeout(failed_attempts);
    let mut should_try_recover = false;

    loop {
        let wait_time = max_wait_time.unwrap_or(try_timeout);
        let err = match recover_and_recv(client, consumer, should_try_recover, buffer, wait_time)
            .await
            .transpose()?
        {
            Ok(_) => return Some(Ok(())),
            Err(err) => err,
        };

        if err.is_scope_disposed() {
            return Some(Err(err));
        }
        should_try_recover = err.should_try_recover();

        failed_attempts += 1;
        let retry_delay = consumer
            .retry_policy
            .calculate_retry_delay(&err, failed_attempts);

        match retry_delay {
            Some(retry_delay) => {
                util::time::sleep(retry_delay).await;
                try_timeout = consumer.retry_policy.calculate_try_timeout(failed_attempts);
            }
            None => return Some(Err(err)),
        }
    }
}

async fn next_event_inner<RP>(
    client: &mut Sharable<AmqpClient>,
    consumer: &mut AmqpConsumer<RP>,
    buffer: &mut VecDeque<ReceivedEvent>,
    max_wait_time: Option<StdDuration>,
) -> Option<Result<ReceivedEvent, RecoverAndReceiveError>>
where
    RP: EventHubsRetryPolicy + Send,
{
    if let Some(event) = buffer.pop_front() {
        return Some(Ok(event));
    }

    loop {
        let result = receive_event_batch(client, consumer, buffer, max_wait_time).await?;

        match buffer.pop_front() {
            Some(event) => return Some(Ok(event)),
            None => {
                if let Err(err) = result {
                    return Some(Err(err));
                }
            }
        }
    }
}

async fn next_event<'a, RP>(
    mut value: EventStreamStateValue<'a, AmqpConsumer<RP>>,
) -> (
    Option<Result<ReceivedEvent, RecoverAndReceiveError>>,
    EventStreamStateValue<'a, AmqpConsumer<RP>>,
)
where
    RP: EventHubsRetryPolicy + Send,
{
    let outcome = next_event_inner(
        value.client,
        &mut value.consumer,
        &mut value.buffer,
        value.max_wait_time,
    )
    .await;
    (outcome, value)
}

async fn close_consumer<'a, RP>(
    value: EventStreamStateValue<'a, AmqpConsumer<RP>>,
) -> Result<(), DisposeConsumerError> {
    value.consumer.close().await
}

pub struct EventStreamStateValue<'a, C> {
    pub(crate) client: &'a mut Sharable<AmqpClient>,
    pub(crate) consumer: C,
    pub(crate) buffer: VecDeque<ReceivedEvent>,
    pub(crate) max_wait_time: Option<StdDuration>,
}

impl<'a, C> EventStreamStateValue<'a, C> {
    pub(crate) fn new(
        client: &'a mut Sharable<AmqpClient>,
        consumer: C,
        max_messages: u32,
        max_wait_time: Option<StdDuration>,
    ) -> Self {
        Self {
            client,
            consumer,
            buffer: VecDeque::with_capacity(max_messages as usize),
            max_wait_time,
        }
    }
}

type StreamBoxedFuture<'a, C> = Pin<
    Box<
        dyn Future<
                Output = (
                    Option<Result<ReceivedEvent, RecoverAndReceiveError>>,
                    EventStreamStateValue<'a, C>,
                ),
            > + Send
            + 'a,
    >,
>;
type ClosingBoxedFuture<'a> = Pin<Box<dyn Future<Output = Result<(), DisposeConsumerError>> + 'a>>;

pin_project_lite::pin_project! {
    #[project = EventStreamStateProj]
    #[project_replace = EventStreamStateProjReplace]
    pub(crate) enum EventStreamState<'a, C> {
        Value {
            value: EventStreamStateValue<'a, C>,
        },
        Future {
            #[pin]
            future: StreamBoxedFuture<'a, C>,
        },
        Closing {
            #[pin]
            future: ClosingBoxedFuture<'a>,
        },
        Empty,
    }
}

impl<'a, C> EventStreamState<'a, C>
where
    C: Send + 'a,
{
    pub(crate) fn project_future(
        self: Pin<&mut Self>,
    ) -> Option<Pin<&mut StreamBoxedFuture<'a, C>>> {
        match self.project() {
            EventStreamStateProj::Future { future } => Some(future),
            _ => None,
        }
    }

    pub(crate) fn project_closing(
        self: Pin<&mut Self>,
    ) -> Option<Pin<&mut ClosingBoxedFuture<'a>>> {
        match self.project() {
            EventStreamStateProj::Closing { future } => Some(future),
            _ => None,
        }
    }

    pub(crate) fn take_value(self: Pin<&mut Self>) -> Option<EventStreamStateValue<'a, C>> {
        match &*self {
            EventStreamState::Value { .. } => match self.project_replace(EventStreamState::Empty) {
                EventStreamStateProjReplace::Value { value } => Some(value),
                _ => unreachable!(),
            },
            _ => None,
        }
    }
}

impl<'a, RP> EventStreamState<'a, AmqpConsumer<RP>>
where
    RP: Send + 'a,
{
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Result<(), DisposeConsumerError>> {
        if let Some(value) = self.as_mut().take_value() {
            self.set(EventStreamState::Closing {
                future: close_consumer(value).boxed(),
            });
        }

        if let Some(future) = self.as_mut().project_future() {
            let (_, value) = ready!(future.poll(cx));
            self.set(EventStreamState::Closing {
                future: close_consumer(value).boxed(),
            });
        }

        let result = match self.as_mut().project_closing() {
            Some(fut) => ready!(fut.poll(cx)),
            None => panic!("EventStream must not be polled after it returned `Poll::Ready(None)`"),
        };

        self.set(EventStreamState::Empty);
        Poll::Ready(result)
    }

    async fn close(mut self) -> Result<(), DisposeConsumerError> {
        poll_fn(|cx| Pin::new(&mut self).poll_close(cx)).await
    }
}

pin_project_lite::pin_project! {
    pub struct EventStream<'a, C> {
        #[pin]
        state: EventStreamState<'a, C>,
    }
}

impl<'a, RP> EventStream<'a, AmqpConsumer<RP>>
where
    RP: Send + 'a,
{
    pub(crate) fn with_consumer(
        client: &'a mut Sharable<AmqpClient>,
        consumer: AmqpConsumer<RP>,
        max_messages: u32,
        max_wait_time: Option<StdDuration>,
    ) -> Self {
        let value = EventStreamStateValue::new(client, consumer, max_messages, max_wait_time);
        let state = EventStreamState::Value { value };

        Self { state }
    }

    pub async fn close(self) -> Result<(), DisposeConsumerError> {
        self.state.close().await
    }
}

impl<'a, RP> Stream for EventStream<'a, AmqpConsumer<RP>>
where
    RP: EventHubsRetryPolicy + Send + 'a,
{
    type Item = Result<ReceivedEvent, RecoverAndReceiveError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        if let Some(state) = this.state.as_mut().take_value() {
            this.state.set(EventStreamState::Future {
                future: next_event(state).boxed(),
            });
        }

        let (item, next_state) = match this.state.as_mut().project_future() {
            Some(fut) => ready!(fut.poll(cx)),
            None => panic!("EventStream must not be polled after it returned `Poll::Ready(None)`"),
        };

        if let Some(item) = item {
            this.state
                .set(EventStreamState::Value { value: next_state });
            Poll::Ready(Some(item))
        } else {
            this.state.set(EventStreamState::Closing {
                future: close_consumer(next_state).boxed(),
            });
            Poll::Ready(None)
        }
    }
}
