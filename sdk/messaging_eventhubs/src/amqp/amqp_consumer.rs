use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration as StdDuration, collections::VecDeque,
};

use fe2o3_amqp::{
    link::{delivery::DeliveryInfo, RecvError},
    session::SessionHandle,
    Receiver,
};
use futures_util::{Future, FutureExt, SinkExt, Stream};
use tokio::sync::mpsc;
use url::Url;

use crate::{
    consumer::EventPosition,
    core::{RecoverableError, RecoverableTransport, TransportClient, TransportConsumer},
    event_hubs_retry_policy::EventHubsRetryPolicy,
    util::{self, sharable::Sharable, time::Sleep, IntoAzureCoreError},
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
        log::debug!("receive_messages_with_timeout: max_wait_time={:?}", max_wait_time);
        futures_util::select_biased! {
            _ = crate::util::time::sleep(max_wait_time).fuse() => Ok(()),
            result = self.receive_messages(buffer, max_messages).fuse() => {
                log::debug!("receive_messages_with_timeout: received messages");
                result?;
                Ok(())
            }
        }
    }

    // async fn receive_and_accept(&mut self) -> Result<ReceivedEvent, RecvError> {
    //     let delivery = self.receiver.recv().await?;
    //     let delivery_info = DeliveryInfo::from(&delivery);
    //     let event = ReceivedEvent::from_raw_amqp_message(delivery.into_message());

    //     if self.track_last_enqueued_event_properties {
    //         self.last_received_event = Some(event.clone());
    //     }

    //     let event_offset = event.offset().unwrap_or(i64::MIN);
    //     if event_offset > i64::MIN {
    //         self.current_event_position = Some(EventPosition::from_offset(event_offset, false));
    //     }

    //     self.receiver.accept(delivery_info).await?;
    //     Ok(event)
    // }
}

async fn recover_and_set_credit<RP>(
    client: &mut Sharable<AmqpClient>,
    consumer: &mut AmqpConsumer<RP>,
    should_try_recover: bool,
    credit: u32,
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

    consumer.receiver.set_credit(credit).await?;
    Ok(())
}

