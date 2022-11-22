use async_trait::async_trait;
use fe2o3_amqp::{
    link::{delivery::DeliveryInfo, DetachError, RecvError},
    Delivery, Receiver,
};
use fe2o3_amqp_management::client::MgmtClient;
use fe2o3_amqp_types::{
    definitions::{ErrorCondition, Fields, ReceiverSettleMode, SequenceNo},
    messaging::{Body, Modified},
    primitives::{Array, OrderedMap, Symbol, Uuid},
};
use serde_amqp::Value;
use std::{collections::HashSet, time::Duration as StdDuration};
use time::OffsetDateTime;

use crate::{
    core::TransportReceiver,
    primitives::{
        service_bus_received_message::{ReceivedMessageLockToken, ServiceBusReceivedMessage},
        service_bus_retry_policy::{
            run_operation, RetryError, ServiceBusRetryPolicy, ServiceBusRetryPolicyState,
        },
    },
    ServiceBusReceiveMode,
};

use super::{
    amqp_client_constants::DEAD_LETTER_NAME,
    amqp_message_constants::{DEAD_LETTER_ERROR_DESCRIPTION_HEADER, DEAD_LETTER_REASON_HEADER},
    amqp_request_message::{
        receive_by_sequence_number::ReceiveBySequenceNumberRequest,
        update_disposition::{DispositionStatus, UpdateDispositionRequest},
    },
    amqp_response_message::receive_by_sequence_number::ReceiveBySequenceNumberResponse,
    error::{AmqpDispositionError, AmqpRecvError, AmqpRequestResponseError},
};

pub struct AmqpReceiver<RP: ServiceBusRetryPolicy> {
    pub(crate) identifier: u32,
    pub(crate) prefetch_count: u32,
    pub(crate) retry_policy: RP,
    pub(crate) receiver: fe2o3_amqp::Receiver,
    pub(crate) receive_mode: ServiceBusReceiveMode,
    pub(crate) is_processor: bool,
    pub(crate) management_client: MgmtClient,
    pub(crate) request_response_locked_messages: HashSet<fe2o3_amqp::types::primitives::Uuid>,
}

impl<RP> AmqpReceiver<RP> where RP: ServiceBusRetryPolicy {}

