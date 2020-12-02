use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::ListAttachmentsResponse;
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListAttachmentsBuilder<'a, 'b> {
    document_client: &'a DocumentClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<Continuation<'b>>,
    max_item_count: MaxItemCount,
    a_im: ChangeFeed,
}

impl<'a, 'b> ListAttachmentsBuilder<'a, 'b> {
    pub(crate) fn new(document_client: &'a DocumentClient) -> Self {
        Self {
            document_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: MaxItemCount::new(-1),
            a_im: ChangeFeed::None,
        }
    }

    pub fn document_client(&self) -> &'a DocumentClient {
        self.document_client
    }

    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }

    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }

    fn max_item_count(&self) -> MaxItemCount {
        self.max_item_count
    }

    fn a_im(&self) -> ChangeFeed {
        self.a_im
    }

    pub fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
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
            continuation: Some(Continuation::new(continuation)),
            ..self
        }
    }

    pub fn with_max_item_count(self, max_item_count: i32) -> Self {
        Self {
            max_item_count: MaxItemCount::new(max_item_count),
            ..self
        }
    }

    pub fn with_a_im(self, a_im: ChangeFeed) -> Self {
        Self { a_im, ..self }
    }

    pub async fn execute(&self) -> Result<ListAttachmentsResponse, CosmosError> {
        let mut req = self.document_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments",
                self.document_client.database_client().database_name(),
                self.document_client.collection_client().collection_name(),
                self.document_client.document_name()
            ),
            http::Method::GET,
            ResourceType::Attachments,
        );

        // add trait headers
        req = crate::headers::add_header(self.if_match_condition(), req);
        req = crate::headers::add_header(self.user_agent(), req);
        req = crate::headers::add_header(self.activity_id(), req);
        req = crate::headers::add_header(self.consistency_level(), req);
        req = crate::headers::add_header(self.continuation(), req);
        req = crate::headers::add_header(Some(self.max_item_count()), req);
        req = crate::headers::add_header(Some(self.a_im()), req);

        req = crate::headers::add_partition_keys_header(self.document_client.partition_keys(), req);

        let req = req.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .document_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }

    pub fn stream(&self) -> impl Stream<Item = Result<ListAttachmentsResponse, CosmosError>> + '_ {
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

impl<'a, 'b> ListAttachmentsBuilder<'a, 'b> {
    fn continuation(&self) -> Option<Continuation<'b>> {
        self.continuation
    }
}
