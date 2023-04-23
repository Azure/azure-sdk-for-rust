//! Primitives for the Azure Event Hubs client.
//!
//! This module matches to the `Azure.Messaging.EventHubs.Primitives` namespace in the .NET SDK.

// TODO: mod blob_checkpoint_store;
// TODO: mod checkpoint_store;
// TODO: mod event_processor;
// TODO: mod event_processor_checkpoint;
// TODO: mod event_processor_options;
// TODO: mod event_processor_partition;
// TODO: mod event_processor_partition_ownership;
// TODO: mod pluggable_checkpoint_store_event_processor;

mod partition_receiver;
mod partition_receiver_options;

pub use partition_receiver::*;
pub use partition_receiver_options::*;
