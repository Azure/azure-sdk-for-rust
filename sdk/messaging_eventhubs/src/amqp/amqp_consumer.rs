use std::{collections::VecDeque, time::Duration as StdDuration, task::{Poll, Context}, pin::Pin};

use fe2o3_amqp::{link::RecvError, session::SessionHandle, Receiver};
use futures_util::{FutureExt, SinkExt, Stream, Future, ready};
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

    async fn receive_messages(
        &mut self,
        buffer: &mut VecDeque<ReceivedEvent>,
        max_messages: u32,
    ) -> Result<(), RecvError> {
        log::debug!("receive_messages: max_messages={}", max_messages);
        // Credit mode is manual, need to set credit
        if self.prefetch_count == 0 {
            // At least one credit is needed
            self.receiver.set_credit(max_messages).await?;
        }

        for _ in 0..max_messages {
            log::debug!("receive_messages: waiting for delivery");
            let delivery = self.receiver.recv().await?;
            self.receiver.accept(&delivery).await?;
            let event = ReceivedEvent::from_raw_amqp_message(delivery.into_message());

            let event_offset = event.offset().unwrap_or(i64::MIN);
            if event_offset > i64::MIN {
                self.current_event_position = Some(EventPosition::from_offset(event_offset, false));
            }

            buffer.push_back(event);
        }

        log::debug!("receive_messages: received {} messages", max_messages);
        Ok(())
    }

    async fn receive_messages_with_timeout(
        &mut self,
        buffer: &mut VecDeque<ReceivedEvent>,
        max_messages: u32,
        max_wait_time: StdDuration,
    ) -> Result<(), RecoverAndReceiveError> {
        log::debug!(
            "receive_messages_with_timeout: max_wait_time={:?}",
            max_wait_time
        );
        futures_util::select_biased! {
            _ = crate::util::time::sleep(max_wait_time).fuse() => Ok(()),
            result = self.receive_messages(buffer, max_messages).fuse() => {
                log::debug!("receive_messages_with_timeout: received messages");
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
    max_messages: u32,
    max_wait_time: StdDuration,
) -> Result<(), RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send,
{
    // Buffer is full, no need to receive more messages
    if buffer.len() == max_messages as usize {
        return Ok(());
    }

    log::debug!("should_try_recover: {:?}", should_try_recover);
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
        .receive_messages_with_timeout(buffer, max_messages, max_wait_time)
        .await?;
    if consumer.track_last_enqueued_event_properties {
        consumer.last_received_event = buffer.back().cloned();
    }
    log::debug!("buffer.len(): {:?}", buffer.len());
    Ok(())
}

async fn receive_event<RP>(
    client: &mut Sharable<AmqpClient>,
    consumer: &mut AmqpConsumer<RP>,
    buffer: &mut VecDeque<ReceivedEvent>,
    max_messages: u32,
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
            max_messages,
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

// pin_project_lite::pin_project! {
//     pub struct EventStreamState<'a, RP> {
//         #[pin]
//         consumer: Option<AmqpConsumer<RP>>,

//         #[pin]
//         client: &'a mut Sharable<AmqpClient>,

//         buffer: VecDeque<ReceivedEvent>,

//         maximum_event_count: u32,

//         maximum_wait_time: Option<StdDuration>,
//     }
// }

// impl<'a, RP> EventStreamState<'a, RP>
// where
//     RP: EventHubsRetryPolicy + Send + Unpin + 'a,
// {
//     pub(crate) fn new(
//         consumer: AmqpConsumer<RP>,
//         client: &'a mut Sharable<AmqpClient>,
//         maximum_event_count: u32,
//         maximum_wait_time: Option<StdDuration>,
//     ) -> Self {
//         log::debug!(
//             "EventStream::new: maximum_wait_time: {:?}",
//             maximum_wait_time
//         );
//         Self {
//             consumer: Some(consumer),
//             client,
//             buffer: VecDeque::with_capacity(maximum_event_count as usize),
//             maximum_event_count,
//             maximum_wait_time,
//         }
//     }

//     pub async fn dispose(self) -> Result<(), azure_core::Error> {
//         if let Some(consumer) = self.consumer {
//             consumer
//                 .dispose()
//                 .await
//                 .map_err(IntoAzureCoreError::into_azure_core_error)
//         } else {
//             Ok(())
//         }
//     }

//     pub async fn next_event(&mut self) -> Result<Option<ReceivedEvent>, RecoverAndReceiveError> {
//         let consumer = match self.consumer.as_mut() {
//             Some(consumer) => consumer,
//             None => return Ok(None),
//         };
//         next_event(
//             &mut self.client,
//             consumer,
//             &mut self.buffer,
//             self.maximum_event_count,
//             self.maximum_wait_time,
//         )
//         .await
//     }

//     pub fn into_stream(self) -> impl Stream<Item = Result<ReceivedEvent, RecoverAndReceiveError>> + Unpin + 'a {
//         Box::pin(
//             futures_util::stream::unfold(self, |mut stream| async move {
//                 match stream.next_event().await {
//                     Ok(Some(event)) => Some((Ok(event), stream)),
//                     Ok(None) => None,
//                     Err(err) => Some((Err(err), stream)),
//                 }
//             })
//         )
//     }
// }

async fn next_event_inner<RP>(
    client: &mut Sharable<AmqpClient>,
    consumer: &mut AmqpConsumer<RP>,
    buffer: &mut VecDeque<ReceivedEvent>,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Option<ReceivedEvent>, RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send,
{
    if let Some(event) = buffer.pop_front() {
        return Ok(Some(event));
    }

    loop {
        let result = receive_event(client, consumer, buffer, max_messages, max_wait_time).await;

        match buffer.pop_front() {
            Some(event) => return Ok(Some(event)),
            None => result?,
        }
    }
}

// pub(crate) fn event_stream<'a, RP, F, Fut>(
//     client: &'a mut Sharable<AmqpClient>,
//     consumer: &'a mut AmqpConsumer<RP>,
//     max_messages: u32,
//     max_wait_time: Option<StdDuration>,
//     f: F,
// ) -> EventStream<'a, RP, F, Fut> {
//     let value = EventStreamStateValue::new(client, consumer, max_messages, max_wait_time);
//     let state = EventStreamState::Value { value };
//     EventStream { state, f }
// }

pub(crate) async fn next_event<'a, RP>(mut value: EventStreamStateValue<'a, RP>) -> Option<(Result<ReceivedEvent, RecoverAndReceiveError>, EventStreamStateValue<'a, RP>)>
where
    RP: EventHubsRetryPolicy + Send,
{
    match next_event_inner(value.client, &mut value.consumer, &mut value.buffer, value.max_messages, value.max_wait_time).await {
        Ok(Some(event)) => Some((Ok(event), value)),
        Ok(None) => None,
        Err(err) => Some((Err(err), value)),
    }
}

pub struct EventStreamStateValue<'a, RP> {
    client: &'a mut Sharable<AmqpClient>,
    consumer: AmqpConsumer<RP>,
    buffer: VecDeque<ReceivedEvent>,
    max_messages: u32,
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
            max_messages,
            max_wait_time,
        }
    }
}

pin_project_lite::pin_project! {
    #[project = EventStreamStateProj]
    #[project_replace = EventStreamStateProjReplace]
    enum EventStreamState<'a, RP, Fut> {
        Value {
            value: EventStreamStateValue<'a, RP>,
        },
        Future {
            #[pin]
            future: Fut
        },
        Empty,
    }
}

