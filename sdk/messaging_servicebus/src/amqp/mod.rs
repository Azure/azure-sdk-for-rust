//!

pub(crate) mod amqp_client;
pub(crate) mod amqp_client_constants;
pub(crate) mod amqp_connection;
pub(crate) mod amqp_connection_scope;
pub(crate) mod amqp_constants;
pub(crate) mod amqp_message_batch;
pub(crate) mod amqp_message_constants;
pub(crate) mod amqp_message_converter;
pub(crate) mod amqp_message_extensions;
pub(crate) mod amqp_receiver;
pub(crate) mod amqp_request_message;
pub(crate) mod amqp_response_message;
pub(crate) mod amqp_rule_manager;
pub(crate) mod amqp_sender;
pub(crate) mod amqp_session;
pub(crate) mod cbs_token_provider;
pub(crate) mod error;
pub(crate) mod filter;
pub(crate) mod management_constants;
pub(crate) mod scheduled_message;
pub(crate) mod token_type;

use std::sync::atomic::AtomicU32;

pub use error::Error;

static CONNECTION_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
static SESSION_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
static LINK_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
