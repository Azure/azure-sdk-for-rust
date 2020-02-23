use crate::clients::{Client, CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::ListDatabasesResponse;
use crate::ClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use futures::stream::{unfold, Stream};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug)]
pub struct ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    client: &'a Client<CUB>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
    continuation: Option<&'a str>,
    max_item_count: i32,
}

impl<'a, CUB> ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(client: &'a Client<CUB>) -> ListDatabasesBuilder<'a, CUB> {
        ListDatabasesBuilder {
            client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
        }
    }
}

impl<'a, CUB> Clone for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn clone(&self) -> Self {
        ListDatabasesBuilder {
            client: self.client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, CUB> ClientRequired<'a, CUB> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn client(&self) -> &'a Client<CUB> {
        self.client
    }
}

impl<'a, CUB> UserAgentOption<'a> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB> ActivityIdOption<'a> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB> ConsistencyLevelOption<'a> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level
    }
}

impl<'a, CUB> ContinuationOption<'a> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn continuation(&self) -> Option<&'a str> {
        self.continuation
    }
}

impl<'a, CUB> MaxItemCountOption for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, CUB> UserAgentSupport<'a> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDatabasesBuilder<'a, CUB>;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        ListDatabasesBuilder {
            client: self.client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, CUB> ActivityIdSupport<'a> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDatabasesBuilder<'a, CUB>;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        ListDatabasesBuilder {
            client: self.client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, CUB> ConsistencyLevelSupport<'a> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDatabasesBuilder<'a, CUB>;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        ListDatabasesBuilder {
            client: self.client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, CUB> ContinuationSupport<'a> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDatabasesBuilder<'a, CUB>;

    fn with_continuation(self, continuation: &'a str) -> Self::O {
        ListDatabasesBuilder {
            client: self.client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: Some(continuation),
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, CUB> MaxItemCountSupport for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDatabasesBuilder<'a, CUB>;

    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        ListDatabasesBuilder {
            client: self.client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<ListDatabasesResponse, AzureError> {
        trace!("ListDatabasesBuilder::execute called");

        let request =
            self.client
                .prepare_request("dbs", hyper::Method::GET, ResourceType::Databases);

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);
        let request = ContinuationOption::add_header(self, request);
        let request = MaxItemCountOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        let future_response = self.client.hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }

    pub fn stream(&self) -> impl Stream<Item = Result<ListDatabasesResponse, AzureError>> + '_ {
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
