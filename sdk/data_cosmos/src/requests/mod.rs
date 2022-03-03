//! Request builder objects for every kind of request.
//!
//! These objects are usually created by calling some sort of method on a client. They
//! then give you the ability to modify your request with certain options and finally
//! execute the request with the `execute` method.

#![allow(missing_docs)]

mod get_partition_key_ranges_builder;
mod replace_reference_attachment_builder;

pub use get_partition_key_ranges_builder::GetPartitionKeyRangesBuilder;
pub use replace_reference_attachment_builder::ReplaceReferenceAttachmentBuilder;