#[async_trait]
impl<RP> TransportReceiver for AmqpReceiver<RP>
where
    RP: ServiceBusRetryPolicy + Send + Sync,
{
    type RequestResponseError = RetryError<AmqpRequestResponseError>;
    type ReceiveError = RetryError<AmqpRecvError>;
    type DispositionError = RetryError<AmqpDispositionError>;
    type CloseError = DetachError;

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
        let max_wait_time =
            max_wait_time.unwrap_or_else(|| self.retry_policy.options().try_timeout);
        let receiver = &mut self.receiver;
        let prefetch_count = self.prefetch_count;
        let receive_mode = &self.receive_mode;
        let retry_policy = &self.retry_policy;
        let mut try_timeout = retry_policy.calculate_try_timeout(0);
        run_operation!(
            retry_policy,
            RP,
            AmqpRecvError,
            try_timeout,
            receive_messages_with_timeout(
                receiver,
                prefetch_count,
                receive_mode,
                max_messages,
                max_wait_time
            )
            .await
        )
    }

    /// <summary>
    /// Closes the connection to the transport consumer instance.
    /// </summary>
    ///
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    async fn close(mut self) -> Result<(), Self::CloseError> {
        self.receiver.drain().await?; // This is only mentioned in an issue but not implemented in the dotnet sdk yet
        self.receiver.close().await?;
        self.management_client.close().await?;
        Ok(())
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
    async fn complete(
        &mut self,
        message: &ServiceBusReceivedMessage,
    ) -> Result<(), Self::DispositionError> {
        let receiver = &mut self.receiver;
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        match &message.lock_token {
            ReceivedMessageLockToken::LockToken(lock_token) => {
                if self.request_response_locked_messages.remove(&lock_token) {
                    let mgmt_client = &mut self.management_client;
                    run_operation!(
                        policy,
                        RP,
                        AmqpDispositionError,
                        try_timeout,
                        complete_message_via_management_client(
                            mgmt_client,
                            lock_token,
                            &try_timeout
                        )
                        .await
                    )?;
                }
            }
            ReceivedMessageLockToken::Delivery(delivery_info) => run_operation!(
                policy,
                RP,
                AmqpDispositionError,
                try_timeout,
                complete_message(receiver, delivery_info).await
            )?,
        };

        Ok(())
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
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), Self::DispositionError> {
        let receiver = &mut self.receiver;
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);
        run_operation!(
            policy,
            RP,
            AmqpDispositionError,
            try_timeout,
            defer_message(receiver, message, &properties_to_modify).await
        )
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
    ) -> Result<ServiceBusReceivedMessage, Self::RequestResponseError> {
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
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), Self::DispositionError> {
        let receiver = &mut self.receiver;
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);
        run_operation!(
            policy,
            RP,
            AmqpDispositionError,
            try_timeout,
            abandon_message(receiver, message, &properties_to_modify).await
        )
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
        message: &ServiceBusReceivedMessage,
        dead_letter_reason: Option<String>,
        dead_letter_error_description: Option<String>,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Result<(), Self::DispositionError> {
        let receiver = &mut self.receiver;
        let policy = &self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);
        run_operation!(
            policy,
            RP,
            AmqpDispositionError,
            try_timeout,
            dead_letter_message(
                receiver,
                message,
                &dead_letter_reason,
                &dead_letter_error_description,
                &properties_to_modify
            )
            .await
        )
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
        sequence_numbers: impl Iterator<Item = i64> + Send,
    ) -> Result<Vec<ServiceBusReceivedMessage>, Self::RequestResponseError> {
        let sequence_numbers: Array<i64> = sequence_numbers.collect();
        let receiver_settle_mode = match self.receive_mode {
            ServiceBusReceiveMode::PeekLock => ReceiverSettleMode::Second,
            ServiceBusReceiveMode::ReceiveAndDelete => ReceiverSettleMode::First,
        };
        let mut request =
            ReceiveBySequenceNumberRequest::new(sequence_numbers, receiver_settle_mode);

        let mgmt_client = &mut self.management_client;
        let policy = &self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        let response = run_operation!(
            policy,
            RP,
            AmqpRequestResponseError,
            try_timeout,
            receive_by_sequence_number(mgmt_client, &mut request, &try_timeout).await
        )?;

        response
            .into_received_messages()
            .map_err(AmqpRequestResponseError::from)
            .map_err(RetryError::Operation)
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
    ) -> Result<OffsetDateTime, Self::RequestResponseError> {
        todo!()
    }
}

async fn receive_messages(
    receiver: &mut Receiver,
    prefetch_count: u32,
    receive_mode: &ServiceBusReceiveMode,
    buffer: &mut Vec<ServiceBusReceivedMessage>,
    max_messages: u32,
) -> Result<(), AmqpRecvError> {
    // Credit mode is manual, need to set credit
    if prefetch_count == 0 {
        receiver.set_credit(max_messages).await?;
    }

    for _ in 0..max_messages {
        let delivery: Delivery<Body<Value>> = receiver.recv().await?;

        let mut is_settled = false;
        if *receive_mode == ServiceBusReceiveMode::ReceiveAndDelete {
            receiver.accept(&delivery).await.map_err(RecvError::from)?;
            is_settled = true;
        }

        let (delivery_info, raw_amqp_message) = delivery.into_parts();
        let lock_token = ReceivedMessageLockToken::Delivery(delivery_info);
        let message = ServiceBusReceivedMessage {
            is_settled,
            raw_amqp_message,
            lock_token,
        };

        buffer.push(message);
    }
    Ok(())
}

async fn receive_messages_with_timeout(
    receiver: &mut Receiver,
    prefetch_count: u32,
    receive_mode: &ServiceBusReceiveMode,
    max_messages: u32,
    max_wait_time: StdDuration,
) -> Result<Vec<ServiceBusReceivedMessage>, AmqpRecvError> {
    let mut message_buffer: Vec<ServiceBusReceivedMessage> =
        Vec::with_capacity(max_messages as usize);

    tokio::select! {
        _ = tokio::time::sleep(max_wait_time) => {
            if prefetch_count == 0 { // credit mode is manual
                if let Err(err) = receiver.drain().await {
                    log::error!("{}", err);
                }
            }
            Ok(message_buffer)
        }
        result = receive_messages(receiver, prefetch_count, receive_mode, &mut message_buffer, max_messages) => {
            result?;
            Ok(message_buffer)
        }
    }
}

async fn complete_message(
    receiver: &mut fe2o3_amqp::Receiver,
    delivery_info: &DeliveryInfo,
) -> Result<(), AmqpDispositionError> {
    // TODO: avoid clone
    receiver.accept(delivery_info.clone()).await?;
    Ok(())
}

