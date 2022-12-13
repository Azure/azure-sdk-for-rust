use std::time::Duration;

use async_trait::async_trait;

use crate::primitives::service_bus_transport_type::ServiceBusTransportType;

#[async_trait]
pub trait TransportConnectionScope {
    type Error;

    fn transport_type(&self) -> ServiceBusTransportType;

    /// Indicates whether this [`TransportConnectionScope`] has been disposed.
    fn is_disposed(&self) -> bool;

    /// The recommended timeout to associate with the session.
    fn session_timeout(&self) -> Duration {
        Duration::from_secs(30)
    }

    /// Disposes of the connection scope.
    async fn dispose(&mut self) -> Result<(), Self::Error>;
}
