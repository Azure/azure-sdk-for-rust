use std::{
    pin::Pin,
    task::{Context, Poll}, future::poll_fn,
};

use futures_util::{Future, Stream, FutureExt, ready};

use crate::{EventHubsRetryPolicy, ReceivedEventData};

use self::single::EventStreamStateValue;

use super::error::{DisposeConsumerError, RecoverAndReceiveError};

pub(crate) mod multiple;
pub(crate) mod single;

pub(crate) use multiple::MultipleAmqpConsumers;
pub(crate) use single::AmqpConsumer;

type StreamBoxedFuture<'a, C> = Pin<
    Box<
        dyn Future<
                Output = (
                    Option<Result<ReceivedEventData, RecoverAndReceiveError>>,
                    EventStreamStateValue<'a, C>,
                ),
            > + Send
            + 'a,
    >,
>;
type ClosingBoxedFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), DisposeConsumerError>> + Send + 'a>>;

enum Consumer<RP> {
    Single(AmqpConsumer<RP>),
    Multiple(MultipleAmqpConsumers<RP>),
}

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

impl<'a, RP> EventStreamState<'a, Consumer<RP>>
where
    RP: Send + 'static,
{
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
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
            None => {
                panic!("Stream must not be polled after completion")
            }
        };

        self.set(EventStreamState::Empty);
        Poll::Ready(result)
    }

    async fn close(mut self) -> Result<(), DisposeConsumerError> {
        poll_fn(|cx| Pin::new(&mut self).poll_close(cx)).await
    }
}

async fn next_event<RP>(
    value: EventStreamStateValue<'_, Consumer<RP>>,
) -> (
    Option<Result<ReceivedEventData, RecoverAndReceiveError>>,
    EventStreamStateValue<'_, Consumer<RP>>,
)
where
    RP: EventHubsRetryPolicy + Send + Unpin + 'static,
{
    let EventStreamStateValue {
        client, consumer, ..
    } = value;

    match consumer {
        Consumer::Single(mut consumer) => {
            let outcome = single::receive_event(client, &mut consumer).await;
            let value = EventStreamStateValue {
                client,
                consumer: Consumer::Single(consumer),
            };
            (Some(outcome), value)
        }
        Consumer::Multiple(mut consumer) => {
            let outcome = multiple::receive_event(client, &mut consumer).await;
            let value = EventStreamStateValue {
                client,
                consumer: Consumer::Multiple(consumer),
            };
            (outcome, value)
        },
    }
}

async fn close_consumer<RP>(
    value: EventStreamStateValue<'_, Consumer<RP>>
) -> Result<(), DisposeConsumerError>
where
    RP: Send + 'static,
{
    let EventStreamStateValue {
        client: _, consumer, ..
    } = value;

    match consumer {
        Consumer::Single(consumer) => {
            consumer.close().await
        }
        Consumer::Multiple(consumer) => {
            let futs = consumer.inner.into_iter().map(|consumer| consumer.close());
            futures_util::future::join_all(futs).await
                .into_iter()
                .fold(Ok(()), |acc, result| acc.and(result))
        },
    }
}

pin_project_lite::pin_project! {
    /// A stream of event.
    ///
    /// This is created by a `EventHubConsumerClient`. It takes the lifetime of the
    /// `EventHubConsumerClient` that created it, and thus the stream must be closed/dropped before
    /// the `EventHubConsumerClient` is dropped.
    ///
    /// # Generic Parameters:
    ///
    /// * `'a` - The lifetime of the `EventHubConsumerClient` that created this stream.
    /// * `RP` - The retry policy to use for recovering from errors.
    pub struct EventStream<'a, RP> {
        #[pin]
        state: EventStreamState<'a, Consumer<RP>>,
    }
}

impl<'a, RP> EventStream<'a, RP>
where
    RP: Send + 'static,
{
    /// Closes the [`EventStream`].
    pub async fn close(self) -> Result<(), DisposeConsumerError> {
        self.state.close().await
    }
}

impl<'a, RP> Stream for EventStream<'a, RP>
where
    RP: EventHubsRetryPolicy + Send + Unpin + 'static,
    Consumer<RP>: Send + 'a,
{
    type Item = Result<ReceivedEventData, azure_core::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        if let Some(state) = this.state.as_mut().take_value() {
            this.state.set(EventStreamState::Future {
                future: next_event(state).boxed(),
            });
        }

        let (item, next_state) = match this.state.as_mut().project_future() {
            Some(fut) => ready!(fut.poll(cx)),
            None => {
                let result = match this.state.as_mut().project_closing() {
                    Some(fut) => ready!(fut.poll(cx)),
                    None => panic!("Stream must not be polled after completion"),
                };

                match result {
                    Ok(_) => return Poll::Ready(None),
                    Err(err) => return Poll::Ready(Some(Err(err.into()))),
                }
            }
        };

        if let Some(item) = item {
            this.state
                .set(EventStreamState::Value { value: next_state });
            Poll::Ready(Some(item.map_err(Into::into)))
        } else {
            this.state.set(EventStreamState::Closing {
                future: close_consumer(next_state).boxed(),
            });
            Poll::Ready(None)
        }
    }
}
