use crate::headers;
use crate::responses::*;
use azure_core::AddAsHeader;
use http::request;
use serde::de::DeserializeOwned;

/// The consistency guarantee provided by Cosmos.
///
/// You can learn more about consistency levels in Cosmos [here](https://docs.microsoft.com/azure/cosmos-db/consistency-levels).
#[derive(Debug, Clone, PartialEq)]
pub enum ConsistencyLevel {
    /// A linearizability guarantee
    Strong,
    /// Reads are guaranteed to honor the consistent-prefix guarantee
    Bounded,
    /// Within a single client session reads are guaranteed to honor the consistent-prefix, monotonic reads, monotonic writes, read-your-writes, and write-follows-reads guarantees.
    Session(String),
    /// Updates that are returned contain some prefix of all the updates, with no gaps.
    ConsistentPrefix,
    /// No ordering guarantee for reads
    Eventual,
}

impl ConsistencyLevel {
    pub(crate) fn to_consistency_level_header(&self) -> &'static str {
        match self {
            Self::Strong => "Strong",
            Self::Bounded => "Bounded",
            Self::Session(_) => "Session",
            Self::ConsistentPrefix => "Prefix", //this is guessed since it's missing here: https://docs.microsoft.com/rest/api/cosmos-db/common-cosmosdb-rest-request-headers
            Self::Eventual => "Eventual",
        }
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

impl AddAsHeader for ConsistencyLevel {
    fn add_as_header(&self, builder: request::Builder) -> request::Builder {
        let builder = builder.header(
            headers::HEADER_CONSISTENCY_LEVEL,
            self.to_consistency_level_header(),
        );

        // if we have a Session consistency level we make sure to pass
        // the x-ms-session-token header too.
        if let ConsistencyLevel::Session(session_token) = self {
            builder.header(headers::HEADER_SESSION_TOKEN, session_token)
        } else {
            builder
        }
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HTTPHeaderError> {
        request.headers_mut().append(
            headers::HEADER_CONSISTENCY_LEVEL,
            http::header::HeaderValue::from_str(self.to_consistency_level_header())?,
        );

        // if we have a Session consistency level we make sure to pass
        // the x-ms-session-token header too.
        if let ConsistencyLevel::Session(session_token) = self {
            request.headers_mut().append(
                headers::HEADER_SESSION_TOKEN,
                http::header::HeaderValue::from_str(session_token)?,
            );
        }

        Ok(())
    }
}
