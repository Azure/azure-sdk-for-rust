use async_trait::async_trait;
use azure_core::Url;
use fe2o3_amqp::{session::SessionHandle, Sender};
use fe2o3_amqp_types::messaging::Outcome;
use tokio::sync::mpsc;

use crate::{
    amqp::amqp_message_converter::create_envelope_from_events,
    core::{RecoverableError, RecoverableTransport, TransportClient, TransportProducer},
    event_hubs_retry_policy::EventHubsRetryPolicy,
    producer::{CreateBatchOptions, SendEventOptions, MINIMUM_BATCH_SIZE_LIMIT_IN_BYTES},
    util::{self},
    EventData,
};

use super::{
    amqp_cbs_link::Command,
    amqp_client::AmqpClient,
    amqp_event_batch::AmqpEventBatch,
    amqp_message_converter::{
        build_amqp_batch_from_messages, BatchEnvelope, BatchEnvelopeState, SendableEnvelope,
    },
    error::{
        AmqpSendError, CreateBatchError, DisposeProducerError, NotAcceptedError,
        RecoverAndSendError,
    },
};

#[derive(Debug)]
pub struct AmqpProducer<RP> {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) _session_identifier: u32,
    pub(crate) sender: Sender,
    pub(crate) link_identifier: u32,

    // /// TODO: this is only useful for idempotent sending, which won't be implemented in the current plan
    // pub(crate) initialized_partition_properties: PartitionPublishingProperties,
    pub(crate) retry_policy: RP,
    pub(crate) endpoint: Url,
    pub(crate) cbs_command_sender: mpsc::Sender<Command>,
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
    ) -> Result<AmqpEventBatch, CreateBatchError> {
        let link_max_message_size = self.sender.max_message_size().unwrap_or(u64::MAX);
        let max_size_in_bytes: u64 = match options.max_size_in_bytes {
            Some(max_size_in_bytes) => {
                if max_size_in_bytes < MINIMUM_BATCH_SIZE_LIMIT_IN_BYTES
                    || max_size_in_bytes > link_max_message_size
                {
                    return Err(CreateBatchError::RequestedSizeOutOfRange);
                }

                max_size_in_bytes
            }
            // If this field is zero or unset, there is no maximum size imposed by the link endpoint.
            None => link_max_message_size,
        };
        Ok(AmqpEventBatch::new(
            max_size_in_bytes,
            options.partition_key,
        )?)
    }

    pub(crate) async fn close(mut self) -> Result<(), DisposeProducerError> {
        // There is no need to remove the refresher if CBS link is already stopped
        let _ = self
            .cbs_command_sender
            .send(Command::RemoveAuthorizationRefresher(self.link_identifier))
            .await;

        self.sender.close().await?;
        self.session_handle.close().await?;
        Ok(())
    }
}

pub struct RecoverableAmqpProducer<'a, RP> {
    producer: &'a mut AmqpProducer<RP>,
    client: &'a mut AmqpClient,
}

impl<'a, RP> RecoverableAmqpProducer<'a, RP>
where
    RP: EventHubsRetryPolicy + Send,
{
    pub(crate) fn new(
        producer: &'a mut AmqpProducer<RP>,
        client: &'a mut AmqpClient,
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
            self.client.recover_producer(self.producer).await?;
        }

        self.producer.send_batch_envelope(batch).await?;
        Ok(())
    }

    async fn send_batch_envelope(
        &mut self,
        mut batch: BatchEnvelope,
    ) -> Result<(), RecoverAndSendError> {
        let mut failed_attempts = 0;
        let mut try_timeout = self
            .producer
            .retry_policy
            .calculate_try_timeout(failed_attempts);
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
                    try_timeout = self
                        .producer
                        .retry_policy
                        .calculate_try_timeout(failed_attempts);
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
    type CreateBatchError = CreateBatchError;

    fn create_batch(
        &self,
        options: CreateBatchOptions,
    ) -> Result<Self::MessageBatch, Self::CreateBatchError> {
        self.producer.create_batch(options)
    }

    async fn send(
        &mut self,
        events: impl Iterator<Item = EventData> + ExactSizeIterator + Send,
        options: SendEventOptions,
    ) -> Result<(), Self::SendError> {
        // TODO: check size of envelope and make sure it's not too big
        match create_envelope_from_events(events, options.into_partition_key()) {
            Some(batch) => self.send_batch_envelope(batch).await,
            None => Ok(()),
        }
    }

    async fn send_batch(
        &mut self,
        batch: Self::MessageBatch,
        options: SendEventOptions,
    ) -> Result<(), Self::SendError> {
        match build_amqp_batch_from_messages(batch.events.into_iter(), options.into_partition_key())
        {
            Some(batch) => self.send_batch_envelope(batch).await,
            None => Ok(()),
        }
    }
}
