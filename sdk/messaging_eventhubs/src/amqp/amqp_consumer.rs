use std::{collections::VecDeque, time::Duration as StdDuration, task::{Poll, Context}, pin::Pin};

use fe2o3_amqp::{link::RecvError, session::SessionHandle, Receiver};
use futures_util::{FutureExt, SinkExt, Stream, Future, ready, future::poll_fn};
use tokio::sync::mpsc;
use url::Url;

use crate::{
    consumer::EventPosition,
    core::{RecoverableError, RecoverableTransport, TransportClient, TransportConsumer},
    event_hubs_retry_policy::EventHubsRetryPolicy,
    util::{self, sharable::Sharable, IntoAzureCoreError},
    ReceivedEvent,
};

use super::{
    amqp_cbs_link::Command,
    amqp_client::AmqpClient,
    error::{DisposeConsumerError, RecoverAndReceiveError},
};

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
    pub async fn dispose(mut self) -> Result<(), DisposeConsumerError> {
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

    #[inline]
    async fn receive_messages(
        &mut self,
        buffer: &mut VecDeque<ReceivedEvent>,
    ) -> Result<(), RecvError> {
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
    async fn receive_messages_with_timeout(
        &mut self,
        buffer: &mut VecDeque<ReceivedEvent>,
        max_wait_time: StdDuration,
    ) -> Result<(), RecoverAndReceiveError> {
        futures_util::select_biased! {
            _ = crate::util::time::sleep(max_wait_time).fuse() => Ok(()),
            result = self.receive_messages(buffer).fuse() => {
                result?;
                Ok(())
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
) -> Result<(), RecoverAndReceiveError>
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

    consumer
        .receive_messages_with_timeout(buffer, max_wait_time)
        .await?;
    if consumer.track_last_enqueued_event_properties {
        consumer.last_received_event = buffer.back().cloned();
    }
    Ok(())
}

async fn receive_event<RP>(
    client: &mut Sharable<AmqpClient>,
    consumer: &mut AmqpConsumer<RP>,
    buffer: &mut VecDeque<ReceivedEvent>,
    max_wait_time: Option<StdDuration>,
) -> Result<(), RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send,
{
    let mut failed_attempts = 0;
    let mut try_timeout = consumer.retry_policy.calculate_try_timeout(failed_attempts);
    let wait_time = max_wait_time.unwrap_or(try_timeout);
    let mut should_try_recover = false;

    loop {
        let err = match recover_and_recv(
            client,
            consumer,
            should_try_recover,
            buffer,
            wait_time,
        )
        .await
        {
            Ok(_) => return Ok(()),
            Err(err) => err,
        };

        if err.is_scope_disposed() {
            return Err(err);
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
            None => return Err(err),
        }
    }
}

async fn next_event_inner<RP>(
    client: &mut Sharable<AmqpClient>,
    consumer: &mut AmqpConsumer<RP>,
    buffer: &mut VecDeque<ReceivedEvent>,
    max_wait_time: Option<StdDuration>,
) -> Result<Option<ReceivedEvent>, RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send,
{
    if let Some(event) = buffer.pop_front() {
        return Ok(Some(event));
    }

    loop {
        let result = receive_event(client, consumer, buffer, max_wait_time).await;

        match buffer.pop_front() {
            Some(event) => return Ok(Some(event)),
            None => result?,
        }
    }
}

async fn next_event<'a, RP>(mut value: EventStreamStateValue<'a, RP>) -> (Option<Result<ReceivedEvent, RecoverAndReceiveError>>, EventStreamStateValue<'a, RP>)
where
    RP: EventHubsRetryPolicy + Send,
{
    match next_event_inner(value.client, &mut value.consumer, &mut value.buffer, value.max_wait_time).await {
        Ok(Some(event)) => (Some(Ok(event)), value),
        Ok(None) => (None, value),
        Err(err) => (Some(Err(err)), value),
    }
}

async fn dispose_consumer<'a, RP>(value: EventStreamStateValue<'a, RP>) -> Result<(), DisposeConsumerError> {
    value.consumer.dispose().await
}

pub struct EventStreamStateValue<'a, RP> {
    client: &'a mut Sharable<AmqpClient>,
    consumer: AmqpConsumer<RP>,
    buffer: VecDeque<ReceivedEvent>,
    max_wait_time: Option<StdDuration>,
}

impl<'a, RP> EventStreamStateValue<'a, RP> {
    pub(crate) fn new(
        client: &'a mut Sharable<AmqpClient>,
        consumer: AmqpConsumer<RP>,
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

type StreamBoxedFuture<'a, RP> = Pin<Box<dyn Future<Output = (Option<Result<ReceivedEvent, RecoverAndReceiveError>>, EventStreamStateValue<'a, RP>)> + 'a>>;
type ClosingBoxedFuture<'a> = Pin<Box<dyn Future<Output = Result<(), DisposeConsumerError>> + 'a>>;

pin_project_lite::pin_project! {
    #[project = EventStreamStateProj]
    #[project_replace = EventStreamStateProjReplace]
    enum EventStreamState<'a, RP> {
        Value {
            value: EventStreamStateValue<'a, RP>,
        },
        Future {
            #[pin]
            future: StreamBoxedFuture<'a, RP>,
        },
        Ending {
            #[pin]
            future: ClosingBoxedFuture<'a>,
        },
        Empty,
    }
}

impl<'a, RP> EventStreamState<'a, RP>
where
    RP: Send + 'a,
{
    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub(crate) fn is_future(&self) -> bool {
        matches!(self, Self::Future { .. })
    }

    pub(crate) fn project_future(self: Pin<&mut Self>) -> Option<Pin<&mut StreamBoxedFuture<'a, RP>>> {
        match self.project() {
            EventStreamStateProj::Future { future } => Some(future),
            _ => None,
        }
    }

    fn project_ending(self: Pin<&mut Self>) -> Option<Pin<&mut ClosingBoxedFuture<'a>>> {
        match self.project() {
            EventStreamStateProj::Ending { future } => Some(future),
            _ => None,
        }
    }

    pub(crate) fn take_value(self: Pin<&mut Self>) -> Option<EventStreamStateValue<'a, RP>> {
        match &*self {
            EventStreamState::Value { .. } => match self.project_replace(EventStreamState::Empty) {
                EventStreamStateProjReplace::Value { value } => Some(value),
                _ => unreachable!(),
            },
            _ => None,
        }
    }

    pub fn poll_dispose(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), DisposeConsumerError>> {
        if let Some(value) = self.as_mut().take_value() {
            self.set(EventStreamState::Ending { future: dispose_consumer(value).boxed() });
        }

        if let Some(future) = self.as_mut().project_future() {
            let (_, value) = ready!(future.poll(cx));
            self.set(EventStreamState::Ending { future: dispose_consumer(value).boxed() });
        }

        let result = match self.as_mut().project_ending() {
            Some(fut) => ready!(fut.poll(cx)),
            None => panic!("EventStream must not be polled after it returned `Poll::Ready(None)`"),
        };

        self.set(EventStreamState::Empty);
        Poll::Ready(result)
    }

    async fn dispose(mut self) -> Result<(), DisposeConsumerError> {
        poll_fn(|cx| Pin::new(&mut self).poll_dispose(cx)).await
    }
}

