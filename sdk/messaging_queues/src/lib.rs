#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;

pub use azure_storage::{Error, Result};

mod clients;
mod message_ttl;
mod number_of_messages;
mod pop_receipt;
pub mod prelude;
//mod queue_service_properties;
mod queue_stored_access_policy;
pub mod requests;
pub mod responses;
mod visibility_timeout;

pub use clients::*;
pub use message_ttl::MessageTTL;
pub use number_of_messages::NumberOfMessages;
pub use pop_receipt::PopReceipt;
//pub use queue_service_properties::QueueServiceProperties;
pub use queue_stored_access_policy::QueueStoredAccessPolicy;
pub use visibility_timeout::VisibilityTimeout;
