use azure::cosmos::collection::Collection;
use azure::cosmos::database::Database;
use azure::cosmos::document::DocumentAttributes;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ListDatabasesResponse {
    _rid: String,
    #[serde(rename = "Databases")]
    pub databases: Vec<Database>,
    #[serde(rename = "_count")]
    pub count: u32,
}

#[derive(Serialize, Debug)]
pub struct CreateDatabaseRequest<'a> {
    pub id: &'a str,
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
pub struct GetDocumentAdditionalHeaders {
    pub charge: f64,
}

#[derive(Debug, Clone)]
pub struct GetDocumentResponse<T> {
    pub document: Option<Document<T>>,
    pub additional_headers: GetDocumentAdditionalHeaders,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryResponseMeta {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_count")]
    pub count: u64,
}
