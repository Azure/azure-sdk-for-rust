use crate::prelude::*;
use crate::responses::ReplaceDocumentResponse;
use crate::ResourceType;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::Serialize;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    collection_client: &'a dyn CollectionClient<C, D>,
    p_partition_keys: PhantomData<PartitionKeysSet>,
    p_document_id: PhantomData<DocumentIdSet>,
    partition_keys: Option<&'b PartitionKeys>,
    document_id: Option<&'b str>,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    allow_tentative_writes: bool,
}

impl<'a, 'b, C, D> ReplaceDocumentBuilder<'a, 'b, C, D, No, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a dyn CollectionClient<C, D>,
    ) -> ReplaceDocumentBuilder<'a, 'b, C, D, No, No> {
        ReplaceDocumentBuilder {
            collection_client,
            p_partition_keys: PhantomData {},
            partition_keys: None,
            p_document_id: PhantomData {},
            document_id: None,
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

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> CollectionClientRequired<'a, C, D>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn collection_client(&self) -> &'a dyn CollectionClient<C, D> {
        self.collection_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, C, D, DocumentIdSet> PartitionKeysRequired<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, Yes, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn partition_keys(&self) -> &'b PartitionKeys {
        self.partition_keys.unwrap()
    }
}

impl<'a, 'b, C, D, PartitionKeysSet> DocumentIdRequired<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, Yes>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn document_id(&self) -> &'b str {
        self.document_id.unwrap()
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> IndexingDirectiveOption
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn indexing_directive(&self) -> IndexingDirective {
        self.indexing_directive
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> IfMatchConditionOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> IfModifiedSinceOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> UserAgentOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> ActivityIdOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> ConsistencyLevelOption<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> AllowTentativeWritesOption
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn allow_tentative_writes(&self) -> bool {
        self.allow_tentative_writes
    }
}

impl<'a, 'b, C, D, DocumentIdSet> PartitionKeysSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, No, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceDocumentBuilder<'a, 'b, C, D, Yes, DocumentIdSet>;

    #[inline]
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

impl<'a, 'b, C, D, PartitionKeysSet> DocumentIdSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, No>
where
    PartitionKeysSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, Yes>;

    #[inline]
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

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> IndexingDirectiveSupport
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_indexing_directive(self, indexing_directive: IndexingDirective) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> IfMatchConditionSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: Some(if_match_condition),
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> IfModifiedSinceSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: Some(if_modified_since),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> UserAgentSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> ActivityIdSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> ConsistencyLevelSupport<'b>
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            allow_tentative_writes: self.allow_tentative_writes,
        }
    }
}

impl<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet> AllowTentativeWritesSupport
    for ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceDocumentBuilder<'a, 'b, C, D, PartitionKeysSet, DocumentIdSet>;

    #[inline]
    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData {},
            p_document_id: PhantomData {},
            partition_keys: self.partition_keys,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
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
impl<'a, 'b, C, D> ReplaceDocumentBuilder<'a, 'b, C, D, Yes, Yes>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
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
            http::Method::PUT,
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
            self.collection_client.http_client().request(req),
            StatusCode::OK,
        )
        .await?;

        (&headers, &body as &[u8]).try_into()
    }
}
