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