async fn complete_message_via_management_client(
    mgmt_client: &mut MgmtClient,
    lock_token: &Uuid,
    try_timeout: &StdDuration,
) -> Result<(), fe2o3_amqp_management::error::Error> {
    let server_timeout = try_timeout.as_millis() as u32;
    let mut request = UpdateDispositionRequest::new(
        DispositionStatus::Completed,
        Array(vec![lock_token.clone()]), // TODO: reduce clone
        None,
        None,
        None,
    );
    request.set_server_timeout(Some(server_timeout));
    let _response = mgmt_client.call(&request).await?;
    Ok(())
}

// TODO: reduce clone
fn map_properties_to_modify_into_fields(
    properties_to_modify: &OrderedMap<String, Value>,
) -> Fields {
    properties_to_modify
        .iter()
        .map(|(k, v)| (Symbol::from(k.as_str()), v.clone()))
        .collect()
}

async fn dead_letter_message(
    receiver: &mut fe2o3_amqp::Receiver,
    message: &ServiceBusReceivedMessage,
    dead_letter_reason: &Option<String>,
    dead_letter_error_description: &Option<String>,
    properties_to_modify: &Option<OrderedMap<String, Value>>,
) -> Result<(), AmqpDispositionError> {
    let mut error = None;
    if dead_letter_reason.is_some()
        || dead_letter_error_description.is_some()
        || properties_to_modify.is_some()
    {
        let condition = ErrorCondition::Custom(Symbol::from(DEAD_LETTER_NAME));
        let mut info = None;

        if let Some(reason) = dead_letter_reason {
            info.get_or_insert(Fields::default()).insert(
                DEAD_LETTER_REASON_HEADER.into(),
                Value::from(reason.as_str()),
            );
        }

        if let Some(description) = dead_letter_error_description {
            info.get_or_insert(Fields::default()).insert(
                DEAD_LETTER_ERROR_DESCRIPTION_HEADER.into(),
                Value::from(description.as_str()),
            );
        }

        if let Some(properties_to_modify) = properties_to_modify {
            for (k, v) in properties_to_modify {
                info.get_or_insert(Fields::default())
                    .insert(Symbol::from(k.as_str()), v.clone()); // TODO: reduce cloning
            }
        }

        error = Some(fe2o3_amqp::types::definitions::Error::new(
            condition, None, info,
        ))
    }

    match ReceivedMessageLockToken::from(message) {
        ReceivedMessageLockToken::LockToken(_) => todo!(),
        ReceivedMessageLockToken::Delivery(delivery_info) => {
            receiver.reject(delivery_info, error).await?
        }
    }
    Ok(())
}

async fn abandon_message(
    receiver: &mut Receiver,
    message: &ServiceBusReceivedMessage,
    properties_to_modify: &Option<OrderedMap<String, Value>>,
) -> Result<(), AmqpDispositionError> {
    let modified = Modified {
        delivery_failed: None,
        undeliverable_here: None,
        message_annotations: properties_to_modify
            .as_ref()
            .map(map_properties_to_modify_into_fields),
    };

    match ReceivedMessageLockToken::from(message) {
        ReceivedMessageLockToken::LockToken(_) => todo!(),
        ReceivedMessageLockToken::Delivery(delivery_info) => {
            receiver.modify(delivery_info, modified).await?
        }
    }

    Ok(())
}

async fn defer_message(
    receiver: &mut Receiver,
    message: &ServiceBusReceivedMessage,
    properties_to_modify: &Option<OrderedMap<String, Value>>,
) -> Result<(), AmqpDispositionError> {
    let modified = Modified {
        delivery_failed: None,
        undeliverable_here: Some(true),
        message_annotations: properties_to_modify
            .as_ref()
            .map(map_properties_to_modify_into_fields),
    };

    match ReceivedMessageLockToken::from(message) {
        ReceivedMessageLockToken::LockToken(_) => todo!(),
        ReceivedMessageLockToken::Delivery(delivery_info) => {
            receiver.modify(delivery_info, modified).await?
        }
    }

    Ok(())
}

async fn receive_by_sequence_number(
    mgmt_client: &mut MgmtClient,
    request: &mut ReceiveBySequenceNumberRequest,
    try_timeout: &StdDuration,
) -> Result<ReceiveBySequenceNumberResponse, AmqpRequestResponseError> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response)
}
