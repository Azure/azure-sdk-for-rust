/// <summary>
///   Specifies the type of protocol and transport that will be used for communicating with
///   Azure Service Bus.
/// </summary>
///
pub enum ServiceBusTransportType {
    /// <summary>The connection uses the AMQP protocol over TCP.</summary>
    AmqpTcp,

    /// <summary>The connection uses the AMQP protocol over web sockets.</summary>
    AmqpWebSockets,
}