pin_project_lite::pin_project! {
    pub struct EventStream<'a, RP> {
        #[pin]
        state: EventStreamState<'a, RP>,
    }
}

impl<'a, RP> EventStream<'a, RP>
where
    RP: Send + 'a,
{
    pub(crate) fn new(
        client: &'a mut Sharable<AmqpClient>,
        consumer: AmqpConsumer<RP>,
        max_messages: u32,
        max_wait_time: Option<StdDuration>,
    ) -> Self {
        let value = EventStreamStateValue::new(client, consumer, max_messages, max_wait_time);
        let state = EventStreamState::Value { value };

        Self { state }
    }

    pub async fn dispose(self) -> Result<(), DisposeConsumerError> {
        self.state.dispose().await
    }
}

impl<'a, RP> Stream for EventStream<'a, RP>
where
    RP: EventHubsRetryPolicy + Send + 'a,
{
    type Item = Result<ReceivedEvent, RecoverAndReceiveError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        if let Some(state) = this.state.as_mut().take_value() {
            this.state.set(EventStreamState::Future { future: next_event(state).boxed() });
        }

        let (item, next_state) = match this.state.as_mut().project_future() {
            Some(fut) => ready!(fut.poll(cx)),
            None => panic!("EventStream must not be polled after it returned `Poll::Ready(None)`"),
        };

        if let Some(item) = item {
            this.state.set(EventStreamState::Value { value: next_state });
            Poll::Ready(Some(item))
        } else {
            this.state.set(EventStreamState::Ending { future: dispose_consumer(next_state).boxed() });
            Poll::Ready(None)
        }
    }
}
