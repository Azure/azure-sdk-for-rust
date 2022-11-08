use async_trait::async_trait;
use fe2o3_amqp::{
    link::{delivery::DeliveryInfo, DetachError, RecvError},
    Delivery,
};
use fe2o3_amqp_types::{definitions::SequenceNo, messaging::Body, primitives::OrderedMap};
use serde_amqp::Value;
use std::time::Duration as StdDuration;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    core::TransportReceiver,
    primitives::{
        service_bus_received_message::ServiceBusReceivedMessage,
        service_bus_retry_policy::{
            run_operation, RetryError, ServiceBusRetryPolicy, ServiceBusRetryPolicyError,
            ServiceBusRetryPolicyState,
        },
    },
    receiver::error::ServiceBusRecvError,
    ServiceBusReceiveMode,
};

use super::amqp_message_converter;

pub struct AmqpReceiver<RP: ServiceBusRetryPolicy> {
    pub(crate) identifier: u32,
    pub(crate) prefetch_count: u32,
    pub(crate) retry_policy: RP,
    pub(crate) receiver: fe2o3_amqp::Receiver,
    pub(crate) receive_mode: ServiceBusReceiveMode,
    pub(crate) is_processor: bool,
}

impl<RP> AmqpReceiver<RP>
where
    RP: ServiceBusRetryPolicy,
{
    async fn receive_messages_inner(
        &mut self,
        buffer: &mut Vec<ServiceBusReceivedMessage>,
        max_messages: u32,
    ) -> Result<(), ServiceBusRecvError> {
        // Need to set credit
        if self.prefetch_count == 0 {
            // credit mode is manual
            self.receiver.set_credit(max_messages).await?;
        }

        for _ in 0..max_messages {
            let delivery: Delivery<Body<Value>> = self.receiver.recv().await?;

            let mut is_settled = false;
            if self.receive_mode == ServiceBusReceiveMode::ReceiveAndDelete {
                self.receiver
                    .accept(&delivery)
                    .await
                    .map_err(RecvError::from)?;
                is_settled = true;
            }

            let message = amqp_message_converter::amqp_delivery_as_service_bus_received_message(
                delivery, is_settled,
            )?;

            buffer.push(message);
        }
        Ok(())
    }
}

