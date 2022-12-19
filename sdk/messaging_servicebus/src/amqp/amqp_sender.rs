use async_trait::async_trait;
use fe2o3_amqp::link::DetachError;
use fe2o3_amqp_management::error::Error as ManagementError;
use fe2o3_amqp_types::messaging::Outcome;
use fe2o3_amqp_types::primitives::Array;
use std::time::Duration as StdDuration;
use tokio::sync::mpsc;

use crate::primitives::error::RetryError;
use crate::sender::MINIMUM_BATCH_SIZE_LIMIT;
use crate::{
    core::TransportSender,
    primitives::service_bus_retry_policy::{
        run_operation, ServiceBusRetryPolicy, ServiceBusRetryPolicyState,
    },
    CreateMessageBatchOptions, ServiceBusMessage,
};

use super::amqp_cbs_link;
use super::amqp_management_link::AmqpManagementLink;
use super::amqp_message_batch::AmqpMessageBatch;
use super::amqp_message_converter::build_amqp_batch_from_messages;
use super::amqp_request_message::cancel_scheduled_message::CancelScheduledMessageRequest;
use super::amqp_request_message::schedule_message::ScheduleMessageRequest;
use super::error::{AmqpRequestResponseError, AmqpSendError, RequestedSizeOutOfRange};
use super::{
    amqp_message_converter::{
        batch_service_bus_messages_as_amqp_message, BatchEnvelope, SendableEnvelope,
    },
    error::NotAcceptedError,
};

#[derive(Debug)]
pub struct AmqpSender<RP> {
    pub(crate) identifier: u32,
    pub(crate) retry_policy: RP,
    pub(crate) sender: fe2o3_amqp::Sender,
    pub(crate) management_link: AmqpManagementLink,
    pub(crate) cbs_command_sender: mpsc::Sender<amqp_cbs_link::Command>,
}

impl<RP> AmqpSender<RP> {
    async fn send_batch_envelope(
        &mut self,
        batch: &Option<BatchEnvelope>,
    ) -> Result<(), AmqpSendError> {
        if let Some(batch) = batch {
            let outcome = match &batch.sendable {
                SendableEnvelope::Single(sendable) => match batch.batchable {
                    true => {
                        let fut = self.sender.send_batchable_ref(sendable).await?;
                        fut.await?
                    }
                    false => self.sender.send_ref(sendable).await?,
                },
                SendableEnvelope::Batch(sendable) => match batch.batchable {
                    true => {
                        let fut = self.sender.send_batchable_ref(sendable).await?;
                        fut.await?
                    }
                    false => self.sender.send_ref(sendable).await?,
                },
            };

            match outcome {
                Outcome::Accepted(_) => return Ok(()),
                Outcome::Rejected(rejected) => {
                    return Err(AmqpSendError::from(NotAcceptedError::Rejected(rejected)))
                }
                Outcome::Released(released) => {
                    return Err(AmqpSendError::from(NotAcceptedError::Released(released)))
                }
                Outcome::Modified(modified) => {
                    return Err(AmqpSendError::from(NotAcceptedError::Modified(modified)))
                }
                #[cfg(feature = "transaction")]
                Outcome::Declared(_) => {
                    unreachable!("Declared is not expected outside txn-control links")
                }
            }
        }

        Ok(())
    }

    async fn schedule_message_inner(
        &mut self,
        request: &mut ScheduleMessageRequest, // Use a reference to avoid repeated serialization
        try_timeout: StdDuration,
    ) -> Result<Vec<i64>, AmqpRequestResponseError> {
        let server_timeout = try_timeout.as_millis() as u32;
        request.set_server_timeout(Some(server_timeout));

        let response = self.management_link.client_mut().call(request).await?;
        Ok(response.into_sequence_numbers())
    }

    async fn cancel_scheduled_messages_inner(
        &mut self,
        request: &mut CancelScheduledMessageRequest,
        try_timeout: &StdDuration,
    ) -> Result<(), AmqpRequestResponseError> {
        let server_timeout = try_timeout.as_millis() as u32;
        request.set_server_timeout(Some(server_timeout));

        let _response = self.management_link.client_mut().call(request).await?;
        Ok(())
    }

}

