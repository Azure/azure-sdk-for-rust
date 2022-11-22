use std::ops::{Deref, DerefMut};

use crate::{
    core::TransportReceiver, ServiceBusReceiveMode, ServiceBusReceiver, ServiceBusReceiverOptions,
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
    pub(crate) inner: ServiceBusReceiver<R>,
    pub(crate) session_id: String,
}

// Use `Deref` and `DerefMut` to avoid having to implement all the methods of `ServiceBusReceiver` on `ServiceBusSessionReceiver`.
impl<R> Deref for ServiceBusSessionReceiver<R> {
    type Target = ServiceBusReceiver<R>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<R> DerefMut for ServiceBusSessionReceiver<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<R> ServiceBusSessionReceiver<R>
where
    R: TransportReceiver,
{
    pub async fn dispose(self) -> Result<(), R::CloseError> {
        self.inner.dispose().await
    }
}
