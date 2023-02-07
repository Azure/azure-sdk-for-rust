//! Defines the type of protocol and transport that will be used for communicating with Azure Service

/// Specifies the type of protocol and transport that will be used for communicating with Azure
/// Service Bus.
#[derive(Debug, Clone, Copy)]
pub enum ServiceBusTransportType {
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
    /// The connection uses the AMQP protocol over TCP.
    AmqpTcp,

    /// The connection uses the AMQP protocol over web sockets.
    AmqpWebSocket,
}

cfg_not_wasm32! {
    impl Default for ServiceBusTransportType {
        fn default() -> Self {
            ServiceBusTransportType::AmqpTcp
        }
    }
}

cfg_wasm32! {
    impl Default for ServiceBusTransportType {
        fn default() -> Self {
            ServiceBusTransportType::AmqpWebSocket
        }
    }
}

impl ServiceBusTransportType {
    pub(crate) const AMQP_SCHEME: &'static str = "amqps";
    pub(crate) const WEBSOCKET_SCHEME: &'static str = "wss";

    /// Returns the URI scheme for the transport type.
    pub fn url_scheme(&self) -> &str {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            ServiceBusTransportType::AmqpTcp => Self::AMQP_SCHEME,
            ServiceBusTransportType::AmqpWebSocket => Self::WEBSOCKET_SCHEME,
        }
    }
}
