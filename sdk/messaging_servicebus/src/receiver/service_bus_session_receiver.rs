use crate::{
    core::TransportReceiver, primitives::service_bus_received_message::ServiceBusReceivedMessage,
    ServiceBusReceiveMode, ServiceBusReceiverOptions,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceBusSessionReceiverOptions {
    /// <summary>
    /// Gets or sets the number of messages that will be eagerly requested from Queues or Subscriptions and queued locally without regard to
    /// whether the receiver is actively receiving, intended to help maximize throughput by allowing the receiver to receive
    /// from a local cache rather than waiting on a service request.
    /// </summary>
    /// <exception cref="ArgumentOutOfRangeException">
    ///   A negative value is attempted to be set for the property.
    /// </exception>
    pub prefetch_count: u32,

    /// <summary>
    /// Gets or sets the <see cref="ReceiveMode"/> used to specify how messages are received. Defaults to PeekLock mode.
    /// </summary>
    pub receive_mode: ServiceBusReceiveMode,

    /// <inheritdoc cref="ServiceBusReceiverOptions.Identifier"/>
    pub identifier: Option<String>,
}

impl From<ServiceBusSessionReceiverOptions> for ServiceBusReceiverOptions {
    fn from(options: ServiceBusSessionReceiverOptions) -> Self {
        ServiceBusReceiverOptions {
            receive_mode: options.receive_mode,
            sub_queue: Default::default(),
            prefetch_count: options.prefetch_count,
            identifier: options.identifier,
        }
    }
}

pub struct ServiceBusSessionReceiver<R> {
    pub(crate) inner: R,
    pub(crate) entity_path: String,
    pub(crate) identifier: String,
    pub(crate) session_id: String,
}

impl<R> ServiceBusSessionReceiver<R>
where
    R: TransportReceiver,
{
    pub async fn dispose(self) -> Result<(), R::CloseError> {
        self.inner.close().await
    }

    /// Receive a single message from the entity.
    pub async fn receive_message(
        &mut self,
    ) -> Result<Option<ServiceBusReceivedMessage>, R::ReceiveError> {
        self.receive_messages(1, None)
            .await
            .map(|mut v| v.drain(..).next())
    }

    pub async fn receive_messages(
        &mut self,
        max_messages: u32,
        max_wait_time: Option<std::time::Duration>,
    ) -> Result<Vec<ServiceBusReceivedMessage>, R::ReceiveError> {
        self.inner
            .receive_messages(max_messages, max_wait_time)
            .await
    }

    pub async fn complete_message(
        &mut self,
        message: &ServiceBusReceivedMessage,
    ) -> Result<(), R::DispositionError> {
        self.inner
            .complete(message, Some(self.session_id.clone())) // TODO: avoid cloning
            .await
    }
}
