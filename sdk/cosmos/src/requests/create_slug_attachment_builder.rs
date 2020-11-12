use crate::prelude::*;
use crate::responses::CreateSlugAttachmentResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    attachment_client: &'a dyn AttachmentClient<C, D, COLL, DOC>,
    p_body: PhantomData<BodySet>,
    p_content_type: PhantomData<ContentTypeSet>,
    body: Option<&'b [u8]>,
    content_type: Option<&'b str>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D, COLL, DOC> CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, No, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    pub(crate) fn new(
        attachment_client: &'a dyn AttachmentClient<C, D, COLL, DOC>,
    ) -> CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, No, No> {
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

impl<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet> AttachmentClientRequired<'a, C, D, COLL, DOC>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn attachment_client(&self) -> &'a dyn AttachmentClient<C, D, COLL, DOC> {
        self.attachment_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet> BodyRequired<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, Yes, ContentTypeSet>
where
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn body(&self) -> &'b [u8] {
        self.body.unwrap()
    }
}

impl<'a, 'b, C, D, COLL, DOC, BodySet> ContentTypeRequired<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, Yes>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn content_type(&self) -> &'b str {
        self.content_type.unwrap()
    }
}

impl<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet> IfMatchConditionOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet> UserAgentOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet> ActivityIdOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet> ConsistencyLevelOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet> BodySupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, No, ContentTypeSet>
where
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, Yes, ContentTypeSet>;

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

impl<'a, 'b, C, D, COLL, DOC, BodySet> ContentTypeSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, No>
where
    BodySet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, Yes>;

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

impl<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet> IfMatchConditionSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>;

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

impl<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet> UserAgentSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>;

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

impl<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet> ActivityIdSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>;

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

impl<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet> ConsistencyLevelSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>
where
    BodySet: ToAssign,
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, BodySet, ContentTypeSet>;

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
impl<'a, 'b, C, D, COLL, DOC> CreateSlugAttachmentBuilder<'a, 'b, C, D, COLL, DOC, Yes, Yes>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
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
            self.attachment_client.http_client().request(req),
            StatusCode::CREATED,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        Ok((&headers, &whole_body as &[u8]).try_into()?)
    }
}