#[async_trait]
impl<RP> TransportReceiver for AmqpReceiver<RP>
where
    RP: ServiceBusRetryPolicy + Send,
{
    type Error = ();
    type ReceiveError = ServiceBusRecvError;
    type CompleteError = RetryError<RP::Error>;
    type CloseError = DetachError;

    /// <summary>
    /// Indicates whether the session link has been closed. This is useful for session receiver scenarios because once the link is closed for a
    /// session receiver it will not be reopened.
    /// </summary>
    fn is_session_link_closed(&self) -> bool {
        todo!()
    }

    /// <summary>
    ///
    /// </summary>
    fn session_id(&self) -> Option<&str> {
        todo!()
    }

    /// <summary>
    /// The Session Id associated with the receiver.
    /// </summary>
    fn session_locked_until(&self) -> Option<OffsetDateTime> {
        todo!()
    }

    /// <summary>
    /// Receives a set of <see cref="ServiceBusReceivedMessage" /> from the entity using <see cref="ServiceBusReceiveMode"/> mode.
    /// </summary>
    /// <param name="maximumMessageCount">The maximum number of messages that will be received.</param>
    /// <param name="maxWaitTime">An optional <see cref="TimeSpan"/> specifying the maximum time to wait for the first message before returning an empty list if no messages have been received.
    ///     If not specified, the <see cref="ServiceBusRetryOptions.TryTimeout"/> will be used.</param>
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    /// <returns>List of messages received. Returns an empty list if no message is found.</returns>
    async fn receive_messages(
        &mut self,
        max_messages: u32,
        max_wait_time: Option<StdDuration>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, Self::ReceiveError> {
        let mut message_buffer: Vec<ServiceBusReceivedMessage> =
            Vec::with_capacity(max_messages as usize);
        let max_wait_time =
            max_wait_time.unwrap_or_else(|| self.retry_policy.options().try_timeout);

        tokio::select! {
            _ = tokio::time::sleep(max_wait_time) => {
                if self.prefetch_count == 0 { // credit mode is manual
                    if let Err(err) = self.receiver.drain().await {
                        log::error!("{}", err);
                    }
                }
                Ok(message_buffer)
            }
            result = self.receive_messages_inner(&mut message_buffer, max_messages) => {
                result?;
                Ok(message_buffer)
            }
        }
    }

    /// <summary>
    /// Closes the connection to the transport consumer instance.
    /// </summary>
    ///
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    async fn close(self) -> Result<(), Self::CloseError> {
        self.receiver.close().await
    }

    /// <summary>
    /// Completes a <see cref="ServiceBusReceivedMessage"/>. This will delete the message from the service.
    /// </summary>
    ///
    /// <param name="lockToken">The lockToken of the <see cref="ServiceBusReceivedMessage"/> to complete.</param>
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    ///
    /// <remarks>
    /// This operation can only be performed on a message that was received by this receiver
    /// when <see cref="ServiceBusReceiveMode"/> is set to <see cref="ServiceBusReceiveMode.PeekLock"/>.
    /// </remarks>
    ///
    /// <returns>A task to be resolved on when the operation has completed.</returns>
    async fn complete(&mut self, delivery_info: DeliveryInfo) -> Result<(), Self::CompleteError> {
        let receiver = &mut self.receiver;
        let policy = &mut self.retry_policy;
        run_operation!(
            policy,
            RP,
            complete_message::<RP::Error>(receiver, delivery_info.clone()).await
        )
    }

    /// <summary> Indicates that the receiver wants to defer the processing for the message.</summary>
    ///
    /// <param name="lockToken">The lockToken of the <see cref="ServiceBusReceivedMessage"/> to defer.</param>
    /// <param name="propertiesToModify">The properties of the message to modify while deferring the message.</param>
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    ///
    /// <remarks>
    /// A lock token can be found in <see cref="ServiceBusReceivedMessage.LockTokenGuid"/>,
    /// only when <see cref="ServiceBusReceiveMode"/> is set to <see cref="ServiceBusReceiveMode.PeekLock"/>.
    /// In order to receive this message again in the future, you will need to save the <see cref="ServiceBusReceivedMessage.SequenceNumber"/>
    /// and receive it using ReceiveDeferredMessageBatchAsync(IEnumerable, CancellationToken).
    /// Deferring messages does not impact message's expiration, meaning that deferred messages can still expire.
    /// This operation can only be performed on messages that were received by this receiver.
    /// </remarks>
    ///
    /// <returns>A task to be resolved on when the operation has completed.</returns>
    async fn defer(
        &mut self,
        _lock_token: impl AsRef<Uuid> + Send,
        _properties_to_modify: Option<OrderedMap<String, String>>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    /// <summary>
    /// Fetches the next batch of active messages without changing the state of the receiver or the message source.
    /// </summary>
    ///
    /// <param name="sequenceNumber">The sequence number from where to read the message.</param>
    /// <param name="messageCount">The maximum number of messages that will be fetched.</param>
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    ///
    /// <remarks>
    /// The first call to PeekBatchBySequenceAsync(long, int, CancellationToken) fetches the first active message for this receiver. Each subsequent call
    /// fetches the subsequent message in the entity.
    /// Unlike a received message, peeked message will not have lock token associated with it, and hence it cannot be Completed/Abandoned/Deferred/Deadlettered/Renewed.
    /// Also, unlike <see cref="ReceiveMessagesAsync"/>, this method will fetch even Deferred messages (but not Deadlettered messages).
    /// </remarks>
    /// <returns></returns>
    async fn peek_message(
        &mut self,
        _sequence_number: Option<u64>,
        _message_count: u32,
    ) -> Result<ServiceBusReceivedMessage, Self::Error> {
        todo!()
    }

    /// <summary>
    /// Abandons a <see cref="ServiceBusReceivedMessage"/>. This will make the message available again for processing.
    /// </summary>
    ///
    /// <param name="lockToken">The lock token of the <see cref="ServiceBusReceivedMessage"/> to abandon.</param>
    /// <param name="propertiesToModify">The properties of the message to modify while abandoning the message.</param>
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    ///
    /// <remarks>
    /// Abandoning a message will increase the delivery count on the message.
    /// This operation can only be performed on messages that were received by this receiver
    /// when <see cref="ServiceBusReceiveMode"/> is set to <see cref="ServiceBusReceiveMode.PeekLock"/>.
    /// </remarks>
    ///
    /// <returns>A task to be resolved on when the operation has completed.</returns>
    async fn abandon(
        &mut self,
        _lock_token: impl AsRef<Uuid> + Send,
        _properties_to_modify: Option<OrderedMap<String, String>>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    /// <summary>
    /// Moves a message to the dead-letter subqueue.
    /// </summary>
    ///
    /// <param name="lockToken">The lock token of the <see cref="ServiceBusReceivedMessage"/> to dead-letter.</param>
    /// <param name="deadLetterReason">The reason for dead-lettering the message.</param>
    /// <param name="deadLetterErrorDescription">The error description for dead-lettering the message.</param>
    /// <param name="propertiesToModify">The properties of the message to modify while moving to subqueue.</param>
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    ///
    /// <remarks>
    /// In order to receive a message from the dead-letter queue, you will need a new
    /// <see cref="ServiceBusReceiver"/> with the corresponding path.
    /// You can use EntityNameHelper.FormatDeadLetterPath(string)"/> to help with this.
    /// This operation can only be performed on messages that were received by this receiver
    /// when <see cref="ServiceBusReceiveMode"/> is set to <see cref="ServiceBusReceiveMode.PeekLock"/>.
    /// </remarks>
    ///
    /// <returns>A task to be resolved on when the operation has completed.</returns>
    async fn dead_letter(
        &mut self,
        _lock_token: impl AsRef<Uuid> + Send,
        _dead_letter_reason: Option<String>,
        _dead_letter_error_description: Option<String>,
        _properties_to_modify: Option<OrderedMap<String, String>>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    /// <summary>
    /// Receives a <see cref="IList{Message}"/> of deferred messages identified by <paramref name="sequenceNumbers"/>.
    /// </summary>
    /// <param name="sequenceNumbers">A <see cref="IList{SequenceNumber}"/> containing the sequence numbers to receive.</param>
    /// <param name="cancellationToken"></param>
    /// <returns>Messages identified by sequence number are returned.
    /// Throws if the messages have not been deferred.</returns>
    /// <seealso cref="DeferAsync"/>
    async fn receive_deferred_messages(
        &mut self,
        _sequence_numbers: impl Iterator<Item = SequenceNo> + Send,
    ) -> Result<Vec<ServiceBusReceivedMessage>, Self::Error> {
        todo!()
    }

    /// <summary>
    /// Renews the lock on the message. The lock will be renewed based on the setting specified on the queue.
    /// </summary>
    /// <returns>New lock token expiry date and time in UTC format.</returns>
    ///
    /// <param name="lockToken">Lock token associated with the message.</param>
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    async fn renew_message_lock(
        &mut self,
        _lock_token: impl AsRef<Uuid> + Send,
    ) -> Result<OffsetDateTime, Self::Error> {
        todo!()
    }

    /// <summary>
    /// Renews the lock on the session specified by the <see cref="SessionId"/>. The lock will be renewed based on the setting specified on the entity.
    /// </summary>
    ///
    /// <returns>New lock token expiry date and time in UTC format.</returns>
    ///
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    async fn renew_session_lock(&mut self) -> Result<OffsetDateTime, Self::Error> {
        todo!()
    }

    /// <summary>
    /// Gets the session state.
    /// </summary>
    ///
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    ///
    /// <returns>The session state as <see cref="BinaryData"/>.</returns>
    async fn get_session_state(&mut self) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }

    /// <summary>
    /// Set a custom state on the session which can be later retrieved using <see cref="GetStateAsync"/>
    /// </summary>
    ///
    /// <param name="sessionState">A <see cref="BinaryData"/> of session state</param>
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    ///
    /// <remarks>This state is stored on Service Bus forever unless you set an empty state on it.</remarks>
    ///
    /// <returns>A task to be resolved on when the operation has completed.</returns>
    // public abstract Task SetStateAsync(
    //     BinaryData sessionState,
    //     CancellationToken cancellationToken);
    async fn set_session_state(
        &mut self,
        _session_state: impl AsRef<u8> + Send,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

async fn complete_message<E: ServiceBusRetryPolicyError>(
    receiver: &mut fe2o3_amqp::Receiver,
    delivery_info: DeliveryInfo,
) -> Result<(), E> {
    receiver.accept(delivery_info).await?;
    Ok(())
}
