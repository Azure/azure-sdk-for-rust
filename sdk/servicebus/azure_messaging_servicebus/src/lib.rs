// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

#![recursion_limit = "128"]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Service Bus client
pub mod client;
mod error;
mod message;
/// Service Bus message receiving functionality and options.
pub mod receiver;

/// Service Bus message sending functionality and options.
pub mod sender;

/// Models and types used throughout the Service Bus client.
pub mod models;

/// Common types and utilities.
mod common;

pub use client::{
    CreateReceiverOptions, CreateSenderOptions, ServiceBusClient, ServiceBusClientBuilder,
    ServiceBusClientOptions, SubQueue,
};
pub use error::{ErrorKind, ServiceBusError};
pub use message::{Message, MessageBatch, ReceivedMessage};
pub use receiver::{
    AbandonMessageOptions, CompleteMessageOptions, DeadLetterMessageOptions, DeferMessageOptions,
    PeekMessagesOptions, ReceiveDeferredMessagesOptions, ReceiveMessageOptions, ReceiveMode,
    Receiver, RenewMessageLockOptions,
};
pub use sender::{
    CancelScheduledMessagesOptions, CreateMessageBatchOptions, ScheduleMessageOptions,
    ScheduleMessagesOptions, SendMessageBatchOptions, SendMessageOptions, SendMessagesOptions,
    Sender,
};

/// Result type used throughout the Service Bus client.
pub type Result<T> = std::result::Result<T, ServiceBusError>;
