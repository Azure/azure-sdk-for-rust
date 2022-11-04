use std::future::Future;
use std::pin::Pin;

use async_trait::async_trait;
use azure_core::Url;
use tokio_util::sync::CancellationToken;

use crate::{
    primitives::service_bus_retry_options::ServiceBusRetryOptions,
    receiver::service_bus_receive_mode::ServiceBusReceiveMode,
};

use super::{
    transport_receiver::TransportReceiver, transport_rule_manager::TransportRuleManager,
    transport_sender::TransportSender,
};

/// Provides an abstraction for generalizing an Service Bus entity client so that a dedicated
/// instance may provide operations for a specific transport, such as AMQP or JMS.  It is intended
/// that the public [ServiceBusConnection] employ a transport client via containment and delegate
/// operations to it rather than understanding protocol-specific details for different transports.
// #[async_trait]
pub(crate) trait TransportClient {
    type CreateSenderError: Send;
    type CreateReceiverError: Send;
    type CreateRuleManagerError: Send;
    type DisposeError: Send;

    type Sender: TransportSender;
    type Receiver: TransportReceiver;
    type RuleManager: TransportRuleManager;

    /// Indicates whether or not this client has been closed.
    ///
    /// Returnss `true` if the client is closed, otherwise `false`
    fn is_closed(&self) -> bool;

    /// The endpoint for the Service Bus service to which the client is associated.
    fn service_endpoint(&self) -> &Url;

    /// Creates a sender strongly aligned with the active protocol and transport,
    /// responsible for sending <see cref="ServiceBusMessage" /> to the entity.
    ///
    /// # Arguments
    ///
    /// * `entity_path` - The entity path to send the message to.
    /// * `retry_policy` - The policy which governs retry behavior and try timeouts
    /// * `identifier` - The identifier for the sender.
    ///
    /// # Returns
    ///
    /// A [TransportSender] configured in the requested manner.
    fn create_sender(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_policy: ServiceBusRetryOptions,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Sender, Self::CreateSenderError>> + '_>>;

    fn create_receiver(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_options: ServiceBusRetryOptions,
        receive_mode: ServiceBusReceiveMode,
        prefetch_count: u32,
        is_processor: bool,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Receiver, Self::CreateReceiverError>> + '_>>;

    fn create_session_receiver(
        &mut self,
        entity_path: String,
        identifier: String,
        retry_options: ServiceBusRetryOptions,
        receive_mode: ServiceBusReceiveMode,
        prefetch_count: u32,
        session_id: String,
        is_processor: bool,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Receiver, Self::CreateReceiverError>> + '_>>;

    /// Creates a rule manager strongly aligned with the active protocol and transport, responsible
    /// for adding, removing and getting rules from the Service Bus subscription.
    ///
    /// # Arguments
    ///
    /// * `subscription_path` - The path of the Service Bus subscription to which the rule manager
    ///   is bound.
    /// * `retry_policy` - The policy which governs retry behavior and try timeouts.
    /// * `identifier` - The identifier for the rule manager.
    ///
    /// # Returns
    ///
    /// A [TransportRuleManager] configured in the requested manner.
    fn create_rule_manager(
        &mut self,
        subscription_path: impl Into<String>,
        retry_policy: ServiceBusRetryOptions,
        identifier: impl Into<String>,
    ) -> Result<Self::RuleManager, Self::CreateRuleManagerError>;

    /// Closes the connection to the transport client instance.
    ///
    /// # Arguments
    ///
    /// An optional [CancellationToken] instance to signal the request to cancel the operation.
    fn close(
        &mut self,
        cancellation_token: Option<CancellationToken>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::DisposeError>> + '_>>;

    /// Performs the task needed to clean up resources used by the client,
    /// including ensuring that the client itself has been closed.
    fn dispose(&mut self) -> Pin<Box<dyn Future<Output = Result<(), Self::DisposeError>> + '_>> {
        Box::pin(async move {
            self.close(None).await?;
            Ok(())
        })
    }
}
