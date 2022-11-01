//!

pub(crate) mod amqp_client;
pub(crate) mod amqp_connection;
pub(crate) mod amqp_connection_scope;
pub(crate) mod amqp_constants;
pub(crate) mod amqp_message_constants;
pub(crate) mod amqp_message_extensions;
pub(crate) mod amqp_receiver;
pub(crate) mod amqp_rule_manager;
pub(crate) mod amqp_sender;
pub(crate) mod amqp_session;
pub(crate) mod cbs_token_provider;
pub(crate) mod error;
pub(crate) mod token_type;

use std::sync::atomic::AtomicU32;

pub use error::Error;

static CONNECTION_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
static SESSION_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
static LINK_IDENTIFIER: AtomicU32 = AtomicU32::new(0);
