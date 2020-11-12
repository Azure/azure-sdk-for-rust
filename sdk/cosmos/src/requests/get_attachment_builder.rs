use crate::prelude::*;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    attachment_client: &'a dyn AttachmentClient<C, D, COLL, DOC>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D, COLL, DOC> GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    pub(crate) fn new(
        attachment_client: &'a dyn AttachmentClient<C, D, COLL, DOC>,
    ) -> GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC> {
        GetAttachmentBuilder {
            attachment_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C, D, COLL, DOC> AttachmentClientRequired<'a, C, D, COLL, DOC>
    for GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
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
impl<'a, 'b, C, D, COLL, DOC> IfMatchConditionOption<'b>
    for GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
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

impl<'a, 'b, C, D, COLL, DOC> UserAgentOption<'b> for GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
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

impl<'a, 'b, C, D, COLL, DOC> ActivityIdOption<'b> for GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
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

impl<'a, 'b, C, D, COLL, DOC> ConsistencyLevelOption<'b>
    for GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
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

impl<'a, 'b, C, D, COLL, DOC> IfMatchConditionSupport<'b>
    for GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        GetAttachmentBuilder {
            attachment_client: self.attachment_client,
            if_match_condition: Some(if_match_condition),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL, DOC> UserAgentSupport<'b> for GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        GetAttachmentBuilder {
            attachment_client: self.attachment_client,
            if_match_condition: self.if_match_condition,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL, DOC> ActivityIdSupport<'b>
    for GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        GetAttachmentBuilder {
            attachment_client: self.attachment_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL, DOC> ConsistencyLevelSupport<'b>
    for GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    type O = GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        GetAttachmentBuilder {
            attachment_client: self.attachment_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D, COLL, DOC> GetAttachmentBuilder<'a, 'b, C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    pub async fn execute(&self) -> Result<crate::responses::GetAttachmentResponse, AzureError> {
        let mut req = self
            .attachment_client
            .prepare_request_with_attachment_name(hyper::Method::GET);

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);

        req = crate::add_partition_keys_header(
            self.attachment_client.document_client().partition_keys(),
            req,
        );

        let req = req.body(hyper::Body::empty())?;

        debug!("req == {:#?}", req);

        let (headers, whole_body) = check_status_extract_headers_and_body(
            self.attachment_client.http_client().request(req),
            StatusCode::OK,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        Ok((&headers, &whole_body as &[u8]).try_into()?)
    }
}
