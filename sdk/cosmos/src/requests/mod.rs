//! Request builder objects for every kind of request.
//!
//! These objects are usually created by calling some sort of method on a client. They
//! then give you the ability to modify your request with certain options and finally
//! execute the request with the `execute` method.

#![allow(missing_docs)]

mod create_collection_builder;
mod create_database_builder;
mod create_document_builder;
mod create_or_replace_trigger_builder;
mod create_or_replace_user_defined_function_builder;
mod create_permission_builder;
mod create_reference_attachment_builder;
mod create_slug_attachment_builder;
mod create_stored_procedure_builder;
mod create_user_builder;
mod delete_attachment_builder;
mod delete_collection_builder;
mod delete_database_builder;
mod delete_document_builder;
mod delete_permission_builder;
mod delete_stored_procedure_builder;
mod delete_trigger_builder;
mod delete_user_builder;
mod delete_user_defined_function_builder;
mod execute_stored_procedure_builder;
mod get_attachment_builder;
mod get_collection_builder;
mod get_database_builder;
mod get_document_builder;
mod get_partition_key_ranges_builder;
mod get_permission_builer;
mod get_user_builder;
mod list_attachments_builder;
mod list_collections_builder;
mod list_databases_builder;
mod list_documents_builder;
mod list_permissions_builder;
mod list_stored_procedures_builder;
mod list_triggers_builder;
mod list_user_defined_functions_builder;
mod list_users_builder;
mod query_documents_builder;
mod replace_collection_builder;
mod replace_document_builder;
mod replace_permission_builder;
mod replace_reference_attachment_builder;
mod replace_slug_attachment_builder;
mod replace_stored_procedure_builder;
mod replace_user_builder;

pub use create_collection_builder::CreateCollectionBuilder;
pub use create_database_builder::CreateDatabaseBuilder;
pub use create_document_builder::CreateDocumentBuilder;
pub use create_or_replace_trigger_builder::CreateOrReplaceTriggerBuilder;
pub use create_or_replace_user_defined_function_builder::CreateOrReplaceUserDefinedFunctionBuilder;
pub use create_permission_builder::CreatePermissionBuilder;
pub use create_reference_attachment_builder::CreateReferenceAttachmentBuilder;
pub use create_slug_attachment_builder::CreateSlugAttachmentBuilder;
pub use create_stored_procedure_builder::CreateStoredProcedureBuilder;
pub use create_user_builder::CreateUserBuilder;
pub use delete_attachment_builder::DeleteAttachmentBuilder;
pub use delete_collection_builder::DeleteCollectionBuilder;
pub use delete_database_builder::DeleteDatabaseBuilder;
pub use delete_document_builder::DeleteDocumentBuilder;
pub use delete_permission_builder::DeletePermissionsBuilder;
pub use delete_stored_procedure_builder::DeleteStoredProcedureBuilder;
pub use delete_trigger_builder::DeleteTriggerBuilder;
pub use delete_user_builder::DeleteUserBuilder;
pub use delete_user_defined_function_builder::DeleteUserDefinedFunctionBuilder;
pub use execute_stored_procedure_builder::ExecuteStoredProcedureBuilder;
pub use get_attachment_builder::GetAttachmentBuilder;
pub use get_collection_builder::GetCollectionBuilder;
pub use get_database_builder::GetDatabaseBuilder;
pub use get_document_builder::GetDocumentBuilder;
pub use get_partition_key_ranges_builder::GetPartitionKeyRangesBuilder;
pub use get_permission_builer::GetPermissionBuilder;
pub use get_user_builder::GetUserBuilder;
pub use list_attachments_builder::ListAttachmentsBuilder;
pub use list_collections_builder::ListCollectionsBuilder;
pub use list_databases_builder::ListDatabasesBuilder;
pub use list_documents_builder::ListDocumentsBuilder;
pub use list_permissions_builder::ListPermissionsBuilder;
pub use list_stored_procedures_builder::ListStoredProceduresBuilder;
pub use list_triggers_builder::ListTriggersBuilder;
pub use list_user_defined_functions_builder::ListUserDefinedFunctionsBuilder;
pub use list_users_builder::ListUsersBuilder;
pub use query_documents_builder::QueryDocumentsBuilder;
pub use replace_collection_builder::ReplaceCollectionBuilder;
pub use replace_document_builder::ReplaceDocumentBuilder;
pub use replace_permission_builder::ReplacePermissionBuilder;
pub use replace_reference_attachment_builder::ReplaceReferenceAttachmentBuilder;
pub use replace_slug_attachment_builder::ReplaceSlugAttachmentBuilder;
pub use replace_stored_procedure_builder::ReplaceStoredProcedureBuilder;
pub use replace_user_builder::ReplaceUserBuilder;
