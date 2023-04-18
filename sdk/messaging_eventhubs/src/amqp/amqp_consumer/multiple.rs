//! A wrapper around a vector of consumers.

use std::{
    collections::VecDeque,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll},
    time::Duration as StdDuration,
};

use fe2o3_amqp::link::RecvError;
use futures_util::{future::poll_fn, ready, Future, FutureExt, Stream};

use crate::{
    amqp::{
        amqp_client::AmqpClient,
        error::{DisposeConsumerError, RecoverAndReceiveError},
    },
    core::{RecoverableError, RecoverableTransport, TransportClient},
    event_hubs_retry_policy::EventHubsRetryPolicy,
    util::{self, sharable::Sharable},
    ReceivedEvent,
};

use super::{AmqpConsumer, EventStream, EventStreamState, EventStreamStateValue};

type ConsumerBoxedFuture<RP> =
    Pin<Box<dyn Future<Output = (Result<ReceivedEvent, RecvError>, AmqpConsumer<RP>)> + Send>>;
type ConsumerClosingBoxedFuture =
    Pin<Box<dyn Future<Output = Result<(), DisposeConsumerError>> + Send>>;

pin_project_lite::pin_project! {
    #[project = ConsumerStateProj]
    #[project_replace = ConsumerStateProjReplace]
    enum ConsumerState<RP> {
        Value {
            value: AmqpConsumer<RP>,
        },
        Future {
            #[pin]
            future: ConsumerBoxedFuture<RP>,
        },
        Closing {
            #[pin]
            future: ConsumerClosingBoxedFuture,
        },
        Empty,
    }
}

async fn recv_and_accept<RP>(
    mut consumer: AmqpConsumer<RP>,
) -> (Result<ReceivedEvent, RecvError>, AmqpConsumer<RP>) {
    if consumer.prefetch_count == 0 {
        // At least one credit is needed
        // TODO: set prefetch to other values
        if let Err(err) = consumer.receiver.set_credit(1).await {
            return (Err(err.into()), consumer);
        }
    }

    let event = consumer.recv_and_accept().await;
    (event, consumer)
}

impl<RP> ConsumerState<RP>
where
    RP: Send + 'static,
{
    fn take_value(self: Pin<&mut Self>) -> Option<AmqpConsumer<RP>> {
        match &*self {
            ConsumerState::Value { .. } => match self.project_replace(ConsumerState::Empty) {
                ConsumerStateProjReplace::Value { value } => Some(value),
                _ => unreachable!(),
            },
            _ => None,
        }
    }

    fn project_future(self: Pin<&mut Self>) -> Option<Pin<&mut ConsumerBoxedFuture<RP>>> {
        match self.project() {
            ConsumerStateProj::Future { future } => Some(future),
            _ => None,
        }
    }

    fn project_ending(self: Pin<&mut Self>) -> Option<Pin<&mut ConsumerClosingBoxedFuture>> {
        match self.project() {
            ConsumerStateProj::Closing { future } => Some(future),
            _ => None,
        }
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Result<(), DisposeConsumerError>> {
        if let Some(value) = self.as_mut().take_value() {
            self.set(ConsumerState::Closing {
                future: value.close().boxed(),
            });
        }

        if let Some(future) = self.as_mut().project_future() {
            let (_, value) = ready!(future.poll(cx));
            self.set(ConsumerState::Closing {
                future: value.close().boxed(),
            });
        }

        let result = match self.as_mut().project_ending() {
            Some(fut) => ready!(fut.poll(cx)),
            None => panic!("EventStream must not be polled after it returned `Poll::Ready(None)`"),
        };

        self.set(ConsumerState::Empty);
        Poll::Ready(result)
    }

    async fn close(mut self) -> Result<(), DisposeConsumerError> {
        poll_fn(|cx| Pin::new(&mut self).poll_close(cx)).await
    }

    fn poll_recv_and_accept(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<ReceivedEvent, RecvError>>> {
        if let Some(consumer) = self.as_mut().take_value() {
            self.set(ConsumerState::Future {
                future: recv_and_accept(consumer).boxed(),
            });
        }

        let (item, next_state) = match self.as_mut().project_future() {
            Some(fut) => ready!(fut.poll(cx)),
            None => return Poll::Ready(None),
        };

        self.set(ConsumerState::Value { value: next_state });
        Poll::Ready(Some(item))
    }
}

pub struct MultipleAmqpConsumers<RP> {
    inner: Vec<ConsumerState<RP>>,
    retry_policy: RP,
}

pin_project_lite::pin_project! {
    pub(crate) struct MultiAmqpConsumerRecv<'a, RP> {
        #[pin]
        state: &'a mut MultipleAmqpConsumers<RP>,
    }
}

impl<'a, RP> Future for MultiAmqpConsumerRecv<'a, RP>
where
    RP: Send + Unpin + 'static,
{
    type Output = Option<Result<ReceivedEvent, RecvError>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let pinned = Pin::new(this.state.get_mut().deref_mut());
        pinned.poll_recv(cx)
    }
}

