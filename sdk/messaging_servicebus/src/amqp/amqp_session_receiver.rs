use async_trait::async_trait;
use fe2o3_amqp_management::client::MgmtClient;
use fe2o3_amqp_types::{
    definitions::Fields,
    primitives::{Binary, OrderedMap, Timestamp},
};
use serde_amqp::Value;
use std::time::Duration as StdDuration;
use time::OffsetDateTime;

use crate::{
    core::{TransportReceiver, TransportSessionReceiver},
    primitives::{
        service_bus_peeked_message::ServiceBusPeekedMessage,
        service_bus_received_message::ServiceBusReceivedMessage,
        service_bus_retry_policy::{run_operation, ServiceBusRetryPolicy},
    }, ServiceBusReceiveMode,
};

use super::{
    amqp_client_constants::LOCKED_UNTIL_UTC,
    amqp_receiver::AmqpReceiver,
    amqp_request_message::{
        get_session_state::GetSessionStateRequest, renew_session_lock::RenewSessionLockRequest,
        set_session_state::SetSessionStateRequest,
    },
    amqp_response_message::{
        get_session_state::GetSessionStateResponse, renew_session_lock::RenewSessionLockResponse,
        set_session_state::SetSessionStateResponse,
    },
    error::AmqpRequestResponseError,
};

pub struct AmqpSessionReceiver<R: ServiceBusRetryPolicy> {
    pub(crate) inner: AmqpReceiver<R>,
}

