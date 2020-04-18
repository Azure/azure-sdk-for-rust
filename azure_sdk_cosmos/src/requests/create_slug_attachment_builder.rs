use crate::clients::{CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::CreateSlugAttachmentResponse;
use crate::AttachmentClient;
use crate::AttachmentClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    attachment_client: &'a AttachmentClient<'a, CUB>,
    p_body: PhantomData<BodySet>,
    p_content_type: PhantomData<ContentTypeSet>,
    body: Option<&'b [u8]>,
    content_type: Option<&'b str>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, CUB> CreateSlugAttachmentBuilder<'a, 'b, CUB, No, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        attachment_client: &'a AttachmentClient<'a, CUB>,
    ) -> CreateSlugAttachmentBuilder<'a, 'b, CUB, No, No> {
        CreateSlugAttachmentBuilder {
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

impl<'a, 'b, CUB, BodySet, ContentTypeSet> AttachmentClientRequired<'a, CUB>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn attachment_client(&self) -> &'a AttachmentClient<'a, CUB> {
        self.attachment_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, CUB, ContentTypeSet> BodyRequired<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, Yes, ContentTypeSet>
where
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn body(&self) -> &'b [u8] {
        self.body.unwrap()
    }
}

impl<'a, 'b, CUB, BodySet> ContentTypeRequired<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, Yes>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn content_type(&self) -> &'b str {
        self.content_type.unwrap()
    }
}

impl<'a, 'b, CUB, BodySet, ContentTypeSet> IfMatchConditionOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB, BodySet, ContentTypeSet> UserAgentOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB, BodySet, ContentTypeSet> ActivityIdOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB, BodySet, ContentTypeSet> ConsistencyLevelOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, CUB, ContentTypeSet> BodySupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, No, ContentTypeSet>
where
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, CUB, Yes, ContentTypeSet>;

    #[inline]
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

impl<'a, 'b, CUB, BodySet> ContentTypeSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, No>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, Yes>;

    #[inline]
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

impl<'a, 'b, CUB, BodySet, ContentTypeSet> IfMatchConditionSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            p_content_type: PhantomData {},
            body: self.body,
            content_type: self.content_type,
            if_match_condition: Some(if_match_condition),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, BodySet, ContentTypeSet> UserAgentSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            p_content_type: PhantomData {},
            body: self.body,
            content_type: self.content_type,
            if_match_condition: self.if_match_condition,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, BodySet, ContentTypeSet> ActivityIdSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            p_content_type: PhantomData {},
            body: self.body,
            content_type: self.content_type,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, BodySet, ContentTypeSet> ConsistencyLevelSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet, ContentTypeSet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            p_content_type: PhantomData {},
            body: self.body,
            content_type: self.content_type,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> CreateSlugAttachmentBuilder<'a, 'b, CUB, Yes, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<CreateSlugAttachmentResponse, AzureError> {
        let mut req = self.attachment_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments",
                self.attachment_client.database_name().name(),
                self.attachment_client.collection_name().name(),
                self.attachment_client.document_name().name(),
            ),
            hyper::Method::POST,
            ResourceType::Attachments,
        );

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