impl<RP> MultipleAmqpConsumers<RP>
where
    RP: Send + Unpin + 'static,
{
    fn poll_recv(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<ReceivedEvent, RecvError>>> {
        if self.inner.is_empty() {
            // Only return None if all consumers are dead
            return Poll::Ready(None);
        }

        let item = self.inner.iter_mut().enumerate().find_map(|(i, consumer)| {
            let pinned = Pin::new(consumer);
            match pinned.poll_recv_and_accept(cx) {
                Poll::Ready(item) => Some((i, item)),
                Poll::Pending => None,
            }
        });

        match item {
            Some((_, Some(item))) => Poll::Ready(Some(item)),
            Some((i, None)) => {
                // Consumer is dead, remove it
                self.inner.swap_remove(i);
                Poll::Pending
            }
            None => Poll::Pending,
        }
    }

    fn recv(&mut self) -> MultiAmqpConsumerRecv<'_, RP> {
        MultiAmqpConsumerRecv { state: self }
    }

    async fn fill_buf(&mut self, buffer: &mut VecDeque<ReceivedEvent>) -> Result<(), RecvError> {
        // Only receive messages if there is space in the buffer
        let max_messages = buffer.capacity() - buffer.len();

        let mut counter = 0;
        while let Some(event) = self.recv().await {
            counter += 1;
            if counter > max_messages {
                break;
            }
            let event = event?;
            let _event_offset = event.offset().unwrap_or(i64::MIN);

            buffer.push_back(event);
        }

        Ok(())
    }

    async fn fill_buf_with_timeout(
        &mut self,
        buffer: &mut VecDeque<ReceivedEvent>,
        max_wait_time: StdDuration,
    ) -> Result<(), RecvError> {
        futures_util::select_biased! {
            _ = crate::util::time::sleep(max_wait_time).fuse() => Ok(()),
            result = self.fill_buf(buffer).fuse() => {
                result?;
                Ok(())
            }
        }
    }
}

async fn recover_consumers<RP>(
    client: &mut AmqpClient,
    consumers: &mut MultipleAmqpConsumers<RP>,
) -> Result<(), RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send,
{
    // Client should be already recovered
    let mut result = Ok(());

    for c in consumers.inner.iter_mut() {
        // Consumers that are polling would probably encounter an error
        // when they poll again, and it will enter another recovery
        if let ConsumerState::Value { value } = c {
            result = result.and(client.recover_consumer(value).await);
        }
    }

    result.map_err(Into::into)
}

async fn recover_and_recv<RP>(
    client: &mut Sharable<AmqpClient>,
    consumers: &mut MultipleAmqpConsumers<RP>,
    should_try_recover: bool,
    buffer: &mut VecDeque<ReceivedEvent>,
    max_wait_time: StdDuration,
) -> Result<(), RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send + Unpin + 'static,
{
    if should_try_recover {
        if let Err(recovery_err) = client.recover().await {
            log::error!("Failed to recover client: {:?}", recovery_err);
            if recovery_err.is_scope_disposed() {
                return Err(recovery_err.into());
            }
        }

        match client {
            Sharable::Owned(c) => recover_consumers(c, consumers).await?,
            Sharable::Shared(c) => {
                let mut guard = c.lock().await;
                recover_consumers(&mut *guard, consumers).await?
            }
            Sharable::None => return Err(RecoverAndReceiveError::ConnectionScopeDisposed),
        };
    }

    consumers
        .fill_buf_with_timeout(buffer, max_wait_time)
        .await?;
    Ok(())
}

