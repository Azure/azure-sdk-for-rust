//! Azure service bus crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).
#![recursion_limit = "128"]

pub mod administration;
pub mod amqp;
pub mod client;
pub mod constants;
pub mod core;
pub mod diagnostics;
pub mod prelude;
pub mod primitives;
pub mod processor;
pub mod receiver;
pub mod sender;
pub mod service_bus;
pub mod utils;

pub use primitives::service_bus_message::ServiceBusMessage;
pub use receiver::{
    service_bus_receive_mode::ServiceBusReceiveMode, service_bus_receiver::ServiceBusReceiver,
    service_bus_receiver::ServiceBusReceiverOptions,
};
pub use sender::{
    create_message_batch_options::CreateMessageBatchOptions,
    service_bus_message_batch::ServiceBusMessageBatch, service_bus_sender::ServiceBusSender,
    service_bus_sender_options::ServiceBusSenderOptions,
};

pub(crate) mod authorization;
pub(crate) mod entity_name_formatter;

#[cfg(test)]
pub(crate) mod test_utils {
    pub(crate) fn setup_dotenv() {
        dotenv::from_filename("./sdk/messaging_servicebus/tests/.env").ok();
    }
}
