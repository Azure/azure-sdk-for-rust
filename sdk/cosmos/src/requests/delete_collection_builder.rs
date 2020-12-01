use crate::prelude::*;
use crate::responses::DeleteCollectionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteCollectionBuilder<'a> {
    collection_client: &'a CollectionClient,
    user_agent: Option<azure_core::UserAgent<'a>>,
    activity_id: Option<&'a str>,
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
}

impl<'a> DeleteCollectionBuilder<'a> {
    pub fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }
}

impl<'a> DeleteCollectionBuilder<'a> {
    fn user_agent(&self) -> Option<azure_core::UserAgent<'a>> {
        self.user_agent
    }
}

impl<'a> ActivityIdOption<'a> for DeleteCollectionBuilder<'a> {
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a> ConsistencyLevelOption<'a> for DeleteCollectionBuilder<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a> UserAgentSupport<'a> for DeleteCollectionBuilder<'a> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a> ActivityIdSupport<'a> for DeleteCollectionBuilder<'a> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a> ConsistencyLevelSupport<'a> for DeleteCollectionBuilder<'a> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a> DeleteCollectionBuilder<'a> {
    pub async fn execute(&self) -> Result<DeleteCollectionResponse, CosmosError> {
        trace!("DeleteCollectionBuilder::execute called");

        let request = self
            .collection_client()
            .prepare_request_with_collection_name(http::Method::DELETE);

        let request = crate::headers::add_header(self.user_agent(), request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .collection_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