#[async_trait]
impl<RP> TransportReceiver for AmqpSessionReceiver<RP>
where
    RP: ServiceBusRetryPolicy + Send + Sync,
{
    type RequestResponseError = <AmqpReceiver<RP> as TransportReceiver>::RequestResponseError;
    type ReceiveError = <AmqpReceiver<RP> as TransportReceiver>::ReceiveError;
    type DispositionError = <AmqpReceiver<RP> as TransportReceiver>::DispositionError;
    type CloseError = <AmqpReceiver<RP> as TransportReceiver>::CloseError;

    fn prefetch_count(&self) -> u32 {
        self.inner.prefetch_count()
    }

    fn receive_mode(&self) -> ServiceBusReceiveMode {
        self.inner.receive_mode()
    }

    async fn receive_messages(
        &mut self,
        max_messages: u32,
        max_wait_time: Option<StdDuration>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, Self::ReceiveError> {
        self.inner
            .receive_messages(max_messages, max_wait_time)
            .await
    }

    async fn close(self) -> Result<(), Self::CloseError> {
        self.inner.close().await
    }

    async fn complete(
        &mut self,
        message: &ServiceBusReceivedMessage,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        self.inner.complete(message, session_id).await
    }

    async fn defer(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        self.inner
            .defer(message, properties_to_modify, session_id)
            .await
    }

    async fn peek_message(
        &mut self,
        sequence_number: Option<i64>,
        message_count: i32,
    ) -> Result<Vec<ServiceBusPeekedMessage>, Self::RequestResponseError> {
        self.inner
            .peek_message(sequence_number, message_count)
            .await
    }

    async fn peek_session_message(
        &mut self,
        sequence_number: Option<i64>,
        message_count: i32,
        session_id: &str,
    ) -> Result<Vec<ServiceBusPeekedMessage>, Self::RequestResponseError> {
        self.inner
            .peek_session_message(sequence_number, message_count, session_id)
            .await
    }

    async fn abandon(
        &mut self,
        message: &ServiceBusReceivedMessage,
        properties_to_modify: Option<OrderedMap<String, Value>>,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        self.inner
            .abandon(message, properties_to_modify, session_id)
            .await
    }

    async fn dead_letter(
        &mut self,
        message: &ServiceBusReceivedMessage,
        dead_letter_reason: Option<String>,
        dead_letter_error_description: Option<String>,
        properties_to_modify: Option<OrderedMap<String, Value>>,
        session_id: Option<&str>,
    ) -> Result<(), Self::DispositionError> {
        self.inner
            .dead_letter(
                message,
                dead_letter_reason,
                dead_letter_error_description,
                properties_to_modify,
                session_id,
            )
            .await
    }

    async fn receive_deferred_messages(
        &mut self,
        sequence_numbers: impl Iterator<Item = i64> + Send,
        session_id: Option<&str>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, Self::RequestResponseError> {
        self.inner
            .receive_deferred_messages(sequence_numbers, session_id)
            .await
    }

    async fn renew_message_lock(
        &mut self,
        lock_token: Vec<fe2o3_amqp::types::primitives::Uuid>,
    ) -> Result<Vec<Timestamp>, Self::RequestResponseError> {
        self.inner.renew_message_lock(lock_token).await
    }
}

#[async_trait]
impl<RP> TransportSessionReceiver for AmqpSessionReceiver<RP>
where
    RP: ServiceBusRetryPolicy + Send + Sync,
{
    /// <summary>
    /// Indicates whether the session link has been closed. This is useful for session receiver scenarios because once the link is closed for a
    /// session receiver it will not be reopened.
    /// </summary>
    fn is_session_link_closed(&self) -> bool {
        todo!()
    }

    /// <summary>
    /// The Session Id associated with the receiver.
    /// </summary>
    fn session_locked_until(&self) -> Option<OffsetDateTime> {
        self.inner.receiver.properties(get_session_locked_until)
    }

    fn set_session_locked_until(&mut self, session_locked_until: OffsetDateTime) {
        let timestamp = Timestamp::from(session_locked_until);
        let op = move |properties: &mut Option<Fields>| {
            properties
                .get_or_insert(Fields::default())
                .insert(LOCKED_UNTIL_UTC.into(), Value::Timestamp(timestamp));
        };
        self.inner.receiver.properties_mut(op);
    }

    /// <summary>
    /// Renews the lock on the session specified by the <see cref="SessionId"/>. The lock will be renewed based on the setting specified on the entity.
    /// </summary>
    ///
    /// <returns>New lock token expiry date and time in UTC format.</returns>
    ///
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    async fn renew_session_lock(
        &mut self,
        session_id: &str,
    ) -> Result<OffsetDateTime, Self::RequestResponseError> {
        let mut request =
            RenewSessionLockRequest::new(session_id, Some(self.inner.receiver.name()));
        let mgmt_client = &mut self.inner.management_client;
        let policy = &self.inner.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        let response = run_operation!(
            policy,
            RP,
            AmqpRequestResponseError,
            try_timeout,
            renew_session_lock(mgmt_client, &mut request, &try_timeout).await
        )?;

        Ok(OffsetDateTime::from(response.expiration))
    }

    /// <summary>
    /// Gets the session state.
    /// </summary>
    ///
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    ///
    /// <returns>The session state as <see cref="BinaryData"/>.</returns>
    async fn session_state(
        &mut self,
        session_id: &str,
    ) -> Result<Vec<u8>, Self::RequestResponseError> {
        let mut request = GetSessionStateRequest::new(session_id, Some(self.inner.receiver.name()));
        let mgmt_client = &mut self.inner.management_client;
        let policy = &self.inner.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        let response = run_operation!(
            policy,
            RP,
            AmqpRequestResponseError,
            try_timeout,
            get_session_state(mgmt_client, &mut request, &try_timeout).await
        )?;

        Ok(response.session_state.into_vec())
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
        session_id: &str,
        session_state: Vec<u8>,
    ) -> Result<(), Self::RequestResponseError> {
        let mut request = SetSessionStateRequest::new(
            session_id,
            Binary::from(session_state),
            Some(self.inner.receiver.name()),
        );
        let mgmt_client = &mut self.inner.management_client;
        let policy = &self.inner.retry_policy;
        let mut try_timeout = policy.calculate_try_timeout(0);

        let _response = run_operation!(
            policy,
            RP,
            AmqpRequestResponseError,
            try_timeout,
            set_session_state(mgmt_client, &mut request, &try_timeout).await
        )?;
        Ok(())
    }
}

pub(super) fn get_session_locked_until(properties: &Option<Fields>) -> Option<OffsetDateTime> {
    // SessionLockedUntil = link.Settings.Properties.TryGetValue<long>(
    //     AmqpClientConstants.LockedUntilUtc, out var lockedUntilUtcTicks)
    //     ? new DateTime(lockedUntilUtcTicks, DateTimeKind.Utc)
    //     : DateTime.MinValue;
    properties
        .as_ref()
        .and_then(|map| map.get(LOCKED_UNTIL_UTC))
        .and_then(|value| match value {
            Value::Timestamp(timestamp) => Some(timestamp.clone()),
            _ => None, // TODO: what if it's not a timestamp?
        })
        .map(OffsetDateTime::from)
    // .unwrap_or(DEFAULT_OFFSET_DATE_TIME)
}

async fn renew_session_lock<'a>(
    mgmt_client: &mut MgmtClient,
    request: &mut RenewSessionLockRequest<'a>,
    try_timeout: &StdDuration,
) -> Result<RenewSessionLockResponse, AmqpRequestResponseError> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response)
}

async fn set_session_state<'a>(
    mgmt_client: &mut MgmtClient,
    request: &mut SetSessionStateRequest<'a>,
    try_timeout: &StdDuration,
) -> Result<SetSessionStateResponse, AmqpRequestResponseError> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response)
}

async fn get_session_state<'a>(
    mgmt_client: &mut MgmtClient,
    request: &mut GetSessionStateRequest<'a>,
    try_timeout: &StdDuration,
) -> Result<GetSessionStateResponse, AmqpRequestResponseError> {
    let server_timeout = try_timeout.as_millis() as u32;
    request.set_server_timeout(Some(server_timeout));

    let response = mgmt_client.call(request).await?;
    Ok(response)
}
