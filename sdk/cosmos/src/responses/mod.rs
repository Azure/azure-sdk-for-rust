//! Responses from any call to the Cosmos API.

#![allow(missing_docs)]

mod create_collection_response;
mod create_database_response;
mod create_document_response;
mod create_permission_response;
mod create_reference_attachment_response;
mod create_slug_attachment_response;
mod create_stored_procedure_response;
mod create_trigger_response;
mod create_user_defined_function_response;
mod create_user_response;
mod delete_attachment_response;
mod delete_collection_response;
mod delete_database_response;
mod delete_document_response;
mod delete_permission_response;
mod delete_stored_procedure_response;
mod delete_trigger_response;
mod delete_user_defined_function_response;
mod delete_user_response;
mod execute_stored_procedure_response;
mod get_attachment_response;
mod get_collection_response;
mod get_database_response;
mod get_document_response;
mod get_partition_key_ranges_response;
mod get_permission_response;
mod list_attachments_response;
mod list_collections_response;
mod list_databases_response;
mod list_documents_response;
mod list_permissions_response;
mod list_stored_procedures_response;
mod list_triggers_response;
mod list_user_defined_functions_response;
mod list_users_response;
mod query_documents_response;
mod replace_document_response;
mod replace_permission_response;
mod replace_reference_attachment_response;
mod replace_stored_procedure_response;

pub use create_collection_response::CreateCollectionResponse;
pub use create_database_response::CreateDatabaseResponse;
pub use create_document_response::CreateDocumentResponse;
pub use create_permission_response::CreatePermissionResponse;
pub use create_reference_attachment_response::CreateReferenceAttachmentResponse;
pub use create_slug_attachment_response::CreateSlugAttachmentResponse;
pub use create_stored_procedure_response::CreateStoredProcedureResponse;
pub use create_trigger_response::CreateTriggerResponse;
pub use create_user_defined_function_response::CreateUserDefinedFunctionResponse;
pub use create_user_response::CreateUserResponse;
pub use delete_attachment_response::DeleteAttachmentResponse;
pub use delete_collection_response::DeleteCollectionResponse;
pub use delete_database_response::DeleteDatabaseResponse;
pub use delete_document_response::DeleteDocumentResponse;
pub use delete_permission_response::DeletePermissionResponse;
pub use delete_stored_procedure_response::DeleteStoredProcedureResponse;
pub use delete_trigger_response::DeleteTriggerResponse;
pub use delete_user_defined_function_response::DeleteUserDefinedFunctionResponse;
pub use delete_user_response::DeleteUserResponse;
pub use execute_stored_procedure_response::ExecuteStoredProcedureResponse;
pub use get_attachment_response::GetAttachmentResponse;
pub use get_collection_response::GetCollectionResponse;
pub use get_database_response::GetDatabaseResponse;
pub use get_document_response::GetDocumentResponse;
pub use get_partition_key_ranges_response::GetPartitionKeyRangesResponse;
pub use get_permission_response::GetPermissionResponse;
pub use list_attachments_response::ListAttachmentsResponse;
pub use list_collections_response::ListCollectionsResponse;
pub use list_databases_response::ListDatabasesResponse;
pub use list_documents_response::{
    ListDocumentsResponse, ListDocumentsResponseAttributes, ListDocumentsResponseEntities,
};
pub use list_permissions_response::ListPermissionsResponse;
pub use list_stored_procedures_response::ListStoredProceduresResponse;
pub use list_triggers_response::ListTriggersResponse;
pub use list_user_defined_functions_response::ListUserDefinedFunctionsResponse;
pub use list_users_response::ListUsersResponse;
pub use query_documents_response::{
    QueryDocumentsResponse, QueryDocumentsResponseDocuments, QueryDocumentsResponseRaw,
    QueryResponseMeta, QueryResult,
};
pub use replace_document_response::ReplaceDocumentResponse;
pub use replace_permission_response::ReplacePermissionResponse;
pub use replace_reference_attachment_response::ReplaceReferenceAttachmentResponse;
pub use replace_stored_procedure_response::ReplaceStoredProcedureResponse;
