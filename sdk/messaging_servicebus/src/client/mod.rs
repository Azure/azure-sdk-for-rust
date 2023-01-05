//! Client and client configuration options for Azure Service Bus.

pub mod error;

cfg_either_rustls_or_native_tls! {
    pub mod service_bus_client;
}
