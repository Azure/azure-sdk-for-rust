/// Provides an abstraction for generalizing an Service Bus entity Producer so that a dedicated instance may provide operations
/// for a specific transport, such as AMQP or JMS.  It is intended that the public <see cref="ServiceBusSender" /> employ
/// a transport producer via containment and delegate operations to it rather than understanding protocol-specific details
/// for different transports.
pub trait TransportSender {}
