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
    content_type: Option<ContentType<'b>>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    media: Option<&'b str>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplaceReferenceAttachmentBuilder<'a, 'b, No, No> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            p_content_type: PhantomData,
            content_type: None,
            p_media: PhantomData,
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
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
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
        req = crate::headers::add_header(self.if_match_condition, req);
        req = crate::headers::add_header(self.user_agent, req);
        req = crate::headers::add_header(self.activity_id, req);
        req = crate::headers::add_header(self.consistency_level.clone(), req);

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
            content_type: self.content_type.unwrap().as_str(),
            media: self.media.unwrap(),
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

impl<'a, 'b, MediaSet> ReplaceReferenceAttachmentBuilder<'a, 'b, No, MediaSet>
where
    MediaSet: ToAssign,
{
    pub fn with_content_type(
        self,
        content_type: &'b str,
    ) -> ReplaceReferenceAttachmentBuilder<'a, 'b, Yes, MediaSet> {
        ReplaceReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData,
            p_media: PhantomData,
            content_type: Some(ContentType::new(content_type)),
            if_match_condition: self.if_match_condition,
            media: self.media,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, ContentTypeSet> ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, No>
where
    ContentTypeSet: ToAssign,
{
    pub fn with_media(
        self,
        media: &'b str,
    ) -> ReplaceReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, Yes> {
        ReplaceReferenceAttachmentBuilder {
            media: Some(media),
            attachment_client: self.attachment_client,
            content_type: self.content_type,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_content_type: PhantomData,
            p_media: PhantomData,
        }
    }
}
