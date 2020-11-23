use crate::responses::*;
use serde::de::DeserializeOwned;
use std::convert::From;

#[derive(Debug, Clone, PartialEq)]
pub enum ConsistencyLevel {
    Strong,
    Bounded,
    Session(String),
    ConsistentPrefix,
    Eventual,
}

impl ConsistencyLevel {
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

impl From<String> for ConsistencyLevel {
    fn from(session_token: String) -> Self {
        ConsistencyLevel::Session(session_token)
    }
}

impl From<&str> for ConsistencyLevel {
    fn from(session_token: &str) -> Self {
        ConsistencyLevel::Session(session_token.to_owned())
    }
}

impl From<&String> for ConsistencyLevel {
    fn from(session_token: &String) -> Self {
        ConsistencyLevel::Session(session_token.clone())
    }
}

macro_rules! implement_from {
    ($response_type:path) => {
        impl From<&$response_type> for ConsistencyLevel {
            fn from(a: &$response_type) -> Self {
                ConsistencyLevel::Session(a.session_token.clone())
            }
        }

        impl From<$response_type> for ConsistencyLevel {
            fn from(a: $response_type) -> Self {
                ConsistencyLevel::Session(a.session_token.clone())
            }
        }
    };
    ($response_type:ident, $generic:tt) => {
        impl<$generic> From<&$response_type<$generic>> for ConsistencyLevel {
            fn from(a: &$response_type<$generic>) -> Self {
                ConsistencyLevel::Session(a.session_token.clone())
            }
        }

        impl<$generic> From<$response_type<$generic>> for ConsistencyLevel {
            fn from(a: $response_type<$generic>) -> Self {
                ConsistencyLevel::Session(a.session_token.clone())
            }
        }
    };
}

implement_from!(CreateSlugAttachmentResponse);
implement_from!(GetDatabaseResponse);
implement_from!(GetCollectionResponse);
implement_from!(CreateUserResponse);
implement_from!(DeleteAttachmentResponse);
implement_from!(ReplaceReferenceAttachmentResponse);
implement_from!(CreateReferenceAttachmentResponse);
implement_from!(ListAttachmentsResponse);
implement_from!(GetAttachmentResponse);
implement_from!(CreateDocumentResponse);
implement_from!(ReplaceDocumentResponse);
implement_from!(DeleteDocumentResponse);
implement_from!(CreateUserDefinedFunctionResponse);
implement_from!(DeleteUserDefinedFunctionResponse);
implement_from!(ListUserDefinedFunctionsResponse);
implement_from!(CreateTriggerResponse);
implement_from!(ListTriggersResponse);
implement_from!(DeleteTriggerResponse);
implement_from!(ListDocumentsResponse, T);
implement_from!(QueryDocumentsResponse, T);
implement_from!(QueryDocumentsResponseRaw, T);
implement_from!(QueryDocumentsResponseDocuments, T);

impl<T> From<&GetDocumentResponse<T>> for ConsistencyLevel {
    fn from(get_document_response: &GetDocumentResponse<T>) -> Self {
        match get_document_response {
            GetDocumentResponse::Found(response) => {
                ConsistencyLevel::Session(response.session_token.clone())
            }
            GetDocumentResponse::NotFound(response) => {
                ConsistencyLevel::Session(response.session_token.clone())
            }
        }
    }
}

impl<T> From<&ExecuteStoredProcedureResponse<T>> for ConsistencyLevel
where
    T: DeserializeOwned,
{
    fn from(execute_stored_procedure_response: &ExecuteStoredProcedureResponse<T>) -> Self {
        ConsistencyLevel::Session(execute_stored_procedure_response.session_token.clone())
    }
}
