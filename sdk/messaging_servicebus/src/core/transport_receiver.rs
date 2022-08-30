/// Provides an abstraction for generalizing a message receiver so that a dedicated instance may provide operations
/// for a specific transport, such as AMQP or JMS.  It is intended that the public <see
/// cref="ServiceBusReceiver" /> and <see cref="ServiceBusProcessor" /> employ a transport receiver
/// via containment and delegate operations to it rather than understanding protocol-specific
/// details for different transports.
pub trait TransportReceiver {}
