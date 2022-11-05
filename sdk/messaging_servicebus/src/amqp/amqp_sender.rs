use async_trait::async_trait;
use fe2o3_amqp::link::DetachError;
use fe2o3_amqp_types::messaging::Outcome;
use tokio_util::sync::CancellationToken;

use crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyError;
use crate::{
    core::TransportSender,
    primitives::service_bus_retry_policy::{
        run_operation, RetryError, ServiceBusRetryPolicy, ServiceBusRetryPolicyState,
    },
    CreateMessageBatchOptions, ServiceBusMessage, ServiceBusMessageBatch,
};

use super::{
    amqp_message_converter::{
        batch_service_bus_messages_as_amqp_message, BatchEnvelope, SendableEnvelope,
    },
    error::NotAcceptedError,
};

pub(crate) struct AmqpSender<RP: ServiceBusRetryPolicy> {
    pub identifier: u32,
    pub retry_policy: RP,
    pub sender: fe2o3_amqp::Sender,
}

impl<RP: ServiceBusRetryPolicy> AmqpSender<RP>
where
    RP: ServiceBusRetryPolicy + Send + Sync,
    RP::State: ServiceBusRetryPolicyState,
    RP::Error: ServiceBusRetryPolicyError,
{
}

#[async_trait]
impl<RP> TransportSender for AmqpSender<RP>
where
    RP: ServiceBusRetryPolicy + Send + Sync,
    RP::State: ServiceBusRetryPolicyState,
    RP::Error: ServiceBusRetryPolicyError,
{
    type Error = ();
    type SendError = RetryError<RP::Error>;
    type CloseError = DetachError;

    /// Creates a size-constraint batch to which <see cref="ServiceBusMessage" /> may be added using
    /// a try-based pattern.  If a message would exceed the maximum allowable size of the batch, the
    /// batch will not allow adding the message and signal that scenario using its return value.
    ///
    /// Because messages that would violate the size constraint cannot be added, publishing a batch
    /// will not trigger an exception when attempting to send the message to the Queue/Topic.
    ///
    /// # Arguments
    ///
    /// * `options` - The set of options to consider when creating this batch.
    /// * `cancellation_token` - An optional <see cref="CancellationToken"/> instance to signal the
    ///   request to cancel the operation.
    ///
    /// # Returns
    ///
    /// An [ServiceBusMessageBatch] with the requested `options`
    async fn create_message_batch(
        &mut self,
        _options: CreateMessageBatchOptions,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    /// Sends a list of messages to the associated Service Bus entity using a batched approach. If
    /// the size of the messages exceed the maximum size of a single batch, an exception will be
    /// triggered and the send will fail. In order to ensure that the messages being sent will fit
    /// in a batch, use <see cref="SendBatchAsync"/> instead.
    ///
    /// # Arguments
    ///
    /// * `messages` - The list of messages to send.
    /// * `cancellationToken` - An optional <see cref="CancellationToken"/> instance to signal the
    ///   request to cancel the operation.
    async fn send(
        &mut self,
        messages: impl Iterator<Item = ServiceBusMessage> + ExactSizeIterator + Send,
        cancellation_token: CancellationToken,
    ) -> Result<(), Self::SendError> {
        // ServiceBusRetryPolicyExt::run_operation(&mut , operation, t1, cancellation_token)
        // TODO: retry policy
        let batch = batch_service_bus_messages_as_amqp_message(messages, false);
        let policy = &mut self.retry_policy;
        let sender = &mut self.sender;
        run_operation! {
            policy,
            RP,
            cancellation_token,
            send_batch_envelope::<RP::Error>(sender, &batch).await
        }
    }

    /// Sends a <see cref="ServiceBusMessageBatch"/> to the associated Queue/Topic.
    ///
    /// # Arguments
    ///
    /// * `message_batch` - The set of messages to send.
    /// * `cancellation_token` - An optional <see cref="CancellationToken"/> instance to signal the
    ///   request to cancel the operation.
    ///
    /// # Returns
    ///
    /// A task to be resolved on when the operation has completed.
    async fn send_batch(
        &mut self,
        _message_batch: ServiceBusMessageBatch,
        _cancellation_token: CancellationToken,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    async fn schedule_messages(
        &mut self,
        _messages: impl Iterator<Item = &ServiceBusMessage> + Send,
        _cancellation_token: CancellationToken,
    ) -> Result<Vec<i64>, Self::Error> {
        todo!()
    }

    async fn cancel_scheduled_messages(
        &mut self,
        _sequence_numbers: &[i64],
        _cancellation_token: CancellationToken,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    /// Closes the connection to the transport producer instance.
    ///
    /// # Arguments
    ///
    /// * `cancellation_token` - An optional [CancellationToken] instance to signal the request to
    ///   cancel the operation.
    async fn close(self) -> Result<(), Self::CloseError> {
        self.sender.close().await
    }
}

async fn send_batch_envelope<E: ServiceBusRetryPolicyError>(
    sender: &mut fe2o3_amqp::Sender,
    batch: &Option<BatchEnvelope>,
) -> Result<(), E> {
    if let Some(batch) = batch {
        let outcome = match &batch.sendable {
            SendableEnvelope::Single(sendable) => match batch.batchable {
                true => {
                    let fut = sender.send_batchable_ref(sendable).await?;
                    fut.await?
                }
                false => sender.send_ref(sendable).await?,
            },
            SendableEnvelope::Batch(sendable) => match batch.batchable {
                true => {
                    let fut = sender.send_batchable_ref(sendable).await?;
                    fut.await?
                }
                false => sender.send_ref(sendable).await?,
            },
        };

        match outcome {
            Outcome::Accepted(_) => return Ok(()),
            Outcome::Rejected(rejected) => {
                return Err(E::from(NotAcceptedError::Rejected(rejected)))
            }
            Outcome::Released(released) => {
                return Err(E::from(NotAcceptedError::Released(released)))
            }
            Outcome::Modified(modified) => {
                return Err(E::from(NotAcceptedError::Modified(modified)))
            }
            Outcome::Declared(_) => {
                unreachable!("Declared is not expected outside txn-control links")
            }
        }
    }

    Ok(())
}
