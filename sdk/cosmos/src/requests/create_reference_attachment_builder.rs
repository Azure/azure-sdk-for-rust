use crate::prelude::*;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
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
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> CreateReferenceAttachmentBuilder<'a, 'b, No, No> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            p_content_type: PhantomData {},
            content_type: None,
            p_media: PhantomData {},
            media: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet>
    CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    pub fn attachment_client(&self) -> &'a AttachmentClient {
        self.attachment_client
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

impl<'a, 'b, MediaSet> CreateReferenceAttachmentBuilder<'a, 'b, Yes, MediaSet>
where
    MediaSet: ToAssign,
{
    fn content_type(&self) -> ContentType<'b> {
        self.content_type.unwrap()
    }
}

impl<'a, 'b, ContentTypeSet> CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, Yes>
where
    ContentTypeSet: ToAssign,
{
    fn media(&self) -> &'b str {
        self.media.unwrap()
    }
}
impl<'a, 'b, MediaSet> CreateReferenceAttachmentBuilder<'a, 'b, No, MediaSet>
where
    MediaSet: ToAssign,
{
    pub fn with_content_type(
        self,
        content_type: &'b str,
    ) -> CreateReferenceAttachmentBuilder<'a, 'b, Yes, MediaSet> {
        CreateReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: Some(ContentType::new(content_type)),
            media: self.media,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, ContentTypeSet> CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, No>
where
    ContentTypeSet: ToAssign,
{
    pub fn with_media(
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
            p_content_type: PhantomData {},
            p_media: PhantomData {},
        }
    }
}

impl<'a, 'b> CreateReferenceAttachmentBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute(
        &self,
    ) -> Result<crate::responses::CreateReferenceAttachmentResponse, CosmosError> {
        let mut req = self.attachment_client.prepare_request(http::Method::POST);

        // add trait headers
        req = azure_core::headers::add_optional_header(&self.user_agent(), req);
        req = azure_core::headers::add_optional_header(&self.activity_id(), req);
        req = azure_core::headers::add_optional_header(&self.consistency_level(), req);

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
            content_type: self.content_type().as_str(),
            media: self.media(),
        })?;

        req = req.header(http::header::CONTENT_TYPE, "application/json");
        req = req.header(http::header::CONTENT_LENGTH, request.len());
        let req = req.body(request.as_bytes())?;
        debug!("req == {:#?}", req);

        Ok(self
            .attachment_client
            .http_client()
            .execute_request_check_status(req, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
