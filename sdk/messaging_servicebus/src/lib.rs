//! An unofficial and experimental AMQP 1.0 client for Azure Service Bus.
//!
//! This crate follows a similar structure to the dotnet sdk
//! ([Azure.Messaging.ServiceBus](https://github.com/Azure/azure-sdk-for-net/tree/main/sdk/servicebus/Azure.Messaging.ServiceBus))
//! and provides more features than
//! [azure_messaging_servicebus](https://crates.io/crates/azure_messaging_servicebus). The list of
//! supported Service Bus features can be found below ([Supported Service Bus
//! Features](#supported-service-bus-features)). A complete comparison of supported features in REST
//! client and AMQP 1.0 client can be found
//! [here](https://learn.microsoft.com/en-us/rest/api/servicebus/rest-dotnet-client-support#features-exposed-using-both-the-rest-client-and-the-net-managed-api).
//!
//! Because this crate currently lives in a fork of `azure-sdk-for-rust` and GitHub doesn't seem to
//! allow raising issue in forks. Please use the upstream AMQP 1.0 crate
//! [fe2o3-amqp](https://github.com/minghuaw/fe2o3-amqp) GitHub repo should you have any
//! issue/feature request.
//!
//! # Content
//!
//! - [Homepage](https://github.com/minghuaw/azure-sdk-for-rust/blob/separate_servicebus_crate/sdk/messaging_servicebus)
//! - [Examples](#examples)
//!   - [Send messages to queue](#send-messages-to-queue)
//!   - [Receive messages from queue](#receive-messages-from-queue)
//! - [Supported Service Bus Features](#supported-service-bus-features)
//! - [TLS Support](#tls-support)
//! - [Feature flags](#feature-flags)
//! - [Change
//!   log](https://github.com/minghuaw/azure-sdk-for-rust/blob/separate_servicebus_crate/sdk/messaging_servicebus/CHANGELOG.md)
//!
//! # Examples
//!
//! Below are two examples of sending and receiving messages from a queue. More examples can be
//! found in the
//! [examples](https://github.com/minghuaw/azure-sdk-for-rust/tree/separate_servicebus_crate/sdk/messaging_servicebus/examples)
//!
//! ## Send messages to queue
//!
//! ```rust,no_run
//! use azservicebus::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Replace "<NAMESPACE-CONNECTION-STRING>" with your connection string,
//!     // which can be found in the Azure portal and should look like
//!     // "Endpoint=sb://<NAMESPACE>.servicebus.windows.net/;SharedAccessKeyName=<KEY_NAME>;SharedAccessKey=<KEY_VALUE>"
//!     let mut client = ServiceBusClient::new_from_connection_string(
//!         "<NAMESPACE-CONNECTION-STRING>",
//!         ServiceBusClientOptions::default()
//!     )
//!     .await?;
//!
//!     // Replace "<QUEUE-NAME>" with the name of your queue
//!     let mut sender = client.create_sender(
//!         "<QUEUE-NAME>",
//!         ServiceBusSenderOptions::default()
//!     )
//!     .await?;
//!
//!     // Create a batch
//!     let mut message_batch = sender.create_message_batch(Default::default())?;
//!
//!     for i in 0..3 {
//!         // Create a message
//!         let message = ServiceBusMessage::new(format!("Message {}", i));
//!         // Try to add the message to the batch
//!         if let Err(e) = message_batch.try_add_message(message) {
//!             // If the batch is full, an error will be returned
//!             println!("Failed to add message {} to batch: {:?}", i, e);
//!             break;
//!         }
//!     }
//!
//!     // Send the batch of messages to the queue
//!     match sender.send_message_batch(message_batch).await {
//!         Ok(()) => println!("Batch sent successfully"),
//!         Err(e) => println!("Failed to send batch: {:?}", e),
//!     }
//!
//!     sender.dispose().await?;
//!     client.dispose().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Receive messages from queue
//!
//! ```rust,no_run
//! use azservicebus::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Replace "<NAMESPACE-CONNECTION-STRING>" with your connection string,
//!     // which can be found in the Azure portal and should look like
//!     // "Endpoint=sb://<NAMESPACE>.servicebus.windows.net/;SharedAccessKeyName=<KEY_NAME>;SharedAccessKey=<KEY_VALUE>"
//!     let mut client = ServiceBusClient::new_from_connection_string(
//!         "<NAMESPACE-CONNECTION-STRING>",
//!         ServiceBusClientOptions::default()
//!     )
//!     .await?;
//!
//!     // Replace "<QUEUE-NAME>" with the name of your queue
//!     let mut receiver = client.create_receiver_for_queue(
//!         "<QUEUE-NAME>",
//!         ServiceBusReceiverOptions::default()
//!     )
//!     .await?;
//!
//!     // Receive messages from the queue
//!     // This will wait indefinitely until at least one message is received
//!     let messages = receiver.receive_messages(3).await?;
//!
//!     for message in &messages {
//!         let body = message.body()?;
//!         println!("Received message: {:?}", std::str::from_utf8(body)?);
//!
//!         // Complete the message so that it is removed from the queue
//!         receiver.complete_message(message).await?;
//!     }
//!
//!     receiver.dispose().await?;
//!     client.dispose().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Supported Service Bus Features
//!
//! Below shows supported Service Bus features
//!
//! | Feature | Supported |
//! | ------- | --------- |
//! | Send messages to queue/topic | Yes |
//! | Receive messages from queue/subscription | Yes |
//! | Session receivers for queue/subscription | Yes |
//! | Prefetch | Yes |
//! | Schedule messages | Yes |
//! | Cancel scheduled messages | Yes |
//! | Peek messages | Yes |
//! | Complete messages | Yes |
//! | Abandon messages | Yes |
//! | Defer messages | Yes |
//! | Receive deferred messages | Yes |
//! | Dead-letter messages | Yes |
//! | Receive dead-lettered messages | Yes |
//! | Batching | Yes |
//! | Manage rule filters for subscriptions | Yes |
//! | Lock renewal | Yes |
//! | Transaction | Not yet |
//! | Processor | Not yet |
//! | Session processor | Not yet |
//!
//! # TLS Support
//!
//! Communication between a client application and an Azure Service Bus namespace is encrypted using
//! Transport Layer Security (TLS). The TLS implementation is exposed to the user through the
//! corresponding feature flags (please see the feature flag section below). The user should ensure
//! either the `rustls` or `native-tls` feature is enabled, and one and only one TLS implementation
//! is enabled. Enabling both features is **not** supported and will result in a compile-time error.
//!
//! The `native-tls` feature is enabled by default, and it will use the `native-tls` crate to
//! provide TLS support. The `rustls` feature will use the `rustls` crate and `webpki-roots` crate
//! to provide TLS support.
//!
//! # Feature flags
//!
//! This crate supports the following feature flags:
//!
//! | Feature | Description |
//! | ------- | ----------- |
//! | `default` | Enables "native-tls" feature |
//! | `rustls` | Enables the use of the `rustls` crate for TLS support |
//! | `native-tls` | Enables the use of the `native-tls` crate for TLS support |
//! | `transaction` | This is reserved for future support of transaction and is not implemented yet |
//!
//! # WebAssembly Support
//!
//! WebAssembly is supported. Please see the [`wasm32_in_browser`
//! example](https://github.com/minghuaw/azure-sdk-for-rust/tree/separate_servicebus_crate/sdk/messaging_servicebus/examples/wasm32_in_browser)
//! for more details.
//!
//! # MSRV (Minimum Supported Rust Version)
//!
//! 1.65.0 is the MSRV for this crate because it uses generic associated types.

