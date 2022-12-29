use async_trait::async_trait;
use fe2o3_amqp::link::DetachError;
use fe2o3_amqp_management::error::Error as ManagementError;
use fe2o3_amqp_types::messaging::Outcome;
use fe2o3_amqp_types::primitives::Array;
use std::sync::Arc;
use std::time::Duration as StdDuration;
use tokio::sync::{mpsc, Mutex};
use url::Url;

use crate::authorization::service_bus_claim;
use crate::core::RecoverableTransport;
use crate::primitives::error::RetryError;
use crate::sealed::Sealed;
use crate::sender::MINIMUM_BATCH_SIZE_LIMIT;
use crate::{
    core::TransportSender,
    primitives::service_bus_retry_policy::{run_operation, ServiceBusRetryPolicy},
    CreateMessageBatchOptions, ServiceBusMessage,
};

use super::amqp_cbs_link;
use super::amqp_connection_scope::AmqpConnectionScope;
use super::amqp_management_link::AmqpManagementLink;
use super::amqp_message_batch::AmqpMessageBatch;
use super::amqp_message_converter::{build_amqp_batch_from_messages, BatchEnvelopeState};
use super::amqp_request_message::cancel_scheduled_message::CancelScheduledMessageRequest;
use super::amqp_request_message::schedule_message::ScheduleMessageRequest;
use super::error::{
    AmqpRequestResponseError, AmqpSendError, RecoverSenderError, RequestedSizeOutOfRange,
};
use super::{
    amqp_message_converter::{
        batch_service_bus_messages_as_amqp_message, BatchEnvelope, SendableEnvelope,
    },
    error::NotAcceptedError,
};

/// An AMQP implementation for Service Bus message sender.
#[derive(Debug)]
pub struct AmqpSender {
    pub(crate) id: u32,
    pub(crate) service_endpoint: Arc<Url>,
    pub(crate) entity_path: String,
    pub(crate) identifier_str: String,
    pub(crate) retry_policy: Box<dyn ServiceBusRetryPolicy>,
    pub(crate) sender: fe2o3_amqp::Sender,
    pub(crate) management_link: AmqpManagementLink,
    pub(crate) cbs_command_sender: mpsc::Sender<amqp_cbs_link::Command>,

    // This is ONLY used for recovery
    pub(crate) connection_scope: Arc<Mutex<AmqpConnectionScope>>,
}

#[async_trait]
impl RecoverableTransport for AmqpSender {
    type RecoverError = RecoverSenderError;

    async fn recover(&mut self) -> Result<(), Self::RecoverError> {
        let mut connection_scope = self.connection_scope.lock().await;
        connection_scope
            .recover()
            .await
            .map_err(|connection_scope_error| {
                log::error!(
                    "Failed to recover connection scope: {:?}",
                    connection_scope_error
                );
                Self::RecoverError::ConnectionScopeDisposed
            })?;

        let endpoint = format!("{}/{}", self.service_endpoint, self.entity_path);
        let resource = endpoint.clone();
        connection_scope
            .request_refreshable_authorization_using_cbs(
                self.id,
                endpoint,
                resource,
                vec![service_bus_claim::SEND.to_string()],
            )
            .await?;
        self.sender
            .detach_then_resume_on_session(&connection_scope.session.handle)
            .await?;
        self.management_link = connection_scope
            .open_management_link(
                &self.service_endpoint,
                &self.entity_path,
                &self.identifier_str,
            )
            .await?;
        self.cbs_command_sender = connection_scope.cbs_link.command_sender().clone();
        Ok(())
    }
}

impl AmqpSender {
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

impl Sealed for AmqpSender {}

#[async_trait]
impl TransportSender for AmqpSender {
    type SendError = RetryError<AmqpSendError>;
    type ScheduleError = RetryError<AmqpRequestResponseError>;
    type CloseError = DetachError;
    type MessageBatch = AmqpMessageBatch;
    type CreateMessageBatchError = RequestedSizeOutOfRange;

    fn entity_path(&self) -> &str {
        &self.entity_path
    }

    fn identifier(&self) -> &str {
        &self.identifier_str
    }

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
    /// in a batch, use [`send_batch`] instead.
    async fn send(
        &mut self,
        messages: impl Iterator<Item = ServiceBusMessage> + ExactSizeIterator + Send,
    ) -> Result<(), Self::SendError> {
        let batch = batch_service_bus_messages_as_amqp_message(messages, false);
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        match batch {
            Some(mut envelope) => {
                let result = run_operation! {
                    {&self.retry_policy},
                    AmqpSendError,
                    try_timeout,
                    self.send_batch_envelope(&mut envelope),
                    self.recover()
                };
                // TODO: Somehow directly returning the result will lead to an error in the macro
                #[allow(clippy::let_and_return)]
                result
            }
            None => Ok(()),
        }
    }

    /// Sends [`ServiceBusMessageBatch`] to the associated Queue/Topic.
    async fn send_batch(
        &mut self,
        message_batch: Self::MessageBatch,
    ) -> Result<(), Self::SendError> {
        let batch = build_amqp_batch_from_messages(message_batch.messages.into_iter(), false);
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        match batch {
            Some(mut envelope) => {
                let result = run_operation! {
                    {&self.retry_policy},
                    AmqpSendError,
                    try_timeout,
                    self.send_batch_envelope(&mut envelope),
                    self.recover()
                };
                #[allow(clippy::let_and_return)]
                result
            }
            None => Ok(()),
        }
    }

    async fn schedule_messages(
        &mut self,
        messages: impl Iterator<Item = ServiceBusMessage> + Send,
    ) -> Result<Vec<i64>, Self::ScheduleError> {
        use super::scheduled_message::ScheduledBatchEnvelope;
        use fe2o3_amqp::link::SendError;

        let encoded_messages = messages
            .map(|m| m.amqp_message)
            .map(ScheduledBatchEnvelope::try_from_amqp_message)
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
                    self.schedule_message_inner(&mut request, try_timeout),
                    self.recover()
                };
                #[allow(clippy::let_and_return)]
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
        let mut request = CancelScheduledMessageRequest::new(
            Array(sequence_numbers),
            Some(self.sender.name().to_string()),
        );

        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);
        run_operation!(
            { &self.retry_policy },
            AmqpRequestResponseError,
            try_timeout,
            self.cancel_scheduled_messages_inner(&mut request, &try_timeout),
            self.recover()
        )
    }

    /// Closes the connection to the transport producer instance.
    async fn close(self) -> Result<(), Self::CloseError> {
        // An error would mean that the AmqpCbsLink event loop has stopped
        let _ = self
            .cbs_command_sender
            .send(amqp_cbs_link::Command::RemoveAuthorizationRefresher(
                self.id,
            ))
            .await;
        self.sender.close().await?;
        self.management_link.close().await?;
        Ok(())
    }
}
