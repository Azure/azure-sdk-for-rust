use async_trait::async_trait;
use fe2o3_amqp_management::client::MgmtClient;
use fe2o3_amqp_types::{
    definitions::Fields,
    primitives::{OrderedMap, Timestamp},
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
    },
};

use super::{
    amqp_receiver::AmqpReceiver, amqp_request_message::renew_session_lock::RenewSessionLockRequest,
    amqp_response_message::renew_session_lock::RenewSessionLockResponse,
    error::AmqpRequestResponseError,
};

pub struct AmqpSessionReceiver<R: ServiceBusRetryPolicy> {
    session_id: String,
    inner: AmqpReceiver<R>,
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

    fn set_sesesion_locked_until(&mut self, session_locked_until: Option<OffsetDateTime>) {
        todo!()
    }

    /// <summary>
    /// Renews the lock on the session specified by the <see cref="SessionId"/>. The lock will be renewed based on the setting specified on the entity.
    /// </summary>
    ///
    /// <returns>New lock token expiry date and time in UTC format.</returns>
    ///
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    async fn renew_session_lock(&mut self) -> Result<Timestamp, Self::RequestResponseError> {
        let mut request =
            RenewSessionLockRequest::new(Some(self.inner.receiver.name()), &self.session_id);
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

        Ok(response.expiration)
    }

    /// <summary>
    /// Gets the session state.
    /// </summary>
    ///
    /// <param name="cancellationToken">An optional <see cref="CancellationToken"/> instance to signal the request to cancel the operation.</param>
    ///
    /// <returns>The session state as <see cref="BinaryData"/>.</returns>
    async fn get_session_state(&mut self) -> Result<Vec<u8>, Self::RequestResponseError> {
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
        session_state: impl AsRef<u8> + Send,
    ) -> Result<(), Self::RequestResponseError> {
        todo!()
    }
}

pub(super) fn get_session_locked_until(properties: &Option<Fields>) -> Timestamp {
    todo!()
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
