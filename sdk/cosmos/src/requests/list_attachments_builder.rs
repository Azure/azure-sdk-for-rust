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
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
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

    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
        continuation: &'b str => Some(Continuation::new(continuation)),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
        a_im: ChangeFeed,
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
        req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        req = azure_core::headers::add_optional_header(&self.user_agent, req);
        req = azure_core::headers::add_optional_header(&self.activity_id, req);
        req = azure_core::headers::add_optional_header(&self.consistency_level, req);
        req = azure_core::headers::add_optional_header(&self.continuation, req);
        req = azure_core::headers::add_mandatory_header(&self.max_item_count, req);
        req = azure_core::headers::add_mandatory_header(&self.a_im, req);

        req = crate::cosmos_entity::add_as_partition_key_header_serialized(
            self.document_client.partition_key_serialized(),
            req,
        );

        let req = req.body(bytes::Bytes::from_static(EMPTY_BODY))?;

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
