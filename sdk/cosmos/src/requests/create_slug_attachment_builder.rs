use crate::prelude::*;
use crate::responses::CreateSlugAttachmentResponse;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use bytes::Bytes;
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    attachment_client: &'a AttachmentClient,
    body: Option<Bytes>,
    content_type: Option<ContentType<'b>>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    p_body: PhantomData<BodySet>,
    p_content_type: PhantomData<ContentTypeSet>,
}

impl<'a, 'b> CreateSlugAttachmentBuilder<'a, 'b, No, No> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            body: None,
            content_type: None,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            p_content_type: PhantomData,
            p_body: PhantomData,
        }
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
    }
}

impl<'a, 'b, ContentTypeSet> CreateSlugAttachmentBuilder<'a, 'b, No, ContentTypeSet>
where
    ContentTypeSet: ToAssign,
{
    pub fn body(self, body: Bytes) -> CreateSlugAttachmentBuilder<'a, 'b, Yes, ContentTypeSet> {
        CreateSlugAttachmentBuilder {
            body: Some(body),
            attachment_client: self.attachment_client,
            content_type: self.content_type,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_body: PhantomData,
            p_content_type: PhantomData,
        }
    }
}

impl<'a, 'b, BodySet> CreateSlugAttachmentBuilder<'a, 'b, BodySet, No>
where
    BodySet: ToAssign,
{
    pub fn content_type(
        self,
        content_type: &'b str,
    ) -> CreateSlugAttachmentBuilder<'a, 'b, BodySet, Yes> {
        CreateSlugAttachmentBuilder {
            content_type: Some(ContentType::new(content_type)),
            attachment_client: self.attachment_client,
            body: self.body,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_body: PhantomData,
            p_content_type: PhantomData,
        }
    }
}

impl<'a, 'b> CreateSlugAttachmentBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute(self) -> Result<CreateSlugAttachmentResponse, CosmosError> {
        let mut req = self.attachment_client.prepare_request(http::Method::POST);

        // add trait headers
        req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        req = azure_core::headers::add_optional_header(&self.user_agent, req);
        req = azure_core::headers::add_optional_header(&self.activity_id, req);
        req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        req = crate::headers::add_partition_keys_header(
            self.attachment_client.document_client().partition_keys(),
            req,
        );

        req = azure_core::headers::add_mandatory_header(&self.content_type.unwrap(), req);

        req = req.header("Slug", self.attachment_client.attachment_name());
        let body = self.body.unwrap();
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
