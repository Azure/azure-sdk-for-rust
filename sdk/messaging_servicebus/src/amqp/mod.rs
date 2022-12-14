//!

pub(crate) mod amqp_cbs_link;
pub(crate) mod amqp_client_constants;
pub(crate) mod amqp_connection;
pub(crate) mod amqp_connection_scope;
pub(crate) mod amqp_constants;
pub(crate) mod amqp_message_batch;
pub(crate) mod amqp_message_constants;
pub(crate) mod amqp_message_converter;
pub(crate) mod amqp_message_extensions;
pub(crate) mod amqp_request_message;
pub(crate) mod amqp_response_message;
pub(crate) mod amqp_session;
pub(crate) mod cbs_token_provider;
pub(crate) mod filters;
pub(crate) mod management_constants;
pub(crate) mod rules;
pub(crate) mod scheduled_message;
pub(crate) mod token_type;

pub mod amqp_client;
pub mod amqp_receiver;
pub mod amqp_sender;
pub mod amqp_session_receiver;
pub mod error;

// TODO:
// pub(crate) mod amqp_rule_manager;

use std::sync::atomic::AtomicU32;

// counter used for generating unique connection/session/link identifiers
static CONNECTION_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
static SESSION_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
static LINK_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
