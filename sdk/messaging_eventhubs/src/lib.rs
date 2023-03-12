//! # Azure Event Hubs

#![deny(missing_docs, missing_debug_implementations)]

pub(crate) mod amqp;
pub(crate) mod authorization;
pub(crate) mod core;
pub(crate) mod diagnostics;
pub(crate) mod primitives;
pub(crate) mod producer;
pub(crate) mod event_data;
pub(crate) mod event_hubs_connection_option;
pub(crate) mod event_hubs_connection_string_properties;
pub(crate) mod event_hubs_connection;
pub(crate) mod event_hubs_properties;
pub(crate) mod event_hubs_retry_mode;
pub(crate) mod event_hubs_retry_options;
pub(crate) mod event_hubs_retry_policy;
pub(crate) mod event_hubs_transport_type;
pub(crate) mod partition_properties;

pub mod consumer;
