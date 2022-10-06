/// Specifies the type of protocol and transport that will be used for communicating with Azure
/// Service Bus.
#[derive(Debug, Clone)]
pub enum ServiceBusTransportType {
    /// The connection uses the AMQP protocol over TCP.
    AmqpTcp,

    /// The connection uses the AMQP protocol over web sockets.
    AmqpWebSockets,
}

impl Default for ServiceBusTransportType {
    fn default() -> Self {
        ServiceBusTransportType::AmqpTcp
    }
}

impl ServiceBusTransportType {
    const AMQP_URI_SCHEME: &'static str = "amqps";
    const WEBSOCKET_SCHEME: &'static str = "wss";

    pub fn url_scheme(&self) -> &str {
        match self {
            ServiceBusTransportType::AmqpTcp => Self::AMQP_URI_SCHEME,
            ServiceBusTransportType::AmqpWebSockets => Self::WEBSOCKET_SCHEME,
        }
    }
}