async fn receive_event<RP>(
    client: &mut Sharable<AmqpClient>,
    consumer: &mut MultipleAmqpConsumers<RP>,
    buffer: &mut VecDeque<ReceivedEvent>,
    max_wait_time: Option<StdDuration>,
) -> Result<(), RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send + Unpin + 'static,
{
    let mut failed_attempts = 0;
    let mut try_timeout = consumer.retry_policy.calculate_try_timeout(failed_attempts);
    let mut should_try_recover = false;

    loop {
        let wait_time = max_wait_time.unwrap_or(try_timeout);
        let err =
            match recover_and_recv(client, consumer, should_try_recover, buffer, wait_time).await {
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
    consumer: &mut MultipleAmqpConsumers<RP>,
    buffer: &mut VecDeque<ReceivedEvent>,
    max_wait_time: Option<StdDuration>,
) -> Result<Option<ReceivedEvent>, RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send + Unpin + 'static,
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

async fn next_event<'a, RP>(
    mut value: EventStreamStateValue<'a, MultipleAmqpConsumers<RP>>,
) -> (
    Option<Result<ReceivedEvent, RecoverAndReceiveError>>,
    EventStreamStateValue<'a, MultipleAmqpConsumers<RP>>,
)
where
    RP: EventHubsRetryPolicy + Send + Unpin + 'static,
{
    match next_event_inner(
        value.client,
        &mut value.consumer,
        &mut value.buffer,
        value.max_wait_time,
    )
    .await
    {
        Ok(Some(event)) => (Some(Ok(event)), value),
        Ok(None) => (None, value),
        Err(err) => (Some(Err(err)), value),
    }
}

async fn close_consumers<'a, RP>(
    value: EventStreamStateValue<'a, MultipleAmqpConsumers<RP>>,
) -> Result<(), DisposeConsumerError>
where
    RP: Send + 'static,
{
    let mut result = Ok(());
    for consumer in value.consumer.inner {
        result = result.and(consumer.close().await);
    }
    result
}

impl<'a, RP> EventStreamState<'a, MultipleAmqpConsumers<RP>>
where
    RP: Send + 'static,
{
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Result<(), DisposeConsumerError>> {
        if let Some(value) = self.as_mut().take_value() {
            self.set(EventStreamState::Closing {
                future: close_consumers(value).boxed(),
            });
        }

        if let Some(future) = self.as_mut().project_future() {
            let (_, value) = ready!(future.poll(cx));
            self.set(EventStreamState::Closing {
                future: close_consumers(value).boxed(),
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

impl<'a, RP> EventStream<'a, MultipleAmqpConsumers<RP>>
where
    RP: Send + 'static,
{
    pub(crate) fn with_multiple_consumers(
        client: &'a mut Sharable<AmqpClient>,
        consumers: Vec<AmqpConsumer<RP>>,
        max_messages: u32,
        max_wait_time: Option<StdDuration>,
        retry_policy: RP,
    ) -> Self {
        let consumers = consumers
            .into_iter()
            .map(|value| ConsumerState::Value { value })
            .collect();
        let consumers = MultipleAmqpConsumers {
            inner: consumers,
            retry_policy,
        };
        let value = EventStreamStateValue::new(client, consumers, max_messages, max_wait_time);
        let state = EventStreamState::Value { value };

        Self { state }
    }

    pub async fn close(self) -> Result<(), DisposeConsumerError> {
        self.state.close().await
    }
}

impl<'a, RP> Stream for EventStream<'a, MultipleAmqpConsumers<RP>>
where
    RP: EventHubsRetryPolicy + Send + Unpin + 'static,
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
                future: close_consumers(next_state).boxed(),
            });
            Poll::Ready(None)
        }
    }
}
