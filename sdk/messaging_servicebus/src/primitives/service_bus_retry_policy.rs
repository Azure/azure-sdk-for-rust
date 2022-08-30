/// An abstract representation of a policy to govern retrying of messaging operations.
///
/// It is recommended that developers without advanced needs not implement custom retry
/// policies but instead configure the default policy by specifying the desired set of
/// retry options when creating one of the Service Bus clients.
pub trait ServiceBusRetryPolicy {}
