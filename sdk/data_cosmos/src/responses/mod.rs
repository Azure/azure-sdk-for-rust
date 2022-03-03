//! Responses from any call to the Cosmos API.

#![allow(missing_docs)]

mod create_reference_attachment_response;
mod create_slug_attachment_response;
mod execute_stored_procedure_response;
mod get_partition_key_ranges_response;
mod replace_reference_attachment_response;

pub use create_reference_attachment_response::CreateReferenceAttachmentResponse;
pub use create_slug_attachment_response::CreateSlugAttachmentResponse;
pub use execute_stored_procedure_response::ExecuteStoredProcedureResponse;
pub use get_partition_key_ranges_response::GetPartitionKeyRangesResponse;
pub use replace_reference_attachment_response::ReplaceReferenceAttachmentResponse;
