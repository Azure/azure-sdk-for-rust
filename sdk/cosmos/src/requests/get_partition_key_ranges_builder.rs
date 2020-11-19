use crate::prelude::*;
use crate::responses::GetPartitionKeyRangesResponse;
use crate::ResourceType;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetPartitionKeyRangesBuilder<'a, 'b> {
    collection_client: &'a CollectionClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
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
}

impl<'a, 'b> CollectionClientRequired<'a> for GetPartitionKeyRangesBuilder<'a, 'b> {
    fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }
}

impl<'a, 'b> IfMatchConditionOption<'b> for GetPartitionKeyRangesBuilder<'a, 'b> {
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b> IfModifiedSinceOption<'b> for GetPartitionKeyRangesBuilder<'a, 'b> {
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b> UserAgentOption<'b> for GetPartitionKeyRangesBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b> ActivityIdOption<'b> for GetPartitionKeyRangesBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b> ConsistencyLevelOption<'b> for GetPartitionKeyRangesBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> IfMatchConditionSupport<'b> for GetPartitionKeyRangesBuilder<'a, 'b> {
    type O = Self;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }
}

impl<'a, 'b> IfModifiedSinceSupport<'b> for GetPartitionKeyRangesBuilder<'a, 'b> {
    type O = Self;

    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        Self {
            if_modified_since: Some(if_modified_since),
            ..self
        }
    }
}

impl<'a, 'b> UserAgentSupport<'b> for GetPartitionKeyRangesBuilder<'a, 'b> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for GetPartitionKeyRangesBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for GetPartitionKeyRangesBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> GetPartitionKeyRangesBuilder<'a, 'b> {
    pub async fn execute(&self) -> Result<GetPartitionKeyRangesResponse, AzureError> {
        trace!("GetPartitionKeyRangesBuilder::execute called");

        let request = self.collection_client().cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/pkranges",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name()
            ),
            hyper::Method::GET,
            ResourceType::PartitionKeyRanges,
        );

        let request = request.header(hyper::header::CONTENT_LENGTH, "0");
        let request = IfMatchConditionOption::add_header(self, request);
        let request = IfModifiedSinceOption::add_header(self, request);
        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        let future_response = self.collection_client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
