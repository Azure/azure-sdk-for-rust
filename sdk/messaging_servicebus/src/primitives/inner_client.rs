use async_trait::async_trait;
use azure_core::Url;
use tokio_util::sync::CancellationToken;

use crate::{core::TransportClient, receiver::service_bus_receive_mode::ServiceBusReceiveMode};

use super::{
    inner_receiver::InnerReceiver, inner_rule_manager::InnerRuleManager, inner_sender::InnerSender,
    service_bus_retry_policy::ServiceBusRetryPolicy,
};

#[derive(Debug)]
pub(crate) enum InnerClient {}

#[async_trait]
impl TransportClient for InnerClient {
    type Error = ();

    type Sender = InnerSender;

    type Receiver = InnerReceiver;

    type RuleManager = InnerRuleManager;

    /// Indicates whether or not this client has been closed.
    ///
    /// Returnss `true` if the client is closed, otherwise `false`
    fn is_closed(&self) -> bool {
        todo!()
    }

    /// The endpoint for the Service Bus service to which the client is associated.
    fn service_endpoint(&self) -> &Url {
        todo!()
    }

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
    ) -> Result<Self::Sender, Self::Error> {
        todo!()
    }

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
    ) -> Result<Self::Receiver, Self::Error> {
        todo!()
    }

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
    ) -> Result<Self::RuleManager, Self::Error> {
        todo!()
    }

    /// Closes the connection to the transport client instance.
    ///
    /// # Arguments
    ///
    /// An optional [CancellationToken] instance to signal the request to cancel the operation.
    async fn close(
        &mut self,
        cancellation_token: impl Into<Option<CancellationToken>> + Send,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
