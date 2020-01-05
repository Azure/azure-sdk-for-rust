mod create_collection_builder;
mod create_database_builder;
mod create_document_builder;
mod create_user_builder;
mod delete_collection_builder;
mod delete_database_builder;
mod delete_document_builder;
mod execute_stored_procedure_builder;
mod get_collection_builder;
mod get_database_builder;
mod get_document_builder;
mod list_collections_builder;
mod list_databases_builder;
mod list_documents_builder;
mod query_documents_builder;
mod replace_collection_builder;
mod replace_document_builder;
pub use self::create_collection_builder::CreateCollectionBuilder;
pub use self::create_database_builder::CreateDatabaseBuilder;
pub use self::create_document_builder::CreateDocumentBuilder;
pub use self::create_user_builder::CreateUserBuilder;
pub use self::delete_collection_builder::DeleteCollectionBuilder;
pub use self::delete_database_builder::DeleteDatabaseBuilder;
pub use self::delete_document_builder::DeleteDocumentBuilder;
pub use self::execute_stored_procedure_builder::ExecuteStoredProcedureBuilder;
pub use self::get_collection_builder::GetCollectionBuilder;
pub use self::get_database_builder::GetDatabaseBuilder;
pub use self::get_document_builder::GetDocumentBuilder;
pub use self::list_collections_builder::ListCollectionsBuilder;
pub use self::list_databases_builder::ListDatabasesBuilder;
pub use self::list_documents_builder::ListDocumentsBuilder;
pub use self::query_documents_builder::QueryDocumentsBuilder;
pub use self::replace_collection_builder::ReplaceCollectionBuilder;
pub use self::replace_document_builder::ReplaceDocumentBuilder;
use crate::headers::*;
use azure_sdk_core::errors::AzureError;
use http::HeaderMap;

pub fn request_charge_from_headers(headers: &HeaderMap) -> Result<f64, AzureError> {
    Ok(headers
        .get(HEADER_REQUEST_CHARGE)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_REQUEST_CHARGE.to_owned()))?
        .to_str()?
        .parse()?)
}

pub fn request_item_count_from_headers(headers: &HeaderMap) -> Result<u64, AzureError> {
    Ok(headers
        .get(HEADER_ITEM_COUNT)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_ITEM_COUNT.to_owned()))?
        .to_str()?
        .parse()?)
}

pub fn number_of_read_regions_from_headers(headers: &HeaderMap) -> Result<u32, AzureError> {
    Ok(headers
        .get(HEADER_NUMBER_OF_READ_REGIONS)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_NUMBER_OF_READ_REGIONS.to_owned()))?
        .to_str()?
        .parse()?)
}

pub fn activity_id_from_headers(headers: &HeaderMap) -> Result<uuid::Uuid, AzureError> {
    let s = headers
        .get(HEADER_ACTIVITY_ID)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_ACTIVITY_ID.to_owned()))?
        .to_str()?;
    Ok(uuid::Uuid::parse_str(s)?)
}
