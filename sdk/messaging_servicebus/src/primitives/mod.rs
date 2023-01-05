//! Primitive types for Azure Service Bus.

cfg_either_rustls_or_native_tls! {
    pub(crate) mod service_bus_connection;
}

pub(crate) mod disposition_status;
pub mod error;
pub mod service_bus_connection_string_properties;
pub mod service_bus_message;
pub mod service_bus_message_state;
pub mod service_bus_peeked_message;
pub mod service_bus_received_message;
pub mod service_bus_retry_mode;
pub mod service_bus_retry_options;
pub mod service_bus_retry_policy;
pub mod service_bus_transport_type;
pub mod sub_queue;
