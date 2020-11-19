use crate::prelude::*;
use crate::responses::DeleteCollectionResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteCollectionBuilder<'a> {
    collection_client: &'a CollectionClient,
    user_agent: Option<&'a str>,
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

impl<'a> CollectionClientRequired<'a> for DeleteCollectionBuilder<'a> {
    fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }
}

impl<'a> UserAgentOption<'a> for DeleteCollectionBuilder<'a> {
    fn user_agent(&self) -> Option<&'a str> {
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
            user_agent: Some(user_agent),
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
    pub async fn execute(&self) -> Result<DeleteCollectionResponse, AzureError> {
        trace!("DeleteCollectionBuilder::execute called");

        let request = self
            .collection_client()
            .prepare_request_with_collection_name(hyper::Method::DELETE);

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.collection_client().hyper_client().request(request),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
