use crate::prelude::*;
use crate::responses::GetCollectionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetCollectionBuilder<'a> {
    collection_client: &'a CollectionClient,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
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
}

impl<'a> GetCollectionBuilder<'a> {
    pub fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }
}

impl<'a> UserAgentOption<'a> for GetCollectionBuilder<'a> {
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a> ActivityIdOption<'a> for GetCollectionBuilder<'a> {
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a> ConsistencyLevelOption<'a> for GetCollectionBuilder<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a> UserAgentSupport<'a> for GetCollectionBuilder<'a> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a> ActivityIdSupport<'a> for GetCollectionBuilder<'a> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a> ConsistencyLevelSupport<'a> for GetCollectionBuilder<'a> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a> GetCollectionBuilder<'a> {
    pub async fn execute(&self) -> Result<GetCollectionResponse, CosmosError> {
        trace!("GetCollectionResponse::execute called");

        let request = self
            .collection_client()
            .prepare_request_with_collection_name(http::Method::GET);

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .collection_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
