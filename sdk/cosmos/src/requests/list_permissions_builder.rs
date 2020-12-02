use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::ListPermissionsResponse;
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListPermissionsBuilder<'a, 'b> {
    user_client: &'a UserClient,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<&'b str>,
    max_item_count: MaxItemCount,
}

impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    pub(crate) fn new(user_client: &'a UserClient) -> Self {
        Self {
            user_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: MaxItemCount::new(-1),
        }
    }
}

impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    pub fn user_client(&self) -> &'a UserClient {
        self.user_client
    }
}

impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }
}

impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }
}

impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> ContinuationOption<'b> for ListPermissionsBuilder<'a, 'b> {
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    fn max_item_count(&self) -> MaxItemCount {
        self.max_item_count
    }
}

impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    pub fn with_activity_id(self, activity_id: &'b str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }
}

impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    pub fn with_continuation(self, continuation: &'b str) -> Self {
        Self {
            continuation: Some(continuation),
            ..self
        }
    }
}

impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    pub fn with_max_item_count(self, max_item_count: i32) -> Self {
        Self {
            max_item_count: MaxItemCount::new(max_item_count),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ListPermissionsBuilder<'a, 'b> {
    pub async fn execute(&self) -> Result<ListPermissionsResponse<'a>, CosmosError> {
        trace!("ListPermissionsBuilder::execute called");

        let request = self.user_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/users/{}/permissions",
                self.user_client.database_client().database_name(),
                self.user_client.user_name()
            ),
            http::Method::GET,
            ResourceType::Permissions,
        );

        let request = crate::headers::add_header(self.user_agent(), request);
        let request = crate::headers::add_header(self.activity_id(), request);
        let request = crate::headers::add_header(self.consistency_level(), request);
        let request = ContinuationOption::add_header(self, request);
        let request = crate::headers::add_header(Some(self.max_item_count()), request);

        let request = request.body(EMPTY_BODY.as_ref())?;
        debug!("\nrequest == {:#?}", request);

        Ok(self
            .user_client
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }

    pub fn stream(
        &self,
    ) -> impl Stream<Item = Result<ListPermissionsResponse<'a>, CosmosError>> + '_ {
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
