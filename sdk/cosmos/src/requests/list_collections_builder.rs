use crate::prelude::*;
use crate::responses::ListCollectionsResponse;
use crate::{DatabaseClientRequired, ResourceType};
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListCollectionsBuilder<'a> {
    database_client: &'a DatabaseClient,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
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

impl<'a> DatabaseClientRequired<'a> for ListCollectionsBuilder<'a> {
    fn database_client(&self) -> &'a DatabaseClient {
        self.database_client
    }
}

impl<'a> UserAgentOption<'a> for ListCollectionsBuilder<'a> {
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a> ActivityIdOption<'a> for ListCollectionsBuilder<'a> {
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a> ConsistencyLevelOption<'a> for ListCollectionsBuilder<'a> {
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

impl<'a> UserAgentSupport<'a> for ListCollectionsBuilder<'a> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a> ActivityIdSupport<'a> for ListCollectionsBuilder<'a> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a> ConsistencyLevelSupport<'a> for ListCollectionsBuilder<'a> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
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
    pub async fn execute(&self) -> Result<ListCollectionsResponse, AzureError> {
        trace!("ListCollectionsBuilder::execute called");
        let request = self.database_client.cosmos_client().prepare_request(
            &format!("dbs/{}/colls", self.database_client.database_name().name()),
            hyper::Method::GET,
            ResourceType::Collections,
        );

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);
        let request = ContinuationOption::add_header(self, request);
        let request = MaxItemCountOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        trace!("request prepared == {:?}", request);

        let future_response = self.database_client.hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;
        Ok((&headers, &body as &[u8]).try_into()?)
    }

    pub fn stream(&self) -> impl Stream<Item = Result<ListCollectionsResponse, AzureError>> + '_ {
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
