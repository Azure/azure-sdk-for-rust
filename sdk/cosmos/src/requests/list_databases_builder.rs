use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::ListDatabasesResponse;
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListDatabasesBuilder<'a> {
    cosmos_client: &'a CosmosClient,
    user_agent: Option<azure_core::UserAgent<'a>>,
    activity_id: Option<azure_core::ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<&'a str>,
    max_item_count: i32,
}

impl<'a> ListDatabasesBuilder<'a> {
    pub(crate) fn new(cosmos_client: &'a CosmosClient) -> ListDatabasesBuilder<'a> {
        ListDatabasesBuilder {
            cosmos_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
        }
    }
}

impl<'a> ListDatabasesBuilder<'a> {
    pub fn cosmos_client(&self) -> &'a CosmosClient {
        self.cosmos_client
    }
}

impl<'a> ListDatabasesBuilder<'a> {
    fn user_agent(&self) -> Option<azure_core::UserAgent<'a>> {
        self.user_agent
    }
}

impl<'a> ListDatabasesBuilder<'a> {
    fn activity_id(&self) -> Option<azure_core::ActivityId<'a>> {
        self.activity_id
    }
}

impl<'a> ListDatabasesBuilder<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a> ContinuationOption<'a> for ListDatabasesBuilder<'a> {
    fn continuation(&self) -> Option<&'a str> {
        self.continuation
    }
}

impl<'a> MaxItemCountOption for ListDatabasesBuilder<'a> {
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a> UserAgentSupport<'a> for ListDatabasesBuilder<'a> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a> ActivityIdSupport<'a> for ListDatabasesBuilder<'a> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }
}

impl<'a> ConsistencyLevelSupport<'a> for ListDatabasesBuilder<'a> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

impl<'a> ContinuationSupport<'a> for ListDatabasesBuilder<'a> {
    type O = Self;

    fn with_continuation(self, continuation: &'a str) -> Self::O {
        Self {
            continuation: Some(continuation),
            ..self
        }
    }
}

impl<'a> MaxItemCountSupport for ListDatabasesBuilder<'a> {
    type O = Self;

    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        Self {
            max_item_count,
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a> ListDatabasesBuilder<'a> {
    pub async fn execute(&self) -> Result<ListDatabasesResponse, CosmosError> {
        trace!("ListDatabasesBuilder::execute called");

        let request =
            self.cosmos_client
                .prepare_request("dbs", http::Method::GET, ResourceType::Databases);

        let request = crate::headers::add_header(self.user_agent(), request);
        let request = crate::headers::add_header(self.activity_id(), request);
        let request = crate::headers::add_header(self.consistency_level(), request);
        let request = ContinuationOption::add_header(self, request);
        let request = MaxItemCountOption::add_header(self, request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .cosmos_client
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }

    pub fn stream(&self) -> impl Stream<Item = Result<ListDatabasesResponse, CosmosError>> + '_ {
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
