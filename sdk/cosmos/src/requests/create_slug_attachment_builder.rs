use crate::prelude::*;
use crate::responses::CreateSlugAttachmentResponse;
use azure_core::prelude::*;
use bytes::Bytes;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateSlugAttachmentBuilder<'a, 'b> {
    attachment_client: &'a AttachmentClient,
    content_type: Option<ContentType<'b>>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> CreateSlugAttachmentBuilder<'a, 'b> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            content_type: None,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> CreateSlugAttachmentBuilder<'a, 'b> {
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
        content_type: ContentType<'b> => Some(content_type),
    }
}

impl<'a, 'b> CreateSlugAttachmentBuilder<'a, 'b> {
    pub async fn execute<B: Into<Bytes>>(
        &self,
        body: B,
    ) -> Result<CreateSlugAttachmentResponse, CosmosError> {
        let body = body.into();
        let mut req = self.attachment_client.prepare_request(http::Method::POST);

        req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        req = azure_core::headers::add_optional_header(&self.user_agent, req);
        req = azure_core::headers::add_optional_header(&self.activity_id, req);
        req = azure_core::headers::add_optional_header(&self.consistency_level, req);
        req = azure_core::headers::add_optional_header(&self.content_type, req);

        req = crate::cosmos_entity::add_as_partition_key_header_serialized(
            self.attachment_client
                .document_client()
                .partition_key_serialized(),
            req,
        );

        req = req.header("Slug", self.attachment_client.attachment_name());
        req = req.header(http::header::CONTENT_LENGTH, body.len());

        let req = req.body(body)?;

        debug!("req == {:#?}", req);

        Ok(self
            .attachment_client
            .http_client()
            .execute_request_check_status(req, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
