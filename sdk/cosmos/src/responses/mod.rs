//! Responses from any call to the Cosmos API.

#![allow(missing_docs)]

mod create_collection_response;
mod create_stored_procedure_response;
mod create_trigger_response;
mod create_user_defined_function_response;
mod delete_database_response;
mod delete_document_response;
mod delete_stored_procedure_response;
mod delete_trigger_response;
mod delete_user_defined_function_response;
mod execute_stored_procedure_response;
mod get_partition_key_ranges_response;
mod list_attachments_response;
mod list_collections_response;
mod list_documents_response;
mod list_permissions_response;
mod list_stored_procedures_response;
mod list_triggers_response;
mod list_user_defined_functions_response;
mod query_documents_response;
mod replace_document_response;
mod replace_stored_procedure_response;

pub use create_collection_response::CreateCollectionResponse;
pub use create_stored_procedure_response::CreateStoredProcedureResponse;
pub use create_trigger_response::CreateTriggerResponse;
pub use create_user_defined_function_response::CreateUserDefinedFunctionResponse;
pub use delete_database_response::DeleteDatabaseResponse;
pub use delete_document_response::DeleteDocumentResponse;
pub use delete_stored_procedure_response::DeleteStoredProcedureResponse;
pub use delete_trigger_response::DeleteTriggerResponse;
pub use delete_user_defined_function_response::DeleteUserDefinedFunctionResponse;
pub use execute_stored_procedure_response::ExecuteStoredProcedureResponse;
pub use get_partition_key_ranges_response::GetPartitionKeyRangesResponse;
pub use list_attachments_response::ListAttachmentsResponse;
pub use list_collections_response::ListCollectionsResponse;
pub use list_documents_response::{
    ListDocumentsResponse, ListDocumentsResponseAttributes, ListDocumentsResponseEntities,
};
pub use list_permissions_response::ListPermissionsResponse;
pub use list_stored_procedures_response::ListStoredProceduresResponse;
pub use list_triggers_response::ListTriggersResponse;
pub use list_user_defined_functions_response::ListUserDefinedFunctionsResponse;
pub use query_documents_response::{
    QueryDocumentsResponse, QueryDocumentsResponseDocuments, QueryDocumentsResponseRaw,
    QueryResponseMeta, QueryResult,
};
pub use replace_document_response::ReplaceDocumentResponse;
pub use replace_stored_procedure_response::ReplaceStoredProcedureResponse;
