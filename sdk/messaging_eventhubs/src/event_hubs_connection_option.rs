use std::time::Duration;
use url::Url;

use crate::event_hubs_transport_type::EventHubsTransportType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventHubConnectionOptions {
    pub connection_idle_timeout: Duration,
    pub transport_type: EventHubsTransportType,
    // send_buffer_size_in_bytes: usize, // TODO: need upstream to support changing buffer size
    // receive_buffer_size_in_bytes: usize, // TODO: need upstream to support changing buffer size
    pub custom_endpoint_address: Option<Url>,
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
}
