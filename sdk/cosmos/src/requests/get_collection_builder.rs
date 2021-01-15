use crate::prelude::*;
use crate::responses::GetCollectionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetCollectionBuilder<'a> {
    collection_client: &'a CollectionClient,
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> GetCollectionBuilder<'a> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    setters! {
        user_agent: &'a str => Some(UserAgent::new(user_agent)),
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub async fn execute(&self) -> Result<GetCollectionResponse, CosmosError> {
        trace!("GetCollectionResponse::execute called");

        let request = self
            .collection_client
            .prepare_request_with_collection_name(http::Method::GET);

        let request = azure_core::headers::add_optional_header(&self.user_agent, request);
        let request = azure_core::headers::add_optional_header(&self.activity_id, request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level, request);

        let request = request.body(bytes::Bytes::from_static(EMPTY_BODY))?;

        Ok(self
            .collection_client
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
