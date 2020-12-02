use crate::prelude::*;
use crate::responses::CreateSlugAttachmentResponse;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
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
    body: Option<&'b [u8]>,
    content_type: Option<ContentType<'b>>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
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
            p_content_type: PhantomData {},
            p_body: PhantomData {},
        }
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    pub fn attachment_client(&self) -> &'a AttachmentClient {
        self.attachment_client
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
}

impl<'a, 'b, ContentTypeSet> CreateSlugAttachmentBuilder<'a, 'b, Yes, ContentTypeSet>
where
    ContentTypeSet: ToAssign,
{
    fn body(&self) -> &'b [u8] {
        self.body.unwrap()
    }
}

impl<'a, 'b, ContentTypeSet> CreateSlugAttachmentBuilder<'a, 'b, No, ContentTypeSet>
where
    ContentTypeSet: ToAssign,
{
    pub fn with_body(
        self,
        body: &'b [u8],
    ) -> CreateSlugAttachmentBuilder<'a, 'b, Yes, ContentTypeSet> {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            p_content_type: PhantomData {},
            body: Some(body),
            content_type: self.content_type,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, BodySet> CreateSlugAttachmentBuilder<'a, 'b, BodySet, No>
where
    BodySet: ToAssign,
{
    pub fn with_content_type(
        self,
        content_type: &'b str,
    ) -> CreateSlugAttachmentBuilder<'a, 'b, BodySet, Yes> {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            p_content_type: PhantomData {},
            body: self.body,
            content_type: Some(ContentType::new(content_type)),
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, BodySet> CreateSlugAttachmentBuilder<'a, 'b, BodySet, Yes>
where
    BodySet: ToAssign,
{
    fn content_type(&self) -> ContentType<'b> {
        self.content_type.unwrap()
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> CreateSlugAttachmentBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute(&self) -> Result<CreateSlugAttachmentResponse, CosmosError> {
        let mut req = self.attachment_client.prepare_request(http::Method::POST);

        // add trait headers
        req = crate::headers::add_header(self.if_match_condition(), req);
        req = crate::headers::add_header(self.user_agent(), req);
        req = crate::headers::add_header(self.activity_id(), req);
        req = crate::headers::add_header(self.consistency_level(), req);

        req = crate::headers::add_partition_keys_header(
            self.attachment_client.document_client().partition_keys(),
            req,
        );

        req = crate::headers::add_header(Some(self.content_type()), req);

        req = req.header("Slug", self.attachment_client.attachment_name());
        req = req.header(http::header::CONTENT_LENGTH, self.body().len());

        let req = req.body(self.body())?;

        debug!("req == {:#?}", req);

        Ok(self
            .attachment_client
            .http_client()
            .execute_request_check_status(req, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
