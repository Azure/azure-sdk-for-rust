use std::time::Duration as StdDuration;

use async_trait::async_trait;
use azure_core::Url;

use crate::{
    authorization::service_bus_token_credential::ServiceBusTokenCredential,
    primitives::{
        service_bus_retry_options::ServiceBusRetryOptions,
        service_bus_transport_type::ServiceBusTransportType,
    },
    receiver::service_bus_receive_mode::ServiceBusReceiveMode,
    sealed::Sealed,
};

use super::{
    transport_receiver::TransportReceiver, transport_sender::TransportSender, TransportRuleManager,
    TransportSessionReceiver,
};

// Conditional import for docs.rs
#[cfg(docsrs)]
use crate::ServiceBusMessage;

/// Provides an abstraction for generalizing an Service Bus entity client so that a dedicated
/// instance may provide operations for a specific transport.
#[async_trait]
pub(crate) trait TransportClient: Sized + Sealed {
    /// Error with creating a client
    type CreateClientError: std::error::Error + Send;

    /// Error with creating a sender
    type CreateSenderError: std::error::Error + Send;

    /// Error with creating a receiver
    type CreateReceiverError: std::error::Error + Send;

    /// Error with creating a rule manager
    type CreateRuleManagerError: std::error::Error + Send;

    /// Error with closing a client
    type DisposeError: std::error::Error + Send;

    /// Sender type
    type Sender: TransportSender;

    /// Receiver type
    type Receiver: TransportReceiver;

    /// Session receiver type
    type SessionReceiver: TransportSessionReceiver;

    /// Rule manager type
    type RuleManager: TransportRuleManager;

    /// Creates a new instance of Self.
    async fn create_transport_client(
        host: &str,
        credential: ServiceBusTokenCredential,
        transport_type: ServiceBusTransportType,
        custom_endpoint: Option<Url>,
        retry_timeout: StdDuration,
    ) -> Result<Self, Self::CreateClientError>;

    /// Get the transport type
    fn transport_type(&self) -> ServiceBusTransportType;

    /// Indicates whether or not this client has been closed.
    ///
    /// Returnss `true` if the client is closed, otherwise `false`
    fn is_closed(&self) -> bool;

    /// The endpoint for the Service Bus service to which the client is associated.
    fn service_endpoint(&self) -> &Url;

    /// Creates a sender strongly aligned with the active protocol and transport,
    /// responsible for sending [`ServiceBusMessage`] to the entity.
    async fn create_sender(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_policy: ServiceBusRetryOptions,
    ) -> Result<Self::Sender, Self::CreateSenderError>;

    /// Creates a receiver
    async fn create_receiver(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_options: ServiceBusRetryOptions,
        receive_mode: ServiceBusReceiveMode,
        prefetch_count: u32,
    ) -> Result<Self::Receiver, Self::CreateReceiverError>;

    /// Creates a session receiver
    async fn create_session_receiver(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_options: ServiceBusRetryOptions,
        receive_mode: ServiceBusReceiveMode,
        session_id: Option<String>,
        prefetch_count: u32,
    ) -> Result<Self::SessionReceiver, Self::CreateReceiverError>;

    /// TODO: Creates a rule manager strongly aligned with the active protocol and transport, responsible
    /// for adding, removing and getting rules from the Service Bus subscription.
    ///
    /// # Parameters
    ///
    /// * `subscription_path` - The path of the Service Bus subscription to which the rule manager
    ///   is bound.
    /// * `retry_policy` - The policy which governs retry behavior and try timeouts.
    /// * `identifier` - The identifier for the rule manager.
    async fn create_rule_manager(
        &mut self,
        subscription_path: String,
        identifier: String,
        retry_policy: ServiceBusRetryOptions,
    ) -> Result<Self::RuleManager, Self::CreateRuleManagerError>;

    /// Closes the connection to the transport client instance.
    async fn close(
        &mut self,
        // cancellation_token: Option<CancellationToken>,
    ) -> Result<(), Self::DisposeError>;

    /// Performs the task needed to clean up resources used by the client,
    /// including ensuring that the client itself has been closed.
    async fn dispose(mut self) -> Result<(), Self::DisposeError> {
        self.close().await?;
        Ok(())
    }
}
