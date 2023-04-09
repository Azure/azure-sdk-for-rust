/// Specifies the type of protocol and transport that will be used for communicating with
/// Azure Event Hubs.
///
/// This is a simple enum, so copying it is cheap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventHubsTransportType {
    /// The connection uses the AMQP protocol over TCP
    AmqpTcp,

    /// The connection uses the AMQP protocol over web sockets
    AmqpWebSockets,
}

impl Default for EventHubsTransportType {
    fn default() -> Self {
        Self::AmqpTcp
    }
}

impl EventHubsTransportType {
    pub(crate) const AMQP_SCHEME: &'static str = "amqps";
    pub(crate) const WEBSOCKET_SCHEME: &'static str = "wss";

    /// Returns the URI scheme for the transport type.
    pub fn url_scheme(&self) -> &str {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::AmqpTcp => Self::AMQP_SCHEME,
            Self::AmqpWebSockets => Self::WEBSOCKET_SCHEME,
        }
    }
}
