use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::ListUsersResponse;
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListUsersBuilder<'a, 'b> {
    database_client: &'a DatabaseClient,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<&'b str>,
    max_item_count: MaxItemCount,
}

impl<'a, 'b> ListUsersBuilder<'a, 'b> {
    pub(crate) fn new(database_client: &'a DatabaseClient) -> Self {
        Self {
            database_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: MaxItemCount::new(-1),
        }
    }

    pub fn database_client(&self) -> &'a DatabaseClient {
        self.database_client
    }

    // TODO: Use this in request
    #[allow(unused)]
    fn max_item_count(&self) -> MaxItemCount {
        self.max_item_count
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

    pub async fn execute(&self) -> Result<ListUsersResponse, CosmosError> {
        trace!("ListUsersBuilder::execute called");

        let req = self.database_client.cosmos_client().prepare_request(
            &format!("dbs/{}/users", self.database_client.database_name()),
            http::Method::GET,
            ResourceType::Users,
        );

        let req = req.body(EMPTY_BODY.as_ref())?;
        debug!("\nreq == {:?}", req);

        Ok(self
            .database_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }

    pub fn stream(&self) -> impl Stream<Item = Result<ListUsersResponse, CosmosError>> + '_ {
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

impl<'a, 'b> ContinuationOption<'b> for ListUsersBuilder<'a, 'b> {
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}
