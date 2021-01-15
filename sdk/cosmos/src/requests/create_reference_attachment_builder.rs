use crate::prelude::*;
use azure_core::prelude::*;
use azure_core::{ActivityId, No, ToAssign, UserAgent, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    attachment_client: &'a AttachmentClient,
    p_content_type: PhantomData<ContentTypeSet>,
    p_media: PhantomData<MediaSet>,
    content_type: Option<ContentType<'b>>,
    media: Option<&'b str>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> CreateReferenceAttachmentBuilder<'a, 'b, No, No> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            content_type: None,
            media: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            p_content_type: PhantomData,
            p_media: PhantomData,
        }
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet>
    CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a, 'b, MediaSet> CreateReferenceAttachmentBuilder<'a, 'b, No, MediaSet>
where
    MediaSet: ToAssign,
{
    pub fn content_type(
        self,
        content_type: &'b str,
    ) -> CreateReferenceAttachmentBuilder<'a, 'b, Yes, MediaSet> {
        CreateReferenceAttachmentBuilder {
            content_type: Some(ContentType::new(content_type)),
            attachment_client: self.attachment_client,
            media: self.media,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_content_type: PhantomData,
            p_media: PhantomData,
        }
    }
}

impl<'a, 'b, ContentTypeSet> CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, No>
where
    ContentTypeSet: ToAssign,
{
    pub fn media(
        self,
        media: &'b str,
    ) -> CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, Yes> {
        CreateReferenceAttachmentBuilder {
            media: Some(media),
            attachment_client: self.attachment_client,
            content_type: self.content_type,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            p_content_type: PhantomData,
            p_media: PhantomData,
        }
    }
}

impl<'a, 'b> CreateReferenceAttachmentBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute(
        &self,
    ) -> Result<crate::responses::CreateReferenceAttachmentResponse, CosmosError> {
        let mut req = self.attachment_client.prepare_request(http::Method::POST);

        // add trait headers
        req = azure_core::headers::add_optional_header(&self.user_agent, req);
        req = azure_core::headers::add_optional_header(&self.activity_id, req);
        req = azure_core::headers::add_optional_header(&self.consistency_level, req);

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

        let request = azure_core::to_json(&_Request {
            id: self.attachment_client.attachment_name(),
            content_type: self.content_type.unwrap().as_str(),
            media: self.media.unwrap(),
        })?;

        req = req.header(http::header::CONTENT_TYPE, "application/json");
        req = req.header(http::header::CONTENT_LENGTH, request.len());
        let req = req.body(request)?;
        debug!("req == {:#?}", req);

        Ok(self
            .attachment_client
            .http_client()
            .execute_request_check_status(req, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
