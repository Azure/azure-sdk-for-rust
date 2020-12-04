use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::CreateDocumentResponse;
use azure_core::errors::UnexpectedHTTPResult;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
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
    partition_keys: Option<&'b PartitionKeys>,
    is_upsert: IsUpsert,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<IfModifiedSince>,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
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
            p_partition_keys: PhantomData {},
        }
    }
}

impl<'a, 'b, PartitionKeysSet> CreateDocumentBuilder<'a, 'b, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
{
    pub fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
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
        CreateDocumentBuilder {
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

    pub fn with_is_upsert(self, is_upsert: bool) -> Self {
        Self {
            is_upsert: if is_upsert {
                IsUpsert::Yes
            } else {
                IsUpsert::No
            },
            ..self
        }
    }

    pub fn with_indexing_directive(self, indexing_directive: IndexingDirective) -> Self {
        Self {
            indexing_directive,
            ..self
        }
    }

    pub fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self {
        CreateDocumentBuilder {
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

    fn is_upsert(&self) -> IsUpsert {
        self.is_upsert
    }

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
}

impl<'a, 'b> CreateDocumentBuilder<'a, 'b, Yes> {
    fn partition_keys(&self) -> &'b PartitionKeys {
        self.partition_keys.unwrap()
    }
}

impl<'a, 'b, PartitionKeysSet> CreateDocumentBuilder<'a, 'b, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
{
    fn if_modified_since(&self) -> Option<IfModifiedSince> {
        self.if_modified_since.clone()
    }
}

impl<'a, 'b> CreateDocumentBuilder<'a, 'b, No> {
    pub fn with_partition_keys(
        self,
        partition_keys: &'b PartitionKeys,
    ) -> CreateDocumentBuilder<'a, 'b, Yes> {
        CreateDocumentBuilder {
            partition_keys: Some(partition_keys),
            collection_client: self.collection_client,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            allow_tentative_writes: self.allow_tentative_writes,
            p_partition_keys: PhantomData {},
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
        req = crate::headers::add_optional_header(self.if_match_condition(), req);
        req = crate::headers::add_optional_header(self.if_modified_since(), req);
        req = crate::headers::add_optional_header(self.user_agent(), req);
        req = crate::headers::add_optional_header(self.activity_id(), req);
        req = crate::headers::add_optional_header(self.consistency_level(), req);
        req = crate::headers::add_optional_header(Some(self.partition_keys()), req);
        req = crate::headers::add_optional_header(Some(self.is_upsert()), req);
        req = crate::headers::add_optional_header(Some(self.indexing_directive()), req);
        req = crate::headers::add_optional_header(Some(self.allow_tentative_writes()), req);

        let serialized = serde_json::to_string(document)?;
        let req = req.body(serialized.as_bytes())?;

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
        if self.is_upsert() == IsUpsert::No && response.status() != StatusCode::CREATED {
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
