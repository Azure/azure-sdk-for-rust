use crate::prelude::*;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    attachment_client: &'a dyn AttachmentClient<C, D, COLL, DOC>,
    p_content_type: PhantomData<ContentTypeSet>,
    p_media: PhantomData<MediaSet>,
    content_type: Option<&'b str>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    media: Option<&'b str>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D, COLL, DOC> ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, No, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    pub(crate) fn new(
        attachment_client: &'a dyn AttachmentClient<C, D, COLL, DOC>,
    ) -> ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, No, No> {
        ReplaceReferenceAttachmentBuilder {
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

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>
    AttachmentClientRequired<'a, C, D, COLL, DOC>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
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
impl<'a, 'b, C, D, COLL, DOC, MediaSet> ContentTypeRequired<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, Yes, MediaSet>
where
    MediaSet: ToAssign,
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

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet> IfMatchConditionOption<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
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

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet> MediaRequired<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, Yes>
where
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn media(&self) -> &'b str {
        self.media.unwrap()
    }
}

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet> UserAgentOption<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
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

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet> ActivityIdOption<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
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

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet> ConsistencyLevelOption<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
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

impl<'a, 'b, C, D, COLL, DOC, MediaSet> ContentTypeSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, No, MediaSet>
where
    MediaSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, Yes, MediaSet>;

    #[inline]
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

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet> IfMatchConditionSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        ReplaceReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: self.content_type,
            if_match_condition: Some(if_match_condition),
            media: self.media,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet> MediaSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, No>
where
    ContentTypeSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, Yes>;

    #[inline]
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

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet> UserAgentSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ReplaceReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: self.content_type,
            if_match_condition: self.if_match_condition,
            media: self.media,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet> ActivityIdSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ReplaceReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: self.content_type,
            if_match_condition: self.if_match_condition,
            media: self.media,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet> ConsistencyLevelSupport<'b>
    for ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>
where
    ContentTypeSet: ToAssign,
    MediaSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, ContentTypeSet, MediaSet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ReplaceReferenceAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_content_type: PhantomData {},
            p_media: PhantomData {},
            content_type: self.content_type,
            if_match_condition: self.if_match_condition,
            media: self.media,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D, COLL, DOC> ReplaceReferenceAttachmentBuilder<'a, 'b, C, D, COLL, DOC, Yes, Yes>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    pub async fn execute(
        &self,
    ) -> Result<crate::responses::ReplaceReferenceAttachmentResponse, AzureError> {
        let mut req = self
            .attachment_client
            .prepare_request_with_attachment_name(hyper::Method::PUT);

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
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
            StatusCode::OK,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        Ok((&headers, &whole_body as &[u8]).try_into()?)
    }
}
