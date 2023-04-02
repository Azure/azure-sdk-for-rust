/// Specifies the type of protocol and transport that will be used for communicating with
/// Azure Event Hubs.
///
/// This is a simple enum, so copying it is cheap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventHubsTransportType
{
    /// The connection uses the AMQP protocol over TCP
    AmqpTcp,

    /// The connection uses the AMQP protocol over web sockets
    AmqpWebSockets
}
