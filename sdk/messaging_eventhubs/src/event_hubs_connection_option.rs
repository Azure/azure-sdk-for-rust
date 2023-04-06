use std::time::Duration;
use url::Url;

use crate::event_hubs_transport_type::EventHubsTransportType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventHubConnectionOptions {
    pub(crate) connection_idle_timeout: Duration,
    pub(crate) transport_type: EventHubsTransportType,
    // send_buffer_size_in_bytes: usize, // TODO: need upstream to support changing buffer size
    // receive_buffer_size_in_bytes: usize, // TODO: need upstream to support changing buffer size
    pub(crate) custom_endpoint_address: Option<Url>,
}

impl Default for EventHubConnectionOptions {
    fn default() -> Self {
        Self {
            connection_idle_timeout: Duration::from_secs(60),
            transport_type: Default::default(),
            custom_endpoint_address: Default::default(),
        }
    }
}

impl EventHubConnectionOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn connection_idle_timeout(&self) -> Duration {
        self.connection_idle_timeout
    }

    pub fn set_connection_idle_timeout(&mut self, connection_idle_timeout: Duration) {
        self.connection_idle_timeout = connection_idle_timeout;
    }

    pub fn transport_type(&self) -> EventHubsTransportType {
        self.transport_type
    }

    pub fn set_transport_type(&mut self, transport_type: EventHubsTransportType) {
        self.transport_type = transport_type;
    }

    pub fn custom_endpoint_address(&self) -> Option<&Url> {
        self.custom_endpoint_address.as_ref()
    }

    pub fn set_custom_endpoint_address(&mut self, custom_endpoint_address: Option<Url>) {
        self.custom_endpoint_address = custom_endpoint_address;
    }
}
