//! Implements the Service Bus Receiver.
use serde_amqp::{primitives::OrderedMap, Value};

pub mod service_bus_receive_mode;
pub mod service_bus_receiver;
pub mod service_bus_session_receiver;

/// The dead letter options.
///
/// Default values are `None` for all fields
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeadLetterOptions {
    /// The reason for dead-lettering the message
    pub dead_letter_reason: Option<String>,

    /// The error description for dead-lettering the message
    pub dead_letter_error_description: Option<String>,

    /// The properties to modify on the message
    pub properties_to_modify: Option<OrderedMap<String, Value>>,
}
