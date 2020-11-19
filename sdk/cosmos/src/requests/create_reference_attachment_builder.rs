use crate::prelude::*;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::StatusCode;
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
    content_type: Option<&'b str>,
    media: Option<&'b str>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
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

impl<'a, 'b, ContentTypeSet, MediaSet> AttachmentClientRequired<'a>
    for CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    fn attachment_client(&self) -> &'a AttachmentClient {
        self.attachment_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, MediaSet> ContentTypeRequired<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, Yes, MediaSet>
where
    MediaSet: ToAssign,
{
    fn content_type(&self) -> &'b str {
        self.content_type.unwrap()
    }
}

impl<'a, 'b, ContentTypeSet> MediaRequired<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, Yes>
where
    ContentTypeSet: ToAssign,
{
    fn media(&self) -> &'b str {
        self.media.unwrap()
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet> UserAgentOption<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet> ActivityIdOption<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet> ConsistencyLevelOption<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, MediaSet> ContentTypeSupport<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, No, MediaSet>
where
    MediaSet: ToAssign,
{
    type O = CreateReferenceAttachmentBuilder<'a, 'b, Yes, MediaSet>;

    fn with_content_type(self, content_type: &'b str) -> Self::O {
        CreateReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: Some(content_type),
            media: self.media,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, ContentTypeSet> MediaSupport<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, No>
where
    ContentTypeSet: ToAssign,
{
    type O = CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, Yes>;

    fn with_media(self, media: &'b str) -> Self::O {
        CreateReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: self.content_type,
            media: Some(media),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet> UserAgentSupport<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet> ActivityIdSupport<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
{
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b, ContentTypeSet, MediaSet> ConsistencyLevelSupport<'b>
    for CreateReferenceAttachmentBuilder<'a, 'b, ContentTypeSet, MediaSet>
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
impl<'a, 'b> CreateReferenceAttachmentBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute(
        &self,
    ) -> Result<crate::responses::CreateReferenceAttachmentResponse, AzureError> {
        let mut req = self.attachment_client.prepare_request(hyper::Method::POST);

        // add trait headers
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);

        req = crate::add_partition_keys_header(
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
            id: self.attachment_client.attachment_name().name(),
            content_type: ContentTypeRequired::content_type(self),
            media: self.media(),
        })?;

        req = req.header(http::header::CONTENT_TYPE, "application/json");
        req = req.header(http::header::CONTENT_LENGTH, request.len());
        let req = req.body(hyper::Body::from(request))?;
        debug!("req == {:#?}", req);

        let (headers, whole_body) = check_status_extract_headers_and_body(
            self.attachment_client.hyper_client().request(req),
            StatusCode::CREATED,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        Ok((&headers, &whole_body as &[u8]).try_into()?)
    }
}
