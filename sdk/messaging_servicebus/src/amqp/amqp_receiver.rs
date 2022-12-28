use async_trait::async_trait;
use fe2o3_amqp::{
    link::{delivery::DeliveryInfo, DetachError, ReceiverAttachExchange, RecvError},
    Delivery, Receiver,
};
use fe2o3_amqp_types::{
    definitions::{ErrorCondition, Fields, ReceiverSettleMode},
    messaging::{annotations::AnnotationKey, Body, Modified},
    primitives::{Array, OrderedMap, Symbol, Timestamp, Uuid},
};
use serde_amqp::Value;
use std::{collections::HashSet, sync::Arc, time::Duration as StdDuration};
use tokio::sync::{mpsc, Mutex};
use url::Url;

use crate::{
    authorization::service_bus_claim,
    core::{RecoverableTransport, TransportReceiver},
    primitives::{
        disposition_status::DispositionStatus,
        error::RetryError,
        service_bus_peeked_message::ServiceBusPeekedMessage,
        service_bus_received_message::{ReceivedMessageLockToken, ServiceBusReceivedMessage},
        service_bus_retry_policy::{run_operation, ServiceBusRetryPolicy},
    },
    sealed::Sealed,
    ServiceBusReceiveMode,
};

use super::{
    amqp_cbs_link,
    amqp_client_constants::DEAD_LETTER_NAME,
    amqp_connection_scope::AmqpConnectionScope,
    amqp_management_link::AmqpManagementLink,
    amqp_message_constants::{DEAD_LETTER_ERROR_DESCRIPTION_HEADER, DEAD_LETTER_REASON_HEADER},
    amqp_message_converter::LOCK_TOKEN_DELIVERY_ANNOTATION,
    amqp_request_message::{
        peek_message::PeekMessageRequest, peek_session_message::PeekSessionMessageRequest,
        receive_by_sequence_number::ReceiveBySequenceNumberRequest, renew_lock::RenewLockRequest,
        update_disposition::UpdateDispositionRequest,
    },
    amqp_response_message::{
        peek_message::PeekMessageResponse, peek_session_message::PeekSessionMessageResponse,
        receive_by_sequence_number::ReceiveBySequenceNumberResponse, renew_lock::RenewLockResponse,
    },
    error::{AmqpDispositionError, AmqpRecvError, AmqpRequestResponseError, RecoverReceiverError},
};

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
            _is_settled: is_settled,
            raw_amqp_message,
            lock_token,
        };

        buffer.push(message);
    }
    Ok(())
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

// TODO: reduce clone
fn map_properties_to_modify_into_fields(
    properties_to_modify: &OrderedMap<String, Value>,
) -> Fields {
    properties_to_modify
        .iter()
        .map(|(k, v)| (Symbol::from(k.as_str()), v.clone()))
        .collect()
}

/// An AMQP receiver implementation for Service Bus
#[derive(Debug)]
pub struct AmqpReceiver {
    pub(crate) id: u32, // TODO: should this info be preserved?
    pub(crate) service_endpoint: Arc<Url>,
    pub(crate) entity_path: String,
    pub(crate) identifier_str: String,

    pub(crate) prefetch_count: u32,
    pub(crate) retry_policy: Box<dyn ServiceBusRetryPolicy>,
    pub(crate) receiver: fe2o3_amqp::Receiver,
    pub(crate) receive_mode: ServiceBusReceiveMode,
    pub(crate) _is_processor: bool, // TODO: implement processor

    pub(crate) management_link: AmqpManagementLink,
    pub(crate) request_response_locked_messages: HashSet<fe2o3_amqp::types::primitives::Uuid>,
    pub(crate) last_peeked_sequence_number: i64,

    pub(crate) cbs_command_sender: mpsc::Sender<amqp_cbs_link::Command>,

    /// This is ONLY used for recovery
    pub(crate) connection_scope: Arc<Mutex<AmqpConnectionScope>>,
}

#[async_trait]
impl RecoverableTransport for AmqpReceiver {
    type RecoverError = RecoverReceiverError;

