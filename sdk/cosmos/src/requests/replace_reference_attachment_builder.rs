use crate::prelude::*;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    attachment_client: &'a AttachmentClient,
    p_content_type: PhantomData<ContentTypeSet>,
    p_media: PhantomData<MediaSet>,
    content_type: Option<&'b str>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    media: Option<&'b str>,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplaceReferenceAttachmentBuilder<'a, 'b, No, No> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            p_content_type: PhantomData {},
            content_type: None,
            p_media: PhantomData {},
            media: None,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet>
    ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    pub fn attachment_client(&self) -> &'a AttachmentClient {
        self.attachment_client
    }
}

impl<'a, 'b, MediaSet> ContentTypeRequired<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, Yes, MediaSet>
where
    MediaSet: ToAssign,
{
    fn content_type(&self) -> &'b str {
        self.content_type.unwrap()
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet>
    ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, ContentTypeSet> MediaRequired<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, Yes>
where
    ContentTypeSet: ToAssign,
{
    fn media(&self) -> &'b str {
        self.media.unwrap()
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet>
    ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet>
    ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet>
    ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, MediaSet> ContentTypeSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, No, MediaSet>
where
    MediaSet: ToAssign,
{
    type O = ReplaceReferenceAttachmentBuilder<'a, 'b, Yes, MediaSet>;

    fn with_content_type(self, content_type: &'b str) -> Self::O {
        ReplaceReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: Some(content_type),
            if_match_condition: self.if_match_condition,
            media: self.media,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet> IfMatchConditionSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    type O = Self;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }
}

impl<'a, 'b, ContentTypeSet> MediaSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, No>
where
    ContentTypeSet: ToAssign,
{
    type O = ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, Yes>;

    fn with_media(self, media: &'b str) -> Self::O {
        ReplaceReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: self.content_type,
            if_match_condition: self.if_match_condition,
            media: Some(media),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet> UserAgentSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet> ActivityIdSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet> ConsistencyLevelSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ReplaceReferenceAttachmentBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute(
        &self,
    ) -> Result<crate::responses::ReplaceReferenceAttachmentResponse, CosmosError> {
        let mut req = self
            .attachment_client
            .prepare_request_with_attachment_name(http::Method::PUT);

        // add trait headers
        req = crate::headers::add_header(self.if_match_condition(), req);
        req = crate::headers::add_header(self.user_agent(), req);
        req = crate::headers::add_header(self.activity_id(), req);
        req = crate::headers::add_header(self.consistency_level(), req);

        req = crate::headers::add_partition_keys_header(
            self.attachment_client.document_client().partition_keys(),
            req,
        );

        // create serialized request
        #[derive(Debug, Clone, Serialize)]
        struct _Request<'r> {
            pub id: &'r str,
            #[serde(rename = "contentType")]
            pub content_type: &'r str,
            pub media: &'r str,
        }

        let request = serde_json::to_string(&_Request {
            id: self.attachment_client.attachment_name(),
            content_type: ContentTypeRequired::content_type(self),
            media: self.media(),
        })?;

        req = req.header(http::header::CONTENT_TYPE, "application/json");
        req = req.header(http::header::CONTENT_LENGTH, request.len());
        let req = req.body(request.as_bytes())?;
        debug!("req == {:#?}", req);

        Ok(self
            .attachment_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
