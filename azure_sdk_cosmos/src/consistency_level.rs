use crate::responses::{
    CreateDocumentResponse, DeleteDocumentResponse, ExecuteStoredProcedureResponse,
    GetDocumentResponse, ListDocumentsResponse, QueryDocumentsResponse, ReplaceDocumentResponse,
};
use serde::de::DeserializeOwned;
use std::convert::From;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConsistencyLevel<'a> {
    Strong,
    Bounded,
    Session(&'a str),
    ConsistentPrefix,
    Eventual,
}

impl<'a> ConsistencyLevel<'a> {
    pub fn to_consistency_level_header(&self) -> &'static str {
        match self {
            Self::Strong => "Strong",
            Self::Bounded => "Bounded",
            Self::Session(_) => "Session",
            Self::ConsistentPrefix => "Prefix", //this is guessed since it's missing here: https://docs.microsoft.com/en-us/rest/api/cosmos-db/common-cosmosdb-rest-request-headers
            Self::Eventual => "Eventual",
        }
    }
}

impl<'a, T> From<&'a ListDocumentsResponse<T>> for ConsistencyLevel<'a> {
    fn from(list_documents_response: &'a ListDocumentsResponse<T>) -> Self {
        ConsistencyLevel::Session(&list_documents_response.additional_headers.session_token)
    }
}

impl<'a, T> From<&'a QueryDocumentsResponse<T>> for ConsistencyLevel<'a> {
    fn from(query_documents_response: &'a QueryDocumentsResponse<T>) -> Self {
        ConsistencyLevel::Session(&query_documents_response.additional_headers.session_token)
    }
}

impl<'a, T> From<&'a GetDocumentResponse<T>> for ConsistencyLevel<'a> {
    fn from(get_document_response: &'a GetDocumentResponse<T>) -> Self {
        ConsistencyLevel::Session(&get_document_response.additional_headers.session_token)
    }
}

impl<'a> From<&'a CreateDocumentResponse> for ConsistencyLevel<'a> {
    fn from(create_document_response: &'a CreateDocumentResponse) -> Self {
        ConsistencyLevel::Session(&create_document_response.additional_headers.session_token)
    }
}

impl<'a> From<&'a ReplaceDocumentResponse> for ConsistencyLevel<'a> {
    fn from(replace_document_response: &'a ReplaceDocumentResponse) -> Self {
        ConsistencyLevel::Session(&replace_document_response.additional_headers.session_token)
    }
}

impl<'a> From<&'a DeleteDocumentResponse> for ConsistencyLevel<'a> {
    fn from(delete_document_response: &'a DeleteDocumentResponse) -> Self {
        ConsistencyLevel::Session(&delete_document_response.session_token)
    }
}

impl<'a, T> From<&'a ExecuteStoredProcedureResponse<T>> for ConsistencyLevel<'a>
where
    T: DeserializeOwned,
{
    fn from(execute_stored_procedure_response: &'a ExecuteStoredProcedureResponse<T>) -> Self {
        ConsistencyLevel::Session(&execute_stored_procedure_response.session_token)
    }
}
