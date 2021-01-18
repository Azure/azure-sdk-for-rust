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
    partition_keys: Option<PartitionKeys>,
    document_id: Option<&'b str>,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<IfModifiedSince<'b>>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TenativeWritesAllowance,
    p_partition_keys: PhantomData<PartitionKeysSet>,
    p_document_id: PhantomData<DocumentIdSet>,
}

impl<'a, 'b> ReplaceDocumentBuilder<'a, 'b, No, No> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            document_id: None,
            partition_keys: None,
            indexing_directive: IndexingDirective::Default,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: TenativeWritesAllowance::Deny,
            p_document_id: PhantomData,
            p_partition_keys: PhantomData,
        }
    }
}

impl<'a, 'b, PartitionKeysSet, DocumentIdSet>
    ReplaceDocumentBuilder<'a, 'b, PartitionKeysSet, DocumentIdSet>
where
    PartitionKeysSet: ToAssign,
    DocumentIdSet: ToAssign,
{
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
        if_modified_since: &'b DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
        allow_tentative_writes: TenativeWritesAllowance,
        indexing_directive: IndexingDirective,
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
                self.document_id.unwrap()
            ),
            http::Method::PUT,
            ResourceType::Documents,
        );

        // add trait headers
        let req = azure_core::headers::add_mandatory_header(&self.indexing_directive, req);
        let req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        let req = azure_core::headers::add_optional_header(&self.if_modified_since, req);
        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);
        let req =
            azure_core::headers::add_mandatory_header(&self.partition_keys.as_ref().unwrap(), req);
        let req = azure_core::headers::add_mandatory_header(&self.allow_tentative_writes, req);

        let serialized = azure_core::to_json(document)?;

        let req = req.body(serialized)?;
        debug!("request == {:#?}", req);

        Ok(self
            .collection_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }
}

impl<'a, 'b, DocumentIdSet> ReplaceDocumentBuilder<'a, 'b, No, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
{
    pub fn partition_keys<P: Into<PartitionKeys>>(
        self,
        partition_keys: P,
    ) -> ReplaceDocumentBuilder<'a, 'b, Yes, DocumentIdSet> {
        ReplaceDocumentBuilder {
            partition_keys: Some(partition_keys.into()),
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
    pub fn document_id(
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
