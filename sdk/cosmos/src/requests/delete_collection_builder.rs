use crate::prelude::*;
use crate::responses::DeleteCollectionResponse;
use azure_core::prelude::*;

use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteCollectionBuilder<'a> {
    collection_client: &'a CollectionClient,
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> DeleteCollectionBuilder<'a> {
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

    pub async fn execute(&self) -> Result<DeleteCollectionResponse, CosmosError> {
        trace!("DeleteCollectionBuilder::execute called");

        let request = self
            .collection_client
            .prepare_request_with_collection_name(http::Method::DELETE);

        let request = crate::headers::add_header(self.user_agent, request);
        let request = crate::headers::add_header(self.activity_id, request);
        let request = crate::headers::add_header(self.consistency_level.clone(), request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .collection_client
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
