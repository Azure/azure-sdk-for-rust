use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::GetPartitionKeyRangesResponse;
use azure_core::prelude::*;
use chrono::{DateTime, Utc};
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetPartitionKeyRangesBuilder<'a, 'b> {
    collection_client: &'a CollectionClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<IfModifiedSince<'b>>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> GetPartitionKeyRangesBuilder<'a, 'b> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    pub fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }

    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }

    fn user_agent(&self) -> Option<UserAgent<'b>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<ActivityId<'b>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }

    pub fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }

    pub fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self {
        Self {
            if_modified_since: Some(IfModifiedSince::new(if_modified_since)),
            ..self
        }
    }

    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'b str) -> Self {
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
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> GetPartitionKeyRangesBuilder<'a, 'b> {
    pub async fn execute(&self) -> Result<GetPartitionKeyRangesResponse, CosmosError> {
        trace!("GetPartitionKeyRangesBuilder::execute called");

        let request = self.collection_client().cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/pkranges",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name()
            ),
            http::Method::GET,
            ResourceType::PartitionKeyRanges,
        );

        let request = request.header(http::header::CONTENT_LENGTH, "0");
        let request = crate::headers::add_header(self.if_match_condition(), request);
        let request = crate::headers::add_header(self.if_modified_since(), request);
        let request = crate::headers::add_header(self.user_agent(), request);
        let request = crate::headers::add_header(self.activity_id(), request);
        let request = crate::headers::add_header(self.consistency_level(), request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .collection_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}

impl<'a, 'b> GetPartitionKeyRangesBuilder<'a, 'b> {
    fn if_modified_since(&self) -> Option<IfModifiedSince> {
        self.if_modified_since.clone()
    }
}
