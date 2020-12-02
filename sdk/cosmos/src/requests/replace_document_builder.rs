use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::ReplaceDocumentResponse;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use http::StatusCode;
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
    if_modified_since: Option<IfModifiedSince>,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TenativeWritesAllowance,
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
            allow_tentative_writes: TenativeWritesAllowance::Deny,
        }
    }

    pub fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet>
    ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    fn indexing_directive(&self) -> IndexingDirective {
        self.indexing_directive
    }

    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
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

    fn allow_tentative_writes(&self) -> TenativeWritesAllowance {
        self.allow_tentative_writes
    }

    pub fn with_indexing_directive(self, indexing_directive: IndexingDirective) -> Self {
        Self {
            indexing_directive,
            ..self
        }
    }

    pub fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }

    pub fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self {
        Self {
            if_modified_since: Some(IfModifiedSince::new(if_modified_since.clone())),
            ..self
        }
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

    pub fn with_allow_tentative_writes(
        self,
        allow_tentative_writes: TenativeWritesAllowance,
    ) -> Self {
        Self {
            allow_tentative_writes,
            ..self
        }
    }
}

impl<'a, 'b> ReplaceDocumentBuilder<'a, 'b, Yes, Yes> {
    pub async fn execute_with_document<T>(
        &self,
        document: &T,
    ) -> Result<ReplaceDocumentResponse, CosmosError>
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
        let req = crate::headers::add_header(Some(self.indexing_directive()), req);
        let req = crate::headers::add_header(self.if_match_condition(), req);
        let req = crate::headers::add_header(self.if_modified_since(), req);
        let req = crate::headers::add_header(self.user_agent(), req);
        let req = crate::headers::add_header(self.activity_id(), req);
        let req = crate::headers::add_header(self.consistency_level(), req);
        let req = crate::headers::add_header(Some(self.partition_keys()), req);
        let req = crate::headers::add_header(Some(self.allow_tentative_writes()), req);

        let serialized = serde_json::to_string(document)?;

        let req = req.body(serialized.as_bytes())?;
        debug!("request == {:#?}", req);

        Ok(self
            .collection_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet>
    ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    fn if_modified_since(&self) -> Option<IfModifiedSince> {
        self.if_modified_since.clone()
    }
}

impl<'a, 'b, DocumentIdSet> ReplaceDocumentBuilder<'a, 'b, Yes, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
{
    fn partition_keys(&self) -> &'b PartitionKeys {
        self.partition_keys.unwrap()
    }
}

impl<'a, 'b, PartitionKeysSet> ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, Yes>
where
    PartitionKeysSet: ToAssign,
{
    fn document_id(&self) -> &'b str {
        self.document_id.unwrap()
    }
}

impl<'a, 'b, DocumentIdSet> ReplaceDocumentBuilder<'a, 'b, No, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
{
    pub fn with_partition_keys(
        self,
        partition_keys: &'b PartitionKeys,
    ) -> ReplaceDocumentBuilder<'a, 'b, Yes, DocumentIdSet> {
        ReplaceDocumentBuilder {
            partition_keys: Some(partition_keys),
            collection_client: self.collection_client,
            document_id: self.document_id,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
            p_partition_keys: PhantomData,
            p_document_id: PhantomData,
        }
    }
}

impl<'a, 'b, PartitionKeysSet> ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, No>
where
    PartitionKeysSet: ToAssign,
{
    pub fn with_document_id(
        self,
        document_id: &'b str,
    ) -> ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, Yes> {
        ReplaceDocumentBuilder {
            collection_client: self.collection_client,
            p_partition_keys: PhantomData,
            p_document_id: PhantomData,
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
