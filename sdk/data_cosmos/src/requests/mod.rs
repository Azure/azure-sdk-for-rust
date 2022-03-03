//! Request builder objects for every kind of request.
//!
//! These objects are usually created by calling some sort of method on a client. They
//! then give you the ability to modify your request with certain options and finally
//! execute the request with the `execute` method.

#![allow(missing_docs)]

mod create_or_replace_trigger_builder;
mod create_or_replace_user_defined_function_builder;
mod create_reference_attachment_builder;
mod create_slug_attachment_builder;
mod delete_attachment_builder;
mod delete_trigger_builder;
mod delete_user_defined_function_builder;
mod execute_stored_procedure_builder;
mod get_attachment_builder;
mod get_partition_key_ranges_builder;
mod replace_reference_attachment_builder;
mod replace_slug_attachment_builder;

pub use create_or_replace_trigger_builder::CreateOrReplaceTriggerBuilder;
pub use create_or_replace_user_defined_function_builder::CreateOrReplaceUserDefinedFunctionBuilder;
pub use create_reference_attachment_builder::CreateReferenceAttachmentBuilder;
pub use create_slug_attachment_builder::CreateSlugAttachmentBuilder;
pub use delete_attachment_builder::DeleteAttachmentBuilder;
pub use delete_trigger_builder::DeleteTriggerBuilder;
pub use delete_user_defined_function_builder::DeleteUserDefinedFunctionBuilder;
pub use execute_stored_procedure_builder::ExecuteStoredProcedureBuilder;
pub use get_attachment_builder::GetAttachmentBuilder;
pub use get_partition_key_ranges_builder::GetPartitionKeyRangesBuilder;
pub use replace_reference_attachment_builder::ReplaceReferenceAttachmentBuilder;
pub use replace_slug_attachment_builder::ReplaceSlugAttachmentBuilder;
