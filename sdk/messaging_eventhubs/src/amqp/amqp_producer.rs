use async_trait::async_trait;
use azure_core::Url;
use fe2o3_amqp::{link::SendError, session::SessionHandle, Sender};
use fe2o3_amqp_types::messaging::Outcome;

use crate::{
    amqp::amqp_message_converter::create_envelope_from_events,
    core::{TransportEventBatch, TransportProducer, TransportProducerFeatures, RecoverableError, RecoverableTransport, TransportClient},
    event_hubs_retry_policy::EventHubsRetryPolicy,
    producer::{
        create_batch_options::CreateBatchOptions,
        event_hub_producer_client::MINIMUM_BATCH_SIZE_LIMIT, send_event_options::SendEventOptions,
        PartitionPublishingOptions,
    },
    util::{self, IntoAzureCoreError, sharable::Sharable},
    Event,
};

use super::{
    amqp_connection_scope::AmqpConnectionScope,
    amqp_event_batch::AmqpEventBatch,
    amqp_message_converter::{
        build_amqp_batch_from_messages, BatchEnvelope, BatchEnvelopeState, SendableEnvelope,
    },
    error::{
        AmqpSendError, DisposeProducerError, NotAcceptedError, OpenProducerError,
        RequestedSizeOutOfRange, RecoverAndSendError,
    }, amqp_client::AmqpClient,
};

pub struct AmqpProducer<RP> {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) sender: Sender,
    pub(crate) link_identifier: u32,
    pub(crate) initialized_partition_properties: PartitionPublishingOptions,
    pub(crate) retry_policy: RP,
    pub(crate) partition_id: Option<String>,
    pub(crate) features: TransportProducerFeatures,
    pub(crate) options: PartitionPublishingOptions,
    pub(crate) endpoint: Url,
}

impl<RP> AmqpProducer<RP> {
    async fn send_batch_envelope(
        &mut self,
        batch: &mut BatchEnvelope,
    ) -> Result<(), AmqpSendError> {
        let outcome = loop {
            match &mut batch.state {
                BatchEnvelopeState::NotSent => match &mut batch.sendable {
                    SendableEnvelope::Single(sendable) => {
                        let fut = self.sender.send_batchable_ref(sendable).await?;
                        batch.state = BatchEnvelopeState::Sent(fut);
                    }
                    SendableEnvelope::Batch(sendable) => {
                        let fut = self.sender.send_batchable_ref(sendable).await?;
                        batch.state = BatchEnvelopeState::Sent(fut);
                    }
                },
                BatchEnvelopeState::Sent(fut) => break fut.await?,
                BatchEnvelopeState::Settled => return Ok(()),
            }
        };

        batch.state = BatchEnvelopeState::Settled;

        match outcome {
            Outcome::Accepted(_) => Ok(()),
            Outcome::Rejected(rejected) => {
                Err(AmqpSendError::from(NotAcceptedError::Rejected(rejected)))
            }
            Outcome::Released(released) => {
                Err(AmqpSendError::from(NotAcceptedError::Released(released)))
            }
            Outcome::Modified(modified) => {
                Err(AmqpSendError::from(NotAcceptedError::Modified(modified)))
            }
            #[cfg(feature = "transaction")]
            Outcome::Declared(_) => {
                unreachable!("Declared is not expected outside txn-control links")
            }
        }
    }

    pub(crate) fn create_batch(
        &self,
        options: CreateBatchOptions,
    ) -> Result<AmqpEventBatch, RequestedSizeOutOfRange> {
        let link_max_message_size = self.sender.max_message_size().unwrap_or(u64::MAX);
        let max_size_in_bytes = match options.max_size_in_bytes {
            Some(max_size_in_bytes) => {
                if max_size_in_bytes < MINIMUM_BATCH_SIZE_LIMIT as u64
                    || max_size_in_bytes > link_max_message_size
                {
                    return Err(RequestedSizeOutOfRange {});
                }

                max_size_in_bytes
            }
            // If this field is zero or unset, there is no maximum size imposed by the link endpoint.
            None => link_max_message_size,
        };
        Ok(AmqpEventBatch::new(max_size_in_bytes, options))
    }

    async fn send(
        &mut self,
        events: impl Iterator<Item = Event> + ExactSizeIterator + Send,
        options: SendEventOptions,
    ) -> Result<(), AmqpSendError> {
        // TODO: check size of envelope and make sure it's not too big
        match create_envelope_from_events(events, options.partition_key) {
            Some(mut batch) => self.send_batch_envelope(&mut batch).await,
            None => Ok(()),
        }
    }