    async fn recover(&mut self) -> Result<(), Self::RecoverError> {
        let mut connection_scope = self.connection_scope.lock().await;
        connection_scope
            .recover()
            .await
            .map_err(|connection_scope_error| {
                log::error!(
                    "Failed to recover connection scope: {}",
                    connection_scope_error
                );
                Self::RecoverError::ConnectionScopeDisposed
            })?;

        // Auth with CBS
        let endpoint = format!("{}/{}", self.service_endpoint, self.entity_path);
        let resource = endpoint.clone();
        let required_claims = vec![service_bus_claim::SEND.to_string()];
        connection_scope
            .request_refreshable_authorization_using_cbs(
                self.id,
                endpoint,
                resource,
                required_claims,
            )
            .await?;

        // Resume the receiver on the new session
        let mut exchange = self
            .receiver
            .detach_then_resume_on_session(&connection_scope.session.handle)
            .await?;

        // `ReceiverAttachExchange::Complete` => Resume is complete
        //
        // `ReceiverAttachExchange::IncompleteUnsettled` => There are unsettled messages, multiple
        // detach and re-attach may happen in order to reduce the number of unsettled messages.
        //
        // `ReceiverAttachExchange::Resume` => There is one message that is partially transferred,
        // so it would be OK to let the user use the receiver to receive the message
        while let ReceiverAttachExchange::IncompleteUnsettled = exchange {
            match self.receiver.recv::<Body<Value>>().await {
                Ok(delivery) => {
                    let modified = Modified {
                        delivery_failed: None,
                        undeliverable_here: None,
                        message_annotations: None,
                    };
                    if let Err(err) = self.receiver.modify(delivery, modified).await {
                        log::error!("Failed to abandon message: {}", err);
                        exchange = self
                            .receiver
                            .detach_then_resume_on_session(&connection_scope.session.handle)
                            .await?;
                    }
                }
                Err(err) => {
                    log::error!("Failed to receive message while trying to settle (abandon) the unsettled: {}", err);
                    exchange = self
                        .receiver
                        .detach_then_resume_on_session(&connection_scope.session.handle)
                        .await?;
                }
            }
        }
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

impl AmqpReceiver {
    async fn receive_messages_with_timeout(
        &mut self,
        prefetch_count: u32,
        buffer: &mut Vec<ServiceBusReceivedMessage>,
        max_messages: u32,
        max_wait_time: StdDuration,
    ) -> Result<(), AmqpRecvError> {
        // let mut message_buffer: Vec<ServiceBusReceivedMessage> =
        //     Vec::with_capacity(max_messages as usize);

        tokio::select! {
            _ = tokio::time::sleep(max_wait_time) => {
                if prefetch_count == 0 { // credit mode is manual
                    if let Err(err) = self.receiver.drain().await {
                        log::error!("{}", err);
                    }
                }
                Ok(())
            }
            result = receive_messages(&mut self.receiver, prefetch_count, &self.receive_mode, buffer, max_messages) => {
                result?;
                Ok(())
            }
        }
    }

    async fn complete_message(
        &mut self,
        delivery_info: &DeliveryInfo,
    ) -> Result<(), AmqpDispositionError> {
        // TODO: avoid clone
        self.receiver.accept(delivery_info.clone()).await?;
        Ok(())
    }

    async fn dead_letter_message(
        &mut self,
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
        self.receiver.reject(delivery_info.clone(), error).await?;
        Ok(())
    }

    async fn abandon_message(
        &mut self,
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

        self.receiver
            .modify(delivery_info.clone(), modified)
            .await?;
        Ok(())
    }

    async fn defer_message(
        &mut self,
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

        self.receiver
            .modify(delivery_info.clone(), modified)
            .await?;
        Ok(())
    }

    async fn receive_by_sequence_number(
        &mut self,
        request: &mut ReceiveBySequenceNumberRequest,
        try_timeout: &StdDuration,
    ) -> Result<ReceiveBySequenceNumberResponse, AmqpRequestResponseError> {
        let server_timeout = try_timeout.as_millis() as u32;
        request.set_server_timeout(Some(server_timeout));

        let response = self.management_link.client_mut().call(request).await?;
        Ok(response)
    }

    async fn update_disposition(
        &mut self,
        request: &mut UpdateDispositionRequest,
        try_timeout: &StdDuration,
    ) -> Result<(), fe2o3_amqp_management::error::Error> {
        let server_timeout = try_timeout.as_millis() as u32;
        request.set_server_timeout(Some(server_timeout));
        let _response = self.management_link.client_mut().call(request).await?;
        Ok(())
    }

    async fn peek_message_inner(
        &mut self,
        request: &mut PeekMessageRequest,
        try_timeout: &StdDuration,
    ) -> Result<PeekMessageResponse, AmqpRequestResponseError> {
        let server_timeout = try_timeout.as_millis() as u32;
        request.set_server_timeout(Some(server_timeout));

        let response = self.management_link.client_mut().call(request).await?;
        Ok(response)
    }

    async fn peek_session_message_inner(
        &mut self,
        request: &mut PeekSessionMessageRequest,
        try_timeout: &StdDuration,
    ) -> Result<PeekSessionMessageResponse, AmqpRequestResponseError> {
        let server_timeout = try_timeout.as_millis() as u32;
        request.set_server_timeout(Some(server_timeout));

        let response = self.management_link.client_mut().call(request).await?;
        Ok(response)
    }

    async fn renew_lock(
        &mut self,
        request: &mut RenewLockRequest,
        try_timeout: &StdDuration,
    ) -> Result<RenewLockResponse, AmqpRequestResponseError> {
        let server_timeout = try_timeout.as_millis() as u32;
        request.set_server_timeout(Some(server_timeout));

        let response = self.management_link.client_mut().call(request).await?;
        Ok(response)
    }
}

impl Sealed for AmqpReceiver {}

#[async_trait]
impl TransportReceiver for AmqpReceiver {
    type RequestResponseError = RetryError<AmqpRequestResponseError>;
    type ReceiveError = RetryError<AmqpRecvError>;
    type DispositionError = RetryError<AmqpDispositionError>;
    type CloseError = DetachError;

    fn entity_path(&self) -> &str {
        &self.entity_path
    }

    fn identifier(&self) -> &str {
        &self.identifier_str
    }

    fn prefetch_count(&self) -> u32 {
        self.prefetch_count
    }

    fn receive_mode(&self) -> ServiceBusReceiveMode {
        self.receive_mode
    }

    async fn receive_messages(
        &mut self,
        max_messages: u32,
    ) -> Result<Vec<ServiceBusReceivedMessage>, Self::ReceiveError> {
        let prefetch_count = self.prefetch_count;
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        let default_max_wait_time = self.retry_policy.options().try_timeout;
        let mut buffer = Vec::with_capacity(max_messages as usize);
        loop {
            run_operation!(
                { &self.retry_policy },
                AmqpRecvError,
                try_timeout,
                self.receive_messages_with_timeout(
                    prefetch_count,
                    &mut buffer,
                    max_messages,
                    default_max_wait_time
                ),
                self.recover()
            )?;

            if !buffer.is_empty() {
                break;
            }
        }
        Ok(buffer)
    }

    /// Receives a set of [`ServiceBusReceivedMessage`] from the entity using [`ServiceBusReceiveMode`] mode.
    async fn receive_messages_with_max_wait_time(
        &mut self,
        max_messages: u32,
        max_wait_time: Option<StdDuration>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, Self::ReceiveError> {
        let max_wait_time =
            max_wait_time.unwrap_or_else(|| self.retry_policy.options().try_timeout);
        let prefetch_count = self.prefetch_count;
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);
        let mut buffer = Vec::with_capacity(max_messages as usize);
        run_operation!(
            { &self.retry_policy },
            AmqpRecvError,
            try_timeout,
            self.receive_messages_with_timeout(
                prefetch_count,
                &mut buffer,
                max_messages,
                max_wait_time
            ),
            self.recover()
        )?;
        Ok(buffer)
    }

    /// Closes the connection to the transport consumer instance.
    async fn close(mut self) -> Result<(), Self::CloseError> {
        let _ = self
            .cbs_command_sender
            .send(amqp_cbs_link::Command::RemoveAuthorizationRefresher(
                self.id,
            ))
            .await;
        self.receiver.drain().await?; // This is only mentioned in an issue but not implemented in the dotnet sdk yet
        self.receiver.close().await?;
        self.management_link.close().await?;
        Ok(())
    }

    /// Completes a [`ServiceBusReceivedMessage`]. This will delete the message from the service.
    async fn complete(
        &mut self,
        message: &ServiceBusReceivedMessage,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        match &message.lock_token {
            ReceivedMessageLockToken::LockToken(lock_token) => {
                if self.request_response_locked_messages.contains(lock_token) {
                    let mut request = UpdateDispositionRequest::new(
                        DispositionStatus::Completed,
                        Array(vec![lock_token.clone()]), // TODO: reduce clone
                        None,
                        None,
                        None,
                        session_id,
                        Some(self.receiver.name().to_string()),
                    );
                    run_operation!(
                        { &self.retry_policy },
                        AmqpDispositionError,
                        try_timeout,
                        self.update_disposition(&mut request, &try_timeout),
                        self.recover()
                    )?;

                    self.request_response_locked_messages.remove(lock_token);
                }
            }
            ReceivedMessageLockToken::Delivery { delivery_info, .. } => {
                run_operation!(
                    { &self.retry_policy },
                    AmqpDispositionError,
                    try_timeout,
                    self.complete_message(delivery_info),
                    self.recover()
                )?;
            }
        };

        Ok(())
    }

    /// Indicates that the receiver wants to defer the processing for the message.
    async fn defer(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        let policy = &mut self.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        match &message.lock_token {
            ReceivedMessageLockToken::LockToken(lock_token) => {
                if self.request_response_locked_messages.contains(lock_token) {
                    let mut request = UpdateDispositionRequest::new(
                        DispositionStatus::Defered,
                        Array(vec![lock_token.clone()]), // TODO: reduce clone
                        None,
                        None,
                        properties_to_modify,
                        session_id,
                        Some(self.receiver.name().to_string()),
                    );
                    run_operation!(
                        { &self.retry_policy },
                        AmqpDispositionError,
                        try_timeout,
                        self.update_disposition(&mut request, &try_timeout),
                        self.recover()
                    )?;

                    self.request_response_locked_messages.remove(lock_token);
                }
            }
            ReceivedMessageLockToken::Delivery { delivery_info, .. } => {
                run_operation!(
                    { &self.retry_policy },
                    AmqpDispositionError,
                    try_timeout,
                    self.defer_message(delivery_info, &properties_to_modify),
                    self.recover()
                )?;
            }
        };
        Ok(())
    }

    /// Fetches the next batch of active messages without changing the state of the receiver or the message source.
    async fn peek_messages(
        &mut self,
        sequence_number: Option<i64>,
        message_count: i32,
    ) -> Result<Vec<ServiceBusPeekedMessage>, Self::RequestResponseError> {
        let mut request = PeekMessageRequest::new(
            sequence_number.unwrap_or(self.last_peeked_sequence_number + 1),
            message_count,
            Some(self.receiver.name().to_string()),
        );
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        let response = run_operation!(
            { &self.retry_policy },
            AmqpRequestResponseError,
            try_timeout,
            self.peek_message_inner(&mut request, &try_timeout),
            self.recover()
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

    async fn peek_session_messages(
        &mut self,
        sequence_number: Option<i64>,
        message_count: i32,
        session_id: &str,
    ) -> Result<Vec<ServiceBusPeekedMessage>, Self::RequestResponseError> {
        let mut request = PeekSessionMessageRequest::new(
            sequence_number.unwrap_or(self.last_peeked_sequence_number + 1),
            message_count,
            session_id,
            Some(self.receiver.name().to_string()),
        );
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        let response = run_operation!(
            { &self.retry_policy },
            AmqpRequestResponseError,
            try_timeout,
            self.peek_session_message_inner(&mut request, &try_timeout),
            self.recover()
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

    /// Abandons a [`ServiceBusReceivedMessage`]. This will make the message available again for processing.
    async fn abandon(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        match &message.lock_token {
            ReceivedMessageLockToken::LockToken(lock_token) => {
                if self.request_response_locked_messages.contains(lock_token) {
                    let mut request = UpdateDispositionRequest::new(
                        DispositionStatus::Abandoned,
                        Array(vec![lock_token.clone()]), // TODO: reduce clone
                        None,
                        None,
                        properties_to_modify,
                        session_id,
                        Some(self.receiver.name().to_string()),
                    );
                    run_operation!(
                        { &self.retry_policy },
                        AmqpDispositionError,
                        try_timeout,
                        self.update_disposition(&mut request, &try_timeout),
                        self.recover()
                    )?;

                    self.request_response_locked_messages.remove(lock_token);
                }
            }
            ReceivedMessageLockToken::Delivery { delivery_info, .. } => {
                run_operation!(
                    { &self.retry_policy },
                    AmqpDispositionError,
                    try_timeout,
                    self.abandon_message(delivery_info, &properties_to_modify),
                    self.recover()
                )?;
            }
        };
        Ok(())
    }

    /// Moves a message to the dead-letter subqueue.
    async fn dead_letter(
        &mut self,
        message: &ServiceBusReceivedMessage,
        dead_letter_reason: Option<String>,
        dead_letter_error_description: Option<String>,
        properties_to_modify: Option<OrderedMap<String, Value>>,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        match &message.lock_token {
            ReceivedMessageLockToken::LockToken(lock_token) => {
                if self.request_response_locked_messages.contains(lock_token) {
                    let mut request = UpdateDispositionRequest::new(
                        DispositionStatus::Suspended,
                        Array(vec![lock_token.clone()]),
                        dead_letter_reason,
                        dead_letter_error_description,
                        properties_to_modify,
                        session_id,
                        Some(self.receiver.name().to_string()),
                    );
                    run_operation!(
                        { &self.retry_policy },
                        AmqpDispositionError,
                        try_timeout,
                        self.update_disposition(&mut request, &try_timeout),
                        self.recover()
                    )?;

                    self.request_response_locked_messages.remove(lock_token);
                }
            }
            ReceivedMessageLockToken::Delivery { delivery_info, .. } => {
                run_operation!(
                    { &self.retry_policy },
                    AmqpDispositionError,
                    try_timeout,
                    self.dead_letter_message(
                        delivery_info,
                        &dead_letter_reason,
                        &dead_letter_error_description,
                        &properties_to_modify
                    ),
                    self.recover()
                )?;
            }
        }

        Ok(())
    }

    /// Receives a [`Vec<ServiceBusReceivedMessages>`] of deferred messages identified by `sequence_numbers`.
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
            Some(self.receiver.name().to_string()),
        );

        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        let response = run_operation!(
            { &self.retry_policy },
            AmqpRequestResponseError,
            try_timeout,
            self.receive_by_sequence_number(&mut request, &try_timeout),
            self.recover()
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

    /// Renews the lock on the message. The lock will be renewed based on the setting specified on the queue.
    async fn renew_message_lock(
        &mut self,
        lock_tokens: Vec<Uuid>,
    ) -> Result<Vec<Timestamp>, Self::RequestResponseError> {
        let mut request =
            RenewLockRequest::new(Array(lock_tokens), Some(self.receiver.name().to_string()));
        let mut try_timeout = self.retry_policy.calculate_try_timeout(0);

        let response = run_operation!(
            { &self.retry_policy },
            AmqpRequestResponseError,
            try_timeout,
            self.renew_lock(&mut request, &try_timeout),
            self.recover()
        )?;
        Ok(response.expirations.into_inner())
    }
}
