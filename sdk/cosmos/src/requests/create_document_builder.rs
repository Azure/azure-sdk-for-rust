use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::CreateDocumentResponse;
use azure_core::errors::UnexpectedHTTPResult;
use azure_core::prelude::*;
use azure_core::{ActivityId, No, ToAssign, UserAgent, Yes};
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::Serialize;
use std::convert::TryFrom;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateDocumentBuilder<'a, 'b, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
{
    collection_client: &'a CollectionClient,
    partition_keys: Option<PartitionKeys>,
    is_upsert: IsUpsert,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<IfModifiedSince<'b>>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TenativeWritesAllowance,
    p_partition_keys: PhantomData<PartitionKeysSet>,
}

impl<'a, 'b> CreateDocumentBuilder<'a, 'b, No> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            partition_keys: None,
            is_upsert: IsUpsert::No,
            indexing_directive: IndexingDirective::Default,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: TenativeWritesAllowance::Deny,
            p_partition_keys: PhantomData,
        }
    }
}

impl<'a, 'b, PartitionKeysSet> CreateDocumentBuilder<'a, 'b, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
{
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
        if_modified_since: &'b DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
        allow_tentative_writes: TenativeWritesAllowance,
        is_upsert: bool => if is_upsert { IsUpsert::Yes } else { IsUpsert::No },
        indexing_directive: IndexingDirective,
    }
}

impl<'a, 'b> CreateDocumentBuilder<'a, 'b, No> {
    pub fn partition_keys<P: Into<PartitionKeys>>(
        self,
        partition_keys: P,
    ) -> CreateDocumentBuilder<'a, 'b, Yes> {
        CreateDocumentBuilder {
            partition_keys: Some(partition_keys.into()),
            collection_client: self.collection_client,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
            p_partition_keys: PhantomData,
        }
    }
}

impl<'a, 'b> CreateDocumentBuilder<'a, 'b, Yes> {
    pub async fn execute_with_document<T>(
        &self,
        document: &T,
    ) -> Result<CreateDocumentResponse, CosmosError>
    where
        T: Serialize,
    {
        let mut req = self.collection_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name()
            ),
            http::Method::POST,
            ResourceType::Documents,
        );

        // add trait headers
        req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        req = azure_core::headers::add_optional_header(&self.if_modified_since, req);
        req = azure_core::headers::add_optional_header(&self.user_agent, req);
        req = azure_core::headers::add_optional_header(&self.activity_id, req);
        req = azure_core::headers::add_optional_header(&self.consistency_level, req);
        req =
            azure_core::headers::add_mandatory_header(&self.partition_keys.as_ref().unwrap(), req);
        req = azure_core::headers::add_mandatory_header(&self.is_upsert, req);
        req = azure_core::headers::add_mandatory_header(&self.indexing_directive, req);
        req = azure_core::headers::add_mandatory_header(&self.allow_tentative_writes, req);

        let serialized = azure_core::to_json(document)?;
        let req = req.body(serialized)?;

        let response = self
            .collection_client
            .http_client()
            .execute_request(req)
            .await?;

        debug!("status_core == {:?}", response.status());
        debug!("headers == {:?}", response.headers());
        debug!("whole body == {:#?}", response.body());

        // expect CREATED is IsUpsert is off. Otherwise either
        // CREATED or OK means success.
        if self.is_upsert == IsUpsert::No && response.status() != StatusCode::CREATED {
            return Err(UnexpectedHTTPResult::new(
                StatusCode::CREATED,
                response.status(),
                std::str::from_utf8(response.body())?,
            )
            .into());
        } else if response.status() != StatusCode::CREATED && response.status() != StatusCode::OK {
            return Err(UnexpectedHTTPResult::new_multiple(
                vec![StatusCode::CREATED, StatusCode::OK],
                response.status(),
                std::str::from_utf8(response.body())?,
            )
            .into());
        }

        CreateDocumentResponse::try_from(response)
    }
}