#![recursion_limit = "128"]
#![deny(missing_docs, missing_debug_implementations)]
// doc_cfg is experimental. This will allow us to indicate what features are required for a given item.
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

pub(crate) mod constants;
pub(crate) mod diagnostics;
pub(crate) mod entity_name_formatter;
pub(crate) mod util;

pub(crate) mod sealed {
    // This is a marker trait to prevent users from implementing certain traits from this crate.
    // This should be kept within a `pub(crate) mod` and MUST NOT be re-exported or made public.
    //
    // TODO: The sealed traits are all currently private.
    pub trait Sealed {}
}

cfg_either_rustls_or_native_tls! {
    pub mod rule_manager;
}

pub mod administration;
pub mod amqp;
pub mod authorization;
pub mod client;
pub mod core;
pub mod primitives;
pub mod receiver;
pub mod sender;

// TODO: reserved for future use
// pub mod processor;

pub mod prelude {
    //! Re-exports

    cfg_either_rustls_or_native_tls! {
        pub use crate::client::service_bus_client::{ServiceBusClient, ServiceBusClientOptions};
        pub use crate::rule_manager::service_bus_rule_manager::ServiceBusRuleManager;
        pub use crate::sender::service_bus_sender::{ServiceBusSender, ServiceBusSenderOptions};
        pub use crate::receiver::service_bus_receiver::{ServiceBusReceiver, ServiceBusReceiverOptions};
        pub use crate::receiver::service_bus_session_receiver::{ServiceBusSessionReceiver, ServiceBusSessionReceiverOptions};
    }

    pub use crate::primitives::{
        service_bus_connection_string_properties::ServiceBusConnectionStringProperties,
        service_bus_message::ServiceBusMessage, service_bus_message_state::ServiceBusMessageState,
        service_bus_peeked_message::ServiceBusPeekedMessage,
        service_bus_received_message::ServiceBusReceivedMessage,
        service_bus_retry_mode::ServiceBusRetryMode,
        service_bus_retry_options::ServiceBusRetryOptions,
        service_bus_retry_policy::ServiceBusRetryPolicy,
        service_bus_transport_type::ServiceBusTransportType, sub_queue::SubQueue,
    };
    pub use crate::receiver::service_bus_receive_mode::ServiceBusReceiveMode;
    pub use crate::sender::{
        service_bus_message_batch::CreateMessageBatchOptions,
        service_bus_message_batch::ServiceBusMessageBatch,
    };
}

// Re-export again to allow user to selectively import components
pub use prelude::*;
