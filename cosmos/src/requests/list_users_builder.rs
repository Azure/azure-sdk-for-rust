use crate::prelude::*;
use crate::responses::ListUsersResponse;
use crate::ResourceType;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use futures::stream::{unfold, Stream};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug)]
pub struct ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    database_client: &'a dyn DatabaseClient<C>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    continuation: Option<&'b str>,
    max_item_count: i32,
}

impl<'a, 'b, C> Clone for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    fn clone(&self) -> Self {
        Self {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level.clone(),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, C> ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    #[inline]
    pub(crate) fn new(database_client: &'a dyn DatabaseClient<C>) -> ListUsersBuilder<'a, 'b, C> {
        ListUsersBuilder {
            database_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
        }
    }
}

impl<'a, 'b, C> DatabaseClientRequired<'a, C> for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    #[inline]
    fn database_client(&self) -> &'a dyn DatabaseClient<C> {
        self.database_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, C> UserAgentOption<'b> for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C> ActivityIdOption<'b> for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C> ConsistencyLevelOption<'b> for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C> ContinuationOption<'b> for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    #[inline]
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b, C> MaxItemCountOption for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    #[inline]
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b, C> UserAgentSupport<'b> for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    type O = ListUsersBuilder<'a, 'b, C>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ListUsersBuilder {
            database_client: self.database_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, C> ActivityIdSupport<'b> for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    type O = ListUsersBuilder<'a, 'b, C>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ListUsersBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, C> ConsistencyLevelSupport<'b> for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    type O = ListUsersBuilder<'a, 'b, C>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ListUsersBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, C> ContinuationSupport<'b> for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    type O = ListUsersBuilder<'a, 'b, C>;

    #[inline]
    fn with_continuation(self, continuation: &'b str) -> Self::O {
        ListUsersBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: Some(continuation),
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, C> MaxItemCountSupport for ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    type O = ListUsersBuilder<'a, 'b, C>;

    #[inline]
    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        ListUsersBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count,
        }
    }
}

// methods callable regardless
impl<'a, 'b, C> ListUsersBuilder<'a, 'b, C> where C: CosmosClient {}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C> ListUsersBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    pub async fn execute(&self) -> Result<ListUsersResponse, AzureError> {
        trace!("ListUsersBuilder::execute called");

        let req = self.database_client.cosmos_client().prepare_request(
            &format!("dbs/{}/users", self.database_client.database_name()),
            hyper::Method::GET,
            ResourceType::Users,
        );

        let req = req.body(hyper::Body::empty())?;
        debug!("\nreq == {:?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.database_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }

    pub fn stream(&self) -> impl Stream<Item = Result<ListUsersResponse, AzureError>> + '_ {
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
