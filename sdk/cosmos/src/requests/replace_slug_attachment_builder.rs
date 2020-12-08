use crate::prelude::*;
use crate::responses::CreateSlugAttachmentResponse;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    attachment_client: &'a AttachmentClient,
    p_body: PhantomData<BodySet>,
    p_content_type: PhantomData<ContentTypeSet>,
    body: Option<&'b [u8]>,
    content_type: Option<ContentType<'b>>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplaceSlugAttachmentBuilder<'a, 'b, No, No> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            p_body: PhantomData,
            body: None,
            p_content_type: PhantomData,
            content_type: None,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> ReplaceSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
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

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ReplaceSlugAttachmentBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute(&self) -> Result<CreateSlugAttachmentResponse, CosmosError> {
        let mut req = self.attachment_client.prepare_request(http::Method::PUT);

        // add trait headers
        req = crate::headers::add_header(self.if_match_condition, req);
        req = crate::headers::add_header(self.user_agent, req);
        req = crate::headers::add_header(self.activity_id, req);
        req = crate::headers::add_header(self.consistency_level.clone(), req);

        req = crate::headers::add_partition_keys_header(
            self.attachment_client.document_client().partition_keys(),
            req,
        );

        req = crate::headers::add_header(Some(self.content_type.unwrap()), req);

        req = req.header("Slug", self.attachment_client.attachment_name());
        req = req.header(http::header::CONTENT_LENGTH, self.body.unwrap().len());

        let req = req.body(self.body.unwrap())?;

        debug!("req == {:#?}", req);

        Ok(self
            .attachment_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }
}

impl<'a, 'b, ContentTypeSet> ReplaceSlugAttachmentBuilder<'a, 'b, No, ContentTypeSet>
where
    ContentTypeSet: ToAssign,
{
    pub fn with_body(
        self,
        body: &'b [u8],
    ) -> ReplaceSlugAttachmentBuilder<'a, 'b, Yes, ContentTypeSet> {
        ReplaceSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData,
            p_content_type: PhantomData,
            body: Some(body),
            content_type: self.content_type,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, BodySet> ReplaceSlugAttachmentBuilder<'a, 'b, BodySet, No>
where
    BodySet: ToAssign,
{
    pub fn with_content_type(
        self,
        content_type: &'b str,
    ) -> ReplaceSlugAttachmentBuilder<'a, 'b, BodySet, Yes> {
        ReplaceSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData,
            p_content_type: PhantomData,
            body: self.body,

            content_type: Some(ContentType::new(content_type)),
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}
