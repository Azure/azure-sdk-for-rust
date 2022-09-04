use async_trait::async_trait;
use azure_core::Url;
use tokio_util::sync::CancellationToken;

use crate::{
    primitives::service_bus_retry_policy::ServiceBusRetryPolicy,
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
#[async_trait]
pub(crate) trait TransportClient {
    type Error: Send;
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
        entity_path: impl Into<String>, // TODO: AsRef<str> or AsRef<Path>?
        retry_policy: impl ServiceBusRetryPolicy,
        identifier: impl Into<String>,
    ) -> Result<Self::Sender, Self::Error>;

    fn create_receiver(
        &mut self,
        entity_path: impl Into<String>,
        retry_policy: impl ServiceBusRetryPolicy,
        receive_mode: ServiceBusReceiveMode,
        prefetch_count: u32,
        identifier: impl Into<String>,
        session_id: impl Into<String>,
        is_session_receiver: bool,
        is_processor: bool,
    ) -> Result<Self::Receiver, Self::Error>;

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
        retry_policy: impl ServiceBusRetryPolicy,
        identifier: impl Into<String>,
    ) -> Result<Self::RuleManager, Self::Error>;

    /// Closes the connection to the transport client instance.
    ///
    /// # Arguments
    ///
    /// An optional [CancellationToken] instance to signal the request to cancel the operation.
    async fn close(
        &mut self,
        cancellation_token: impl Into<Option<CancellationToken>>,
    ) -> Result<(), Self::Error>;

    /// Performs the task needed to clean up resources used by the client,
    /// including ensuring that the client itself has been closed.
    async fn dispose(&mut self) -> Result<(), Self::Error> {
        // TODO: Is this right?
        self.close(None).await
    }
}
