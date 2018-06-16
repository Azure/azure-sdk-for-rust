use azure::core::{errors::AzureError, util::HeaderMapExt};
use azure::cosmos::{client::headers::HEADER_REQUEST_CHARGE, collection::Collection, database::Database, document::DocumentAttributes};
use serde::de::DeserializeOwned;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ListDatabasesResponse {
    _rid: String,
    #[serde(rename = "Databases")]
    pub databases: Vec<Database>,
    #[serde(rename = "_count")]
    pub count: u32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ListCollectionsResponse {
    _rid: String,
    #[serde(rename = "DocumentCollections")]
    pub collections: Vec<Collection>,
    #[serde(rename = "_count")]
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDocumentsResponseAttributes {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "Documents")]
    pub documents: Vec<DocumentAttributes>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDocumentsResponseEntities<T> {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "Documents")]
    pub entities: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document<T> {
    pub document_attributes: DocumentAttributes,
    pub entity: T,
}

impl<T: DeserializeOwned> Document<T> {
    pub(crate) fn from_json(json: &[u8]) -> Result<Document<T>, AzureError> {
        Ok(Document {
            document_attributes: ::serde_json::from_slice::<DocumentAttributes>(json)?,
            entity: ::serde_json::from_slice::<T>(json)?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult<T> {
    pub document_attributes: Option<DocumentAttributes>,
    pub result: T,
}

#[derive(Debug, Clone)]
pub struct ListDocumentsResponseAdditionalHeaders {
    pub continuation_token: Option<String>,
    pub charge: f64,
    pub etag: Option<String>,
}

#[derive(Debug, Clone)]
pub struct QueryDocumentResponseAdditonalHeaders {
    pub continuation_token: Option<String>,
    pub charge: f64,
}

#[derive(Debug, Clone)]
pub struct QueryDocumentResponse<T> {
    pub query_response_meta: QueryResponseMeta,
    pub results: Vec<QueryResult<T>>,
    pub additional_headers: QueryDocumentResponseAdditonalHeaders,
}

#[derive(Debug, Clone)]
pub struct ListDocumentsResponse<T> {
    pub rid: String,
    pub documents: Vec<Document<T>>,
    pub additional_headers: ListDocumentsResponseAdditionalHeaders,
}

#[derive(Debug, Clone)]
pub struct DocumentAdditionalHeaders {
    pub charge: f64,
}

impl DocumentAdditionalHeaders {
    pub(crate) fn derive_from(headers: &::hyper::HeaderMap) -> DocumentAdditionalHeaders {
        DocumentAdditionalHeaders {
            charge: headers.get_as_str(HEADER_REQUEST_CHARGE).unwrap().parse::<f64>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetDocumentResponse<T> {
    pub document: Option<Document<T>>,
    pub additional_headers: DocumentAdditionalHeaders,
}

#[derive(Debug, Clone)]
pub struct ReplaceDocumentResponse<T> {
    pub document: Document<T>,
    pub additional_headers: DocumentAdditionalHeaders,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryResponseMeta {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_count")]
    pub count: u64,
}

#[derive(Debug, Clone)]
pub struct ExecuteStoredProcedureResponse<T> {
    pub result: T,
    pub additional_headers: DocumentAdditionalHeaders,
}