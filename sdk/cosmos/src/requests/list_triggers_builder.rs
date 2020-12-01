use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::ListTriggersResponse;
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListTriggersBuilder<'a, 'b> {
    collection_client: &'a CollectionClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<&'b str>,
    max_item_count: i32,
}

impl<'a, 'b> ListTriggersBuilder<'a, 'b> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
        }
    }
}

impl<'a, 'b> ListTriggersBuilder<'a, 'b> {
    pub fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }
}

impl<'a, 'b> IfMatchConditionOption<'b> for ListTriggersBuilder<'a, 'b> {
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b> ListTriggersBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }
}

impl<'a, 'b> ActivityIdOption<'b> for ListTriggersBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b> ConsistencyLevelOption<'b> for ListTriggersBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> ContinuationOption<'b> for ListTriggersBuilder<'a, 'b> {
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b> MaxItemCountOption for ListTriggersBuilder<'a, 'b> {
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b> IfMatchConditionSupport<'b> for ListTriggersBuilder<'a, 'b> {
    type O = Self;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }
}

impl<'a, 'b> UserAgentSupport<'b> for ListTriggersBuilder<'a, 'b> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for ListTriggersBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for ListTriggersBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

impl<'a, 'b> ContinuationSupport<'b> for ListTriggersBuilder<'a, 'b> {
    type O = Self;

    fn with_continuation(self, continuation: &'b str) -> Self::O {
        Self {
            continuation: Some(continuation),
            ..self
        }
    }
}

impl<'a, 'b> MaxItemCountSupport for ListTriggersBuilder<'a, 'b> {
    type O = Self;

    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        Self {
            max_item_count,
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ListTriggersBuilder<'a, 'b> {
    pub async fn execute(&self) -> Result<ListTriggersResponse, CosmosError> {
        trace!("ListTriggersBuilder::execute called");

        let request = self.collection_client().cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/triggers",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name()
            ),
            http::Method::GET,
            ResourceType::Triggers,
        );

        // add trait headers
        let request = IfMatchConditionOption::add_header(self, request);
        let request = crate::headers::add_header(self.user_agent(), request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);
        let request = ContinuationOption::add_header(self, request);
        let request = MaxItemCountOption::add_header(self, request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .collection_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }

    pub fn stream(&self) -> impl Stream<Item = Result<ListTriggersResponse, CosmosError>> + '_ {
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
