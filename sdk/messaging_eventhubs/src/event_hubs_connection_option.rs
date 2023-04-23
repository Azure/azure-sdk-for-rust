use std::time::Duration;
use url::Url;

use crate::event_hubs_transport_type::EventHubsTransportType;

/// The set of options that can be specified when creating [`EventHubConnection`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventHubConnectionOptions {
    /// The amount of time to allow a connection to have no observed traffic before considering it idle
    pub connection_idle_timeout: Duration,

    /// The type of protocol and transport that will be used for communicating with the Event Hubs
    /// service.
    pub transport_type: EventHubsTransportType,

    // send_buffer_size_in_bytes: usize, // TODO: need upstream to support changing buffer size
    // receive_buffer_size_in_bytes: usize, // TODO: need upstream to support changing buffer size

    /// The custom address to use for establishing a connection to the Event Hubs service
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
    /// Create a new instance of [`EventHubConnectionOptions`] with default values
    pub fn new() -> Self {
        Default::default()
    }
}
