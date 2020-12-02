use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::ListDocumentsResponse;
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListDocumentsBuilder<'a, 'b> {
    collection_client: &'a CollectionClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<&'b str>,
    max_item_count: MaxItemCount,
    a_im: ChangeFeed,
    partition_range_id: Option<PartitionRangeId<'b>>,
}

impl<'a, 'b> ListDocumentsBuilder<'a, 'b> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: MaxItemCount::new(-1),
            a_im: ChangeFeed::None,
            partition_range_id: None,
        }
    }

    pub fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }

    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }

    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }

    fn max_item_count(&self) -> MaxItemCount {
        self.max_item_count
    }

    fn a_im(&self) -> ChangeFeed {
        self.a_im
    }

    fn partition_range_id(&self) -> Option<PartitionRangeId<'b>> {
        self.partition_range_id
    }

    pub fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self {
        Self {
            if_match_condition: Some(if_match_condition),
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

    pub fn with_continuation(self, continuation: &'b str) -> Self {
        Self {
            continuation: Some(continuation),
            ..self
        }
    }

    pub fn with_max_item_count(self, max_item_count: i32) -> Self {
        Self {
            max_item_count: MaxItemCount::new(max_item_count),
            ..self
        }
    }

    pub fn with_a_im(self, a_im: ChangeFeed) -> Self {
        Self { a_im, ..self }
    }

    pub fn with_partition_range_id(self, partition_range_id: &'b str) -> Self {
        Self {
            partition_range_id: Some(PartitionRangeId::new(partition_range_id)),
            ..self
        }
    }

    pub async fn execute<T>(&self) -> Result<ListDocumentsResponse<T>, CosmosError>
    where
        T: DeserializeOwned,
    {
        let req = self.collection_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name()
            ),
            http::Method::GET,
            ResourceType::Documents,
        );

        // add trait headers
        let req = crate::headers::add_header(self.if_match_condition(), req);
        let req = crate::headers::add_header(self.user_agent(), req);
        let req = crate::headers::add_header(self.activity_id(), req);
        let req = crate::headers::add_header(self.consistency_level(), req);
        let req = ContinuationOption::add_header(self, req);
        let req = crate::headers::add_header(Some(self.max_item_count()), req);
        let req = crate::headers::add_header(Some(self.a_im()), req);
        let req = crate::headers::add_header(self.partition_range_id(), req);

        let req = req.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .collection_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }

    pub fn stream<T>(
        &self,
    ) -> impl Stream<Item = Result<ListDocumentsResponse<T>, CosmosError>> + '_
    where
        T: DeserializeOwned,
    {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            Continuation(String),
        };

        unfold(
            Some(States::Init),
            move |continuation_token: Option<States>| {
                async move {
                    debug!("continuation_token == {:?}", &continuation_token);
                    let response = match continuation_token {
                        Some(States::Init) => self.execute().await,
                        Some(States::Continuation(continuation_token)) => {
                            self.clone()
                                .with_continuation(&continuation_token)
                                .execute()
                                .await
                        }
                        None => return None,
                    };

                    // the ? operator does not work in async move (yet?)
                    // so we have to resort to this boilerplate
                    let response = match response {
                        Ok(response) => response,
                        Err(err) => return Some((Err(err), None)),
                    };

                    let continuation_token = match &response.continuation_token {
                        Some(ct) => Some(States::Continuation(ct.to_owned())),
                        None => None,
                    };

                    Some((Ok(response), continuation_token))
                }
            },
        )
    }
}

impl<'a, 'b> ContinuationOption<'b> for ListDocumentsBuilder<'a, 'b> {
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}
