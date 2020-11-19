use crate::prelude::*;
use crate::responses::CreateSlugAttachmentResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    attachment_client: &'a AttachmentClient,
    p_body: PhantomData<BodySet>,
    p_content_type: PhantomData<ContentTypeSet>,
    body: Option<&'b [u8]>,
    content_type: Option<&'b str>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> CreateSlugAttachmentBuilder<'a, 'b, No, No> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            p_body: PhantomData {},
            body: None,
            p_content_type: PhantomData {},
            content_type: None,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> AttachmentClientRequired<'a>
    for CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    fn attachment_client(&self) -> &'a AttachmentClient {
        self.attachment_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, ContentTypeSet> BodyRequired<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, Yes, ContentTypeSet>
where
    ContentTypeSet: ToAssign,
{
    fn body(&self) -> &'b [u8] {
        self.body.unwrap()
    }
}

impl<'a, 'b, BodySet> ContentTypeRequired<'b> for CreateSlugAttachmentBuilder<'a, 'b, BodySet, Yes>
where
    BodySet: ToAssign,
{
    fn content_type(&self) -> &'b str {
        self.content_type.unwrap()
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> IfMatchConditionOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> UserAgentOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> ActivityIdOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> ConsistencyLevelOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, ContentTypeSet> BodySupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, No, ContentTypeSet>
where
    ContentTypeSet: ToAssign,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, Yes, ContentTypeSet>;

    fn with_body(self, body: &'b [u8]) -> Self::O {
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

impl<'a, 'b, BodySet> ContentTypeSupport<'b> for CreateSlugAttachmentBuilder<'a, 'b, BodySet, No>
where
    BodySet: ToAssign,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, BodySet, Yes>;

    fn with_content_type(self, content_type: &'b str) -> Self::O {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            p_content_type: PhantomData {},
            body: self.body,
            content_type: Some(content_type),
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> IfMatchConditionSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    type O = Self;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> UserAgentSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> ActivityIdSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
{
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b, BodySet, ContentTypeSet> ConsistencyLevelSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
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
impl<'a, 'b> CreateSlugAttachmentBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute(&self) -> Result<CreateSlugAttachmentResponse, AzureError> {
        let mut req = self.attachment_client.prepare_request(hyper::Method::POST);

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);

        req = crate::add_partition_keys_header(
            self.attachment_client.document_client().partition_keys(),
            req,
        );

        req = ContentTypeRequired::add_header(self, req);

        req = req.header("Slug", self.attachment_client.attachment_name().name());
        req = req.header(http::header::CONTENT_LENGTH, self.body().len());

        let req = req.body(hyper::Body::from(self.body().to_owned()))?;

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
