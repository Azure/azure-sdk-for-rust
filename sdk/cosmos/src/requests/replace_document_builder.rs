use crate::prelude::*;
use crate::responses::ReplaceDocumentResponse;
use crate::ResourceType;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::Serialize;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    collection_client: &'a CollectionClient,
    p_partition_keys: PhantomData<PartitionKeysSet>,
    p_document_id: PhantomData<DocumentIdSet>,
    partition_keys: Option<&'b PartitionKeys>,
    document_id: Option<&'b str>,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: bool,
}

impl<'a, 'b> ReplaceDocumentBuilder<'a, 'b, No, No> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            p_document_id: PhantomData,
            p_partition_keys: PhantomData,
            document_id: None,
            partition_keys: None,
            indexing_directive: IndexingDirective::Default,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: false,
        }
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> CollectionClientRequired<'a>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    #[inline]
    fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, DocumentIdSet> PartitionKeysRequired<'b>
    for ReplaceDocumentBuilder<'a, 'b, Yes, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
{
    fn partition_keys(&self) -> &'b PartitionKeys {
        self.partition_keys.unwrap()
    }
}

impl<'a, 'b, PartitionKeysSet> DocumentIdRequired<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, Yes>
where
    PartitionKeysSet: ToAssign,
{
    fn document_id(&self) -> &'b str {
        self.document_id.unwrap()
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> IndexingDirectiveOption
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    fn indexing_directive(&self) -> IndexingDirective {
        self.indexing_directive
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> IfMatchConditionOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> IfModifiedSinceOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> UserAgentOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> ActivityIdOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> ConsistencyLevelOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> AllowTentativeWritesOption
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    fn allow_tentative_writes(&self) -> bool {
        self.allow_tentative_writes
    }
}

impl<'a, 'b, DocumentIdSet> PartitionKeysSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, No, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
{
    type O = ReplaceDocumentBuilder<'a, 'b, Yes, DocumentIdSet>;

    fn with_partition_keys(self, partition_keys: &'b PartitionKeys) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            partition_keys: Some(partition_keys),
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, PartitionKeysSet> DocumentIdSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, No>
where
    PartitionKeysSet: ToAssign,
{
    type O = ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, Yes>;

    fn with_document_id(self, document_id: &'b str) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            partition_keys: self.partition_keys,
            document_id: Some(document_id),
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> IndexingDirectiveSupport
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    type O = Self;

    fn with_indexing_directive(self, indexing_directive: IndexingDirective) -> Self::O {
        Self {
            indexing_directive,
            ..self
        }
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> IfMatchConditionSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    type O = Self;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> IfModifiedSinceSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    type O = Self;

    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        Self {
            if_modified_since: Some(if_modified_since),
            ..self
        }
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> UserAgentSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> ActivityIdSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> ConsistencyLevelSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet> AllowTentativeWritesSupport
    for ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    type O = Self;

    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O {
        Self {
            allow_tentative_writes,
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ReplaceDocumentBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute_with_document<T>(
        &self,
        document: &T,
    ) -> Result<ReplaceDocumentResponse, AzureError>
    where
        T: Serialize,
    {
        trace!("ReplaceDocumentBuilder::execute() called");

        let req = self.collection_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name(),
                self.document_id()
            ),
            hyper::Method::PUT,
            ResourceType::Documents,
        );

        // add trait headers
        let req = IndexingDirectiveOption::add_header(self, req);
        let req = IfMatchConditionOption::add_header(self, req);
        let req = IfModifiedSinceOption::add_header(self, req);
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);
        let req = PartitionKeysRequired::add_header(self, req);
        let req = AllowTentativeWritesOption::add_header(self, req);

        let serialized = serde_json::to_string(document)?;

        let req = req.body(hyper::Body::from(serialized))?;
        debug!("request == {:#?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.collection_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        (&headers, &body as &[u8]).try_into()
    }
}