#[async_trait]
impl<RP> TransportSender for AmqpSender<RP>
where
    RP: ServiceBusRetryPolicy + Send + Sync,
    RP::State: ServiceBusRetryPolicyState,
{
    type SendError = RetryError<AmqpSendError>;
    type ScheduleError = RetryError<AmqpRequestResponseError>;
    type CloseError = DetachError;
    type MessageBatch = AmqpMessageBatch;
    type CreateMessageBatchError = RequestedSizeOutOfRange;

    /// Creates a size-constraint batch to which [`ServiceBusMessage`] may be added using
    /// a try-based pattern.  If a message would exceed the maximum allowable size of the batch, the
    /// batch will not allow adding the message and returns an error.
    ///
    /// Because messages that would violate the size constraint cannot be added, publishing a batch
    /// will not trigger an exception when attempting to send the message to the Queue/Topic.
    fn create_message_batch(
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
    /// the size of the messages exceed the maximum size of a single batch, an error will be
    /// returned and the send will fail. In order to ensure that the messages being sent will fit
    /// in a batch, use [`send_batch()`] instead.
    async fn send(
        &mut self,
        messages: impl Iterator<Item = ServiceBusMessage> + ExactSizeIterator + Send,
    ) -> Result<(), Self::SendError> {
        // ServiceBusRetryPolicyExt::run_operation(&mut , operation, t1, cancellation_token)
        // TODO: retry policy
        let batch = batch_service_bus_messages_as_amqp_message(messages, false);
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);
        let result = run_operation! {
            {&self.retry_policy},
            AmqpSendError,
            try_timeout,
            self.send_batch_envelope(&batch)
        };
        result
    }

    /// Sends [`ServiceBusMessageBatch`] to the associated Queue/Topic.
    async fn send_batch(
        &mut self,
        message_batch: Self::MessageBatch,
    ) -> Result<(), Self::SendError> {
        let batch = build_amqp_batch_from_messages(message_batch.messages.into_iter(), false);
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);
        let result = run_operation! {
            {&self.retry_policy},
            AmqpSendError,
            try_timeout,
            self.send_batch_envelope(&batch)
        };
        // TODO: Somehow directly returning the result will lead to an error in the macro
        result
    }

    async fn schedule_messages(
        &mut self,
        messages: impl Iterator<Item = ServiceBusMessage> + Send,
    ) -> Result<Vec<i64>, Self::ScheduleError> {
        use super::scheduled_message::ScheduledBatchEnvelope;
        use fe2o3_amqp::link::SendError;

        let encoded_messages = messages
            .map(|m| m.amqp_message)
            .map(ScheduledBatchEnvelope::from_amqp_message)
            .map(|result| result.map(|opt| opt.map(|m| m.into_ordered_map())))
            .collect::<Result<Option<Vec<_>>, _>>()
            .map_err(|_| ManagementError::Send(SendError::MessageEncodeError))
            .map_err(AmqpRequestResponseError::RequestResponse)
            .map_err(RetryError::Operation)?;

        match encoded_messages {
            Some(messages) => {
                // Use a wrapper type to avoid mistakes
                let mut try_timeout = self.retry_policy.calculate_try_timeout(0);
                let associated_link_name = self.sender.name().to_string();
                let mut request = ScheduleMessageRequest::new(messages, Some(associated_link_name));

                let result = run_operation! {
                    {&self.retry_policy},
                    AmqpRequestResponseError,
                    try_timeout,
                    self.schedule_message_inner(&mut request, try_timeout)
                };
                result
            }
            None => Ok(vec![]),
        }
    }

    async fn cancel_scheduled_messages(
        &mut self,
        sequence_numbers: Vec<i64>,
    ) -> Result<(), Self::ScheduleError> {
        if sequence_numbers.is_empty() {
            return Ok(());
        }

        // TODO: solve lifetime issue if link name is borrowed
        let mut request =
            CancelScheduledMessageRequest::new(Array(sequence_numbers), Some(self.sender.name().to_string()));

        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);
        run_operation!(
            {&self.retry_policy},
            AmqpRequestResponseError,
            try_timeout,
            self.cancel_scheduled_messages_inner(&mut request, &try_timeout)
        )
    }

    /// Closes the connection to the transport producer instance.
    async fn close(self) -> Result<(), Self::CloseError> {
        // An error would mean that the AmqpCbsLink event loop has stopped
        let _ = self
            .cbs_command_sender
            .send(amqp_cbs_link::Command::RemoveAuthorizationRefresher(
                self.identifier,
            ))
            .await;
        self.sender.close().await?;
        self.management_link.close().await?;
        Ok(())
    }
}
