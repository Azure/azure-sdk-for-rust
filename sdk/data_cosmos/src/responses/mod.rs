//! Responses from any call to the Cosmos API.

#![allow(missing_docs)]

mod execute_stored_procedure_response;
mod get_partition_key_ranges_response;
mod replace_reference_attachment_response;

pub use execute_stored_procedure_response::ExecuteStoredProcedureResponse;
pub use get_partition_key_ranges_response::GetPartitionKeyRangesResponse;
pub use replace_reference_attachment_response::ReplaceReferenceAttachmentResponse;
