use async_trait::async_trait;
use fe2o3_amqp::{
    link::{delivery::DeliveryInfo, DetachError, RecvError},
    Delivery, Receiver,
};
use fe2o3_amqp_management::client::MgmtClient;
use fe2o3_amqp_types::{
    definitions::{ErrorCondition, Fields, ReceiverSettleMode},
    messaging::{annotations::AnnotationKey, Body, Modified},
    primitives::{Array, OrderedMap, Symbol, Timestamp, Uuid},
};
use serde_amqp::Value;
use std::{collections::HashSet, time::Duration as StdDuration};

use crate::{
    core::TransportReceiver,
    primitives::{
        disposition_status::DispositionStatus,
        service_bus_peeked_message::ServiceBusPeekedMessage,
        service_bus_received_message::{ReceivedMessageLockToken, ServiceBusReceivedMessage},
        service_bus_retry_policy::{
            run_operation, RetryError, ServiceBusRetryPolicy,
        },
    },
    ServiceBusReceiveMode,
};

use super::{
    amqp_client_constants::DEAD_LETTER_NAME,
    amqp_message_constants::{DEAD_LETTER_ERROR_DESCRIPTION_HEADER, DEAD_LETTER_REASON_HEADER},
    amqp_message_converter::LOCK_TOKEN_DELIVERY_ANNOTATION,
    amqp_request_message::{
        peek_message::PeekMessageRequest, peek_session_message::PeekSessionMessageRequest,
        receive_by_sequence_number::ReceiveBySequenceNumberRequest, renew_lock::RenewLockRequest, update_disposition::UpdateDispositionRequest,
    },
    amqp_response_message::{
        peek_message::PeekMessageResponse, peek_session_message::PeekSessionMessageResponse,
        receive_by_sequence_number::ReceiveBySequenceNumberResponse, renew_lock::RenewLockResponse,
    },
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
    pub(crate) last_peeked_sequence_number: i64,
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
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        match &message.lock_token {
            ReceivedMessageLockToken::LockToken(lock_token) => {
                if self.request_response_locked_messages.contains(&lock_token) {
                    let mgmt_client = &mut self.management_client;
                    let mut request = UpdateDispositionRequest::new(
                        DispositionStatus::Completed,
                        Array(vec![lock_token.clone()]), // TODO: reduce clone
                        None,
                        None,
                        None,
                        session_id,
                        Some(self.receiver.name()),
                    );
                    run_operation!(
                        policy,
                        RP,
                        AmqpDispositionError,
                        try_timeout,
                        update_disposition(mgmt_client, &mut request, &try_timeout).await
                    )?;

                    self.request_response_locked_messages.remove(&lock_token);
                }
            }
            ReceivedMessageLockToken::Delivery { delivery_info, .. } => {
                let receiver = &mut self.receiver;
                run_operation!(
                    policy,
                    RP,
                    AmqpDispositionError,
                    try_timeout,
                    complete_message(receiver, delivery_info).await
                )?;
            }
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
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        let receiver = &mut self.receiver;
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        match &message.lock_token {
            ReceivedMessageLockToken::LockToken(lock_token) => {
                if self.request_response_locked_messages.contains(&lock_token) {
                    let mgmt_client = &mut self.management_client;
                    let mut request = UpdateDispositionRequest::new(
                        DispositionStatus::Defered,
                        Array(vec![lock_token.clone()]), // TODO: reduce clone
                        None,
                        None,
                        properties_to_modify,
                        session_id,
                        Some(self.receiver.name()),
                    );
                    run_operation!(
                        policy,
                        RP,
                        AmqpDispositionError,
                        try_timeout,
                        update_disposition(mgmt_client, &mut request, &try_timeout).await
                    )?;

                    self.request_response_locked_messages.remove(&lock_token);
                }
            }
            ReceivedMessageLockToken::Delivery { delivery_info, .. } => {
                run_operation!(
                    policy,
                    RP,
                    AmqpDispositionError,
                    try_timeout,
                    defer_message(receiver, delivery_info, &properties_to_modify).await
                )?;
            }
        };
        Ok(())
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
        sequence_number: Option<i64>,
        message_count: i32,
    ) -> Result<Vec<ServiceBusPeekedMessage>, Self::RequestResponseError> {
        let mut request = PeekMessageRequest::new(
            sequence_number.unwrap_or(self.last_peeked_sequence_number + 1),
            message_count,
            Some(self.receiver.name()),
        );

        let mgmt_client = &mut self.management_client;
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        let response = run_operation!(
            policy,
            RP,
            AmqpRequestResponseError,
            try_timeout,
            peek_message(mgmt_client, &mut request, &try_timeout).await
        )?;

        let peeked_messages = response
            .into_peeked_messages()
            .map_err(AmqpRequestResponseError::from)
            .map_err(RetryError::Operation)?;

        if let Some(last) = peeked_messages.last() {
            self.last_peeked_sequence_number = last.sequence_number();
        }
        Ok(peeked_messages)
    }

    async fn peek_session_message(
        &mut self,
        sequence_number: Option<i64>,
        message_count: i32,
        session_id: &str,
    ) -> Result<Vec<ServiceBusPeekedMessage>, Self::RequestResponseError> {
        let mut request = PeekSessionMessageRequest::new(
            sequence_number.unwrap_or(self.last_peeked_sequence_number + 1),
            message_count,
            session_id,
            Some(self.receiver.name()),
        );

        let mgmt_client = &mut self.management_client;
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        let response = run_operation!(
            policy,
            RP,
            AmqpRequestResponseError,
            try_timeout,
            peek_session_message(mgmt_client, &mut request, &try_timeout).await
        )?;

        let peeked_messages = response
            .into_peeked_messages()
            .map_err(AmqpRequestResponseError::from)
            .map_err(RetryError::Operation)?;

        if let Some(last) = peeked_messages.last() {
            self.last_peeked_sequence_number = last.sequence_number();
        }
        Ok(peeked_messages)
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
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        let receiver = &mut self.receiver;
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        match &message.lock_token {
            ReceivedMessageLockToken::LockToken(lock_token) => {
                if self.request_response_locked_messages.contains(&lock_token) {
                    let mgmt_client = &mut self.management_client;
                    let mut request = UpdateDispositionRequest::new(
                        DispositionStatus::Abandoned,
                        Array(vec![lock_token.clone()]), // TODO: reduce clone
                        None,
                        None,
                        properties_to_modify,
                        session_id,
                        Some(self.receiver.name()),
                    );
                    run_operation!(
                        policy,
                        RP,
                        AmqpDispositionError,
                        try_timeout,
                        update_disposition(mgmt_client, &mut request, &try_timeout).await
                    )?;

                    self.request_response_locked_messages.remove(&lock_token);
                }
            }
            ReceivedMessageLockToken::Delivery { delivery_info, .. } => {
                run_operation!(
                    policy,
                    RP,
                    AmqpDispositionError,
                    try_timeout,
                    abandon_message(receiver, delivery_info, &properties_to_modify).await
                )?;
            }
        };
        Ok(())
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
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        let policy = &self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        match &message.lock_token {
            ReceivedMessageLockToken::LockToken(lock_token) => {
                if self.request_response_locked_messages.contains(lock_token) {
                    let mgmt_client = &mut self.management_client;
                    let mut request = UpdateDispositionRequest::new(
                        DispositionStatus::Suspended,
                        Array(vec![lock_token.clone()]),
                        dead_letter_reason,
                        dead_letter_error_description,
                        properties_to_modify,
                        session_id,
                        Some(self.receiver.name()),
                    );
                    run_operation!(
                        policy,
                        RP,
                        AmqpDispositionError,
                        try_timeout,
                        update_disposition(mgmt_client, &mut request, &try_timeout).await
                    )?;

                    self.request_response_locked_messages.remove(lock_token);
                }
            }
            ReceivedMessageLockToken::Delivery { delivery_info, .. } => {
                let receiver = &mut self.receiver;
                run_operation!(
                    policy,
                    RP,
                    AmqpDispositionError,
                    try_timeout,
                    dead_letter_message(
                        receiver,
                        delivery_info,
                        &dead_letter_reason,
                        &dead_letter_error_description,
                        &properties_to_modify
                    )
                    .await
                )?;
            }
        }

        Ok(())
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
        session_id: Option<&str>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, Self::RequestResponseError> {
        let sequence_numbers = sequence_numbers.collect();
        let receiver_settle_mode = match self.receive_mode {
            ServiceBusReceiveMode::PeekLock => ReceiverSettleMode::Second,
            ServiceBusReceiveMode::ReceiveAndDelete => ReceiverSettleMode::First,
        };
        let mut request = ReceiveBySequenceNumberRequest::new(
            sequence_numbers,
            receiver_settle_mode,
            session_id,
            Some(self.receiver.name()),
        );

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

        let received_messages = response
            .into_received_messages()
            .map_err(AmqpRequestResponseError::from)
            .map_err(RetryError::Operation)?;

        for message in &received_messages {
            if let ReceivedMessageLockToken::LockToken(lock_token) = &message.lock_token {
                self.request_response_locked_messages
                    .insert(lock_token.clone());
            }
        }

        Ok(received_messages)
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
        lock_tokens: Vec<Uuid>,
    ) -> Result<Vec<Timestamp>, Self::RequestResponseError> {
        let mut request = RenewLockRequest::new(Array(lock_tokens), Some(self.receiver.name()));
        let mgmt_client = &mut self.management_client;
        let policy = &self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        let response = run_operation!(
            policy,
            RP,
            AmqpRequestResponseError,
            try_timeout,
            renew_lock(mgmt_client, &mut request, &try_timeout).await
        )?;
        Ok(response.expirations.into_inner())
    }
}

fn lock_token_from_delivery<B>(delivery: &Delivery<B>) -> Option<Uuid> {
    match delivery
        .message()
        .delivery_annotations
        .as_ref()
        .and_then(|da| da.get(&LOCK_TOKEN_DELIVERY_ANNOTATION as &dyn AnnotationKey))
    {
        Some(Value::Uuid(uuid)) => Some(uuid.clone()),
        _ => {
            let delivery_tag = delivery.delivery_tag().as_ref();
            match Uuid::try_from(delivery_tag) {
                Ok(uuid) => Some(uuid),
                _ => None,
            }
        }
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

        let lock_token =
            lock_token_from_delivery(&delivery).ok_or(AmqpRecvError::LockTokenNotFound)?;
        let (delivery_info, raw_amqp_message) = delivery.into_parts();
        let lock_token = ReceivedMessageLockToken::Delivery {
            delivery_info,
            lock_token,
        };
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
    delivery_info: &DeliveryInfo,
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

    // TODO: avoid clone
    receiver.reject(delivery_info.clone(), error).await?;
    Ok(())
}

async fn abandon_message(
    receiver: &mut Receiver,
    delivery_info: &DeliveryInfo,
    properties_to_modify: &Option<OrderedMap<String, Value>>,
) -> Result<(), AmqpDispositionError> {
    let modified = Modified {
        delivery_failed: None,
        undeliverable_here: None,
        message_annotations: properties_to_modify
            .as_ref()
            .map(map_properties_to_modify_into_fields),
    };

    receiver.modify(delivery_info.clone(), modified).await?;
    Ok(())
}

async fn defer_message(
    receiver: &mut Receiver,
    delivery_info: &DeliveryInfo,
    properties_to_modify: &Option<OrderedMap<String, Value>>,
) -> Result<(), AmqpDispositionError> {
    let modified = Modified {
        delivery_failed: None,
        undeliverable_here: Some(true),
        message_annotations: properties_to_modify
            .as_ref()
            .map(map_properties_to_modify_into_fields),
    };

    receiver.modify(delivery_info.clone(), modified).await?;
    Ok(())
}

async fn receive_by_sequence_number<'a>(
    mgmt_client: &mut MgmtClient,
    request: &mut ReceiveBySequenceNumberRequest<'a>,
    try_timeout: &StdDuration,
) -> Result<ReceiveBySequenceNumberResponse, AmqpRequestResponseError> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response)
}

async fn update_disposition<'a>(
    mgmt_client: &mut MgmtClient,
    request: &mut UpdateDispositionRequest<'a>,
    try_timeout: &StdDuration,
) -> Result<(), fe2o3_amqp_management::error::Error> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));
    let _response = mgmt_client.call(request).await?;
    Ok(())
}

async fn peek_message<'a>(
    mgmt_client: &mut MgmtClient,
    request: &mut PeekMessageRequest<'a>,
    try_timeout: &StdDuration,
) -> Result<PeekMessageResponse, AmqpRequestResponseError> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response)
}

async fn peek_session_message<'a>(
    mgmt_client: &mut MgmtClient,
    request: &mut PeekSessionMessageRequest<'a>,
    try_timeout: &StdDuration,
) -> Result<PeekSessionMessageResponse, AmqpRequestResponseError> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response)
}

async fn renew_lock<'a>(
    mgmt_client: &mut MgmtClient,
    request: &mut RenewLockRequest<'a>,
    try_timeout: &StdDuration,
) -> Result<RenewLockResponse, AmqpRequestResponseError> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response)
}
