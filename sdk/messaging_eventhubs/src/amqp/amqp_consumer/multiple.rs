//! A wrapper around a vector of consumers.

use std::{
    fmt::Debug,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll},
};

use fe2o3_amqp::link::RecvError;
use futures_util::{future::poll_fn, ready, Future, FutureExt};
use tokio_util::sync::CancellationToken;

use crate::{
    amqp::{
        amqp_client::AmqpClient,
        error::{DisposeConsumerError, RecoverAndReceiveError},
    },
    core::{RecoverableError, RecoverableTransport, TransportClient},
    event_hubs_retry_policy::EventHubsRetryPolicy,
    util::{self, time::timeout},
    ReceivedEventData,
};

use super::{AmqpConsumer, EventStream, EventStreamState, EventStreamStateValue};

type ConsumerBoxedFuture<RP> =
    Pin<Box<dyn Future<Output = (Result<ReceivedEventData, RecvError>, AmqpConsumer<RP>)> + Send>>;
type ConsumerClosingBoxedFuture =
    Pin<Box<dyn Future<Output = Result<(), DisposeConsumerError>> + Send>>;

pin_project_lite::pin_project! {
    #[project = ConsumerStateProj]
    #[project_replace = ConsumerStateProjReplace]
    pub(crate) enum ConsumerState<RP> {
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

impl<RP> Debug for ConsumerState<RP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value { .. } => f.debug_struct("Value").finish(),
            Self::Future { .. } => f.debug_struct("Future").finish(),
            Self::Closing { .. } => f.debug_struct("Closing").finish(),
            Self::Empty => write!(f, "Empty"),
        }
    }
}

async fn recv_and_accept<RP>(
    mut consumer: AmqpConsumer<RP>,
) -> (Result<ReceivedEventData, RecvError>, AmqpConsumer<RP>) {
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
        log::debug!("poll_close() self = {:?}", self);

        if let Some(value) = self.as_mut().take_value() {
            self.set(ConsumerState::Closing {
                future: value.close().boxed(),
            });
        }

        if let Some(_future) = self.as_mut().project_future() {
            // let (_, value) = ready!(future.poll(cx));
            // self.set(ConsumerState::Closing {
            //     future: value.close().boxed(),
            // });

            // TODO: what to do with the value?
            //
            // This is a temporary hack which simply drops the future.
            // Both Link and Session have internal states that implements Drop trait and
            // will exchange messages with the server to close the link/session.
            // drop(future);
            self.set(ConsumerState::Empty);
            return Poll::Ready(Ok(()));
        }

        let result = match self.as_mut().project_ending() {
            Some(fut) => ready!(fut.poll(cx)),
            None => panic!("EventStream must not be polled after it returned `Poll::Ready(None)`"),
        };

        self.set(ConsumerState::Empty);
        Poll::Ready(result)
    }

    pub(crate) async fn close(mut self) -> Result<(), DisposeConsumerError> {
        poll_fn(|cx| Pin::new(&mut self).poll_close(cx)).await
    }

    fn poll_recv_and_accept(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<ReceivedEventData, RecvError>>> {
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

#[derive(Debug)]
pub(crate) struct MultipleAmqpConsumers<RP> {
    pub(crate) inner: Vec<ConsumerState<RP>>,
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
    type Output = Option<Result<ReceivedEventData, RecvError>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        log::debug!("MultiAmqpConsumerRecv::poll()");
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
    ) -> Poll<Option<Result<ReceivedEventData, RecvError>>> {
        log::debug!("MultipleAmqpConsumers::poll_recv()");

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
        MultiAmqpConsumerRecv { state: self } // TODO: remove clone
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

// There is a possibility that no child consumer exists, so a None is returned.
async fn recover_and_recv<RP>(
    client: &mut AmqpClient,
    consumers: &mut MultipleAmqpConsumers<RP>,
    should_try_recover: bool,
) -> Result<Option<ReceivedEventData>, RecoverAndReceiveError>
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

        recover_consumers(client, consumers).await?;
    }

    consumers.recv().await.transpose().map_err(Into::into)
}

pub(crate) async fn receive_event<RP>(
    client: &mut AmqpClient,
    consumers: &mut MultipleAmqpConsumers<RP>,
) -> Option<Result<ReceivedEventData, RecoverAndReceiveError>>
where
    RP: EventHubsRetryPolicy + Send + Unpin + 'static,
{
    let mut failed_attempts = 0;
    let mut try_timeout = consumers.retry_policy.calculate_try_timeout(failed_attempts);
    let mut should_try_recover = false;

    loop {
        let err = match timeout(
            try_timeout,
            recover_and_recv(client, consumers, should_try_recover)
        ).await {
            Ok(result) => match result.transpose()? {
                Ok(event) => return Some(Ok(event)),
                Err(err) => err,
            },
            Err(_try_timeout_elapsed) => {
                // There is no error returned from client, everything is fine and keep waiting
                // TODO: is this correct?
                continue;
            },
        };

        if err.is_scope_disposed() {
            return Some(Err(err));
        }
        should_try_recover = err.should_try_recover();

        failed_attempts += 1;
        let retry_delay = consumers
            .retry_policy
            .calculate_retry_delay(&err, failed_attempts);

        match retry_delay {
            Some(retry_delay) => {
                util::time::sleep(retry_delay).await;
                try_timeout = consumers.retry_policy.calculate_try_timeout(failed_attempts);
            }
            None => return Some(Err(err)),
        }
    }
}

impl<'a, RP> EventStream<'a, RP>
where
    RP: Send + 'static,
{
    pub(crate) fn with_multiple_consumers(
        client: &'a mut AmqpClient,
        consumers: Vec<AmqpConsumer<RP>>,
        retry_policy: RP,
    ) -> Self {
        let cancel_source = CancellationToken::new();
        let _cancellation_token = cancel_source.child_token();
        let consumers = consumers
            .into_iter()
            .map(|value| ConsumerState::Value { value })
            .collect();
        let consumers = MultipleAmqpConsumers {
            inner: consumers,
            retry_policy,
        };
        let consumers = super::Consumer::Multiple(consumers);
        let value = EventStreamStateValue::new(client, consumers);
        let state = EventStreamState::Value { value };

        Self { state }
    }
}