impl<'a, RP, Fut> EventStreamState<'a, RP, Fut> {
    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub(crate) fn is_future(&self) -> bool {
        matches!(self, Self::Future { .. })
    }

    pub(crate) fn project_future(self: Pin<&mut Self>) -> Option<Pin<&mut Fut>> {
        match self.project() {
            EventStreamStateProj::Future { future } => Some(future),
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
}

pin_project_lite::pin_project! {
    pub struct EventStream<'a, RP, F, Fut> {
        f: F,

        #[pin]
        state: EventStreamState<'a, RP, Fut>,
    }
}

impl<'a, RP, F, Fut> EventStream<'a, RP, F, Fut> {
    pub(crate) fn new(
        client: &'a mut Sharable<AmqpClient>,
        consumer: AmqpConsumer<RP>,
        max_messages: u32,
        max_wait_time: Option<StdDuration>,
        f: F,
    ) -> Self {
        let value = EventStreamStateValue::new(client, consumer, max_messages, max_wait_time);
        let state = EventStreamState::Value { value };

        Self { state, f }
    }
}

impl<'a, RP, F, Fut> Stream for EventStream<'a, RP, F, Fut>
where
    F: FnMut(EventStreamStateValue<'a, RP>) -> Fut,
    Fut: Future<Output = Option<(Result<ReceivedEvent, RecoverAndReceiveError>, EventStreamStateValue<'a, RP>)>>,
{
    type Item = Result<ReceivedEvent, RecoverAndReceiveError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        if let Some(state) = this.state.as_mut().take_value() {
            this.state.set(EventStreamState::Future { future: (this.f)(state) });
        }

        let step = match this.state.as_mut().project_future() {
            Some(fut) => ready!(fut.poll(cx)),
            None => panic!("Unfold must not be polled after it returned `Poll::Ready(None)`"),
        };

        if let Some((item, next_state)) = step {
            this.state.set(EventStreamState::Value { value: next_state });
            Poll::Ready(Some(item))
        } else {
            this.state.set(EventStreamState::Empty);
            Poll::Ready(None)
        }
    }
}
