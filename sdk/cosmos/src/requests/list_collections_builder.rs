use crate::prelude::*;
use crate::responses::ListCollectionsResponse;
use crate::{DatabaseClientRequired, ResourceType};
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug)]
pub struct ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    database_client: &'a dyn DatabaseClient<C>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
    continuation: Option<&'a str>,
    max_item_count: i32,
}

impl<'a, C> Clone for ListCollectionsBuilder<'a, C>
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

impl<'a, C> ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    pub(crate) fn new(database_client: &'a dyn DatabaseClient<C>) -> ListCollectionsBuilder<'a, C> {
        ListCollectionsBuilder {
            database_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
        }
    }
}

impl<'a, C> DatabaseClientRequired<'a, C> for ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    fn database_client(&self) -> &'a dyn DatabaseClient<C> {
        self.database_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C> UserAgentOption<'a> for ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, C> ActivityIdOption<'a> for ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, C> ConsistencyLevelOption<'a> for ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level.clone()
    }
}

impl<'a, C> ContinuationOption<'a> for ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    fn continuation(&self) -> Option<&'a str> {
        self.continuation
    }
}

impl<'a, C> MaxItemCountOption for ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, C> UserAgentSupport<'a> for ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    type O = ListCollectionsBuilder<'a, C>;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        ListCollectionsBuilder {
            database_client: self.database_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, C> ActivityIdSupport<'a> for ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    type O = ListCollectionsBuilder<'a, C>;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        ListCollectionsBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, C> ConsistencyLevelSupport<'a> for ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    type O = ListCollectionsBuilder<'a, C>;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        ListCollectionsBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, C> ContinuationSupport<'a> for ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    type O = ListCollectionsBuilder<'a, C>;

    fn with_continuation(self, continuation: &'a str) -> Self::O {
        ListCollectionsBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: Some(continuation),
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, C> MaxItemCountSupport for ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
    type O = ListCollectionsBuilder<'a, C>;

    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        ListCollectionsBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> ListCollectionsBuilder<'a, C>
where
    C: CosmosClient,
{
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