    async fn send_batch(
        &mut self,
        batch: AmqpEventBatch,
        options: SendEventOptions,
    ) -> Result<(), AmqpSendError> {
        match build_amqp_batch_from_messages(batch.events.into_iter(), options.partition_key) {
            Some(mut batch) => self.send_batch_envelope(&mut batch).await,
            None => Ok(()),
        }
    }

    pub(crate) async fn dispose(mut self) -> Result<(), DisposeProducerError> {
        self.sender.close().await?;
        self.session_handle.close().await?;
        Ok(())
    }
}

pub struct RecoverableAmqpProducer<'a, RP> {
    producer: &'a mut AmqpProducer<RP>,
    client: &'a mut Sharable<AmqpClient>,
}

impl<'a, RP> RecoverableAmqpProducer<'a, RP>
where
    RP: EventHubsRetryPolicy + Send,
{
    pub(crate) fn new(
        producer: &'a mut AmqpProducer<RP>,
        client: &'a mut Sharable<AmqpClient>,
    ) -> RecoverableAmqpProducer<'a, RP> {
        RecoverableAmqpProducer { producer, client }
    }

    async fn recover_and_send_batch_envelope(
        &mut self,
        should_try_recover: bool,
        batch: &mut BatchEnvelope,
    ) -> Result<(), RecoverAndSendError> {
        if should_try_recover {
            if let Err(recovery_err) = self.client.recover().await {
                log::error!("Failed to recover client: {:?}", recovery_err);
                if recovery_err.is_scope_disposed() {
                    return Err(RecoverAndSendError::ConnectionScopeDisposed);
                }
            }

            // reattach the link
            match self.client {
                Sharable::Owned(client) => client.recover_producer(&mut self.producer).await?,
                Sharable::Shared(client) => {
                    client.lock().await
                        .recover_producer(&mut self.producer).await?
                },
                Sharable::None => return Err(RecoverAndSendError::ConnectionScopeDisposed),
            }
        }

        self.producer.send_batch_envelope(batch).await?;
        Ok(())
    }

    async fn send_batch_envelope(&mut self, mut batch: BatchEnvelope) -> Result<(), RecoverAndSendError> {
        let mut failed_attempts = 0;
        let mut try_timeout = self.producer.retry_policy.calculate_try_timeout(failed_attempts);
        let mut should_try_recover = false;

        loop {
            let fut = self.recover_and_send_batch_envelope(should_try_recover, &mut batch);
            let err = match util::time::timeout(try_timeout, fut).await {
                Ok(Ok(_)) => return Ok(()),
                Ok(Err(err)) => err,
                Err(elapsed) => elapsed.into(),
            };

            // Scope is disposed, so we can't recover or retry
            if err.is_scope_disposed() {
                return Err(err);
            }
            should_try_recover = err.should_try_recover();

            failed_attempts += 1;
            let retry_delay = self
                .producer
                .retry_policy
                .calculate_retry_delay(&err, failed_attempts);

            match retry_delay {
                Some(retry_delay) => {
                    util::time::sleep(retry_delay).await;
                    try_timeout = self.producer.retry_policy.calculate_try_timeout(failed_attempts);
                }
                None => return Err(err),
            }
        }
    }

}

#[async_trait]
impl<'a, RP> TransportProducer for RecoverableAmqpProducer<'a, RP>
where
    RP: EventHubsRetryPolicy + Send,
{
    type MessageBatch = AmqpEventBatch;

    type SendError = RecoverAndSendError;
    type CreateBatchError = RequestedSizeOutOfRange;

    fn create_batch(
        &self,
        options: CreateBatchOptions,
    ) -> Result<Self::MessageBatch, Self::CreateBatchError> {
        self.producer.create_batch(options)
    }

    async fn send(
        &mut self,
        events: impl Iterator<Item = Event> + ExactSizeIterator + Send,
        options: SendEventOptions,
    ) -> Result<(), Self::SendError> {
        // TODO: check size of envelope and make sure it's not too big
        match create_envelope_from_events(events, options.partition_key) {
            Some(batch) => self.send_batch_envelope(batch).await,
            None => Ok(()),
        }
    }

    async fn send_batch(
        &mut self,
        batch: Self::MessageBatch,
        options: SendEventOptions,
    ) -> Result<(), Self::SendError> {
        match build_amqp_batch_from_messages(batch.events.into_iter(), options.partition_key) {
            Some(batch) => self.send_batch_envelope(batch).await,
            None => Ok(()),
        }
    }
}
