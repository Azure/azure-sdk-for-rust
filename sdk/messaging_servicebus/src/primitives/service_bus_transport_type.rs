//! Defines the type of protocol and transport that will be used for communicating with Azure Service

/// Specifies the type of protocol and transport that will be used for communicating with Azure
/// Service Bus.
#[derive(Debug, Clone)]
pub enum ServiceBusTransportType {
    /// The connection uses the AMQP protocol over TCP.
    AmqpTcp,

    /// The connection uses the AMQP protocol over web sockets.
    AmqpWebSocket,
}

impl Default for ServiceBusTransportType {
    fn default() -> Self {
        ServiceBusTransportType::AmqpTcp
    }
}

impl ServiceBusTransportType {
    pub(crate) const AMQP_SCHEME: &'static str = "amqps";
    pub(crate) const WEBSOCKET_SCHEME: &'static str = "wss";

    /// Returns the URI scheme for the transport type.
    pub fn url_scheme(&self) -> &str {
        match self {
            ServiceBusTransportType::AmqpTcp => Self::AMQP_SCHEME,
            ServiceBusTransportType::AmqpWebSocket => Self::WEBSOCKET_SCHEME,
        }
    }
}
