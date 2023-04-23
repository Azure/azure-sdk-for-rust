//! # Azure Event Hubs

// #![deny(missing_docs, missing_debug_implementations)]

#[macro_use]
mod cfg;

pub(crate) mod amqp;
pub(crate) mod authorization;
pub(crate) mod constants;
pub(crate) mod core;
pub(crate) mod diagnostics;
pub(crate) mod event_data;
pub(crate) mod event_hubs_connection;
pub(crate) mod event_hubs_connection_option;
pub(crate) mod event_hubs_connection_string_properties;
pub(crate) mod event_hubs_properties;
pub(crate) mod event_hubs_retry_mode;
pub(crate) mod event_hubs_retry_options;
pub(crate) mod event_hubs_retry_policy;
pub(crate) mod event_hubs_transport_type;
pub(crate) mod partition_properties;
pub(crate) mod util;

pub mod consumer;
pub mod primitives;
pub mod producer;

pub mod prelude {
    //! Prelude for the Azure Event Hubs crate.
    pub use crate::event_data::*;
    pub use crate::event_hubs_connection::*;
    pub use crate::event_hubs_connection_option::*;
    pub use crate::event_hubs_properties::*;
    pub use crate::event_hubs_connection_string_properties::*;
    pub use crate::event_hubs_retry_options::*;
    pub use crate::event_hubs_retry_policy::*;
    pub use crate::partition_properties::*;
    pub use crate::event_hubs_transport_type::*;
    pub use crate::event_hubs_retry_mode::*;
}

pub use prelude::*;
