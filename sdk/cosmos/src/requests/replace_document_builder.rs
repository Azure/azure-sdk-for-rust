use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::ReplaceDocumentResponse;
use azure_core::prelude::*;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::Serialize;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ReplaceDocumentBuilder<'a, 'b> {
    collection_client: &'a CollectionClient,
    partition_keys: Option<PartitionKeys>,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<IfModifiedSince<'b>>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TenativeWritesAllowance,
}

impl<'a, 'b> ReplaceDocumentBuilder<'a, 'b> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
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
}

impl<'a, 'b> ReplaceDocumentBuilder<'a, 'b> {
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
        if_modified_since: &'b DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
        allow_tentative_writes: TenativeWritesAllowance,
        indexing_directive: IndexingDirective,
        partition_keys: PartitionKeys => Some(partition_keys),
    }
}

impl<'a, 'b> ReplaceDocumentBuilder<'a, 'b> {
    pub async fn execute<T>(
        &self,
        document_id: &str,
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
                document_id
            ),
            http::Method::PUT,
            ResourceType::Documents,
        );

        let req = azure_core::headers::add_mandatory_header(&self.indexing_directive, req);
        let req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        let req = azure_core::headers::add_optional_header(&self.if_modified_since, req);
        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);
        let req = azure_core::headers::add_optional_header(&self.partition_keys.as_ref(), req);
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
