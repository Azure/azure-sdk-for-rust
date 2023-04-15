use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration as StdDuration,
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
    util::{self, sharable::Sharable, time::Sleep},
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

    async fn receive_and_accept(&mut self) -> Result<ReceivedEvent, RecvError> {
        let delivery = self.receiver.recv().await?;
        let delivery_info = DeliveryInfo::from(&delivery);
        let event = ReceivedEvent::from_raw_amqp_message(delivery.into_message());

        if self.track_last_enqueued_event_properties {
            self.last_received_event = Some(event.clone());
        }

        let event_offset = event.offset().unwrap_or(i64::MIN);
        if event_offset > i64::MIN {
            self.current_event_position = Some(EventPosition::from_offset(event_offset, false));
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
) -> Result<ReceivedEvent, RecoverAndReceiveError>
where
    RP: EventHubsRetryPolicy + Send,
{
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
    let mut should_try_recover = false;

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

// #[async_trait]
// impl<'a, RP> TransportConsumer for RecoverableAmqpConsumer<'a, RP>
// where
//     RP: EventHubsRetryPolicy + Send + Unpin,
// {
//     type ReceivedEvent = ReceivedEvent;
//     type ReceiveError = RecoverAndReceiveError;
//     type Stream<'s> = EventStream<'s, RP> where RP: 's, Self: 's;

//     fn last_received_event(&self) -> Option<&Self::ReceivedEvent> {
//         self.consumer.as_ref().and_then(|consumer| consumer.last_received_event.as_ref())
//     }

//     fn receive(
//         &mut self,
//         maximum_event_count: Option<u32>,
//         maximum_wait_time: Option<StdDuration>,
//     ) -> Self::Stream<'_> {
//         EventStream {
//             consumer: self.consumer.take(),
//             client: self.client,
//             maximum_event_count,
//             maximum_wait_time,
//         }
//     }
// }

pin_project_lite::pin_project! {
    pub struct EventStream<'a, RP> {
        #[pin]
        consumer: Option<AmqpConsumer<RP>>,

        #[pin]
        client: &'a mut Sharable<AmqpClient>,

        maximum_event_count: Option<u32>,

        maximum_wait_time: Option<Sleep>,
    }
}

impl<'a, RP> EventStream<'a, RP> {
    pub(crate) fn new(
        consumer: AmqpConsumer<RP>,
        client: &'a mut Sharable<AmqpClient>,
        maximum_event_count: Option<u32>,
        maximum_wait_time: Option<StdDuration>,
    ) -> Self {
        Self {
            consumer: Some(consumer),
            client,
            maximum_event_count,
            maximum_wait_time: maximum_wait_time.map(|d| Sleep::new(d)),
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
        if *this.maximum_event_count == Some(0) {
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

            if consumer.prefetch_count == 0 {
                let credit = this.maximum_event_count.unwrap_or(1);
                let fut = set_credit(*client, &mut *consumer, credit);
                futures_util::pin_mut!(fut);
                let poll = fut.poll(cx);
                if let Poll::Ready(Err(err)) = poll {
                    *this.maximum_event_count = Some(0);
                    return Poll::Ready(Some(Err(err)));
                }
            }

            let fut = receive_event(*client, &mut *consumer);
            futures_util::pin_mut!(fut);

            if let Poll::Ready(result) = fut.poll(cx) {
                match result {
                    Ok(event) => {
                        *this.maximum_event_count = this.maximum_event_count.map(|x| x - 1);
                        return Poll::Ready(Some(Ok(event)));
                    }
                    Err(err) => {
                        *this.maximum_event_count = Some(0);
                        return Poll::Ready(Some(Err(err)));
                    }
                }
            }
        }

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
