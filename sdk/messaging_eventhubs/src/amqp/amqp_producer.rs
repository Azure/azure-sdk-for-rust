use async_trait::async_trait;
use fe2o3_amqp::{session::SessionHandle, Sender, link::SendError};
use fe2o3_amqp_types::messaging::Outcome;

use crate::{core::{transport_producer::TransportProducer, transport_producer_features::TransportProducerFeatures, transport_event_batch::TransportEventBatch}, event_hubs_retry_policy::EventHubsRetryPolicy, producer::{PartitionPublishingOptions, create_batch_options::CreateBatchOptions, send_event_options::SendEventOptions, event_hub_producer_client::MINIMUM_BATCH_SIZE_LIMIT}, util::{IntoAzureCoreError, self}, Event, amqp::amqp_message_converter::create_envelope_from_events};

use super::{amqp_connection_scope::AmqpConnectionScope, error::{OpenProducerError, DisposeProducerError, RequestedSizeOutOfRange, AmqpSendError, NotAcceptedError}, amqp_event_batch::AmqpEventBatch, amqp_message_converter::{BatchEnvelope, BatchEnvelopeState, SendableEnvelope, build_amqp_batch_from_messages}};

pub struct AmqpProducer<RP> {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) sender: Sender,
    pub(crate) link_identifier: u32,
    pub(crate) initialized_partition_properties: PartitionPublishingOptions,
    pub(crate) retry_policy: RP,
}

impl<RP> AmqpProducer<RP>
where
    RP: EventHubsRetryPolicy + Send,
{
    async fn send_batch_envelope(
        &mut self,
        mut batch: BatchEnvelope
    ) -> Result<(), AmqpSendError> {
        let mut failed_attempts = 0;
        let mut try_timeout = self.retry_policy.calculate_try_timeout(failed_attempts);

        loop {
            match self.send_batch_envelope_inner(&mut batch).await {
                Ok(_) => return Ok(()),
                Err(err) => {
                    failed_attempts += 1;
                    let retry_delay = self.retry_policy.calculate_retry_delay(&err, failed_attempts);

                    match retry_delay {
                        Some(retry_delay) => {
                            util::time::sleep(retry_delay).await;
                            try_timeout = self.retry_policy.calculate_try_timeout(failed_attempts);
                        }
                        None => return Err(err),
                    }
                }
            }
        }
    }

    async fn send_batch_envelope_inner(
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
}

#[async_trait]
impl<RP> TransportProducer for AmqpProducer<RP>
where
    RP: EventHubsRetryPolicy + Send,
{
    type MessageBatch = AmqpEventBatch;

    type SendError = AmqpSendError;
    type CreateBatchError = RequestedSizeOutOfRange;

    type DisposeError = DisposeProducerError;

    fn create_batch(
        &self,
        options: CreateBatchOptions,
    ) -> Result<Self::MessageBatch, Self::CreateBatchError> {
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

    async fn send(&mut self, events: impl Iterator<Item = Event> + ExactSizeIterator + Send, options: SendEventOptions) -> Result<(), Self::SendError> {
        // TODO: check size of envelope and make sure it's not too big
        match create_envelope_from_events(events, options.partition_key) {
            Some(batch) => self.send_batch_envelope(batch).await,
            None => Ok(())
        }
    }

    async fn send_batch(&mut self, batch: Self::MessageBatch, options: SendEventOptions) -> Result<(), Self::SendError> {
        match build_amqp_batch_from_messages(batch.events.into_iter(), options.partition_key) {
            Some(batch) => self.send_batch_envelope(batch).await,
            None => Ok(())
        }
    }

    // async fn read_initialization_publishing_properties(&mut self) -> Result<PartitionPublishingOptions, ()> { todo!() }


    async fn close(mut self) -> Result<(), Self::DisposeError> {
        self.sender.close().await?;
        self.session_handle.close().await?;
        Ok(())
    }
}
