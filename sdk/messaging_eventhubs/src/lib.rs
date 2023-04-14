//! # Azure Event Hubs

// #![deny(missing_docs, missing_debug_implementations)]

#[macro_use]
mod cfg;

pub(crate) mod amqp;
pub(crate) mod authorization;
pub(crate) mod constants;
pub(crate) mod core;
pub(crate) mod diagnostics;
pub(crate) mod event;
pub(crate) mod event_hubs_connection;
pub(crate) mod event_hubs_connection_option;
pub(crate) mod event_hubs_connection_string_properties;
pub(crate) mod event_hubs_properties;
pub(crate) mod event_hubs_retry_mode;
pub(crate) mod event_hubs_retry_options;
pub(crate) mod event_hubs_retry_policy;
pub(crate) mod event_hubs_transport_type;
pub(crate) mod partition_properties;
pub(crate) mod primitives;
pub(crate) mod util;

pub mod consumer;
pub mod producer;

pub mod prelude {
    //! Prelude for the Azure Event Hubs crate.

    pub use crate::amqp::amqp_system_properties::AmqpSystemProperties;
    pub use crate::core::BasicRetryPolicy;
    pub use crate::event::*;
    pub use crate::event_hubs_connection::EventHubConnection;
    pub use crate::event_hubs_connection_option::EventHubConnectionOptions;
    pub use crate::event_hubs_retry_options::*;
    pub use crate::event_hubs_transport_type::EventHubsTransportType;
    pub use crate::partition_properties::*;
}

pub use prelude::*;
