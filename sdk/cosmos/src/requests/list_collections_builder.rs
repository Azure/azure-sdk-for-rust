use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::ListCollectionsResponse;
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListCollectionsBuilder<'a> {
    database_client: &'a DatabaseClient,
    user_agent: Option<azure_core::UserAgent<'a>>,
    activity_id: Option<azure_core::ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<&'a str>,
    max_item_count: i32,
}

impl<'a> ListCollectionsBuilder<'a> {
    pub(crate) fn new(database_client: &'a DatabaseClient) -> ListCollectionsBuilder<'a> {
        ListCollectionsBuilder {
            database_client,
            max_item_count: -1,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
        }
    }
}

impl<'a> ListCollectionsBuilder<'a> {
    pub fn database_client(&self) -> &'a DatabaseClient {
        self.database_client
    }
}

impl<'a> ListCollectionsBuilder<'a> {
    fn user_agent(&self) -> Option<azure_core::UserAgent<'a>> {
        self.user_agent
    }
}

impl<'a> ListCollectionsBuilder<'a> {
    fn activity_id(&self) -> Option<azure_core::ActivityId<'a>> {
        self.activity_id
    }
}

impl<'a> ListCollectionsBuilder<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a> ContinuationOption<'a> for ListCollectionsBuilder<'a> {
    fn continuation(&self) -> Option<&'a str> {
        self.continuation
    }
}

impl<'a> MaxItemCountOption for ListCollectionsBuilder<'a> {
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a> ListCollectionsBuilder<'a> {
    pub fn with_user_agent(self, user_agent: &'a str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a> ListCollectionsBuilder<'a> {
    pub fn with_activity_id(self, activity_id: &'a str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }
}

impl<'a> ListCollectionsBuilder<'a> {
    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

impl<'a> ContinuationSupport<'a> for ListCollectionsBuilder<'a> {
    type O = Self;

    fn with_continuation(self, continuation: &'a str) -> Self::O {
        Self {
            continuation: Some(continuation),
            ..self
        }
    }
}

impl<'a> MaxItemCountSupport for ListCollectionsBuilder<'a> {
    type O = Self;

    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        Self {
            max_item_count,
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a> ListCollectionsBuilder<'a> {
    pub async fn execute(&self) -> Result<ListCollectionsResponse, CosmosError> {
        trace!("ListCollectionsBuilder::execute called");
        let request = self.database_client.cosmos_client().prepare_request(
            &format!("dbs/{}/colls", self.database_client.database_name()),
            http::Method::GET,
            ResourceType::Collections,
        );

        let request = crate::headers::add_header(self.user_agent(), request);
        let request = crate::headers::add_header(self.activity_id(), request);
        let request = crate::headers::add_header(self.consistency_level(), request);
        let request = ContinuationOption::add_header(self, request);
        let request = MaxItemCountOption::add_header(self, request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        trace!("request prepared == {:?}", request);

        Ok(self
            .database_client
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }

    pub fn stream(&self) -> impl Stream<Item = Result<ListCollectionsResponse, CosmosError>> + '_ {
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
