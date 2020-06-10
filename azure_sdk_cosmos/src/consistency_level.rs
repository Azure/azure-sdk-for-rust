use crate::responses::*;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::convert::From;

#[derive(Debug, Clone, PartialEq)]
pub enum ConsistencyLevel<'a> {
    Strong,
    Bounded,
    Session(Cow<'a, str>),
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

impl From<String> for ConsistencyLevel<'_> {
    fn from(session_token: String) -> Self {
        ConsistencyLevel::Session(Cow::from(session_token))
    }
}

impl<'a> From<&'a str> for ConsistencyLevel<'a> {
    fn from(session_token: &'a str) -> Self {
        ConsistencyLevel::Session(Cow::from(session_token))
    }
}

impl<'a> From<&'a String> for ConsistencyLevel<'a> {
    fn from(session_token: &'a String) -> Self {
        ConsistencyLevel::Session(Cow::from(session_token))
    }
}

macro_rules! implement_from {
    ($response_type:ident) => {
        impl<'a> From<&'a $response_type> for ConsistencyLevel<'a> {
            fn from(a: &'a $response_type) -> Self {
                ConsistencyLevel::Session(Cow::from(&a.session_token))
            }
        }

        impl<'a> From<$response_type> for ConsistencyLevel<'a> {
            fn from(a: $response_type) -> Self {
                ConsistencyLevel::Session(Cow::from(a.session_token))
            }
        }
    };
    ($response_type:ident, $generic:tt) => {
        impl<'a, $generic> From<&'a $response_type<$generic>> for ConsistencyLevel<'a> {
            fn from(a: &'a $response_type<$generic>) -> Self {
                ConsistencyLevel::Session(Cow::from(&a.session_token))
            }
        }

        impl<'a, $generic> From<$response_type<$generic>> for ConsistencyLevel<'a> {
            fn from(a: $response_type<$generic>) -> Self {
                ConsistencyLevel::Session(Cow::from(a.session_token))
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

impl<'a, T> From<&'a GetDocumentResponse<T>> for ConsistencyLevel<'a> {
    fn from(get_document_response: &'a GetDocumentResponse<T>) -> Self {
        match get_document_response {
            GetDocumentResponse::Found(response) => {
                ConsistencyLevel::Session(Cow::from(&response.session_token))
            }
            GetDocumentResponse::NotFound(response) => {
                ConsistencyLevel::Session(Cow::from(&response.session_token))
            }
        }
    }
}

impl<'a, T> From<&'a ExecuteStoredProcedureResponse<T>> for ConsistencyLevel<'a>
where
    T: DeserializeOwned,
{
    fn from(execute_stored_procedure_response: &'a ExecuteStoredProcedureResponse<T>) -> Self {
        ConsistencyLevel::Session(Cow::from(&execute_stored_procedure_response.session_token))
    }
}
