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
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<Continuation<'a>>,
    max_item_count: MaxItemCount,
}

impl<'a> ListCollectionsBuilder<'a> {
    pub(crate) fn new(database_client: &'a DatabaseClient) -> ListCollectionsBuilder<'a> {
        ListCollectionsBuilder {
            database_client,
            max_item_count: MaxItemCount::new(-1),
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
        }
    }

    setters! {
        user_agent: &'a str => Some(UserAgent::new(user_agent)),
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        continuation: &'a str => Some(Continuation::new(continuation)),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
    }

    pub async fn execute(&self) -> Result<ListCollectionsResponse, CosmosError> {
        trace!("ListCollectionsBuilder::execute called");
        let request = self.database_client.cosmos_client().prepare_request(
            &format!("dbs/{}/colls", self.database_client.database_name()),
            http::Method::GET,
            ResourceType::Collections,
        );

        let request = azure_core::headers::add_optional_header(&self.user_agent, request);
        let request = azure_core::headers::add_optional_header(&self.activity_id, request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level, request);
        let request = azure_core::headers::add_optional_header(&self.continuation, request);
        let request = azure_core::headers::add_mandatory_header(&self.max_item_count, request);

        let request = request.body(bytes::Bytes::from_static(EMPTY_BODY))?;

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
        }

        unfold(
            Some(States::Init),
            move |continuation_token: Option<States>| {
                async move {
                    debug!("continuation_token == {:?}", &continuation_token);
                    let response = match continuation_token {
                        Some(States::Init) => self.execute().await,
                        Some(States::Continuation(continuation_token)) => {
                            self.clone()
                                .continuation(continuation_token.as_str())
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

                    let continuation_token = response
                        .continuation_token
                        .as_ref()
                        .map(|ct| States::Continuation(ct.to_owned()));

                    Some((Ok(response), continuation_token))
                }
            },
        )
    }
}
