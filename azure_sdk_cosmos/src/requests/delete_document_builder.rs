use crate::clients::{CosmosUriBuilder, DocumentClient};
use crate::prelude::*;
use crate::responses::DeleteDocumentResponse;
use crate::DocumentBuilderTrait;
use crate::DocumentClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{IfMatchConditionOption, IfMatchConditionSupport};
use azure_sdk_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    document_client: &'a DocumentClient<'a, CUB>,
    p_partition_keys: PhantomData<PartitionKeysSet>,
    partition_keys: Option<&'a PartitionKeys>,
    if_match_condition: Option<IfMatchCondition<'a>>,
    if_modified_since: Option<&'a DateTime<Utc>>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
    allow_tentative_writes: bool,
}

impl<'a, CUB> DeleteDocumentBuilder<'a, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        document_client: &'a DocumentClient<'a, CUB>,
    ) -> DeleteDocumentBuilder<'a, CUB, No> {
        DeleteDocumentBuilder {
            document_client,
            p_partition_keys: PhantomData {},
            partition_keys: None,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: false,
        }
    }
}

impl<'a, CUB, PartitionKeysSet> DocumentClientRequired<'a, CUB>
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn document_client(&self) -> &'a DocumentClient<'a, CUB> {
        self.document_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB> PartitionKeysRequired<'a> for DeleteDocumentBuilder<'a, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_keys(&self) -> &'a PartitionKeys {
        self.partition_keys.unwrap()
    }
}

impl<'a, CUB, PartitionKeysSet> IfMatchConditionOption<'a>
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a, CUB, PartitionKeysSet> IfModifiedSinceOption<'a>
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_modified_since(&self) -> Option<&'a DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, CUB, PartitionKeysSet> UserAgentOption<'a>
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB, PartitionKeysSet> ActivityIdOption<'a>
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB, PartitionKeysSet> ConsistencyLevelOption<'a>
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level
    }
}

impl<'a, CUB, PartitionKeysSet> AllowTentativeWritesOption
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn allow_tentative_writes(&self) -> bool {
        self.allow_tentative_writes
    }
}

impl<'a, CUB> PartitionKeysSupport<'a> for DeleteDocumentBuilder<'a, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    type O = DeleteDocumentBuilder<'a, CUB, Yes>;

    #[inline]
    fn with_partition_keys(self, partition_keys: &'a PartitionKeys) -> Self::O {
        DeleteDocumentBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: Some(partition_keys),
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, CUB, PartitionKeysSet> IfMatchConditionSupport<'a>
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        DeleteDocumentBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: Some(if_match_condition),
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, CUB, PartitionKeysSet> IfModifiedSinceSupport<'a>
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>;

    #[inline]
    fn with_if_modified_since(self, if_modified_since: &'a DateTime<Utc>) -> Self::O {
        DeleteDocumentBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: self.if_match_condition,
            if_modified_since: Some(if_modified_since),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, CUB, PartitionKeysSet> UserAgentSupport<'a>
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        DeleteDocumentBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, CUB, PartitionKeysSet> ActivityIdSupport<'a>
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        DeleteDocumentBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, CUB, PartitionKeysSet> ConsistencyLevelSupport<'a>
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        DeleteDocumentBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, CUB, PartitionKeysSet> AllowTentativeWritesSupport
    for DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = DeleteDocumentBuilder<'a, CUB, PartitionKeysSet>;

    #[inline]
    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O {
        DeleteDocumentBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> DeleteDocumentBuilder<'a, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<DeleteDocumentResponse, AzureError> {
        trace!("DeleteDocumentBuilder::execute called");

        let mut req = self.document_client.prepare_request(hyper::Method::DELETE);

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = IfModifiedSinceOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);
        req = PartitionKeysRequired::add_header(self, req);
        req = AllowTentativeWritesOption::add_header(self, req);

        let req = req.body(hyper::Body::empty())?;
        debug!("{:?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.document_client.hyper_client().request(req),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