async fn set_credit<RP>(
    client: &mut Sharable<AmqpClient>,
    consumer: &mut AmqpConsumer<RP>,
    credit: u32,
) -> Result<(), RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send,
{
    let mut failed_attempts = 0;
    let mut try_timeout = consumer.retry_policy.calculate_try_timeout(failed_attempts);
    let mut should_try_recover = true;

    loop {
        let fut = recover_and_set_credit(client, consumer, should_try_recover, credit);
        let err = match util::time::timeout(try_timeout, fut).await {
            Ok(Ok(())) => return Ok(()),
            Ok(Err(err)) => err,
            Err(elapsed) => elapsed.into(),
        };

        if err.is_scope_disposed() {
            return Err(err);
        }

        failed_attempts += 1;
        try_timeout = consumer.retry_policy.calculate_try_timeout(failed_attempts);
        should_try_recover = true;
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
) -> Result<Option<ReceivedEvent>, RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send,
{
    if let Some(event) = buffer.pop_front() {
        return Ok(Some(event));
    }

    let mut failed_attempts = 0;
    let mut try_timeout = consumer.retry_policy.calculate_try_timeout(failed_attempts);
    let mut should_try_recover = false;

    loop {
        // let fut = recover_and_recv(client, consumer, should_try_recover, prefetch_count, buffer, max_messages, max_wait_time);
        // let err = match util::time::timeout(try_timeout, fut).await {
        //     Ok(Ok(event)) => return Ok(event),
        //     Ok(Err(err)) => err,
        //     Err(elapsed) => elapsed.into(),
        // };

        let err = match recover_and_recv(
            client,
            consumer,
            should_try_recover,
            buffer,
            max_messages,
            try_timeout,
        )
        .await
        {
            Ok(_) => return Ok(buffer.pop_front()),
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

pub struct RecoverableAmqpConsumer<'a, RP> {
    /// Wrapping the consumer in an Option doesn't change the memory size.
    consumer: Option<AmqpConsumer<RP>>,
    client: &'a mut Sharable<AmqpClient>,
}

impl<'a, RP> RecoverableAmqpConsumer<'a, RP> {
    pub(crate) fn new(consumer: AmqpConsumer<RP>, client: &'a mut Sharable<AmqpClient>) -> Self {
        Self {
            consumer: Some(consumer),
            client,
        }
    }
}

pin_project_lite::pin_project! {
    pub struct EventStream<'a, RP> {
        #[pin]
        consumer: Option<AmqpConsumer<RP>>,

        #[pin]
        client: &'a mut Sharable<AmqpClient>,

        buffer: VecDeque<ReceivedEvent>,

        maximum_event_count: u32,

        maximum_wait_time: Option<Sleep>,
    }
}

impl<'a, RP> EventStream<'a, RP> {
    pub(crate) fn new(
        consumer: AmqpConsumer<RP>,
        client: &'a mut Sharable<AmqpClient>,
        maximum_event_count: u32,
        maximum_wait_time: Option<StdDuration>,
    ) -> Self {
        Self {
            consumer: Some(consumer),
            client,
            maximum_event_count,
            maximum_wait_time: maximum_wait_time.map(|d| Sleep::new(d)),
            buffer: VecDeque::with_capacity(maximum_event_count as usize),
        }
    }

    pub async fn dispose(self) -> Result<(), azure_core::Error> {
        if let Some(consumer) = self.consumer {
            consumer.dispose().await.map_err(IntoAzureCoreError::into_azure_core_error)
        } else {
            Ok(())
        }
    }
}

impl<'a, RP> Stream for EventStream<'a, RP>
where
    RP: EventHubsRetryPolicy + Send + Unpin,
{
    type Item = Result<ReceivedEvent, RecoverAndReceiveError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        if let Some(event) = this.buffer.pop_front() {
            return Poll::Ready(Some(Ok(event)));
        }

        if *this.maximum_event_count == 0 {
            if let Some(consumer) = this.consumer.take() {
                let fut = consumer.dispose();
                futures_util::pin_mut!(fut);
                match Future::poll(fut, cx) {
                    Poll::Ready(Ok(_)) => {}
                    Poll::Ready(Err(err)) => {
                        return Poll::Ready(Some(Err(RecoverAndReceiveError::from(err))))
                    }
                    Poll::Pending => return Poll::Pending,
                }
            }
            return Poll::Ready(None);
        } else {
            let mut client = this.client;
            let mut consumer = match this.consumer.as_mut().as_pin_mut() {
                Some(consumer) => consumer,
                None => return Poll::Ready(None),
            };

            let buffer = &mut this.buffer;
            let fut = receive_event(
                *client,
                &mut *consumer,
                buffer,
                *this.maximum_event_count,
            );
            futures_util::pin_mut!(fut);
            let poll = fut.poll(cx);
            log::debug!("poll: {:?}", poll);
            match poll {
                Poll::Ready(Ok(Some(event))) => {
                    return Poll::Ready(Some(Ok(event)));
                }
                Poll::Ready(Err(err)) => return Poll::Ready(Some(Err(RecoverAndReceiveError::from(err)))),
                _ => (),
            }
        }

        log::debug!("buffer capacity: {:?}", this.buffer.capacity());
        log::debug!("polling maximum_wait_time: {:?}", this.maximum_wait_time);

        match this
            .maximum_wait_time
            .as_mut()
            .map(|delay| Future::poll(Pin::new(delay), cx))
        {
            Some(Poll::Ready(_)) => {
                if let Some(consumer) = this.consumer.take() {
                    let fut = consumer.dispose();
                    futures_util::pin_mut!(fut);
                    match Future::poll(fut, cx) {
                        Poll::Ready(Ok(_)) => {}
                        Poll::Ready(Err(err)) => {
                            return Poll::Ready(Some(Err(RecoverAndReceiveError::from(err))))
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }
                Poll::Ready(None)
            }
            _ => Poll::Pending,
        }
    }
}
