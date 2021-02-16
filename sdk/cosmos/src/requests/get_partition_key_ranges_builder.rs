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

    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
        if_modified_since: &'b DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
    }

    pub async fn execute(&self) -> Result<GetPartitionKeyRangesResponse, CosmosError> {
        trace!("GetPartitionKeyRangesBuilder::execute called");

        let request = self.collection_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/pkranges",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name()
            ),
            http::Method::GET,
            ResourceType::PartitionKeyRanges,
        );

        let request = request.header(http::header::CONTENT_LENGTH, "0");
        let request = azure_core::headers::add_optional_header(&self.if_match_condition, request);
        let request = azure_core::headers::add_optional_header(&self.if_modified_since, request);
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
