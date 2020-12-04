use crate::prelude::*;
use crate::responses::DeleteCollectionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteCollectionBuilder<'a> {
    collection_client: &'a CollectionClient,
    user_agent: Option<azure_core::UserAgent<'a>>,
    activity_id: Option<azure_core::ActivityId<'a>>,
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

    pub fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }

    pub fn with_user_agent(self, user_agent: &'a str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'a str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }

    pub async fn execute(&self) -> Result<DeleteCollectionResponse, CosmosError> {
        trace!("DeleteCollectionBuilder::execute called");

        let request = self
            .collection_client()
            .prepare_request_with_collection_name(http::Method::DELETE);

        let request = crate::headers::add_optional_header(self.user_agent(), request);
        let request = crate::headers::add_optional_header(self.activity_id(), request);
        let request = crate::headers::add_optional_header(self.consistency_level(), request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .collection_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }

    fn user_agent(&self) -> Option<azure_core::UserAgent<'a>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'a>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}
