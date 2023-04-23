pub(crate) mod amqp_cbs_link;
pub(crate) mod amqp_client;
pub(crate) mod amqp_connection;
pub(crate) mod amqp_connection_scope;
pub(crate) mod amqp_constants;
pub(crate) mod amqp_consumer;
pub(crate) mod amqp_event_batch;
pub(crate) mod amqp_filter;
pub(crate) mod amqp_management;
pub(crate) mod amqp_management_link;
pub(crate) mod amqp_message_converter;
pub(crate) mod amqp_message_extension;
pub(crate) mod amqp_phantom_message;
pub(crate) mod amqp_producer;
pub(crate) mod amqp_property;
pub(crate) mod cbs_token_provider;
pub(crate) mod error;
pub(crate) mod token_type;

// pub(crate) mod amqp_system_properties; // TODO: implement processor

use std::sync::atomic::AtomicU32;

// counter used for generating unique connection/session/link identifiers
static CONNECTION_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
static SESSION_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
static LINK_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
