use crate::headers::from_headers::{activity_id_from_headers, request_charge_from_headers};
use crate::prelude::*;
use crate::resources::User;
use azure_core::{
    collect_pinned_stream,
    headers::{continuation_token_from_headers_optional, session_token_from_headers},
    prelude::{Continuation, MaxItemCount},
    Request as HttpRequest, Response as HttpResponse, SessionToken,
};

#[derive(Debug, Clone)]
pub struct ListUsersOptions<'a> {
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<Continuation<'a>>,
    max_item_count: MaxItemCount,
}

impl<'a> ListUsersOptions<'a> {
    pub fn new() -> Self {
        Self {
            consistency_level: None,
            continuation: None,
            max_item_count: MaxItemCount::new(-1),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        continuation: Continuation<'a> => Some(continuation),
        max_item_count: MaxItemCount => max_item_count,
    }

    pub(crate) fn decorate_request(&self, request: &mut HttpRequest) -> Result<(), crate::Error> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        azure_core::headers::add_optional_header2(&self.continuation, request)?;
        azure_core::headers::add_optional_header2(&Some(self.max_item_count), request)?;
        request.set_body(bytes::Bytes::from_static(&[]).into());

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ListUsersResponse {
    #[serde(rename = "Users")]
    pub users: Vec<User>,
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_count")]
    pub count: u64,

    #[serde(skip_deserializing)]
    pub charge: f64,
    #[serde(skip_deserializing)]
    pub activity_id: uuid::Uuid,
    #[serde(skip_deserializing)]
    pub session_token: SessionToken,
    #[serde(skip_deserializing)]
    pub continuation_token: Option<String>,
}

impl ListUsersResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        let mut list_users_response: ListUsersResponse = serde_json::from_slice(&body)?;
        list_users_response.charge = request_charge_from_headers(&headers)?;
        list_users_response.activity_id = activity_id_from_headers(&headers)?;
        list_users_response.continuation_token =
            continuation_token_from_headers_optional(&headers)?;
        list_users_response.session_token = session_token_from_headers(&headers)?;

        Ok(list_users_response)
    }
}
