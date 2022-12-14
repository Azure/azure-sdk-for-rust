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
    pub dead_letter_reason: Option<String>,
    pub dead_letter_error_description: Option<String>,
    pub properties_to_modify: Option<OrderedMap<String, Value>>,
}
