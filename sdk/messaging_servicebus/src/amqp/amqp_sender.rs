use async_trait::async_trait;
use fe2o3_amqp::link::DetachError;
use fe2o3_amqp_management::client::MgmtClient;
use fe2o3_amqp_types::messaging::Outcome;
use fe2o3_amqp_types::primitives::Array;
use std::time::Duration as StdDuration;

use crate::primitives::service_bus_retry_policy::ServiceBusRetryPolicyError;
use crate::sender::MINIMUM_BATCH_SIZE_LIMIT;
use crate::{
    core::TransportSender,
    primitives::service_bus_retry_policy::{
        run_operation, RetryError, ServiceBusRetryPolicy, ServiceBusRetryPolicyState,
    },
    CreateMessageBatchOptions, ServiceBusMessage,
};

use super::amqp_message_batch::AmqpMessageBatch;
use super::amqp_message_converter::build_amqp_batch_from_messages;
use super::amqp_request_message::cancel_scheduled_message::CancelScheduledMessageRequest;
use super::amqp_request_message::schedule_message::ScheduleMessageRequest;
use super::error::RequestedSizeOutOfRange;
use super::{
    amqp_message_converter::{
        batch_service_bus_messages_as_amqp_message, BatchEnvelope, SendableEnvelope,
    },
    error::NotAcceptedError,
};

pub struct AmqpSender<RP: ServiceBusRetryPolicy> {
    pub identifier: u32,
    pub retry_policy: RP,
    pub name: String,
    pub sender: fe2o3_amqp::Sender,
    pub management_client: MgmtClient,
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
    type Error = DetachError;
    type SendError = RetryError<RP::Error>;
    type CloseError = DetachError;
    type MessageBatch = AmqpMessageBatch;
    type CreateMessageBatchError = RequestedSizeOutOfRange;

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
        &self,
        options: CreateMessageBatchOptions,
    ) -> Result<Self::MessageBatch, Self::CreateMessageBatchError> {
        let link_max_message_size = self.sender.max_message_size().unwrap_or(u64::MAX);
        let max_size_in_bytes = match options.max_size_in_bytes {
            Some(max_size_in_bytes) => {
                if max_size_in_bytes < MINIMUM_BATCH_SIZE_LIMIT
                    || max_size_in_bytes > link_max_message_size
                {
                    return Err(RequestedSizeOutOfRange {});
                }

                max_size_in_bytes
            }
            // If this field is zero or unset, there is no maximum size imposed by the link endpoint.
            None => link_max_message_size,
        };
        Ok(AmqpMessageBatch::new(max_size_in_bytes))
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
    ) -> Result<(), Self::SendError> {
        // ServiceBusRetryPolicyExt::run_operation(&mut , operation, t1, cancellation_token)
        // TODO: retry policy
        let batch = batch_service_bus_messages_as_amqp_message(messages, false);
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);
        let sender = &mut self.sender;
        run_operation! {
            policy,
            RP,
            try_timeout,
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
        message_batch: Self::MessageBatch,
    ) -> Result<(), Self::SendError> {
        let batch = build_amqp_batch_from_messages(message_batch.messages.into_iter(), false);
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);
        let sender = &mut self.sender;
        run_operation! {
            policy,
            RP,
            try_timeout,
            send_batch_envelope::<RP::Error>(sender, &batch).await
        }
    }

    async fn schedule_messages(
        &mut self,
        messages: impl Iterator<Item = ServiceBusMessage> + Send,
    ) -> Result<Vec<i64>, Self::SendError> {
        use super::scheduled_message::ScheduledBatchEnvelope;
        use fe2o3_amqp::link::SendError;

        let encoded_messages = messages
            .map(|m| m.amqp_message)
            .map(ScheduledBatchEnvelope::from_amqp_message)
            .map(|result| result.map(|opt| opt.map(|m| m.into_ordered_map())))
            .collect::<Result<Option<Vec<_>>, _>>()
            .map_err(|_| SendError::MessageEncodeError)
            .map_err(RP::Error::from)
            .map_err(RetryError::Operation)?;

        match encoded_messages {
            Some(messages) => {
                let policy = &mut self.retry_policy;

                // Use a wrapper type to avoid mistakes
                let mut try_timeout = policy.calculate_try_timeout(0);
                let mut request = ScheduleMessageRequest::new(messages);

                let management_client = &mut self.management_client;
                run_operation! {
                    policy,
                    RP,
                    try_timeout,
                    schedule_message::<RP::Error>(management_client, &mut request, try_timeout).await
                }
            }
            None => Ok(vec![]),
        }
    }

    async fn cancel_scheduled_messages(
        &mut self,
        sequence_numbers: Vec<i64>,
    ) -> Result<(), Self::SendError> {
        if sequence_numbers.is_empty() {
            return Ok(());
        }

        // TODO: solve lifetime issue if link name is borrowed
        let mut request = CancelScheduledMessageRequest::new(Array(sequence_numbers));

        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);
        let management_client = &mut self.management_client;

        run_operation!(
            policy,
            RP,
            try_timeout,
            cancel_scheduled_messages::<RP::Error>(management_client, &mut request, try_timeout)
                .await
        )
    }

    /// Closes the connection to the transport producer instance.
    ///
    /// # Arguments
    ///
    /// * `cancellation_token` - An optional [CancellationToken] instance to signal the request to
    ///   cancel the operation.
    async fn close(self) -> Result<(), Self::CloseError> {
        self.sender.close().await?;
        self.management_client.close().await?;
        Ok(())
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

async fn schedule_message<E: ServiceBusRetryPolicyError>(
    mgmt_client: &mut MgmtClient,
    request: &mut ScheduleMessageRequest, // Use a reference to avoid repeated serialization
    try_timeout: StdDuration,
) -> Result<Vec<i64>, E> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response
        .into_sequence_numbers()
        .map(|array| array.into_inner())
        .unwrap_or_else(|| Vec::with_capacity(0)))
}

async fn cancel_scheduled_messages<'a, E: ServiceBusRetryPolicyError>(
    mgmt_client: &'a mut MgmtClient,
    request: &'a mut CancelScheduledMessageRequest,
    try_timeout: StdDuration,
) -> Result<(), E> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let _response = mgmt_client.call(request).await?;
    Ok(())
}
