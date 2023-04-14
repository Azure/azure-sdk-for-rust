use std::{time::Duration as StdDuration, task::{Poll, Context}, pin::Pin};

use async_trait::async_trait;
use fe2o3_amqp::{session::SessionHandle, Receiver, link::{RecvError, delivery::DeliveryInfo}};
use futures_util::{Sink, SinkExt, Stream, FutureExt};

use crate::{core::{TransportConsumer, RecoverableTransport, RecoverableError, TransportClient}, event_hubs_retry_policy::EventHubsRetryPolicy, util::{sharable::Sharable, self}, ReceivedEvent, consumer::EventPosition};

use super::{amqp_client::AmqpClient, error::{RecoverAndReceiveError, DisposeConsumerError}};

pub struct AmqpConsumer<RP> {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) receiver: Receiver,
    pub(crate) link_identifier: u32,
    pub(crate) invalidate_consumer_when_partition_stolen: bool,
    pub(crate) track_last_enqueued_event_properties: bool,
    pub(crate) last_received_event: Option<ReceivedEvent>,
    pub(crate) current_event_position: Option<EventPosition>,
    pub(crate) retry_policy: RP,
    pub(crate) prefetch_count: u32,
}

impl<RP> AmqpConsumer<RP> {
    pub async fn dispose(mut self) -> Result<(), DisposeConsumerError> {
        self.receiver.close().await?;
        self.session_handle.close().await?;
        Ok(())
    }

    async fn receive_and_accept(&mut self) -> Result<ReceivedEvent, RecvError> {
        let delivery = self.receiver.recv().await?;
        let delivery_info = DeliveryInfo::from(&delivery);
        let event = ReceivedEvent::from_raw_amqp_message(delivery.into_message());

        if self.track_last_enqueued_event_properties {
            self.last_received_event = Some(event.clone());
        }

        if event.offset() > i64::MIN {
            self.current_event_position = Some(EventPosition::from_offset(event.offset(), false));
        }

        self.receiver.accept(delivery_info).await?;
        Ok(event)
    }
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
            Sharable::Shared(client) => {
                client
                    .lock()
                    .await
                    .recover_consumer(consumer)
                    .await?
            }
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
) -> Result<ReceivedEvent, RecoverAndReceiveError>
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
            Sharable::Shared(client) => {
                client
                    .lock()
                    .await
                    .recover_consumer(consumer)
                    .await?
            }
            Sharable::None => return Err(RecoverAndReceiveError::ConnectionScopeDisposed),
        }
    }

    let event = consumer.receive_and_accept().await?;
    Ok(event)
}

async fn receive_event<RP>(
    client: &mut Sharable<AmqpClient>,
    consumer: &mut AmqpConsumer<RP>,
) -> Result<ReceivedEvent, RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send,
{
    let mut failed_attempts = 0;
    let mut try_timeout = consumer.retry_policy.calculate_try_timeout(failed_attempts);
    let mut should_try_recover = true;

    loop {
        let fut = recover_and_recv(client, consumer, should_try_recover);
        let err = match util::time::timeout(try_timeout, fut).await {
            Ok(Ok(event)) => return Ok(event),
            Ok(Err(err)) => err,
            Err(elapsed) => elapsed.into(),
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
                try_timeout = consumer
                    .retry_policy
                    .calculate_try_timeout(failed_attempts);
            }
            None => return Err(err),
        }
    }
}

pub struct RecoverableAmqpConsumer<'a, RP> {
    consumer: &'a mut AmqpConsumer<RP>,
    client: &'a mut Sharable<AmqpClient>,
}

#[async_trait]
impl<'a, RP> TransportConsumer for RecoverableAmqpConsumer<'a, RP>
where
    RP: EventHubsRetryPolicy + Send,
{
    type ReceivedEvent = ReceivedEvent;
    type ReceiveError = RecoverAndReceiveError;
    type Stream<'s> = EventStream<'s, RP> where RP: 's, Self: 's;

    fn last_received_event(&self) -> Option<&Self::ReceivedEvent> {
        self.consumer.last_received_event.as_ref()
    }

    fn receive(
        &mut self,
        maximum_event_count: Option<u32>,
        maximum_wait_time: Option<StdDuration>,
    ) -> Self::Stream<'_> {
        EventStream {
            consumer: self.consumer,
            client: self.client,
            maximum_event_count,
            maximum_wait_time,
        }
    }
}

pin_project_lite::pin_project! {
    pub struct EventStream<'a, RP> {
        #[pin]
        consumer: &'a mut AmqpConsumer<RP>,

        #[pin]
        client: &'a mut Sharable<AmqpClient>,

        maximum_event_count: Option<u32>,

        maximum_wait_time: Option<StdDuration>,
    }
}

impl<'a, RP> Stream for EventStream<'a, RP>
where
    RP: EventHubsRetryPolicy + Send,
{
    type Item = Result<ReceivedEvent, RecoverAndReceiveError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.maximum_event_count == Some(0) {
            return Poll::Ready(None);
        }

        let this = self.project();
        let client = this.client.get_mut();
        let consumer = this.consumer.get_mut();

        if consumer.prefetch_count == 0 {
            let credit = this.maximum_event_count.unwrap_or(1);
            let fut = set_credit(client, consumer, credit);
            futures_util::pin_mut!(fut);
            let poll = fut.poll_unpin(cx);
            if let Err(err) = futures_util::ready!(poll) {
                *this.maximum_event_count = Some(0);
                return Poll::Ready(Some(Err(err)));
            }
        }

        let fut = receive_event(client, consumer);
        futures_util::pin_mut!(fut);
        let poll = fut.poll_unpin(cx);
        match futures_util::ready!(poll) {
            Ok(event) => {
                *this.maximum_event_count = this.maximum_event_count.map(|x| x - 1);
                Poll::Ready(Some(Ok(event)))
            },
            Err(err) => {
                *this.maximum_event_count = Some(0);
                Poll::Ready(Some(Err(err)))
            },
        }
    }
}
